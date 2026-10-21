//! The Owed documents for one Client and one collection Period (B4): a
//! default list that follows the forms the Client files, edited per Client.
//! The defaults are our synthesis, not an official BIR list
//! (`docs/research/bir-calendar.md`).

use std::collections::BTreeSet;

use time::Month;

use super::calendar::BirForm;

/// Which collection Periods an item is owed in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Recurs {
    /// Every Period.
    EveryPeriod,
    /// Periods ending in March, June, September and December.
    QuarterEnd,
    /// The December Period.
    YearEnd,
}

impl Recurs {
    /// Every choice, in display order.
    pub const ALL: [Self; 3] = [Self::EveryPeriod, Self::QuarterEnd, Self::YearEnd];

    /// The stored name.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EveryPeriod => "every-period",
            Self::QuarterEnd => "quarter-end",
            Self::YearEnd => "year-end",
        }
    }

    /// Reads a stored name.
    pub fn parse(text: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|recurs| recurs.as_str() == text)
    }

    fn includes(self, month: Month) -> bool {
        match self {
            Self::EveryPeriod => true,
            Self::QuarterEnd => u8::from(month) % 3 == 0,
            Self::YearEnd => month == Month::December,
        }
    }
}

/// One built-in item of the default checklist.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DefaultItem {
    /// A stable key, so a Client's removals survive wording changes.
    pub key: &'static str,
    /// What the Contact is asked for.
    pub label: &'static str,
    /// When it's owed.
    pub recurs: Recurs,
    /// It's owed only if the Client files one of these (empty: always).
    pub when_filing: &'static [BirForm],
}

/// The default checklist. Items say "invoices", not "official receipts"
/// (RR 7-2024).
pub const DEFAULT_ITEMS: [DefaultItem; 7] = [
    DefaultItem {
        key: "sales-invoices",
        label: "Sales invoices issued, including cancelled ones",
        recurs: Recurs::EveryPeriod,
        when_filing: &[],
    },
    DefaultItem {
        key: "purchase-invoices",
        label: "Purchase and expense invoices and receipts",
        recurs: Recurs::EveryPeriod,
        when_filing: &[],
    },
    DefaultItem {
        key: "bank-statements",
        label: "Bank and e-wallet (GCash, Maya) statements",
        recurs: Recurs::EveryPeriod,
        when_filing: &[],
    },
    DefaultItem {
        key: "payroll",
        label: "Payroll register and SSS, PhilHealth and Pag-IBIG reports",
        recurs: Recurs::EveryPeriod,
        when_filing: &[BirForm::F1601C],
    },
    DefaultItem {
        key: "2307-issued",
        label: "Copies of 2307s issued to suppliers, with payment vouchers",
        recurs: Recurs::EveryPeriod,
        when_filing: &[BirForm::F0619E, BirForm::F1601EQ],
    },
    DefaultItem {
        key: "2307-received",
        label: "2307s received from customers",
        recurs: Recurs::QuarterEnd,
        when_filing: &[
            BirForm::F2550Q,
            BirForm::F2551Q,
            BirForm::F1701Q,
            BirForm::F1702Q,
            BirForm::F1701,
            BirForm::F1702,
        ],
    },
    DefaultItem {
        key: "year-end",
        label: "Year-end inventory count, fixed-asset purchases and loan statements",
        recurs: Recurs::YearEnd,
        when_filing: &[BirForm::F1701, BirForm::F1702, BirForm::F1604C, BirForm::F1604E],
    },
];

/// An item the bookkeeper added for one Client.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtraItem {
    /// What the Contact is asked for.
    pub label: String,
    /// When it's owed.
    pub recurs: Recurs,
}

/// How one Client's checklist differs from the default.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ChecklistEdits {
    /// Keys of default items this Client never owes.
    pub removed: BTreeSet<String>,
    /// Items added for this Client.
    pub extra: Vec<ExtraItem>,
    /// The Client's Payors (H2), each owing its own 2307 in place of the
    /// general "2307s received" item.
    pub payors: Vec<String>,
}

/// The default item that the Client's Payors replace.
const RECEIVED_2307_KEY: &str = "2307-received";

/// What the upload page offers for a 2307 from a business not on the
/// Client's Payor list; the Contact types its name.
pub const ANOTHER_PAYOR_2307: &str = "BIR Form 2307 from another business";

/// The Owed document for one Payor's 2307.
pub fn payor_2307(payor: &str) -> String {
    format!("{PAYOR_2307_PREFIX}{payor}")
}

const PAYOR_2307_PREFIX: &str = "BIR Form 2307 from ";

/// Whether `document` is a 2307 a customer gave the Client: the general
/// item, a listed Payor's, or one from another business.
pub fn is_received_2307(document: &str) -> bool {
    document.starts_with(PAYOR_2307_PREFIX)
        || DEFAULT_ITEMS
            .iter()
            .any(|item| item.key == RECEIVED_2307_KEY && item.label == document)
}

/// The labels of the documents a Client filing `forms` owes for the
/// collection Period ending in `month`.
pub fn owed_documents(forms: &[BirForm], month: Month, edits: &ChecklistEdits) -> Vec<String> {
    let defaults = DEFAULT_ITEMS
        .iter()
        .filter(|item| item.recurs.includes(month))
        .filter(|item| item.when_filing.is_empty() || item.when_filing.iter().any(|form| forms.contains(form)))
        .filter(|item| !edits.removed.contains(item.key))
        .flat_map(|item| {
            if item.key == RECEIVED_2307_KEY && !edits.payors.is_empty() {
                edits.payors.iter().map(|payor| payor_2307(payor)).collect()
            } else {
                vec![item.label.to_owned()]
            }
        });
    let extra = edits
        .extra
        .iter()
        .filter(|item| item.recurs.includes(month))
        .map(|item| item.label.clone());
    defaults.chain(extra).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kompleto::calendar::BirForm::*;

    #[test]
    fn b4_default_owed_documents_follow_the_forms_and_the_month() {
        let vat_employer = [F1601C, F0619E, F1601EQ, F2550Q, F1701Q, F1701];
        let none = ChecklistEdits::default();

        assert_eq!(
            owed_documents(&vat_employer, Month::November, &none),
            [
                "Sales invoices issued, including cancelled ones",
                "Purchase and expense invoices and receipts",
                "Bank and e-wallet (GCash, Maya) statements",
                "Payroll register and SSS, PhilHealth and Pag-IBIG reports",
                "Copies of 2307s issued to suppliers, with payment vouchers",
            ]
        );
        let december = owed_documents(&vat_employer, Month::December, &none);
        assert!(december.ends_with(&[
            "2307s received from customers".to_owned(),
            "Year-end inventory count, fixed-asset purchases and loan statements".to_owned(),
        ]));
        assert_eq!(owed_documents(&[F2551Q], Month::March, &none).len(), 4);
    }

    #[test]
    fn b4_a_clients_edits_remove_and_add_items() {
        let edits = ChecklistEdits {
            removed: BTreeSet::from(["bank-statements".to_owned()]),
            extra: vec![
                ExtraItem {
                    label: "Lazada payout reports".to_owned(),
                    recurs: Recurs::EveryPeriod,
                },
                ExtraItem {
                    label: "Rental contracts".to_owned(),
                    recurs: Recurs::YearEnd,
                },
            ],
            ..ChecklistEdits::default()
        };

        assert_eq!(
            owed_documents(&[F2551Q], Month::January, &edits),
            [
                "Sales invoices issued, including cancelled ones",
                "Purchase and expense invoices and receipts",
                "Lazada payout reports",
            ]
        );
    }
}
