# How to Contribute

## Reporting Issues

The easiest way to contribute is to [raise a project issue](https://github.com/vaclavblazej/parameters-code/issues).

## Making Changes

For more technically inclined contributors, you can create a pull request with changes to the `collection.rs` file.
This section explains how the hierarchy information is stored.

## Collection Structure

All information is stored in `/code/src/collection.rs`.
The data is created using a *builder* pattern for convenient input.

```rust
let mut create = CollectionBuilder::new();

// Create graph classes with: id, name, score, definition
let forest = graph_class("JngPPm", "forest", 5, "A graph without cycles.").done(&mut create);

// Create graph properties
let connected = graph_property("KlMP0i", "connected", 5, "A graph where every pair of vertices has a path between them.").done(&mut create);

// Create parameters with: id, name, score, definition
let vertex_cover = parameter("4lp9Yj", "vertex cover", 7, "Minimum number of vertices...").done(&mut create);
```

### Identifiers

All data is associated with a 6-character alphanumeric ID.
This ID maintains consistency when names change over time.
When adding new items, you can generate a random ID or leave it empty (`""`) - maintainers will assign one.

### Scores

Each item has a score from 0-9 indicating its relevance/importance.
Higher scores mean the item appears more prominently in diagrams and tables.

## Adding Sources

Sources represent scientific publications.
Reference citations must be added to `/handcrafted/main.bib` in BibLaTeX format before they can be used in code.

```rust
create.source("a3idzk", "BiblatexKey1993")
    .wrote(Pp(1), // page of the result, can be Unknown or NotApplicable,
        "Corollary 3. For any graph $G$ ...", // short text of the result
        vec![ // list of definitions and relations implied by the text
            (
                "Xdg7Hv", // result id
                Original, // status
                definition(&chordality) // result
            ),
            ("D5VlqV", Original, relation(&size, &chordality, UpperBound(Linear)),
        ])
    .done(&mut create);
```

The status dictates the attribution for the result.
It can be 

| RawWroteStatus | Description |
| -------------- | ----------- |
| Assumed | taken as given by HOPS, mainly due to being out of project's scope |
| Conjectured | posed as an open problem |
| Original | first or independent |
| Derivative | improvements or later proofs |
| Noted(RawNotedSource) | results claimed to be somewhere else |
| TodoStatus | is to be filled by the mainteiners or contributors |

### LaTeX in Text

Text fields can use LaTeX notation.
Each backslash must be escaped: use `\\` instead of `\`.

## Development Workflow

1. Edit `code/src/collection.rs`
2. Run `cd code && cargo run fast` to regenerate the website
3. Preview locally with `cd web && hugo server`
4. Submit a pull request

