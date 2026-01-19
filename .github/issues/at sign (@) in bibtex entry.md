# [\#83 Issue](https://github.com/typst/biblatex/issues/83) `open`: Allow "at sign" `@` in BibTeX entry

#### <img src="https://avatars.githubusercontent.com/u/4743113?u=e32e63a5305963e450106342af33555233e8362a&v=4" width="50">[4e554c4c](https://github.com/4e554c4c) opened issue at [2025-06-03 14:46](https://github.com/typst/biblatex/issues/83):

BibTeX doesn't support `%` as a comment character. Instead, the ordinary [suggestion](https://tex.stackexchange.com/a/21710) is to comment out fields by using `@`. Unfortunately the biblatex parser (and, by extension, hayagriva and typst) fails to parse BibTeX fields that begin with an `@`.

This crate seems to [support](https://github.com/typst/biblatex/pull/80/) the `%` character for commenting, but if it does not support `@` for commenting then interoperability with BibTeX and Biber is limited. 

## Proposal

```bibtex
@article{foo,
   title={bar},
  year={2025},
  @location={Mom's Basement}
}
````
should parse as two entries, with the third "commented" entry removed.

#### <img src="https://avatars.githubusercontent.com/u/9021226?u=b0bb0f3e28c0714ce0ff8f3356d550c0afa88309&v=4" width="50">[PgBiel](https://github.com/PgBiel) commented at [2025-06-03 16:50](https://github.com/typst/biblatex/issues/83#issuecomment-2936290417):

Your link suggests that unknown fields are ignored, not that you should use `@`. Writing `zzz-location={...}` would work here for example. Is that not enough? Or are there cases where this `@` is actually needed?

#### <img src="https://avatars.githubusercontent.com/u/4743113?u=e32e63a5305963e450106342af33555233e8362a&v=4" width="50">[4e554c4c](https://github.com/4e554c4c) commented at [2025-06-03 19:00](https://github.com/typst/biblatex/issues/83#issuecomment-2936756413):

The problem is not whether this is enough to "comment" out a field. Rather, it is an issue concerning compatibility between this library and actual BibTeX parsers. There are many bibliographies in the wild (mine included) which have these `@` signs in field names, and if this crate is not able to parse them then that creates a lack of compatibility.

#### <img src="https://avatars.githubusercontent.com/u/9021226?u=b0bb0f3e28c0714ce0ff8f3356d550c0afa88309&v=4" width="50">[PgBiel](https://github.com/PgBiel) commented at [2025-06-03 21:17](https://github.com/typst/biblatex/issues/83#issuecomment-2937229794):

Okay. Just to better understand it, do you have more details about this syntax? Is it only valid at the start of a line, or of a field name, or can it also be used at the end of a line after a field? And do you have some more examples if there are other cases?

#### <img src="https://avatars.githubusercontent.com/u/4743113?u=e32e63a5305963e450106342af33555233e8362a&v=4" width="50">[4e554c4c](https://github.com/4e554c4c) commented at [2025-06-04 00:53](https://github.com/typst/biblatex/issues/83#issuecomment-2937911115):

I've done some deep digging into `biber`, and its `Text::BibTex` library. The lexer appears to actually have two distinct comment mechanisms The first is `%` comments, and the second is toplevel declarations of the form `@comment(....)` where anything within the `....` has balanced parentheses and braces.

Other than that, I think `biber` doesn't really like `@` in field names. I think this is something that only works with `bibtex`, and I belive that `bibtex` skips to the next line if it sees an `@` sign anywhere other than toplevel.


-------------------------------------------------------------------------------



[Export of Github issue for [typst/biblatex](https://github.com/typst/biblatex). Generated on 2026.01.15 at 20:45:12.]
