//! Uploads (F1, F3): files a Contact sends through an Upload link, each
//! labelled as one Owed document (or "something else", with a note).
//!
//! - Only photos (JPEG, PNG, HEIC/HEIF, WebP) and PDFs up to 10 MB are
//!   kept, recognised by their first bytes, never by their names. They are
//!   never opened, resized or converted, so no untrusted image is decoded.
//! - At most 60 files and 200 MB per link.
//! - Each file is encrypted (`engine::files`) and deleted 90 days after
//!   upload, or 30 days after the Firm marks it received if that's sooner.

use std::collections::BTreeSet;
use std::sync::Arc;

use sqlx::PgPool;
use time::{Duration, OffsetDateTime};

use super::clients::ClientId;
use super::firms::FirmId;
use super::upload_links::OpenLink;
use crate::engine::audit::{self, AuditEvent};
use crate::engine::clock::Clock;
use crate::engine::files::{FileError, Files};
use crate::engine::ids::FileId;

/// The largest file accepted, in bytes (F1).
pub const MAX_FILE_BYTES: usize = 10 * 1024 * 1024;
/// The most files one link accepts.
pub const MAX_FILES_PER_LINK: i64 = 60;
/// The most bytes one link accepts.
pub const MAX_BYTES_PER_LINK: i64 = 200 * 1024 * 1024;
/// How long a file is kept at most (F3).
pub const KEPT_AT_MOST_DAYS: i64 = 90;
/// How long a file is kept after it's marked received (F3).
pub const KEPT_AFTER_RECEIVED_DAYS: i64 = 30;
/// The longest Payor name or note.
pub const MAX_LABEL_CHARS: usize = 100;

/// The `ftyp` brands of HEIC/HEIF photos.
const HEIF_BRANDS: [&[u8]; 8] = [b"heic", b"heix", b"hevc", b"hevx", b"heim", b"heis", b"mif1", b"msf1"];

/// The kinds of file accepted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileKind {
    /// A JPEG photo.
    Jpeg,
    /// A PNG image.
    Png,
    /// An iPhone photo (HEIC/HEIF).
    Heic,
    /// A WebP image.
    Webp,
    /// A PDF.
    Pdf,
}

impl FileKind {
    /// The kind `bytes` start like, if it's one we accept.
    pub fn sniff(bytes: &[u8]) -> Option<Self> {
        let starts = |prefix: &[u8]| bytes.starts_with(prefix);
        if starts(&[0xFF, 0xD8, 0xFF]) {
            Some(Self::Jpeg)
        } else if starts(b"\x89PNG\r\n\x1a\n") {
            Some(Self::Png)
        } else if starts(b"%PDF-") {
            Some(Self::Pdf)
        } else if starts(b"RIFF") && bytes.get(8..12) == Some(b"WEBP") {
            Some(Self::Webp)
        } else if bytes.get(4..8) == Some(b"ftyp".as_slice())
            && bytes.get(8..12).is_some_and(|brand| HEIF_BRANDS.contains(&brand))
        {
            Some(Self::Heic)
        } else {
            None
        }
    }

    /// The stored name, which is also the media type.
    pub fn media_type(self) -> &'static str {
        match self {
            Self::Jpeg => "image/jpeg",
            Self::Png => "image/png",
            Self::Heic => "image/heic",
            Self::Webp => "image/webp",
            Self::Pdf => "application/pdf",
        }
    }

    /// The file name extension, for downloads.
    pub fn extension(self) -> &'static str {
        match self {
            Self::Jpeg => "jpg",
            Self::Png => "png",
            Self::Heic => "heic",
            Self::Webp => "webp",
            Self::Pdf => "pdf",
        }
    }

    /// Reads a stored media type.
    pub fn parse(media_type: &str) -> Option<Self> {
        [Self::Jpeg, Self::Png, Self::Heic, Self::Webp, Self::Pdf]
            .into_iter()
            .find(|kind| kind.media_type() == media_type)
    }
}

/// What a Contact says a file is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Label {
    /// One of the Period's Owed documents; a 2307 names its Payor.
    Owed {
        /// The Owed document, as listed.
        document: String,
        /// The Payor, for a 2307.
        payor: Option<String>,
    },
    /// Something not on the list, described in a short note.
    SomethingElse {
        /// What it is.
        note: String,
    },
}

