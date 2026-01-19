use biblatex::Bibliography;
use std::fs;

fn main() {
    let contents = fs::read_to_string("comprehensive_test.bib").unwrap();
    let bibliography = Bibliography::parse(&contents).unwrap();
    // back and forth check
    let serialized = bibliography.to_biblatex_string();
    let bibliography2 = Bibliography::parse(&serialized).unwrap();
    assert_eq!(bibliography.len(), bibliography2.len());
}
