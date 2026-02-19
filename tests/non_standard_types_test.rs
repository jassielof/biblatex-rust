use biblatex::{Bibliography, DateValue, PermissiveType};
use std::fs;

#[test]
fn test_non_standard_types_parse() {
    let contents = fs::read_to_string("tests/fixtures/valid/non_standard_types.bib")
        .expect("Failed to read file");

    let bibliography =
        Bibliography::parse(&contents).expect("Failed to parse bibliography");

    assert_eq!(bibliography.len(), 15, "Should have 15 non-standard type entries");

    for entry in bibliography.iter() {
        // Verify each entry can be accessed
        assert!(!entry.key.is_empty(), "Entry key should not be empty");

        // Title is optional for some types like bibnote
        // Just verify it doesn't panic when accessed
        let _ = entry.title();

        // Author is optional for some types like jurisdiction, legislation, etc.
        // Just verify it doesn't panic when accessed
        let _ = entry.author();

        // Verify date is parseable if present
        if let Ok(PermissiveType::Typed(d)) = entry.date()
            && let DateValue::At(dt) = d.value
        {
            assert!(dt.year > 0, "Entry '{}' should have a valid year", entry.key);
        }
    }
}
