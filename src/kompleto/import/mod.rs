//! Importing Clients from a CSV or Excel file (B2), including the contact
//! and customer lists Xero and QuickBooks Online export (B5). Reading turns
//! the file into rows; checking sorts them into valid and invalid with
//! reasons. Nothing is stored until the bookkeeper confirms.

pub mod payors;
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
/// Columns that hold a company's name; a row with it blank falls back to the
/// customer's own name, as QuickBooks leaves Company blank for individuals.
const COMPANY: [&str; 2] = ["company", "companyname"];
const FIRST_NAME: [&str; 2] = ["contactfirstname", "firstname"];
/// A person's whole name beside the company's, as in a QuickBooks customer
/// list, whose first word is the contact's first name.
const PERSON: [&str; 3] = ["fullname", "customer", "name"];
const EMAIL: [&str; 3] = ["email", "emailaddress", "primaryemail"];
const LANGUAGE: [&str; 1] = ["language"];
const FORMS: [&str; 2] = ["forms", "birforms"];

/// The Language and Forms for a file without those columns, such as a Xero
/// or QuickBooks export, chosen for the whole file.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BatchDefaults {
    /// `taglish` or `english`.
    pub language: String,
    /// BIR form codes.
    pub forms: Vec<String>,
}

/// A file's rows, and which of the optional columns it has.
#[derive(Debug, Clone)]
pub struct Sheet {
    rows: Vec<ImportRow>,
    has_language: bool,
    has_forms: bool,
}

impl Sheet {
    /// The rows. With `defaults`, they fill the Language and Forms columns
    /// the file lacks; a column the file has keeps its cells, blank or not.
    /// Without `defaults`, both columns are required.
    pub fn rows(self, defaults: Option<&BatchDefaults>) -> Result<Vec<ImportRow>, ImportError> {
        let Some(defaults) = defaults else {
            let missing: Vec<&'static str> = [(self.has_language, COLUMNS[3]), (self.has_forms, COLUMNS[4])]
                .into_iter()
                .filter(|(has, _)| !has)
                .map(|(_, column)| column)
                .collect();
            if missing.is_empty() {
                return Ok(self.rows);
            }
            return Err(ImportError::MissingColumns(missing));
        };
        let mut rows = self.rows;
        for row in &mut rows {
            if !self.has_language {
                row.input.language.clone_from(&defaults.language);
            }
            if !self.has_forms {
                row.input.forms.clone_from(&defaults.forms);
            }
        }
        Ok(rows)
    }
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
pub fn read_csv(bytes: &[u8]) -> Result<Sheet, ImportError> {
    rows_from_records(csv_records(bytes)?)
}

/// A CSV file's records, each with its row number as a spreadsheet shows it.
pub fn csv_records(bytes: &[u8]) -> Result<Vec<(usize, Vec<String>)>, ImportError> {
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
    Ok(records)
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
pub fn rows_from_cells(cells: Vec<Vec<String>>) -> Result<Sheet, ImportError> {
    rows_from_records(numbered(cells))
}

/// A sheet's rows numbered from 1, header first.
pub fn numbered(cells: Vec<Vec<String>>) -> Vec<(usize, Vec<String>)> {
    cells.into_iter().enumerate().map(|(i, row)| (i + 1, row)).collect()
}

/// Rows numbered as the bookkeeper sees them, header first.
fn rows_from_records(records: Vec<(usize, Vec<String>)>) -> Result<Sheet, ImportError> {
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
    // The business name comes from one column: the best one the file has.
    let business = find(&BUSINESS).first().copied();
    let is_company = |column: usize| COMPANY.iter().any(|name| header[column] == *name);
    let person: Vec<usize> = find(&PERSON).into_iter().filter(|&c| Some(c) != business).collect();
    let (first_name, email) = (find(&FIRST_NAME), find(&EMAIL));
    let (language, forms) = (find(&LANGUAGE), find(&FORMS));
    let mut missing = Vec::new();
    if business.is_none() {
        missing.push(COLUMNS[0]);
    }
    if first_name.is_empty() && person.is_empty() {
        missing.push(COLUMNS[1]);
    }
    if email.is_empty() {
        missing.push(COLUMNS[2]);
    }
    let Some(business) = business.filter(|_| missing.is_empty()) else {
        return Err(ImportError::MissingColumns(missing));
    };
    let mut rows = Vec::new();
    for (row, cells) in records.filter(|(_, cells)| cells.iter().any(|c| !c.trim().is_empty())) {
        if rows.len() == MAX_ROWS {
            return Err(ImportError::TooManyRows);
        }
        let cell = |column: usize| cells.get(column).map_or("", |c| c.trim());
        // The first non-empty cell among `columns`.
        let first = |columns: &[usize]| {
            columns
                .iter()
                .map(|&column| cell(column))
                .find(|value| !value.is_empty())
                .unwrap_or("")
                .to_owned()
        };
        let person_name = first(&person);
        let mut business_name = cell(business).to_owned();
        if business_name.is_empty() && is_company(business) {
            // An individual: QuickBooks leaves their Company blank.
            business_name.clone_from(&person_name);
        }
        let mut contact_first_name = first(&first_name);
        // A person's name beside a company's gives the first name, unless
        // it's the company's own name again.
        let first_word = |text: &str| text.split_whitespace().next().unwrap_or("").to_owned();
        let company = cell(business);
        if contact_first_name.is_empty()
            && is_company(business)
            && !company.is_empty()
            && !first_word(&person_name).eq_ignore_ascii_case(&first_word(company))
        {
            contact_first_name = first_word(&person_name);
        }
        rows.push(ImportRow {
            row,
            input: ClientInput {
                business_name,
                contact_first_name,
                email: first(&email),
                language: first(&language),
                forms: first(&forms)
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
    Ok(Sheet {
        rows,
        has_language: !language.is_empty(),
        has_forms: !forms.is_empty(),
    })
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
