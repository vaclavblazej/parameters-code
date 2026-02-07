# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

HOPS (Hierarchy of Parameters) is a Rust database and code generator for a website documenting structural graph parameters, their properties, and relationships in parameterized complexity theory. The project generates web content from a structured database defined in code.

Repository structure:
- `code/` - This Rust project (database + generator)
- `handcrafted/` - Manual website content and bibliography (`main.bib`)
- `web/` - Generated website (Hugo, git submodule)

## Build Commands

```bash
cargo build                    # Build
cargo run                      # Run with no arguments
cargo run fast                 # Clear + preprocess + dots + pages (common dev workflow)
cargo run all                  # Full generation including API and tables
cargo clippy                   # Lint
cargo test                     # Run tests
cargo test <test_name>         # Run specific test
```

**Development with bacon** (hot-reload):
```bash
bacon                          # Auto-runs `check` on file changes
bacon clippy-all               # Run clippy on all targets
bacon test                     # Run tests
```

All bacon jobs use `--target-dir /tmp/hops` for faster compilation.

## CLI Arguments

- `preprocess` - Process raw data into typed structures
- `dots` - Generate GraphViz DOT diagram files
- `pages` - Generate markdown pages
- `table` - Generate relation tables
- `api` - Generate JSON API
- `clear` - Clear output directories
- `fast` - Shortcut: clear + preprocess + dots + pages
- `all` - Full generation
- `debug` / `trace` - Enable verbose logging
- `interactive` / `i` - Enter interactive command mode

## Architecture

Data flows through 5 layers:

```
COLLECTION (src/collection.rs)  Database definitions (~166KB, defines all parameters)
       ↓
INPUT (src/input/)           Raw data structures, builder API
       ↓
DATA (src/data/)             Core typed structures (Parameter, Relation, etc.)
       ↓
WORK (src/work/)             Processing, relation inference, hierarchy computation
       ↓
OUTPUT (src/output/)         Markdown pages, API JSON, DOT diagrams, tables
```

### Key Modules

- **`collection.rs`** - The database itself. All graph classes, parameters, tags, relations, and bibliographic sources are defined here using the `CollectionBuilder` API. This is the primary file for adding/modifying data.

- **`input/build.rs`** - `CollectionBuilder` provides methods like `tag()`, `parameter()`, `relation()`, `source()` for building the database.

- **`data/id.rs`** - Strongly-typed ID system (e.g., `ParameterId`, `RelationId`) preventing ID confusion.

- **`data/data.rs`** - Core `Data` struct containing all processed parameters, relations, and metadata.

- **`work/processing.rs`** - Main processing pipeline transforming raw data into typed structures.

- **`work/inference.rs`** - Relation inference (transitive closure, implied bounds).

- **`output/markdown.rs`** - Primary page content generation.

### Data Model

- **Parameter** - A graph parameter (e.g., treewidth, pathwidth)
- **Relation** - A bound between parameters (e.g., `treewidth ≤ pathwidth`)
- **GraphClass** - A class of graphs (e.g., planar graphs)
- **Source** - Bibliography reference linking to `handcrafted/main.bib`

### Path Conventions

Output goes to `../web/content/`:
- Pages: `../web/content/html/`
- API: `../web/content/api/`
- Processing cache: `/tmp/tmp/data.json`

## Adding New Parameters

1. Edit `src/collection.rs`
2. Use the builder API:
   ```rust
   let my_param = create.parameter("uniqueId", "parameter name", "description").done(&mut create);
   ```
3. Add relations using `create.relation()` with source references
4. Run `cargo run fast` to regenerate
