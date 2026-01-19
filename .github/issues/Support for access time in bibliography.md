# [\#54 Issue](https://github.com/typst/biblatex/issues/54) `open`: Support for access time in bibliography

#### <img src="https://avatars.githubusercontent.com/u/47303535?u=4e19f8e0a80245052286ee36fc06e78eef81430d&v=4" width="50">[robertosw](https://github.com/robertosw) opened issue at [2024-05-09 16:03](https://github.com/typst/biblatex/issues/54):

### Description

I want to show what time I accessed a resource.

As hayagriva does not support the use of a time, I have switched to the BibTeX format.
Using this entry compiles without errors:
```.bib
@online{online-source,
    author    = "Donald Knuth",
    title     = "Knuth: Computers and Typesetting",
    url       = "http://www-cs-faculty.stanford.edu/~uno/abcde.html",
    urldate   = {2024-05-09T14:30:00},
}
```
But the bibliography only renders the date:
![Bildschirmfoto vom 2024-05-09 17-58-20](https://github.com/typst/typst/assets/47303535/642972b3-8b1b-440b-9238-e717d1987207)

I have used the `IEEE` style and tried several others and none of them rendered the time, so I suspect it is not a matter of the style used.

I have already added an issue in the hayagriva repo to add support for the access time: https://github.com/typst/hayagriva/issues/160



### Use Case

To be able to use a more detailed access timestamp for online ressources

#### <img src="https://avatars.githubusercontent.com/u/20374810?v=4" width="50">[quachpas](https://github.com/quachpas) commented at [2024-05-13 09:44](https://github.com/typst/biblatex/issues/54#issuecomment-2107154967):

This should probably be transferred to https://github.com/typst/biblatex

If I understand well:
1. The field `urldate` is parsed for year, month and day; no information about the time is parsed
2. Parsing a Datetime is already implemented in `date.rs`
3. In biblatex, options `dateusetime` and `seconds` must be set to `true` in order to print the full timestamp.

[1] https://github.com/typst/biblatex/blob/main/src/macros.rs#L58
[2] https://github.com/typst/biblatex/blob/a43de9ca4fb5c981c39c4c162e6df54684015fc8/src/types/date.rs#L37

#### <img src="https://avatars.githubusercontent.com/u/70302993?v=4" width="50">[wnhrt](https://github.com/wnhrt) commented at [2024-08-19 13:13](https://github.com/typst/biblatex/issues/54#issuecomment-2296549171):

Is there a workaround to enable those options in a typst project or does it have to be hard-coded?

#### <img src="https://avatars.githubusercontent.com/u/20374810?v=4" width="50">[quachpas](https://github.com/quachpas) commented at [2024-08-20 09:45](https://github.com/typst/biblatex/issues/54#issuecomment-2298436588):

@wnhrt Only the date fields are parsed in the biblatex crate, hence there are no options to enable this.
If you have the time, don't hesitate to draft a PR implementing this!


-------------------------------------------------------------------------------



[Export of Github issue for [typst/biblatex](https://github.com/typst/biblatex). Generated on 2026.01.15 at 20:45:12.]
