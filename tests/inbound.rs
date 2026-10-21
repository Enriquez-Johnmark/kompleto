//! Seam 1 (M5): inbound email from sample messages (G1).

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

use kompleto::engine::inbound::{InboundError, MAX_RAW_BYTES, parse};

fn sample(name: &str) -> Vec<u8> {
    let path = format!("{}/tests/fixtures/mail/{name}.eml", env!("CARGO_MANIFEST_DIR"));
    std::fs::read_to_string(path)
        .unwrap()
        .replace("{{TO}}", "reply+abc@getkompleto.test")
        .into_bytes()
}

#[test]
fn g1_a_human_reply_is_read_with_its_sender_recipients_and_text() {
    let email = parse(&sample("human_reply")).unwrap();

    assert_eq!(email.from, "nena@example.test", "lowercased");
    assert_eq!(email.recipients, ["reply+abc@getkompleto.test"]);
    assert_eq!(email.subject, "Re: Mga dokumento para sa October 2026");
    assert!(email.text.starts_with("Hi po! Ipapadala ko po bukas"), "{}", email.text);
    assert!(!email.automatic);
    assert!(email.attachments.is_empty());
}

#[test]
fn g1_out_of_office_replies_are_recognised_as_automatic() {
    for name in ["outlook_ooo", "gmail_vacation", "subject_only_ooo"] {
        assert!(parse(&sample(name)).unwrap().automatic, "{name}");
    }
}

#[test]
fn g1_attachments_are_kept_as_bytes_for_the_upload_rules_to_judge() {
    let email = parse(&sample("with_attachments")).unwrap();

    assert_eq!(email.text.trim(), "Eto po ang resibo.");
    assert_eq!(email.attachments.len(), 2);
    assert!(email.attachments[0].starts_with(b"\xFF\xD8\xFF"));
    assert!(email.attachments[1].starts_with(b"MZ"));
}

#[test]
fn g1_unreadable_or_oversized_mail_is_refused() {
    assert!(matches!(parse(b"not an email at all"), Err(InboundError::Unreadable)));
    assert!(matches!(
        parse(&vec![b'a'; MAX_RAW_BYTES + 1]),
        Err(InboundError::TooLarge)
    ));
}
