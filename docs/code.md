# Codebase

This document provides a technical view on the project.

This project is written in [Rust](https://www.rust-lang.org/).

## Prerequisites

* [Rust](https://rustup.rs/) (latest stable)
* [Graphviz](https://graphviz.org/) (for diagram generation)
* [Hugo](https://gohugo.io/) (optional, for local website preview)

## Build Commands

```bash
cd code
cargo run <arguments>
```

| Argument | Description |
|----------|-------------|
| `preprocess` | Process raw data into typed structures |
| `dots` | Generate GraphViz DOT diagram files |
| `pages` | Generate markdown pages |
| `table` | Generate relation tables |
| `api` | Generate JSON API |
| `clear` | Clear output directories |
| `fast` | Shortcut: clear + preprocess + dots + pages |
| `all` | Full generation |
| `debug` / `trace` | Enable verbose logging |
| `interactive` / `i` | Enter interactive command mode |

## Code Structure

Data flows through 5 layers:

```
COLLECTION (src/collection.rs)  Database parameter definitions
       ↓
INPUT (src/input/)           Raw data structures, raw data builder
       ↓
WORK (src/work/)             Processing, relation inference, hierarchy computation
       ↓
DATA (src/data/)             Core typed structures (Parameter, Relation, etc.)
       ↓
OUTPUT (src/output/)         Markdown pages, API JSON, DOT diagrams, tables
```

### Key Modules

* **`collection.rs`** - The database itself. All graph classes, parameters, tags, relations, and bibliographic sources are defined here using the `CollectionBuilder` API. This is the primary file for adding/modifying data.

* **`input/build.rs`** - `CollectionBuilder` provides methods like `tag()`, `parameter()`, `graph_class()`, `source()` for building the database.

* **`data/id.rs`** - Strongly-typed ID system (e.g., `ParameterId`, `GraphClassId`) preventing ID confusion.

* **`data/data.rs`** - Core `Data` struct containing all processed parameters, relations, and metadata.

* **`work/processing.rs`** - Main processing pipeline transforming raw data into typed structures.

* **`output/markdown.rs`** - Primary page content generation.

### Data Model

* **Parameter** - A graph parameter (e.g., treewidth, pathwidth)
* **Relation** - A bound between parameters (e.g., `treewidth ≤ pathwidth`)
* **GraphClass** - A class of graphs (e.g., planar graphs)
* **Source** - Bibliography reference linking to `handcrafted/main.bib`
* **Tag** - Categorization for parameters and graph classes

### Path Conventions

Output goes to `../web/content/`:
* Pages: `../web/content/html/`
* API: `../web/content/api/`
* Processing cache: `/tmp/tmp/data.json`
