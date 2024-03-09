# HOPS code

*Hierarchy of Parameters* (HOPS) is meant to provide overview of structural parameters, their relations, and auxiliary information.
This repository is the true source of the [website](https://vaclavblazej.github.io/parameters/) content and is the correct place to give [feedback](https://github.com/vaclavblazej/parameters/issues).

The project consists of three parts:

* [Hierarchy of Parameters website](https://vaclavblazej.github.io/parameters/)
* [Code that generates website content (this repository)](https://github.com/vaclavblazej/parameters-code)
* [Scripts that generate the website](https://github.com/vaclavblazej/parameters)

The web is meant to provide:

* a quick overview of
    * (partially done) parameter definitions
    * (partially done) bounding connections
    * (in future) graph classes that distinguish parameters (via ISGCI)
* (current focus) display relevant references
* (in future) single PDF
* (in future) interactive mode
    * let users with visualize the boundary of tractability and hardness for their problems
    * customize the view by hiding and positioning the nodes
    * output TikZ code that can be directly used in a scientific paper

It will take a while before the website has the majority of the parameter relations and their references.
To give your suggestions and fixes (with references) please open a [github issue](https://github.com/vaclavblazej/parameters/issues) or [mail us](vaclav.blazej@warwick.ac.uk).

## Repository folders and files

* main content
    * `code` source data and codes that export the data into human-readable form to the `web/content` folder
    * `handcrafted` are the non-generated parts of the website
* extras
    * `.git`, `.gitignore`, `.gitmodules` git versioning data
    * `README.md` documentation
    * `web` git submodule that points to the repository with the website sources

## Inspired by

* 2013-2019 [Parameterized Hierarchy](https://manyu.pro/assets/parameter-hierarchy.pdf) by M. Sorge.
* [Comparing Graph Parameters](https://fpt.akt.tu-berlin.de/publications/theses/BA-Schr%C3%B6der.pdf) by J. Ch. B. Schröder
* 2010 [Comparing 17 graph parameters](https://core.ac.uk/download/pdf/30926677.pdf) by Róbert Sasák

## Generating content

* Install `rust`, `pdflatex`, and `graphviz`
* Change to the project directory
* Change to code subdirectory `cd code`
* Compile and run the generating code `cargo run`

## License

This project is licensed under [MIT](LICENSE) license.

## Authors and Contributors

* [Václav Blažej](https://blazeva1.pages.fit/)

information contributors

* [Šimon Schierreich](https://pages.fit.cvut.cz/schiesim/)
* Jan Pokorný
