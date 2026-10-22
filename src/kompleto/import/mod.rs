//! Importing Clients from a CSV or Excel file (B2), including the contact
//! and customer lists Xero and QuickBooks Online export (B5). Reading turns
//! the file into rows; checking sorts them into valid and invalid with
//! reasons. Nothing is stored until the bookkeeper confirms.

pub mod xlsx;

use std::collections::HashMap;

use super::clients::{ClientInput, NewClient};

/// The most rows one file may hold.
pub const MAX_ROWS: usize = 500;
/// The largest file accepted, in bytes.
pub const MAX_BYTES: usize = 2 * 1024 * 1024;

/// The columns of Kompleto's own template.
pub const COLUMNS: [&str; 5] = ["Business name", "Contact first name", "Email", "Language", "Forms"];

/// Header names, squashed (lowercase letters and digits only), that hold
/// each field, best first. Kompleto's template, Xero's contact export
/// (`*ContactName`, `EmailAddress`, `FirstName`) and QuickBooks Online's
/// customer list (`Customer` or `Name`, `Company`, `Email`). Any other
/// column, such as a TIN, is never read.
const BUSINESS: [&str; 7] = [
    "businessname",
    "company",
    "companyname",
    "contactname",
    "customer",
    "displayname",
    "name",
];
const FIRST_NAME: [&str; 2] = ["contactfirstname", "firstname"];
/// A person's whole name, whose first word is the contact's first name when
/// no column gives it, as in a QuickBooks customer list.
const PERSON: [&str; 3] = ["fullname", "customer", "name"];
const EMAIL: [&str; 3] = ["email", "emailaddress", "primaryemail"];
const LANGUAGE: [&str; 1] = ["language"];
const FORMS: [&str; 2] = ["forms", "birforms"];

/// What fills a row's empty Language and Forms, chosen for the whole file,
/// e.g. for a Xero or QuickBooks export, which has neither.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BatchDefaults {
    /// `taglish` or `english`.
    pub language: String,
    /// BIR form codes.
    pub forms: Vec<String>,
}

/// `rows` with `defaults` in every empty Language and Forms; values in the
/// file win.
pub fn with_defaults(mut rows: Vec<ImportRow>, defaults: &BatchDefaults) -> Vec<ImportRow> {
    for row in &mut rows {
        if row.input.language.trim().is_empty() {
            row.input.language.clone_from(&defaults.language);
        }
        if row.input.forms.is_empty() {
            row.input.forms.clone_from(&defaults.forms);
        }
    }
    rows
}

/// One data row, numbered as in a spreadsheet (the header is row 1).
#[derive(Debug, Clone)]
pub struct ImportRow {
    /// The spreadsheet row number.
    pub row: usize,
    /// Its cells.
    pub input: ClientInput,
}

/// Why a whole file was refused.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ImportError {
    /// Larger than [`MAX_BYTES`].
    #[error("the file is larger than 2 MB")]
    TooLarge,
    /// More than [`MAX_ROWS`] rows.
    #[error("the file has more than {MAX_ROWS} rows")]
    TooManyRows,
    /// No data rows.
    #[error("the file has no clients in it")]
    Empty,
    /// Required columns are missing from the header row.
    #[error("the first row needs these columns: {}", .0.join(", "))]
    MissingColumns(Vec<&'static str>),
    /// Not a readable CSV or Excel file.
    #[error("couldn't read the file: {0}")]
    Unreadable(String),
}

/// The result of checking an import file's rows.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Preview {
    /// Rows that will be added, with their row numbers.
    pub valid: Vec<(usize, NewClient)>,
    /// Rows that won't, with every reason.
    pub invalid: Vec<(usize, Vec<String>)>,
}

/// Reads a CSV file: UTF-8 (with or without a byte-order mark), or
/// Windows-1252 as older Excel saves it.
pub fn read_csv(bytes: &[u8]) -> Result<Vec<ImportRow>, ImportError> {
    if bytes.len() > MAX_BYTES {
        return Err(ImportError::TooLarge);
    }
    let bytes = bytes.strip_prefix(b"\xef\xbb\xbf").unwrap_or(bytes);
    let text = match std::str::from_utf8(bytes) {
        Ok(text) => text.to_owned(),
        Err(_) => encoding_rs::WINDOWS_1252
            .decode_without_bom_handling(bytes)
            .0
            .into_owned(),
    };
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_reader(text.as_bytes());
    let mut records = Vec::new();
    // Rows as a spreadsheet numbers them: blank lines count (the csv crate
    // skips them), line breaks inside a quoted cell don't, and \r, \n and
    // \r\n all end a line.
    let (mut counted_to, mut line, mut breaks_in_previous) = (0, 1, 0);
    for record in reader.records() {
        let record = record.map_err(|err| ImportError::Unreadable(err.to_string()))?;
        // Its position can point at blank lines before it.
        let start = record.position().map_or(0, |p| usize::try_from(p.byte()).unwrap_or(0));
        let bytes = text.as_bytes();
        let start = start
            + bytes
                .iter()
                .skip(start)
                .take_while(|b| matches!(b, b'\r' | b'\n'))
                .count();
        line += line_breaks(bytes.get(counted_to..start).unwrap_or_default()).saturating_sub(breaks_in_previous);
        counted_to = start;
        breaks_in_previous = record.iter().map(|cell| line_breaks(cell.as_bytes())).sum();
        records.push((line, record.iter().map(str::to_owned).collect()));
    }
    rows_from_records(records)
}

