# BibLaTeX

A Rust crate for parsing and writing BibTeX and BibLaTeX files.

BibLaTeX can help you to parse `.bib` bibliography files. As opposed to other available crates, this crate attempts to parse the data within the fields into easily usable structs and enums like `Person` and `Date` for downstream consumption.

## Usage

You can add this library as a dependecy as `biblatex`.

```rust
use biblatex::Bibliography;

let src = "@book{tolkien1937, author = {J. R. R. Tolkien}}";
let bibliography = Bibliography::parse(src).unwrap();
let entry = bibliography.get("tolkien1937").unwrap();
let author = entry.author().unwrap();
assert_eq!(author[0].name, "Tolkien");
```

This library operates on a `Bibliography` struct, which is a collection of _entries_ (the items in your `.bib` file that start with an `@` and are wrapped in curly braces). The entries may hold multiple fields. Entries have getter methods for each of the possible fields in a Bib(La)TeX file which handle possible field aliases, composition and type conversion automatically.

### External Abbreviations

Many BibLaTeX styles define `@string` abbreviations in separate style files (`.bst`). To use these external abbreviations, you can provide them when parsing:

```rust
use biblatex::Bibliography;

// Define external abbreviations from a style file
let external_abbrevs = vec![
    ("AEspA", "Archäologischer Anzeiger"),
    ("AJA", "American Journal of Archaeology"),
];

let src = r#"
@article{test,
  author = {Smith, John},
  title = {Archaeological Study},
  journaltitle = AEspA,
  date = 2024
}
"#;

let bibliography = Bibliography::parse_with_abbreviations(src, &external_abbrevs).unwrap();
```

Abbreviations defined in the `.bib` file itself will take precedence over external abbreviations.

### Alternative `@string` Syntax

This library supports an alternative syntax for `@string` definitions where multiple abbreviations can be defined in a single block (comma-separated):

```bibtex
@string{
  cup = {Cambridge University Press},
  oup = {Oxford University Press},
  mit = {MIT Press}
}
```

This is in addition to the standard BibTeX syntax where each abbreviation requires its own `@string` entry.

## Compliance and specification

This library is tested against BibLaTeX version 3.12 (2025-07-10) on CTAN.

## References

### BibLaTeX references

- [WikiBook section on LaTeX bibliography management](https://en.wikibooks.org/wiki/LaTeX/Bibliography_Management)
- [BibLaTeX on CTAN](https://ctan.org/pkg/biblatex)
  - [Documentation PDF (English)](https://mirrors.ctan.org/macros/latex/contrib/biblatex/doc/biblatex.pdf)

### BibTeX references

- [BibTeXing by Oren Patashnik](http://www.bibtex.org/Format/)

The generated documentation more specifically describes the selection and behavior of the getters but generally, they follow the convention of being the snake-case name of the corresponding field (such that the getter for `booktitleaddon` is named `book_title_addon`).

## Limitations

This library attempts to provide fairly comprehensive coverage of t he BibLaTeX spec with which most of the `.bib` files in circulation can be processed.

However, the crate currently has some limitations:

- There is no explicit support for entry sets, although it is easy to account for them by manually getting the `entryset` field and calling `parse::<Vec<String>>()` on it
