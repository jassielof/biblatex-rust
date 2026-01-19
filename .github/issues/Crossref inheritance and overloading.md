# [\#51 Issue](https://github.com/typst/biblatex/issues/51) `open`: Crossref inheritance and overloading

#### <img src="https://avatars.githubusercontent.com/u/20145354?u=437e931d93ab646399da347a1e79820a35345e2a&v=4" width="50">[wrenger](https://github.com/wrenger) opened issue at [2024-03-05 15:25](https://github.com/typst/biblatex/issues/51):

Hi, I'm a bit confused about the inheritance order of crossref. If both entries contain the same field with different values (like year), which one should be used?

Here is an example:

```rust
let bib = r#"
    @inproceedings{foo,
        author = {Max Müller},
        title = {Lorem Ipsum et Dolor},
        month = sep,
        year = 2005,
        crossref = {ref},
    }
    @proceedings{ref,
        month = jan,
        year = 2001,
        title = {Book Title},
        category = {baz},
    }
"#;
let parsed = Bibliography::parse(bib).unwrap();
println!("{parsed:#?}");
```

In this example, I would expect that the year from the referrer `foo` overwrites the year from the referee `ref`.

However, this is not the case:

```
Bibliography {
    entries: [
        Entry {
            key: "foo",
            entry_type: InProceedings,
            fields: {
                "author": [
                    Normal(
                        "Max Müller",
                    ) <43..54>,
                ],
                "booktitle": [
                    Normal(
                        "Book Title",
                    ) <252..262>,
                ],
                "crossref": [
                    Normal(
                        "ref",
                    ) <159..162>,
                ],
                "date": [
                    Normal(
                        "2001-01",
                    ) <18446744073709551615..18446744073709551615>,
                ],
                "title": [
                    Normal(
                        "Lorem Ipsum et Dolor",
                    ) <74..94>,
                ],
            },
        },
        Entry {
            key: "ref",
            entry_type: Proceedings,
            fields: {
                "category": [
                    Normal(
                        "baz",
                    ) <285..288>,
                ],
                "month": [
                    Normal(
                        "January",
                    ) <209..212>,
                ],
                "title": [
                    Normal(
                        "Book Title",
                    ) <252..262>,
                ],
                "year": [
                    Normal(
                        "2001",
                    ) <229..233>,
                ],
            },
        },
    ],
    keys: {
        "foo": 0,
        "ref": 1,
    },
}
```

PS: Also, the `category` is not inherited.

PS2: PS: Thank you for this crate. This is a good replacement for the very slow bibtexparser python package.




-------------------------------------------------------------------------------



[Export of Github issue for [typst/biblatex](https://github.com/typst/biblatex). Generated on 2026.01.15 at 20:45:12.]