impl Label {
    /// The Owed document, or "Something else".
    pub fn document(&self) -> &str {
        match self {
            Self::Owed { document, .. } => document,
            Self::SomethingElse { .. } => "Something else",
        }
    }

    /// The Payor or the note, if any.
    pub fn detail(&self) -> Option<&str> {
        match self {
            Self::Owed { payor, .. } => payor.as_deref(),
            Self::SomethingElse { note } => Some(note),
        }
    }
}

/// Whether an Owed document is a 2307, which must name its Payor.
pub fn is_2307(document: &str) -> bool {
    document.contains("2307")
}

/// Why an upload wasn't kept, in words for the Contact.
#[derive(Debug, thiserror::Error)]
pub enum UploadError {
    /// Not a photo or PDF.
    #[error("only photos and PDF files can be uploaded")]
    NotAllowedType,
    /// Over 10 MB, or empty.
    #[error("each file must be under 10 MB")]
    TooBig,
    /// The link has taken all the files it can.
    #[error("this link can't take any more files")]
    LinkFull,
    /// No such Upload for this Client, or its file was deleted.
    #[error("upload not found")]
    NotFound,
    /// The label is missing or wrong.
    #[error("{0}")]
    Invalid(LabelProblem),
    /// Storing the file failed.
    #[error(transparent)]
    File(#[from] FileError),
    /// The database failed.
    #[error(transparent)]
    Database(#[from] sqlx::Error),
}

/// What's wrong with a label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum LabelProblem {
    /// It doesn't name one of the Period's Owed documents.
    #[error("choose what this file is")]
    NotOwed,
    /// A 2307 without its Payor.
    #[error("say which business the 2307 is from")]
    NoPayor,
    /// "Something else" without a note.
    #[error("say what this file is")]
    NoNote,
    /// A Payor or note longer than one short line.
    #[error("keep it to one short line")]
    TooLong,
}

/// One Upload, as the Firm sees it.
#[derive(Debug, Clone)]
pub struct Upload {
    /// The stored file's ID, which is also the Upload's.
    pub id: FileId,
    /// The collection Period's key.
    pub period: String,
    /// What the Contact said it is.
    pub label: Label,
    /// Its kind.
    pub kind: FileKind,
    /// Its size in bytes.
    pub size_bytes: i32,
    /// When it arrived.
    pub uploaded_at: OffsetDateTime,
    /// When the Firm marked it received, if it has.
    pub received_at: Option<OffsetDateTime>,
    /// Whether the file itself has been deleted.
    pub deleted: bool,
    /// When the file is deleted, at the latest.
    pub delete_by: OffsetDateTime,
}

/// A file to download.
#[derive(Debug)]
pub struct Download {
    /// A name made by Kompleto, never the Contact's own.
    pub name: String,
    /// Its kind.
    pub kind: FileKind,
    /// The decrypted contents.
    pub bytes: Vec<u8>,
}

/// Uploads, stored encrypted.
#[derive(Clone)]
pub struct Uploads {
    pool: PgPool,
    clock: Arc<dyn Clock>,
    files: Files,
}

impl Uploads {
    /// Uploads kept in `files`.
    pub fn new(pool: PgPool, clock: Arc<dyn Clock>, files: Files) -> Self {
        Self { pool, clock, files }
    }

