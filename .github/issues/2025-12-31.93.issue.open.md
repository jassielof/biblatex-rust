# [\#93 Issue](https://github.com/typst/biblatex/issues/93) `open`: Poor error message: "failed to parse BibLaTeX (missing number)"

#### <img src="https://avatars.githubusercontent.com/u/73519753?u=4ef62eb32135ad337a19c2c5cd18af72cf8d446a&v=4" width="50">[tak2hu](https://github.com/tak2hu) opened issue at [2025-12-31 03:52](https://github.com/typst/biblatex/issues/93):

### Description

I tried copy pasting this bibtex from https://universe.roboflow.com/sumit-cpavr/cleaning-ghas1
```
@misc{cleaning-ghas1_dataset,
  title = { cleaning Dataset },
  type = { Open Source Dataset },
  author = { sumit },
  howpublished = { \url{ https://universe.roboflow.com/sumit-cpavr/cleaning-ghas1 } },
  url = { https://universe.roboflow.com/sumit-cpavr/cleaning-ghas1 },
  journal = { Roboflow Universe },
  publisher = { Roboflow },
  year = { 2025 },
  month = { nov },
  note = { visited on 2025-12-31 },
}
```

after `typst compile main.typ`  it just shows the first bibtex entry without showing which key was invalid:
```
error: failed to parse BibLaTeX (missing number)
  ┌─ refs.bib:2:4
  │
2 │ @software{yolov8_ultralytics,
  │     ^

help: error occurred in this call of function `bibliography`
   ┌─ main.typ:35:16
   │
35 │   bibliography: bibliography("refs.bib"),
   │                 ^^^^^^^^^^^^^^^^^^^^^^^^
```

after changing `month = { nov }` to `month = { 11 }`  it can compile, just poor error message.

notes:
```
typst --version
typst 0.14.2 (unknown hash)
```

### Reproduction URL

_No response_

### Operating system

Linux

### Typst version

- [x] I am using the latest version of Typst

#### <img src="https://avatars.githubusercontent.com/u/2191754?v=4" width="50">[Enivex](https://github.com/Enivex) commented at [2025-12-31 06:35](https://github.com/typst/biblatex/issues/93#issuecomment-3709696095):

This should be moved to https://github.com/typst/biblatex

https://github.com/typst/biblatex/issues/47 is relevant, but in the present case I believe it's not just an error message issue. Three letter month names are valid iirc.

#### <img src="https://avatars.githubusercontent.com/u/2191754?v=4" width="50">[Enivex](https://github.com/Enivex) commented at [2026-01-07 10:33](https://github.com/typst/biblatex/issues/93#issuecomment-3718253557):

I think this is a bug @laurmaedje . If I read the specification right, nov should be valid

#### <img src="https://avatars.githubusercontent.com/u/17899797?u=ea942abcf76f046f173918483824b2fb5beb791a&v=4" width="50">[laurmaedje](https://github.com/laurmaedje) commented at [2026-01-07 10:40](https://github.com/typst/biblatex/issues/93#issuecomment-3718277363):

If I'm reading things correctly, `month = nov` would be correct (it also works), but `month = { nov }` is incorrect.

> For backwards compatibility, you may also
use the following three-letter abbreviations in the month field: jan, feb, mar, apr, may,
jun, jul, aug, sep, oct, nov, dec. Note that these abbreviations are BibTeX strings
which must be given without any braces or quotes. When using them, don’t say
month={jan} or month="jan" but month=jan.

From https://mirrors.ibiblio.org/CTAN/macros/latex/contrib/biblatex/doc/biblatex.pdf

#### <img src="https://avatars.githubusercontent.com/u/2191754?v=4" width="50">[Enivex](https://github.com/Enivex) commented at [2026-01-07 10:44](https://github.com/typst/biblatex/issues/93#issuecomment-3718291142):

My bad!


-------------------------------------------------------------------------------



[Export of Github issue for [typst/biblatex](https://github.com/typst/biblatex). Generated on 2026.01.15 at 20:45:12.]
