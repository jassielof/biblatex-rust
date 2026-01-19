# [\#47 Issue](https://github.com/typst/biblatex/issues/47) `open`: Improve error messages

#### <img src="https://avatars.githubusercontent.com/u/10547444?u=f3d5b5fd734ab0c2659cbd1ee08194d67cdd337f&v=4" width="50">[LasseRosenow](https://github.com/LasseRosenow) opened issue at [2023-12-06 15:27](https://github.com/typst/biblatex/issues/47):

When using typst, the error messages received by the `biblatex` crate are not good yet.

For example this code:

```bibtex
@article{asdf,
  month     = {Sep--Oct},
}
```

Produces the following error in `typst`:
```
Failed to parse BibLaTeX file (test.bib:1: invalid number)
```

In issue typst/typst#2849 I was informed, that this partially relies on the `biblatex` crate not yet being able to return good error messages.

This issues purpose is to track this :)




-------------------------------------------------------------------------------



[Export of Github issue for [typst/biblatex](https://github.com/typst/biblatex). Generated on 2026.01.15 at 20:45:12.]