    /// Keeps `bytes`, labelled `label`, sent through `link`. `owed` is the
    /// Period's Owed documents; the label must name one of them.
    pub async fn add(
        &self,
        link: &OpenLink,
        owed: &[String],
        label: Label,
        bytes: &[u8],
    ) -> Result<FileId, UploadError> {
        let label = checked(label, owed)?;
        if bytes.is_empty() || bytes.len() > MAX_FILE_BYTES {
            return Err(UploadError::TooBig);
        }
        let kind = FileKind::sniff(bytes).ok_or(UploadError::NotAllowedType)?;
        let size = i32::try_from(bytes.len()).map_err(|_| UploadError::TooBig)?;
        let mut tx = self.pool.begin().await?;
        // One upload at a time per link, so two can't both squeeze under the limits.
        sqlx::query!(
            "select id from upload_links where id = $1 and firm_id = $2 for update",
            link.id.as_uuid(),
            link.firm.as_uuid(),
        )
        .fetch_one(&mut *tx)
        .await?;
        let used = sqlx::query!(
            r#"select count(*) as "files!", coalesce(sum(size_bytes), 0)::bigint as "bytes!"
               from uploads where link_id = $1 and firm_id = $2"#,
            link.id.as_uuid(),
            link.firm.as_uuid(),
        )
        .fetch_one(&mut *tx)
        .await?;
        if used.files >= MAX_FILES_PER_LINK || used.bytes + i64::from(size) > MAX_BYTES_PER_LINK {
            return Err(UploadError::LinkFull);
        }
        let now = self.clock.now();
        let id = self
            .files
            .save(link.firm, bytes, now + Duration::days(KEPT_AT_MOST_DAYS))
            .await?;
        let (document, payor, note) = match &label {
            Label::Owed { document, payor } => (Some(document.as_str()), payor.as_deref(), None),
            Label::SomethingElse { note } => (None, None, Some(note.as_str())),
        };
        let recorded = sqlx::query!(
            "insert into uploads
                 (id, firm_id, client_id, link_id, period, owed_document, payor, note, media_type, size_bytes, uploaded_at)
             values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
            id.as_uuid(),
            link.firm.as_uuid(),
            link.client.as_uuid(),
            link.id.as_uuid(),
            link.period,
            document,
            payor,
            note,
            kind.media_type(),
            size,
            now,
        )
        .execute(&mut *tx)
        .await;
        match recorded {
            Ok(_) => tx.commit().await?,
            Err(err) => {
                // A file nobody can see or mark received goes now, not in 90 days.
                if let Err(discard) = self.files.discard(link.firm, id).await {
                    tracing::error!(file = %id, %discard, "discarding an unrecorded upload failed");
                }
                return Err(err.into());
            }
        }
        Ok(id)
    }

    /// A Client's Uploads for one Period, oldest first.
    pub async fn for_period(&self, firm: FirmId, client: ClientId, period: &str) -> Result<Vec<Upload>, UploadError> {
        Ok(self
            .for_client(firm, client)
            .await?
            .into_iter()
            .filter(|upload| upload.period == period)
            .collect())
    }

    /// All of a Client's Uploads, oldest first.
    pub async fn for_client(&self, firm: FirmId, client: ClientId) -> Result<Vec<Upload>, UploadError> {
        let rows = sqlx::query!(
            r#"select u.id, u.period, u.owed_document, u.payor, u.note, u.media_type, u.size_bytes,
                      u.uploaded_at, u.received_at, f.deleted_at, f.delete_by
               from uploads u join stored_files f on f.id = u.id and f.tenant_id = u.firm_id
               where u.firm_id = $1 and u.client_id = $2
               order by u.uploaded_at, u.id"#,
            firm.as_uuid(),
            client.as_uuid(),
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows
            .into_iter()
            .map(|row| Upload {
                id: row.id.into(),
                period: row.period,
                label: match row.owed_document {
                    Some(document) => Label::Owed {
                        document,
                        payor: row.payor,
                    },
                    None => Label::SomethingElse {
                        note: row.note.unwrap_or_default(),
                    },
                },
                kind: FileKind::parse(&row.media_type).unwrap_or(FileKind::Pdf),
                size_bytes: row.size_bytes,
                uploaded_at: row.uploaded_at,
                received_at: row.received_at,
                deleted: row.deleted_at.is_some(),
                delete_by: row.delete_by,
            })
            .collect())
    }

    /// Upload `id` of a Client called `business_name`, decrypted, named like
    /// `Nenas-Store_2026-10_Bank-statements_1.jpg`.
    pub async fn download(
        &self,
        firm: FirmId,
        client: ClientId,
        id: FileId,
        business_name: &str,
    ) -> Result<Download, UploadError> {
        let uploads = self.for_client(firm, client).await?;
        let position = uploads
            .iter()
            .position(|upload| upload.id == id)
            .ok_or(UploadError::NotFound)?;
        let upload = &uploads[position];
        let number = uploads[..=position]
            .iter()
            .filter(|other| other.period == upload.period && other.label.document() == upload.label.document())
            .count();
        let bytes = self.files.read(firm, id).await.map_err(|err| match err {
            FileError::NotFound => UploadError::NotFound,
            other => other.into(),
        })?;
        Ok(Download {
            name: format!(
                "{}_{}_{}_{number}.{}",
                slug(business_name),
                upload.period,
                slug(upload.label.document()),
                upload.kind.extension()
            ),
            kind: upload.kind,
            bytes,
        })
    }

