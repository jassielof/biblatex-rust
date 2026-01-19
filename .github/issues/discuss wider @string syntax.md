# [\#32 Issue](https://github.com/typst/biblatex/issues/32) `open`: Discuss wider `@string` syntax

#### <img src="https://avatars.githubusercontent.com/u/63655535?u=11bf577cb8af1d37ffb444425cd94774f61c5e5d&v=4" width="50">[Neved4](https://github.com/Neved4) opened issue at [2023-05-03 14:05](https://github.com/typst/biblatex/issues/32):

Best suited for _Discussions_, but I'm opening an issue for reference.

After reversing for a while I found this interesting one, and it's so cozy that I find it not a bug but a feature.

In BibTeX/BibLaTeX there's the [`@STRING`](https://ctan.javinator9889.com/biblio/bibtex/base/btxdoc.pdf) special entry, that defines abbreviations of strings and which the preprocessor will take care of.

So we can define our list of aliases or abbreviations:
```bib
@string{anch-ie = {Angew.~Chem. Int.~Ed.}}
@string{cup     = {Cambridge University Press}}
@string{dtv     = {Deutscher Taschenbuch-Verlag}}
@string{hup     = {Harvard University Press}}
@string{jams    = {J.~Amer. Math. Soc.}}
@string{jchph   = {J.~Chem. Phys.}}
@string{jomch   = {J.~Organomet. Chem.}}
@string{pup     = {Princeton University Press}}
```

And then we can use it instead of typing the full journal name, notice the field `journaltitle`:
```bib
@article{aksin,
  author       = {Aks{\i}n, {\"O}zge and T{\"u}rkmen, Hayati and Artok, Levent
                  and {\c{C}}etinkaya, Bekir and Ni, Chaoying and
                  B{\"u}y{\"u}kg{\"u}ng{\"o}r, Orhan and {\"O}zkal, Erhan},
  title        = {Effect of immobilization on catalytic characteristics of
                  saturated {Pd-N}-heterocyclic carbenes in {Mizoroki-Heck}
                  reactions},
  journaltitle = jomch,
  date         = 2006,
  volume       = 691,
  number       = 13,
  pages        = {3027-3036},
  indextitle   = {Effect of immobilization on catalytic characteristics},
}
```

At the same time, `typst` allows us an _alternative syntax_:
```bib
@string{
  anch-ie = {Angew.~Chem. Int.~Ed.},
  cup     = {Cambridge University Press},
  dtv     = {Deutscher Taschenbuch-Verlag},
  hup     = {Harvard University Press},
  jams    = {J.~Amer. Math. Soc.},
  jchph   = {J.~Chem. Phys.},
  jomch   = {J.~Organomet. Chem.},
  pup     = {Princeton University Press}
}
```

That will work and compile just fine, just like in the example above.

I'm not aware if this has unintended consequences or if there's good reasons to disallow it, personally I'd love that in the TeX world we supported that alternative syntax.

Again, I find it nice of `typst` 🧡, even if it won't work on BibTeX/BibLaTeX.

Examples come from [biblatex-examples.bib](https://raw.githubusercontent.com/plk/biblatex/dev/bibtex/bib/biblatex/biblatex-examples.bib).




-------------------------------------------------------------------------------



[Export of Github issue for [typst/biblatex](https://github.com/typst/biblatex). Generated on 2026.01.15 at 20:45:12.]
