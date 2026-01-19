use biblatex::Bibliography;
use std::fs;

#[test]
fn test_official_example() {
    let contents = fs::read_to_string("tests/fixtures/valid/example.bib").unwrap();
    let bibliography = Bibliography::parse(&contents).unwrap();

    // Verify it parses correctly and contains entries
    assert!(!bibliography.is_empty(), "Bibliography should contain entries");
}
