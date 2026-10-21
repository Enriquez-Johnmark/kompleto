//! Seam 3: Excel imports are parsed in a child process with time and memory
//! limits, so a crafted file can't take the server down (B2).

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

use std::io::Write;
use std::path::PathBuf;

use kompleto::kompleto::import::ImportError;
use kompleto::kompleto::import::xlsx::XlsxReader;
use zip::write::SimpleFileOptions;

fn reader() -> XlsxReader {
    XlsxReader::new(PathBuf::from(env!("CARGO_BIN_EXE_kompleto")))
}

/// A minimal .xlsx whose first sheet's XML is `sheet_data` (with any
/// `<dimension>` given before it).
fn xlsx(dimension: &str, sheet_data: &str) -> Vec<u8> {
    let mut file = zip::ZipWriter::new(std::io::Cursor::new(Vec::new()));
    let parts = [
        (
            "[Content_Types].xml",
            r#"<?xml version="1.0" encoding="UTF-8"?><Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/><Default Extension="xml" ContentType="application/xml"/><Override PartName="/xl/workbook.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"/><Override PartName="/xl/worksheets/sheet1.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"/></Types>"#.to_owned(),
        ),
        (
            "_rels/.rels",
            r#"<?xml version="1.0" encoding="UTF-8"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="xl/workbook.xml"/></Relationships>"#.to_owned(),
        ),
        (
            "xl/workbook.xml",
            r#"<?xml version="1.0" encoding="UTF-8"?><workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><sheets><sheet name="Clients" sheetId="1" r:id="rId1"/></sheets></workbook>"#.to_owned(),
        ),
        (
            "xl/_rels/workbook.xml.rels",
            r#"<?xml version="1.0" encoding="UTF-8"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet1.xml"/></Relationships>"#.to_owned(),
        ),
        (
            "xl/worksheets/sheet1.xml",
            format!(
                r#"<?xml version="1.0" encoding="UTF-8"?><worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">{dimension}<sheetData>{sheet_data}</sheetData></worksheet>"#
            ),
        ),
    ];
    for (name, content) in parts {
        file.start_file(name, SimpleFileOptions::default()).unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }
    file.finish().unwrap().into_inner()
}

fn row(n: usize, cells: &[&str]) -> String {
    let cells: String = cells
        .iter()
        .zip('A'..)
        .map(|(text, column)| format!(r#"<c r="{column}{n}" t="inlineStr"><is><t>{text}</t></is></c>"#))
        .collect();
    format!(r#"<row r="{n}">{cells}</row>"#)
}

#[tokio::test]
async fn b2_an_xlsx_file_is_read_like_a_csv() {
    let sheet = [
        row(
            1,
            &["Business name", "Contact first name", "Email", "Language", "Forms"],
        ),
        row(
            2,
            &["Nena's Store", "Nena", "nena@example.test", "Taglish", "2551Q, 1701Q"],
        ),
    ]
    .concat();

    let rows = reader().read(&xlsx("", &sheet)).await.unwrap();

    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].row, 2);
    assert_eq!(rows[0].input.business_name, "Nena's Store");
    assert_eq!(rows[0].input.forms, ["2551Q", "1701Q"]);
}

#[tokio::test]
async fn b2_a_crafted_xlsx_is_refused_and_the_server_carries_on() {
    // Two cells at opposite corners of the sheet make the parser try to
    // allocate every cell in between: billions of them.
    let sheet = [
        row(1, &["Business name"]),
        r#"<row r="1048576"><c r="XFD1048576" t="inlineStr"><is><t>x</t></is></c></row>"#.to_owned(),
    ]
    .concat();

    let result = reader()
        .read(&xlsx(r#"<dimension ref="A1:XFD1048576"/>"#, &sheet))
        .await;

    assert!(matches!(result, Err(ImportError::Unreadable(_))), "{result:?}");
}

#[tokio::test]
async fn b2_a_file_that_is_not_xlsx_is_refused() {
    let result = reader().read(b"Business name,Email\n").await;

    assert!(matches!(result, Err(ImportError::Unreadable(_))), "{result:?}");
}
