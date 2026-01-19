use biblatex::{Bibliography, ChunksExt, DateValue, PermissiveType};
use std::fs;

fn main() {
    let contents = fs::read_to_string("tests/fixtures/valid/non_standard_types.bib")
        .expect("Failed to read file");

    let bibliography =
        Bibliography::parse(&contents).expect("Failed to parse bibliography");

    println!("Parsed {} entries from non_standard_types.bib", bibliography.len());
    println!();

    for entry in bibliography.iter() {
        println!("Entry: {}", entry.key);
        println!("  Type: {}", entry.entry_type);
        if let Ok(title) = entry.title() {
            println!("  Title: {}", title.format_sentence());
        }
        if let Ok(author) = entry.author() {
            println!(
                "  Author(s): {}",
                author.iter().map(|p| format!("{}", p)).collect::<Vec<_>>().join(", ")
            );
        }
        if let Ok(PermissiveType::Typed(d)) = entry.date() {
            if let DateValue::At(dt) = d.value {
                println!("  Year: {}", dt.year);
            }
        }
        println!();
    }
}
