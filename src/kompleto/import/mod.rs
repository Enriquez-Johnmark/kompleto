//! Importing Clients from a CSV or Excel file (B2). Reading turns the file
//! into rows; checking sorts them into valid and invalid with reasons.
//! Nothing is stored until the bookkeeper confirms.

use std::collections::HashMap;

use super::clients::{ClientInput, NewClient};

/// The most rows one file may hold.
pub const MAX_ROWS: usize = 500;
/// The largest file accepted, in bytes.
pub const MAX_BYTES: usize = 2 * 1024 * 1024;

/// The columns an import file must have, as named in the template.
pub const COLUMNS: [&str; 5] = ["Business name", "Contact first name", "Email", "Language", "Forms"];

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
    let (mut counted_to, mut line) = (0, 1);
    for record in reader.records() {
        let record = record.map_err(|err| ImportError::Unreadable(err.to_string()))?;
        // The csv crate's line count skips blank lines; count them, as a
        // spreadsheet shows them.
        // Its position can point at blank lines before it.
        let start = record.position().map_or(0, |p| usize::try_from(p.byte()).unwrap_or(0));
        let start = start
            + text
                .as_bytes()
                .iter()
                .skip(start)
                .take_while(|b| matches!(b, b'\r' | b'\n'))
                .count();
        let newlines = text.as_bytes().get(counted_to..start).unwrap_or_default();
        line += newlines.iter().filter(|&&b| b == b'\n').count();
        counted_to = start;
        records.push((line, record.iter().map(str::to_owned).collect()));
    }
    rows_from_records(records)
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
    let mut positions = [0; COLUMNS.len()];
    let mut missing = Vec::new();
    for (position, column) in positions.iter_mut().zip(COLUMNS) {
        match header.iter().position(|name| *name == squash(column)) {
            Some(found) => *position = found,
            None => missing.push(column),
        }
    }
    if !missing.is_empty() {
        return Err(ImportError::MissingColumns(missing));
    }
    let cell = |cells: &[String], column: usize| cells.get(positions[column]).map_or("", |c| c.trim()).to_owned();
    let mut rows = Vec::new();
    for (row, cells) in records.filter(|(_, cells)| cells.iter().any(|c| !c.trim().is_empty())) {
        if rows.len() == MAX_ROWS {
            return Err(ImportError::TooManyRows);
        }
        let forms = cell(&cells, 4);
        rows.push(ImportRow {
            row,
            input: ClientInput {
                business_name: cell(&cells, 0),
                contact_first_name: cell(&cells, 1),
                email: cell(&cells, 2),
                language: cell(&cells, 3),
                forms: forms
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
