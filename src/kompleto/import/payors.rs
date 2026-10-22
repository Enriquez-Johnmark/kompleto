//! Payors from a sales export (H3): the names of customers on rows that
//! show tax withheld. Amounts are only checked for a non-zero digit, never
//! parsed into numbers, and no column other than the ones below is read.

use super::ImportError;

/// The most names one file may give.
pub const MAX_NAMES: usize = 200;
/// How many rows from the top may come before the header row, as a
/// report's title block does.
const HEADER_SEARCH_ROWS: usize = 10;

/// Header names, squashed, that hold the customer's name: Xero's
/// `*ContactName`, QuickBooks' `Customer` or `Name`.
const NAME: [&str; 7] = [
    "contactname",
    "customer",
    "customername",
    "customerfullname",
    "customerjob",
    "name",
    "payor",
];
/// Header names, squashed, whose text can name withholding.
const TEXT: [&str; 9] = [
    "account",
    "accountname",
    "description",
    "memodescription",
    "memo",
    "productservice",
    "item",
    "taxtype",
    "taxrate",
];

/// Words in a squashed header that mark a column of identifiers, not amounts.
const IDENTIFIERS: [&str; 5] = ["tin", "code", "number", "agent", "atc"];

/// Why a file gave no names.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum PayorImportError {
    /// Too large or unreadable.
    #[error(transparent)]
    File(#[from] ImportError),
    /// No header row with the columns needed.
    #[error(
        "couldn't find a header row with a customer name column and a withholding, account, description or tax type column"
    )]
    NoHeader,
    /// More than [`MAX_NAMES`] names.
    #[error("the file names more than {MAX_NAMES} customers")]
    TooMany,
}

/// The customers, in first-seen order and each once, on rows that show
/// tax withheld; with `every_row`, on every row, as in a report of the
/// withholding account alone.
pub fn withholding_customers(
    records: Vec<(usize, Vec<String>)>,
    every_row: bool,
) -> Result<Vec<String>, PayorImportError> {
    let mut records = records.into_iter().map(|(_, cells)| cells);
    let columns = records
        .by_ref()
        .take(HEADER_SEARCH_ROWS)
        .find_map(|header| Columns::find(&header).filter(|c| every_row || c.shows_withholding()))
        .ok_or(PayorImportError::NoHeader)?;
    let mut names: Vec<String> = Vec::new();
    for cells in records {
        let cell = |column: usize| cells.get(column).map_or("", |c| c.trim());
        let Some(name) = columns.name.iter().map(|&c| cell(c)).find(|n| !n.is_empty()) else {
            continue;
        };
        let withheld = every_row
            || columns
                .amount
                .iter()
                .any(|&c| cell(c).chars().any(|d| ('1'..='9').contains(&d)))
            || columns.text.iter().any(|&c| names_withholding(cell(c)));
        // Compared as the Payor list's unique index does.
        if withheld && !names.iter().any(|known| known.to_lowercase() == name.to_lowercase()) {
            if names.len() == MAX_NAMES {
                return Err(PayorImportError::TooMany);
            }
            names.push(name.to_owned());
        }
    }
    Ok(names)
}

/// Where a header row's useful columns are.
struct Columns {
    /// The customer's name, best first.
    name: Vec<usize>,
    /// Amounts or rates of tax withheld.
    amount: Vec<usize>,
    /// Text that can name withholding.
    text: Vec<usize>,
}

impl Columns {
    /// The columns of `header`, if it has a customer name column.
    fn find(header: &[String]) -> Option<Self> {
        let squashed: Vec<String> = header.iter().map(|h| super::squash(h)).collect();
        let name: Vec<usize> = NAME
            .iter()
            .filter_map(|n| squashed.iter().position(|h| h == n))
            .collect();
        let is_text = |c: &usize| TEXT.contains(&squashed[*c].as_str());
        let text = (0..header.len()).filter(is_text).collect();
        // A column that identifies rather than measures, such as the
        // withholding agent's TIN or the ATC, is never read.
        let identifies = |c: &usize| IDENTIFIERS.iter().any(|word| squashed[*c].contains(word));
        let amount = (0..header.len())
            .filter(|c| !is_text(c) && !identifies(c) && names_withholding(&header[*c]))
            .collect();
        (!name.is_empty()).then_some(Self { name, amount, text })
    }

    /// Whether some column can show tax withheld.
    fn shows_withholding(&self) -> bool {
        !self.amount.is_empty() || !self.text.is_empty()
    }
}

