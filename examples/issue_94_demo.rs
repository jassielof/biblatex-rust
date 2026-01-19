/// Example demonstrating the "and others" feature in name lists (Issue #94)
use biblatex::{Bibliography, ChunksExt, NameListEntry};

fn main() {
    println!("=== Issue #94: 'and others' in Name Lists ===\n");

    let src = r#"
@article{paper1,
    title = {A Survey of Modern Techniques},
    author = {Alice Johnson and Bob Smith and Charlie Brown and others},
    year = {2024},
}

@book{book1,
    title = {Complete Guide to Everything},
    editor = {Diana Lee and others},
    year = {2023},
}

@incollection{chapter1,
    title = {Advanced Topics},
    author = {Eve Wilson and Frank Miller and Grace Taylor and Harry Davis and others},
    booktitle = {The Big Book},
    year = {2025},
}
"#;

    let bibliography = Bibliography::parse(src).unwrap();

    // Example 1: Basic "and others" usage
    println!("Example 1: Paper with multiple authors and 'and others'");
    let entry1 = bibliography.get("paper1").unwrap();
    let authors = entry1.author_list().unwrap();
    println!("  Title: {}", entry1.title().unwrap().format_verbatim());
    println!("  Authors:");
    for (i, author) in authors.iter().enumerate() {
        match author {
            NameListEntry::Person(p) => {
                println!(
                    "    {}. {} {}",
                    i + 1,
                    p.given_name,
                    if !p.prefix.is_empty() {
                        format!("{} {}", p.prefix, p.name)
                    } else {
                        p.name.clone()
                    }
                );
            }
            NameListEntry::AndOthers => {
                println!("    ... and others");
            }
        }
    }
    println!();

    // Example 2: Editor with "and others"
    println!("Example 2: Book with editor and 'and others'");
    let entry2 = bibliography.get("book1").unwrap();
    let editors = entry2.editor_list().unwrap();
    println!("  Title: {}", entry2.title().unwrap().format_verbatim());
    println!("  Editors:");
    for editor in &editors {
        match editor {
            NameListEntry::Person(p) => {
                println!("    {} {}", p.given_name, p.name);
            }
            NameListEntry::AndOthers => {
                println!("    ... and others");
            }
        }
    }
    println!();

    // Example 3: Demonstrating helper methods
    println!("Example 3: Using helper methods");
    let entry3 = bibliography.get("chapter1").unwrap();
    let authors = entry3.author_list().unwrap();
    println!("  Title: {}", entry3.title().unwrap().format_verbatim());
    println!("  Total entries: {}", authors.len());
    println!(
        "  Named authors: {}",
        authors.iter().filter(|a| !a.is_and_others()).count()
    );
    println!("  Has 'and others': {}", authors.iter().any(|a| a.is_and_others()));

    // Get just the person entries
    let people: Vec<_> = authors.iter().filter_map(|a| a.as_person()).collect();
    println!("  First author: {} {}", people[0].given_name, people[0].name);
    println!(
        "  Last named author: {} {}",
        people.last().unwrap().given_name,
        people.last().unwrap().name
    );
    println!();

    // Example 4: Backward compatibility
    println!("Example 4: Backward compatibility with author() method");
    let old_style_authors = entry1.author().unwrap();
    println!("  Old method (author()) returns {} entries", old_style_authors.len());
    println!("  Note: 'others' is treated as a person name");
    println!("  Last entry name: '{}'", old_style_authors.last().unwrap().name);
    println!();

    // Example 5: Roundtrip test
    println!("Example 5: Roundtrip serialization");
    let serialized = bibliography.to_biblatex_string();
    let reparsed = Bibliography::parse(&serialized).unwrap();
    let reparsed_entry = reparsed.get("paper1").unwrap();
    let reparsed_authors = reparsed_entry.author_list().unwrap();
    println!(
        "  Original 'and others' count: {}",
        authors.iter().filter(|a| a.is_and_others()).count()
    );
    println!(
        "  Reparsed 'and others' count: {}",
        reparsed_authors.iter().filter(|a| a.is_and_others()).count()
    );
    println!(
        "  Roundtrip successful: {}",
        authors.iter().filter(|a| a.is_and_others()).count()
            == reparsed_authors.iter().filter(|a| a.is_and_others()).count()
    );
}
