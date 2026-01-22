# Hierarchy of Parameters -- code

*Hierarchy of Parameters* (HOPS) is a database of structural graph parameters, properties, and their relations, with references to publications and related projects.

The project consists of three main parts:

* [Database and code that generates the website's content](https://github.com/vaclavblazej/parameters-code) (this repository)
* [Scripts that generate the website](https://github.com/vaclavblazej/parameters) (intermediate)
* [Hierarchy of Parameters](https://vaclavblazej.github.io/parameters/) (website)

To give **feedback** on any part of the project, you can use:

* [Google form](https://docs.google.com/forms/d/e/1FAIpQLSdX5_IoxMmlguQGQzR1NhvbeQRiHTQlytK2jAOZAgZfjdcGDQ/viewform?usp=sharing&ouid=114574344732763059842) (anonymous)
* [GitHub issues](https://github.com/vaclavblazej/parameters-code/issues) (requires a GitHib account)

## The web is meant to provide

### Overview of parameters

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

