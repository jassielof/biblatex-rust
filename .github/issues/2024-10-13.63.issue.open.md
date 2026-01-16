# [\#63 Issue](https://github.com/typst/biblatex/issues/63) `open`: Not possible to resolve `@String` which is defined in style file

#### <img src="https://avatars.githubusercontent.com/u/103174551?u=55a1ca2e5a4cba8b97cbd7477dca5b2666250882&v=4" width="50">[lukeflo](https://github.com/lukeflo) opened issue at [2024-10-13 07:55](https://github.com/typst/biblatex/issues/63):

First of all, this is very likely not a direct issue of the `biblatex` crate itself, but, nevertheless, might be of interest for some users.

I'm working on an own little Rust project writing a TUI for fast and easy interaction with someones BibLaTeX database and use this great crate for parsing the input files. While testing it, a friend encountered a little problem with `@String`s.

If the `@String` is defined in the `.bib` file itself, the crate can parse the entry without any errors. But there are some BibLaTeX styles which offer predefined set of `@String`s in their `.bst` files. 

Using a `.bib` file containing those abbreviations defined in the style file without (re)defining them in the `.bib` file itself leads to a panic and aborts the program:

```
➜  release git:(main) ./bibiman ~/@mycene/00-09_system/01_emacs/01.02_bib/auf-dem-forum-thesis.bib

thread 'main' panicked at src/backend/bib.rs:40:74:
called `Result::unwrap()` on an `Err` value: ParseError { span: 1780..1785, kind: UnknownAbbreviation("AEspA") }

note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

In this case the BibLaTeX style [`archaeologie`](https://github.com/LukasCBossert/biblatex-archaeologie/blob/cedcb5052fb81e1ba6de17dff478a03096fd5610/archaeologie.dtx#L1918) was used which defines a large set of `@String`s.

This is very unlikely to be solved by the `biblatex` crate itself, since it definitely cannot source/parse all BibLaTeX style files. But it might be worth knowing for someone who wants to use the crate or Typst with a specific `.bib` file containing those `@String`s




-------------------------------------------------------------------------------



[Export of Github issue for [typst/biblatex](https://github.com/typst/biblatex). Generated on 2026.01.15 at 20:45:12.]
