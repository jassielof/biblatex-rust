# [\#76 Issue](https://github.com/typst/biblatex/issues/76) `open`: Fix `to_biblatex_string` and `to_bibtex_string`

#### <img src="https://avatars.githubusercontent.com/u/13305603?v=4" width="50">[topoinix](https://github.com/topoinix) opened issue at [2025-05-05 16:14](https://github.com/typst/biblatex/issues/76)

Hello,

I have a rather large and incomplete `bib` file, and I am writing a tool that fetches bibliographic data using DOI, arXiv, MathSciNet and similar APIs to fill in missing fields in the original file. `biblatex` seems the most developed `rust` crate that supports [biblatex]( https://ctan.org/pkg/biblatex?lang=en). I would like to use it, especially for the provided structures like `Person` and `Date`.

The main issue I am facing is that `biblatex` does not preserve the parsed fields, i.e. when a `bib` file is parsed as `Bibliography` and later serialised using `to_biblatex_string` or `to_bibtex_string`, the resulting string problematically differs from the original one. For example,

```rs
fn main() {
    use biblatex::Bibliography;
    let src = r#"
    @article{key1,
    title = {Explicit homotopy limits of $\mathrm{dg}$-categories and twisted complexes},
    }
    @article{key2,
    title = {Hyper-K\"ahler Fourfolds Fibered by Elliptic Products},
    }
    "#;
    let bibliography = Bibliography::parse(src).unwrap();
    println!("{}", bibliography.to_biblatex_string());
}
```

outputs

```bib
@article{key1,
title = {Explicit homotopy limits of $\\mathrm\{dg\}$-categories and twisted complexes},
}

@article{key2,
title = {Hyper-Kähler Fourfolds Fibered by Elliptic Products},
}
```

The output differs not only by escaped Unicode characters but also by adding solidus and affecting braces. In the first case, the mathematical expression is messed up. The result in the second is not `BibTeX`-valid even when `to_bibtex_string` is used, and compiling the second case with `bibtex` results in an error:

```log
document.bbl: error: 61: Invalid UTF-8 byte sequence (��h). \newblock Hyper-k��h
```

Given the project name, it is understandable if it does support `BibTeX`. Still, having the function `to_bibtex_string` available might be confusing when its results is not `BibTeX`-valid.

I would like to use the structures `Person` and `Date` as they simplify comparisons. Still, I want to preserve the fetched fields, as I want to use the final result with `bibtex` or `biber`. Also, as a basic check, I would like to be able to export the parsed bibliography before processing it and compare it with the original file to ensure that it was read completely. This is because some of the entries I have contain duplicate fields, and currently, some of the duplicate fields are silently ignored.

The issue seems to stem from the fact that functions used for parsing, like `ContentParser::parse_impl` and `resolve::flatten`, make irreversible changes. While this issue does not exist for `RawBibliography`, and in principle, I should be able to use it, `RawBibliography` lacks the desired structures and serialisation functions.

I realise that my objective does not necessarily align with this project’s as I would like the exported `bib` file to be usable with `LaTeX` rather than `typst`. Still, I think, in general, it’s easier to reason about the code if processing is separated from parsing. Also, a basic search on `Google` and `GitHub` suggests that `to_biblatex_string` and `to_bibtex_string` are not used by other `typst` repositories, so fixing the above does not seem to conflict with relevant projects.

If addressing the above is of interest, one may proceed in at least one of two directions,

- Add the structures and serialisation functions to `RawBibliography`, in which case, `Bibliography` could be thought of as `ProcessedBibliography` or
- Remove the processing from all functions called by `Bibliography::parse` to a separate function, possibly `ChunksExt::process`, which would mainly be called by `ChunksExt::format_sentence` and `ChunksExt::format_verbatim`.

I would gladly like to help if you would like to proceed with either direction or an alternative solution for the issue above.

Thank you for your time and consideration!

#### <img src="https://avatars.githubusercontent.com/u/37874270?u=2a46acc8b3741d4aad5af2e707a57a7bfc888352&v=4" width="50">[saona-raimundo](https://github.com/saona-raimundo) commented at [2025-07-31 07:52](https://github.com/typst/biblatex/issues/76#issuecomment-3138914743)

To add another example to the above, I have problems with scaped spaces in non-verbatim fields.
Input

```
@techreport{r:86,
  institution = "Dept.\ of Computer Science, Stanford Univ.",
}
```

Ouptut

```
@techreport{r:86,
    institution = {Dept.\\  of Computer Science, Stanford Univ.},
}
```

Problem
The TeX space control `\` is not recognized.

#### <img src="https://avatars.githubusercontent.com/u/38179369?u=fff7b5364453da0f7855d082dfdf7bea470c30d0&v=4" width="50">[clysto](https://github.com/clysto) commented at [2025-09-04 07:32](https://github.com/typst/biblatex/issues/76#issuecomment-3252310081)

Is there a planned fix? @saona-raimundo  @anwaralameddin

#### <img src="https://avatars.githubusercontent.com/u/37874270?u=2a46acc8b3741d4aad5af2e707a57a7bfc888352&v=4" width="50">[saona-raimundo](https://github.com/saona-raimundo) commented at [2025-09-04 16:55](https://github.com/typst/biblatex/issues/76#issuecomment-3254604919)

Hi @clysto
I tried to look into it a bit, but there was no obvious easy solution.
For starters, I am not sure if the fields should be parsed as full tex input or there is a well-defined subset of macros that can be parts of the fields in bib(la)tex.
If there is reasonable specification somewhere, please do share it, as I think it is the first step to propose a solution.

-------------------------------------------------------------------------------

[Export of Github issue for [typst/biblatex](https://github.com/typst/biblatex). Generated on 2026.01.15 at 20:45:12.]
