use biblatex::Bibliography;
use std::fs;

#[test]
fn test_comprehensive_roundtrip() {
    let contents = fs::read_to_string("comprehensive_test.bib").unwrap();
    let bibliography = Bibliography::parse(&contents).unwrap();
    
    // back and forth check
    let serialized = bibliography.to_biblatex_string();
    let bibliography2 = Bibliography::parse(&serialized).unwrap();
    
    assert_eq!(bibliography.len(), bibliography2.len());

    // check keys
    for key in bibliography.keys() {
        assert!(
            bibliography2.get(key).is_some(),
            "Key '{}' missing after roundtrip",
            key
        );
    }
}
