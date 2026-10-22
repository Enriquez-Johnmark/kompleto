use super::*;

/// The rows of a CSV file with Language and Forms columns required.
fn read(bytes: &[u8]) -> Result<Vec<ImportRow>, ImportError> {
    read_csv(bytes).and_then(|sheet| sheet.rows(None))
}
use crate::kompleto::calendar::BirForm;

const HEADER: &str = "Business name,Contact first name,Email,Language,Forms\n";

fn csv(body: &str) -> Vec<u8> {
    format!("{HEADER}{body}").into_bytes()
}

#[test]
fn b2_valid_rows_are_kept_and_each_invalid_row_has_its_reason() {
    let rows = read(&csv("Nena's Store,Nena,nena@example.test,Taglish,\"2551Q, 1701Q\"\n\
         ,Ramon,ramon@example.test,English,2550Q\n\
         Lito Hardware,Lito,lito@example.test,English,2550Q; 0619-E; 1601-EQ\n\
         Bad Email Co,Bea,bea-at-example,English,2550M\n"))
    .unwrap();

    let preview = preview(&rows, &[]);

    let valid: Vec<_> = preview
        .valid
        .iter()
        .map(|(row, c)| (*row, c.business_name.as_str()))
        .collect();
    assert_eq!(valid, [(2, "Nena's Store"), (4, "Lito Hardware")]);
    assert_eq!(
        preview.valid[1].1.forms,
        [BirForm::F0619E, BirForm::F1601EQ, BirForm::F2550Q]
    );
    assert_eq!(
        preview.invalid,
        [
            (3, vec!["Business name: is required".to_owned()]),
            (
                5,
                vec![
                    "Email: \"bea-at-example\" is not an email address".to_owned(),
                    "Forms: unknown form \"2550M\"".to_owned(),
                ]
            ),
        ]
    );
}

#[test]
fn b2_a_sixty_row_file_is_checked_row_by_row() {
    let body: String = (1..=60)
        .map(|n| format!("Client {n},Ana,ana{n}@example.test,English,2551Q\n"))
        .collect();
    let rows = read(&csv(&body)).unwrap();

    let preview = preview(&rows, &["client 7".to_owned()]);

    assert_eq!(preview.valid.len(), 59);
    assert_eq!(
        preview.invalid,
        [(8, vec!["Business name: \"Client 7\" is already a client".to_owned()])]
    );
}

#[test]
fn b2_a_name_repeated_in_the_file_is_refused_after_its_first_row() {
    let rows = read(&csv(
        "Nena's Store,Nena,nena@example.test,Taglish,2551Q\nNENA'S STORE,Nena,nena@example.test,Taglish,2551Q\n",
    ))
    .unwrap();

    let preview = preview(&rows, &[]);

    assert_eq!(preview.valid.len(), 1);
    assert_eq!(
        preview.invalid,
        [(3, vec!["Business name: also on row 2 of this file".to_owned()])]
    );
}

#[test]
fn b2_columns_are_found_by_header_in_any_order_and_case() {
    let file = "FORMS,email,Language,Business Name,Contact First Name,Notes\n2551Q,a@b.test,english,Shop,Ana,ignored\n";

    let rows = read(file.as_bytes()).unwrap();

    assert_eq!(rows[0].input.business_name, "Shop");
    assert_eq!(rows[0].input.forms, ["2551Q"]);
}

#[test]
fn b2_a_file_without_the_columns_is_refused_whole() {
    let missing = read(b"Phone,Notes\n0917,regular\n");

    assert_eq!(
        missing.unwrap_err(),
        ImportError::MissingColumns(vec!["Business name", "Contact first name", "Email"])
    );
    assert_eq!(read(HEADER.as_bytes()).unwrap_err(), ImportError::Empty);
}

#[test]
fn b2_limits_rows_and_size() {
    let body: String = (0..=MAX_ROWS)
        .map(|n| format!("C{n},A,a@b.test,English,2551Q\n"))
        .collect();
    assert_eq!(read(&csv(&body)).unwrap_err(), ImportError::TooManyRows);
    assert_eq!(read(&vec![b'a'; MAX_BYTES + 1]).unwrap_err(), ImportError::TooLarge);
}

#[test]
fn b2_windows_1252_and_byte_order_marks_are_read() {
    // "Niño" in Windows-1252, as older Excel saves CSV.
    let mut legacy = csv("Ni").to_vec();
    legacy.extend_from_slice(b"\xf1o Shop,Ana,a@b.test,English,2551Q\n");
    assert_eq!(read(&legacy).unwrap()[0].input.business_name, "Niño Shop");

    let mut bom = b"\xef\xbb\xbf".to_vec();
    bom.extend(csv("Shop,Ana,a@b.test,English,2551Q\n"));
    assert_eq!(read(&bom).unwrap()[0].input.business_name, "Shop");
}

#[test]
fn b2_blank_rows_are_skipped() {
    let rows = read(&csv(
        "Shop,Ana,a@b.test,English,2551Q\n,,,,\n\nOther,Bo,b@b.test,English,2551Q\n",
    ))
    .unwrap();

    assert_eq!(rows.iter().map(|r| r.row).collect::<Vec<_>>(), [2, 5]);
}

