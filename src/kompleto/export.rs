//! A Firm's data export (I1): one zip of CSV files, one per kind of
//! record, plus the uploaded files Kompleto still holds. Every query is
//! scoped by the Firm (A2).
//!
//! - Times are written in Manila time (RFC 3339).
//! - A cell that a spreadsheet would run as a formula (starting with `=`,
//!   `+`, `-`, `@`, a tab or a return) gets a leading `'`: Contacts' text
//!   is untrusted.

use std::collections::HashSet;
use std::io::{Seek, Write};

use sqlx::PgPool;
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;
use zip::ZipWriter;
use zip::write::SimpleFileOptions;

use super::activity;
use super::clients::Clients;
use super::firms::FirmId;
use super::schedule::MANILA;
use super::uploads::{UploadError, Uploads};

/// Why an export failed.
#[derive(Debug, thiserror::Error)]
pub enum ExportError {
    /// The database failed.
    #[error(transparent)]
    Database(#[from] sqlx::Error),
    /// Reading a Client or an Upload failed.
    #[error("{0}")]
    Records(String),
    /// Writing the zip failed.
    #[error(transparent)]
    Zip(#[from] zip::result::ZipError),
    /// Writing a CSV or the file failed.
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl From<UploadError> for ExportError {
    fn from(err: UploadError) -> Self {
        Self::Records(err.to_string())
    }
}

impl From<super::clients::ClientError> for ExportError {
    fn from(err: super::clients::ClientError) -> Self {
        Self::Records(err.to_string())
    }
}

/// Writes everything `firm` has to `out` as a zip.
pub async fn write_zip<W: Write + Seek>(
    pool: &PgPool,
    clients: &Clients,
    uploads: &Uploads,
    firm: FirmId,
    out: W,
) -> Result<W, ExportError> {
    let mut zip = ZipWriter::new(out);
    let options = SimpleFileOptions::default();
    let add = |zip: &mut ZipWriter<W>, name: &str, bytes: &[u8]| -> Result<(), ExportError> {
        zip.start_file(name, options)?;
        zip.write_all(bytes)?;
        Ok(())
    };

    let list = clients.list(firm).await?;
    let mut rows = Vec::new();
    for client in &list {
        let d = &client.details;
        rows.push(vec![
            d.business_name.clone(),
            d.contact_first_name.clone(),
            d.email.clone(),
            d.language.as_str().to_owned(),
            d.forms.iter().map(|form| form.code()).collect::<Vec<_>>().join(" "),
            client
                .cut_off_offset_days
                .map(|days| days.to_string())
                .unwrap_or_default(),
            client
                .reminder_days
                .days()
                .iter()
                .map(u8::to_string)
                .collect::<Vec<_>>()
                .join(" "),
            client.paused.to_string(),
        ]);
    }
    let header = [
        "business_name",
        "contact_first_name",
        "email",
        "language",
        "forms",
        "cut_off_offset_days",
        "reminder_days",
        "paused",
    ];
    add(&mut zip, "clients.csv", &csv(&header, rows)?)?;

    let rows = sqlx::query!(
        "select c.business_name, p.name from payors p
         join clients c on c.id = p.client_id and c.firm_id = p.firm_id
         where p.firm_id = $1 order by lower(c.business_name), lower(p.name)",
        firm.as_uuid(),
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|row| vec![row.business_name, row.name])
    .collect();
    add(&mut zip, "payors.csv", &csv(&["client", "payor"], rows)?)?;

    let rows = sqlx::query!(
        r#"select c.business_name as "client!", 'removed default item' as "change!", r.item_key as "item!", '' as "recurs!"
           from client_removed_items r join clients c on c.id = r.client_id and c.firm_id = r.firm_id
           where r.firm_id = $1
           union all
           select c.business_name, 'added item', e.label, e.recurs
           from client_extra_items e join clients c on c.id = e.client_id and c.firm_id = e.firm_id
           where e.firm_id = $1"#,
        firm.as_uuid(),
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|row| vec![row.client, row.change, row.item, row.recurs])
    .collect();
    add(
        &mut zip,
        "checklist.csv",
        &csv(&["client", "change", "item", "recurs"], rows)?,
    )?;

    let rows = sqlx::query!(
        "select c.business_name, d.form, d.period, d.due_date from due_date_overrides d
         join clients c on c.id = d.client_id and c.firm_id = d.firm_id
         where d.firm_id = $1 order by lower(c.business_name), d.period, d.form",
        firm.as_uuid(),
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|row| vec![row.business_name, row.form, row.period, row.due_date.to_string()])
    .collect();
    add(
        &mut zip,
        "due_dates.csv",
        &csv(&["client", "form", "period", "due_date"], rows)?,
    )?;

    let rows = sqlx::query!(
        r#"select c.business_name as "client?", m.to_address, m.subject, m.body_text, m.status,
                  m.created_at, m.approved_by, m.sent_at
           from outbox_messages m
           left join scheduled_drafts s on s.message_id = m.id and s.firm_id = m.tenant_id
           left join clients c on c.id = s.client_id and c.firm_id = s.firm_id
           where m.tenant_id = $1 order by m.created_at, m.id"#,
        firm.as_uuid(),
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|row| {
        vec![
            row.client.unwrap_or_default(),
            row.to_address,
            row.subject,
            row.body_text,
            row.status,
            when(row.created_at),
            row.approved_by.unwrap_or_default(),
            row.sent_at.map(when).unwrap_or_default(),
        ]
    })
    .collect();
    let header = [
        "client",
        "to",
        "subject",
        "body",
        "status",
        "created_at",
        "approved_by",
        "sent_at",
    ];
    add(&mut zip, "messages.csv", &csv(&header, rows)?)?;

    // Uploads, and each file still held under the name a download gets.
    let mut rows = Vec::new();
    let mut names = HashSet::new();
    for client in &list {
        let listed = uploads.for_client(firm, client.id).await?;
        for (position, upload) in listed.iter().enumerate() {
            let download = if upload.deleted {
                None
            } else {
                match uploads
                    .download_listed(firm, &listed, position, &client.details.business_name)
                    .await
                {
                    Ok(download) => Some(download),
                    // Deleted by the sweep a moment ago: listed, not included.
                    Err(UploadError::NotFound) => None,
                    Err(err) => return Err(err.into()),
                }
            };
            let file = match download {
                Some(download) => {
                    let name = unique(&mut names, &format!("files/{}", download.name));
                    add(&mut zip, &name, &download.bytes)?;
                    name
                }
                None => String::new(),
            };
            rows.push(vec![
                client.details.business_name.clone(),
                upload.period.clone(),
                upload.label.document().to_owned(),
                upload.label.detail().unwrap_or_default().to_owned(),
                upload.kind.media_type().to_owned(),
                upload.size_bytes.to_string(),
                when(upload.uploaded_at),
                upload.received_at.map(when).unwrap_or_default(),
                file,
            ]);
        }
    }
    let header = [
        "client",
        "period",
        "document",
        "payor_or_note",
        "type",
        "size_bytes",
        "uploaded_at",
        "received_at",
        "file",
    ];
    add(&mut zip, "uploads.csv", &csv(&header, rows)?)?;

    let rows = sqlx::query!(
        "select c.business_name, r.period, r.from_address, r.subject, r.body_text, r.automatic, r.signed, r.received_at
         from replies r join clients c on c.id = r.client_id and c.firm_id = r.firm_id
         where r.firm_id = $1 order by r.received_at, r.id",
        firm.as_uuid(),
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|row| {
        vec![
            row.business_name,
            row.period,
            row.from_address,
            row.subject.unwrap_or_default(),
            row.body_text.unwrap_or_default(),
            row.automatic.to_string(),
            row.signed.to_string(),
            when(row.received_at),
        ]
    })
    .collect();
    let header = [
        "client",
        "period",
        "from",
        "subject",
        "text",
        "automatic",
        "signed",
        "received_at",
    ];
    add(&mut zip, "replies.csv", &csv(&header, rows)?)?;

    let rows = sqlx::query!(
        "select c.business_name, b.address, b.kind, b.reason, b.at from bounces b
         join clients c on c.id = b.client_id and c.firm_id = b.firm_id
         where b.firm_id = $1 order by b.at",
        firm.as_uuid(),
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|row| {
        vec![
            row.business_name,
            row.address,
            row.kind,
            row.reason.unwrap_or_default(),
            when(row.at),
        ]
    })
    .collect();
    add(
        &mut zip,
        "bounces.csv",
        &csv(&["client", "address", "kind", "reason", "at"], rows)?,
    )?;

    let rows = activity::since(pool, firm, OffsetDateTime::UNIX_EPOCH)
        .await?
        .into_iter()
        .rev()
        .map(|entry| {
            vec![
                when(entry.at),
                entry.actor,
                entry.action,
                entry.subject_type,
                entry.subject_id.to_string(),
            ]
        })
        .collect();
    let header = ["at", "actor", "action", "record_type", "record_id"];
    add(&mut zip, "activity.csv", &csv(&header, rows)?)?;

    Ok(zip.finish()?)
}

/// `name`, or with " (2)", " (3)"… before its extension if already taken:
/// two Clients' names can make the same file name, and a zip can't hold two
/// files with one name.
fn unique(taken: &mut HashSet<String>, name: &str) -> String {
    let (stem, extension) = name.rsplit_once('.').unwrap_or((name, ""));
    let mut candidate = name.to_owned();
    let mut number = 1;
    while !taken.insert(candidate.clone()) {
        number += 1;
        candidate = format!("{stem} ({number}).{extension}");
    }
    candidate
}

/// `at` in Manila time.
fn when(at: OffsetDateTime) -> String {
    at.to_offset(MANILA).format(&Rfc3339).unwrap_or_default()
}

/// A CSV file with `header` and `rows`, every cell safe to open in a
/// spreadsheet.
fn csv(header: &[&str], rows: Vec<Vec<String>>) -> Result<Vec<u8>, std::io::Error> {
    let mut writer = csv::Writer::from_writer(Vec::new());
    writer.write_record(header)?;
    for row in rows {
        writer.write_record(row.iter().map(|cell| inert(cell)))?;
    }
    writer.into_inner().map_err(|err| err.into_error())
}

/// `cell`, with a leading `'` if a spreadsheet would read it as a formula.
fn inert(cell: &str) -> String {
    if cell.starts_with(['=', '+', '-', '@', '\t', '\r']) {
        format!("'{cell}")
    } else {
        cell.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn i1_cells_a_spreadsheet_would_run_get_a_quote() {
        assert_eq!(inert("=HYPERLINK(\"x\")"), "'=HYPERLINK(\"x\")");
        assert_eq!(inert("-2+3"), "'-2+3");
        assert_eq!(inert("@SUM(A1)"), "'@SUM(A1)");
        assert_eq!(inert("Nena's Store"), "Nena's Store");
    }
}