    /// Marks Upload `id` received by `by`; its file is then deleted within
    /// 30 days (F3). Marking it again changes nothing.
    pub async fn mark_received(&self, firm: FirmId, client: ClientId, id: FileId, by: &str) -> Result<(), UploadError> {
        let received = sqlx::query_scalar!(
            "select received_at from uploads where firm_id = $1 and client_id = $2 and id = $3",
            firm.as_uuid(),
            client.as_uuid(),
            id.as_uuid(),
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or(UploadError::NotFound)?;
        if received.is_some() {
            return Ok(());
        }
        let now = self.clock.now();
        // The deadline moves first: if recording fails, the file only goes sooner.
        match self
            .files
            .delete_no_later_than(firm, id, now + Duration::days(KEPT_AFTER_RECEIVED_DAYS))
            .await
        {
            Ok(()) | Err(FileError::NotFound) => {}
            Err(err) => return Err(err.into()),
        }
        let mut tx = self.pool.begin().await?;
        sqlx::query!(
            "update uploads set received_at = $4, received_by = $5
             where firm_id = $1 and client_id = $2 and id = $3 and received_at is null",
            firm.as_uuid(),
            client.as_uuid(),
            id.as_uuid(),
            now,
            by,
        )
        .execute(&mut *tx)
        .await?;
        audit::record(
            &mut *tx,
            &AuditEvent {
                tenant: firm,
                at: now,
                actor: by,
                action: "upload.received",
                subject_type: "upload",
                subject_id: id.as_uuid(),
                details: serde_json::json!({}),
            },
        )
        .await?;
        tx.commit().await?;
        Ok(())
    }
}

/// The Owed documents with at least one Upload for the Client's `period`.
pub async fn uploaded_documents(
    pool: &PgPool,
    firm: FirmId,
    client: ClientId,
    period: &str,
) -> Result<BTreeSet<String>, sqlx::Error> {
    let documents = sqlx::query_scalar!(
        r#"select distinct owed_document as "document!" from uploads
           where firm_id = $1 and client_id = $2 and period = $3 and owed_document is not null"#,
        firm.as_uuid(),
        client.as_uuid(),
        period,
    )
    .fetch_all(pool)
    .await?;
    Ok(documents.into_iter().collect())
}

/// `label`, trimmed, if it names one of `owed` (with a Payor for a 2307),
/// or is "something else" with a note.
fn checked(label: Label, owed: &[String]) -> Result<Label, UploadError> {
    let short = |text: &str, missing: LabelProblem| {
        let text = text.trim();
        if text.is_empty() {
            Err(UploadError::Invalid(missing))
        } else if text.chars().count() > MAX_LABEL_CHARS || text.contains(['\n', '\r']) {
            Err(UploadError::Invalid(LabelProblem::TooLong))
        } else {
            Ok(text.to_owned())
        }
    };
    match label {
        Label::Owed { document, payor } => {
            if !owed.contains(&document) {
                return Err(UploadError::Invalid(LabelProblem::NotOwed));
            }
            let payor = if is_2307(&document) {
                Some(short(payor.as_deref().unwrap_or(""), LabelProblem::NoPayor)?)
            } else {
                None
            };
            Ok(Label::Owed { document, payor })
        }
        Label::SomethingElse { note } => Ok(Label::SomethingElse {
            note: short(&note, LabelProblem::NoNote)?,
        }),
    }
}

/// `text` as a file-name part: ASCII letters and digits, other runs as one
/// `-`, apostrophes dropped, at most 40 characters.
fn slug(text: &str) -> String {
    let mut slug = String::new();
    for c in text.chars().filter(|c| !matches!(c, '\'' | '’')) {
        if c.is_ascii_alphanumeric() {
            slug.push(c);
        } else if !slug.is_empty() && !slug.ends_with('-') {
            slug.push('-');
        }
    }
    slug.truncate(40);
    slug.trim_end_matches('-').to_owned()
}