#[test]
fn b2_row_numbers_match_the_spreadsheet_with_multi_line_cells_and_mac_line_endings() {
    let multi_line = read(&csv(
        "Shop,Ana,a@b.test,English,\"2551Q\n1701Q\"\n\nOther,Bo,b@b.test,English,2551Q\n",
    ))
    .unwrap();
    let mac_file =
        format!("{HEADER}Shop,Ana,a@b.test,English,2551Q\nOther,Bo,b@b.test,English,2551Q\n").replace('\n', "\r");
    let mac = read(mac_file.as_bytes()).unwrap();

    assert_eq!(multi_line.iter().map(|r| r.row).collect::<Vec<_>>(), [2, 4]);
    assert_eq!(multi_line[0].input.forms, ["2551Q", "1701Q"]);
    assert_eq!(mac.iter().map(|r| r.row).collect::<Vec<_>>(), [2, 3]);
}

fn taglish_with(forms: &[&str]) -> BatchDefaults {
    BatchDefaults {
        language: "taglish".to_owned(),
        forms: forms.iter().map(|&form| form.to_owned()).collect(),
    }
}

#[test]
fn b5_a_xero_contacts_export_imports_with_the_batchs_language_and_forms() {
    let file = "*ContactName,AccountNumber,EmailAddress,FirstName,LastName,TaxNumber\n\
                Nena's Store,C-001,nena@example.test,Nena,Cruz,123-456-789-000\n\
                Lito Hardware,C-002,lito@example.test,Lito,Reyes,\n";

    let rows = read_csv(file.as_bytes())
        .unwrap()
        .rows(Some(&taglish_with(&["2551Q", "1701Q"])))
        .unwrap();
    let preview = preview(&rows, &[]);

    assert!(preview.invalid.is_empty(), "{:?}", preview.invalid);
    let (_, nena) = &preview.valid[0];
    assert_eq!(
        (
            nena.business_name.as_str(),
            nena.contact_first_name.as_str(),
            nena.email.as_str()
        ),
        ("Nena's Store", "Nena", "nena@example.test")
    );
    assert_eq!(nena.forms, [BirForm::F2551Q, BirForm::F1701Q]);
    assert!(
        rows.iter().all(|row| !format!("{:?}", row.input).contains("123-456")),
        "the TIN column is never read"
    );
}

#[test]
fn b5_a_quickbooks_customer_list_takes_the_company_and_the_persons_first_name() {
    let file = "Customer,Company,Phone,Email Address\n\
                Nena Cruz,Nena's Store,0917 000 0000,nena@example.test\n\
                Lito Reyes,,,lito@example.test\n";

    let rows = read_csv(file.as_bytes())
        .unwrap()
        .rows(Some(&taglish_with(&["2550Q"])))
        .unwrap();
    let preview = preview(&rows, &[]);

    let named: Vec<_> = preview
        .valid
        .iter()
        .map(|(_, c)| (c.business_name.as_str(), c.contact_first_name.as_str()))
        .collect();
    assert_eq!(named, [("Nena's Store", "Nena")]);
    // An individual: their name is the business name, and their first name
    // isn't guessed from it.
    assert_eq!(preview.invalid[0].0, 3);
    assert!(preview.invalid[0].1.join("; ").contains("Contact first name"));
}

#[test]
fn b5_a_company_name_never_becomes_the_greeting() {
    let file = "Customer,Company,Email\nGolden Harvest Trading,Golden Harvest Trading Corp,gh@example.test\n";

    let rows = read_csv(file.as_bytes())
        .unwrap()
        .rows(Some(&taglish_with(&["2550Q"])))
        .unwrap();

    assert_eq!(rows[0].input.contact_first_name, "", "not \"Golden\"");
}

#[test]
fn b5_a_blank_business_name_isnt_taken_from_another_column() {
    let file = "Business name,Name,Email,Language,Forms\n,Nena Cruz,nena@example.test,Taglish,2551Q\n";

    let rows = read(file.as_bytes()).unwrap();

    assert_eq!(rows[0].input.business_name, "");
}

#[test]
fn b5_a_file_without_language_and_forms_needs_the_batch_choice() {
    let file = "*ContactName,EmailAddress,FirstName\nNena's Store,nena@example.test,Nena\n";

    assert_eq!(
        read(file.as_bytes()).unwrap_err(),
        ImportError::MissingColumns(vec!["Language", "Forms"])
    );
}

#[test]
fn b5_the_batch_choice_never_fills_a_column_the_file_has() {
    let rows = read_csv(&csv("Nena's Store,Nena,nena@example.test,,\n"))
        .unwrap()
        .rows(Some(&taglish_with(&["1601-C"])))
        .unwrap();

    assert_eq!(
        rows[0].input.language, "",
        "a blank cell stays blank, and the row is flagged"
    );
    assert!(rows[0].input.forms.is_empty());
}
