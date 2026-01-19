use biblatex::Bibliography;
use std::fs;

fn main() {
    let contents = fs::read_to_string("tests/fixtures/valid/example.bib").unwrap();
    let bibliography = Bibliography::parse(&contents).unwrap();
    // just check if it parses correctly, nothing else
    println!("{:#?}", bibliography);
}