/// Whether `text` names withholding: "withholding", "withheld", or the
/// words CWT, EWT, WHT or 2307.
fn names_withholding(text: &str) -> bool {
    let text = text.to_lowercase();
    text.contains("withholding")
        || text.contains("withheld")
        || text
            .split(|c: char| !c.is_ascii_alphanumeric())
            .any(|word| matches!(word, "cwt" | "ewt" | "wht" | "2307"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn records(rows: &[&[&str]]) -> Vec<(usize, Vec<String>)> {
        super::super::numbered(
            rows.iter()
                .map(|row| row.iter().map(|&cell| cell.to_owned()).collect())
                .collect(),
        )
    }

    #[test]
    fn h3_keeps_customers_with_a_non_zero_withholding_column() {
        let file = records(&[
            &["Customer", "Invoice", "Amount", "CWT", "TIN"],
            &["Jollibee Foods", "INV-1", "10,000.00", "200.00", "000-123-456-000"],
            &["Puregold", "INV-2", "5,000.00", "0.00", ""],
            &["SM Prime", "INV-3", "8,000.00", "(160.00)", ""],
            &["jollibee foods ", "INV-4", "1,000.00", "20.00", ""],
            &["Robinsons", "INV-5", "1,000.00", "-", ""],
        ]);
        assert_eq!(
            withholding_customers(file, false),
            Ok(vec!["Jollibee Foods".to_owned(), "SM Prime".to_owned()])
        );
    }

    #[test]
    fn h3_keeps_customers_on_lines_naming_withholding() {
        // Xero's invoice export, a "Less: CWT" line per invoice.
        let file = records(&[
            &[
                "*ContactName",
                "EmailAddress",
                "*InvoiceNumber",
                "*Description",
                "*UnitAmount",
                "*AccountCode",
            ],
            &["Ayala Land", "ap@ayala.ph", "INV-9", "Consulting", "50000", "200"],
            &["Ayala Land", "ap@ayala.ph", "INV-9", "Less: CWT 2%", "-1000", "1240"],
            &["Mercury Drug", "", "INV-10", "Consulting", "3000", "200"],
            &["Meralco", "", "INV-11", "Expanded withholding tax", "-60", "1240"],
            &["Globe", "", "INV-12", "Newt figurines", "60", "200"],
        ]);
        assert_eq!(
            withholding_customers(file, false),
            Ok(vec!["Ayala Land".to_owned(), "Meralco".to_owned()])
        );
    }

    #[test]
    fn h3_skips_a_report_title_block_and_takes_every_row_when_asked() {
        // QuickBooks' Transaction Detail by Account for the CWT account.
        let file = records(&[
            &["Dela Cruz Pharmacy"],
            &["Transaction Detail by Account"],
            &["July - September, 2026"],
            &[],
            &["", "Date", "Transaction Type", "Num", "Name", "Amount"],
            &["Creditable Withholding Tax", "", "", "", "", ""],
            &["", "07/15/2026", "Payment", "", "Jollibee Foods", "200.00"],
            &["", "08/15/2026", "Payment", "", "Puregold", "100.00"],
            &["Total for Creditable Withholding Tax", "", "", "", "", "300.00"],
        ]);
        assert_eq!(
            withholding_customers(file, true),
            Ok(vec!["Jollibee Foods".to_owned(), "Puregold".to_owned()])
        );
    }

    #[test]
    fn h3_never_reads_a_withholding_tin_or_code_column() {
        let file = records(&[
            &["Customer", "Amount", "Withholding Agent TIN", "WHT Code", "EWT"],
            &["Jollibee Foods", "100", "000-123-456-000", "WC158", "0"],
            &["Niño Foods", "100", "000-999-456-000", "WC158", "2"],
            &["NIÑO FOODS", "100", "", "", "2"],
        ]);
        assert_eq!(withholding_customers(file, false), Ok(vec!["Niño Foods".to_owned()]));
    }

    #[test]
    fn h3_refuses_a_file_without_the_columns_needed() {
        let file = records(&[&["Customer", "Amount"], &["Jollibee Foods", "100"]]);
        assert_eq!(withholding_customers(file, false), Err(PayorImportError::NoHeader));
        let file = records(&[&["Invoice", "CWT"], &["INV-1", "100"]]);
        assert_eq!(withholding_customers(file, true), Err(PayorImportError::NoHeader));
    }

    #[test]
    fn h3_refuses_more_names_than_the_limit() {
        let header: &[&str] = &["Customer", "CWT"];
        let names: Vec<String> = (0..=MAX_NAMES).map(|i| format!("Customer {i}")).collect();
        let mut rows: Vec<&[&str]> = vec![header];
        let cells: Vec<[&str; 2]> = names.iter().map(|name| [name.as_str(), "1"]).collect();
        rows.extend(cells.iter().map(|row| &row[..]));
        assert_eq!(
            withholding_customers(records(&rows), false),
            Err(PayorImportError::TooMany)
        );
    }
}