/// Line breaks in `bytes`, counting `\r\n` once.
fn line_breaks(bytes: &[u8]) -> usize {
    let newlines = bytes.iter().filter(|&&b| b == b'\n').count();
    let lone_returns = bytes
        .iter()
        .enumerate()
        .filter(|&(i, &b)| b == b'\r' && bytes.get(i + 1) != Some(&b'\n'))
        .count();
    newlines + lone_returns
}

/// Turns a sheet's cells, header row first, into rows.
pub fn rows_from_cells(cells: Vec<Vec<String>>) -> Result<Vec<ImportRow>, ImportError> {
    rows_from_records(cells.into_iter().enumerate().map(|(i, row)| (i + 1, row)).collect())
}

/// Rows numbered as the bookkeeper sees them, header first.
fn rows_from_records(records: Vec<(usize, Vec<String>)>) -> Result<Vec<ImportRow>, ImportError> {
    let mut records = records.into_iter();
    let header = records.next().map(|(_, cells)| cells).unwrap_or_default();
    let header: Vec<String> = header.iter().map(|name| squash(name)).collect();
    // Every column with one of `names`, in the order of `names`.
    let find = |names: &[&str]| -> Vec<usize> {
        names
            .iter()
            .filter_map(|name| header.iter().position(|found| found == name))
            .collect()
    };
    let (business, first_name, person, email) = (find(&BUSINESS), find(&FIRST_NAME), find(&PERSON), find(&EMAIL));
    let (language, forms) = (find(&LANGUAGE), find(&FORMS));
    let mut missing = Vec::new();
    if business.is_empty() {
        missing.push(COLUMNS[0]);
    }
    if email.is_empty() {
        missing.push(COLUMNS[2]);
    }
    if !missing.is_empty() {
        return Err(ImportError::MissingColumns(missing));
    }
    let mut rows = Vec::new();
    for (row, cells) in records.filter(|(_, cells)| cells.iter().any(|c| !c.trim().is_empty())) {
        if rows.len() == MAX_ROWS {
            return Err(ImportError::TooManyRows);
        }
        // The first non-empty cell among `columns`, and which column it was.
        let first = |columns: &[usize]| -> (Option<usize>, String) {
            columns
                .iter()
                .map(|&column| (column, cells.get(column).map_or("", |c| c.trim())))
                .find(|(_, value)| !value.is_empty())
                .map_or((None, String::new()), |(column, value)| {
                    (Some(column), value.to_owned())
                })
        };
        let (business_column, business_name) = first(&business);
        let mut contact_first_name = first(&first_name).1;
        if contact_first_name.is_empty() {
            // A person's name beside the company name, or, when the business
            // is listed under the person's name, that name.
            let others: Vec<usize> = person.iter().copied().filter(|&c| Some(c) != business_column).collect();
            let mut whole = first(&others).1;
            if whole.is_empty() && business_column.is_some_and(|c| person.contains(&c)) {
                whole.clone_from(&business_name);
            }
            contact_first_name = whole.split_whitespace().next().unwrap_or("").to_owned();
        }
        rows.push(ImportRow {
            row,
            input: ClientInput {
                business_name: business_name.clone(),
                contact_first_name,
                email: first(&email).1,
                language: first(&language).1,
                forms: first(&forms)
                    .1
                    .split([',', ';', '/', '\n'])
                    .map(|f| f.trim().to_owned())
                    .filter(|f| !f.is_empty())
                    .collect(),
            },
        });
    }
    if rows.is_empty() {
        return Err(ImportError::Empty);
    }
    Ok(rows)
}

/// A header name reduced to lowercase letters and digits.
fn squash(name: &str) -> String {
    name.chars()
        .filter(char::is_ascii_alphanumeric)
        .map(|c| c.to_ascii_lowercase())
        .collect()
}

/// Checks every row; `existing` holds the Firm's current business names.
pub fn preview(rows: &[ImportRow], existing: &[String]) -> Preview {
    let existing: Vec<String> = existing.iter().map(|name| name.trim().to_lowercase()).collect();
    let mut first_row_of: HashMap<String, usize> = HashMap::new();
    let mut preview = Preview::default();
    for ImportRow { row, input } in rows {
        let parsed = NewClient::parse(input);
        let mut reasons: Vec<String> = parsed
            .as_ref()
            .err()
            .into_iter()
            .flatten()
            .map(ToString::to_string)
            .collect();
        let name = input.business_name.trim();
        let key = name.to_lowercase();
        if existing.contains(&key) {
            reasons.push(format!("Business name: {name:?} is already a client"));
        } else if let Some(first) = first_row_of.get(&key).filter(|_| !key.is_empty()) {
            reasons.push(format!("Business name: also on row {first} of this file"));
        } else {
            first_row_of.insert(key, *row);
        }
        match parsed {
            Ok(client) if reasons.is_empty() => preview.valid.push((*row, client)),
            _ => preview.invalid.push((*row, reasons)),
        }
    }
    preview
}

#[cfg(test)]
mod tests;
