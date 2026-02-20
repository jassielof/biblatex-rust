use biblatex::Bibliography;

fn main() {
    let gral = include_str!("../tests/fixtures/valid/gral.bib");
    let bibliography = Bibliography::parse(gral).unwrap();
    let entry = bibliography.get("rashid2016").unwrap();
    let author = entry.author().unwrap();
    println!("Author: {}", author[0]);
}
