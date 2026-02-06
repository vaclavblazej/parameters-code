# Hierarchy of Parameters (HOPS)

A database of structural graph parameters, their properties, and relationships in parameterized complexity theory.

**[View the Website](https://vaclavblazej.github.io/parameters/)**

## About

HOPS provides a comprehensive reference for researchers working with graph parameters. The database includes:

* Parameter definitions (treewidth, pathwidth, clique-width, etc.)
* Bounding relationships between parameters
* References to relevant publications
* Visual diagrams showing parameter hierarchies

## Repository Structure

```
parameters/
├── code/           # Rust database and content generator
├── handcrafted/    # Manual website content and bibliography (main.bib)
├── web/            # Generated Hugo website (git submodule)
├── docs/           # Project documentation
└── scripts/        # Utility scripts
```

## Quick Start

### Prerequisites

* [Rust](https://rustup.rs/) (latest stable)
* [Graphviz](https://graphviz.org/) (for diagram generation)
* [Hugo](https://gohugo.io/) (for local website preview)

### Generate the Website

```bash
cd code
cargo run all
```

### Preview Locally

```bash
cd web
hugo server
```

## Contributing

Contributions are welcome!
You can help by:

* **Adding parameters** - Edit `code/src/collection.rs` using the builder API
* **Adding relations** - Document bounds between parameters with source references
* **Improving content** - Enhance descriptions or add missing information
* **Reporting issues** - Submit bug reports or feature requests

## Documentation

* [Contributing Guide](docs/contributing.md) - How to add parameters and relations
* [Codebase Overview](docs/code.md) - Technical architecture and module structure

## Feedback

* [GitHub Issues](https://github.com/vaclavblazej/parameters-code/issues) - Bug reports, feature requests
* [Feedback Form](https://docs.google.com/forms/d/e/1FAIpQLSdX5_IoxMmlguQGQzR1NhvbeQRiHTQlytK2jAOZAgZfjdcGDQ/viewform) - Anonymous feedback

## Related Work

This project builds upon earlier work in the field:

* [Parameterized Hierarchy](https://manyu.pro/assets/parameter-hierarchy.pdf) (2013-2019) by Manuel Sorge
* [Comparing Graph Parameters](https://fpt.akt.tu-berlin.de/publications/theses/BA-Schr%C3%B6der.pdf) by J. Ch. B. Schröder
* [Comparing 17 Graph Parameters](https://core.ac.uk/download/pdf/30926677.pdf) (2010) by Róbert Sasák

## License

[MIT License](LICENSE) - Copyright (c) 2023-2026 Vaclav Blazej & contributors

