use std::path::{Path, PathBuf};

use sls_validators::validate_path;

fn repository_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(relative)
}

#[test]
fn plural_alternatives_and_plural_only_nouns_have_distinct_contracts() {
    let schema = repository_path("schemas/lexicon-entry.schema.json");
    let valid = validate_path(
        &schema,
        &repository_path("tools/validators/tests/fixtures/valid-lexicon-number.jsonl"),
    )
    .expect("valid number fixtures should be readable");
    assert!(valid.is_valid(), "{:?}", valid.diagnostics);
    assert_eq!(valid.records_checked, 2);

    let invalid = validate_path(
        &schema,
        &repository_path("tools/validators/tests/fixtures/invalid-lexicon-number.jsonl"),
    )
    .expect("invalid number fixtures should be readable");
    assert_eq!(invalid.records_checked, 4);
    for (line, path) in [
        (1, "/plural"),
        (2, "/plural"),
        (3, "/part_of_speech"),
        (4, "/plural_variants"),
    ] {
        assert!(
            invalid.diagnostics.iter().any(|diagnostic| diagnostic.line == Some(line) && diagnostic.instance_path == path),
            "missing diagnostic for line {line} {path}: {:?}", invalid.diagnostics
        );
    }
}

#[test]
fn valid_lexicon_entry_passes_with_local_metadata_reference() {
    let report = validate_path(
        &repository_path("schemas/lexicon-entry.schema.json"),
        &repository_path("tools/validators/tests/fixtures/valid-lexicon-entry.json"),
    )
    .expect("schemas and fixture should be readable");

    assert!(report.is_valid(), "{:?}", report.diagnostics);
    assert_eq!(report.records_checked, 1);
}

#[test]
fn mass_nouns_and_unresolved_loanword_status_are_explicit() {
    let report = validate_path(
        &repository_path("schemas/lexicon-entry.schema.json"),
        &repository_path("tools/validators/tests/fixtures/valid-mass-lexicon-entry.json"),
    )
    .expect("schemas and fixture should be readable");

    assert!(report.is_valid(), "{:?}", report.diagnostics);
    assert_eq!(report.records_checked, 1);

    let invalid_report = validate_path(
        &repository_path("schemas/lexicon-entry.schema.json"),
        &repository_path("tools/validators/tests/fixtures/invalid-unresolved-loan-origin.json"),
    )
    .expect("schemas and fixture should be readable");

    assert!(!invalid_report.is_valid());
    assert!(
        invalid_report
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.message.contains("loan_origin"))
    );
}

#[test]
fn invalid_lexicon_entry_fails_bounded_contracts() {
    let report = validate_path(
        &repository_path("schemas/lexicon-entry.schema.json"),
        &repository_path("tools/validators/tests/fixtures/invalid-lexicon-entry.json"),
    )
    .expect("schemas and fixture should be readable");

    assert!(!report.is_valid());
    for expected_path in ["/sls_id", "/part_of_speech", "/dialect", "/definitions"] {
        assert!(
            report
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.instance_path == expected_path),
            "missing diagnostic for {expected_path}: {:?}",
            report.diagnostics
        );
    }
    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.message.contains("loan_origin"))
    );
    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.message.contains("source"))
    );
}

#[test]
fn noun_entries_require_reviewed_gender_and_plural() {
    let report = validate_path(
        &repository_path("schemas/lexicon-entry.schema.json"),
        &repository_path("tools/validators/tests/fixtures/invalid-noun-lexicon-entry.json"),
    )
    .expect("schemas and fixture should be readable");

    assert!(!report.is_valid());
    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.message.contains("gender"))
    );
    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.message.contains("plural"))
    );
}
