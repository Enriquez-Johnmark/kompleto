//! Seam 5: the engine never depends on the product (ADR 0001).

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

use std::fs;
use std::path::Path;

#[test]
fn engine_never_uses_the_kompleto_product_module() {
    let mut violations = Vec::new();
    scan(
        Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/engine")),
        &mut violations,
    );

    assert!(
        violations.is_empty(),
        "src/engine must not use the product module:\n{}",
        violations.join("\n")
    );
}

fn scan(dir: &Path, violations: &mut Vec<String>) {
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            scan(&path, violations);
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            let source = fs::read_to_string(&path).unwrap();
            for (n, line) in source.lines().enumerate() {
                let code = line.split("//").next().unwrap_or("");
                // `crate::kompleto`, `super::kompleto`, `kompleto::kompleto`:
                // any path into the product module contains `::kompleto`.
                if code.contains("::kompleto") {
                    violations.push(format!("{}:{}: {}", path.display(), n + 1, line.trim()));
                }
            }
        }
    }
}
