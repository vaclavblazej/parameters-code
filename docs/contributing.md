# How to add missing information

Easiest way is to go to [raise a project issue](https://github.com/vaclavblazej/parameters-code/issues).
Those are reviewed and added to the project.

For more technically inclined one may also create a pull request for changes of the `collection.rs` file.
To accommodate that let us discuss how the hierarchy information is stored.

## Collection

Currently all the information is stored in `/code/src/collection.rs` source file.
The information is created using *builder* pattern that makes the input less teadeous.

```
let mut create = Builder::new(); // The builder is created once at the beginning.
...
let connected = create.graph_class("KlMP0i", "connected");  // Create graph class
let forest = create.graph_class("JngPPm", "forest");        // Create graph class
let tree = create.intersection("rJyICu", &connected, &forest, "tree"); // Intersection of sets
let vertex_cover = create.parameter("4lp9Yj", "vertex cover"); // Create parameter
```

All the data is associated with its 6-alphanumeric code.
This id is meant to keep consistency in case of changes -- e.g. a parameter may change its name through time but its id stays the same.

Once the variables for parameters are created we may add further information through sources.
Source is meant to represent a scientific publication.
Structure uses a *builder* pattern to make it convenient to input all information about a publication.
References to the publications are kept in file `/handmade/main.bib` and it is *necessary* to add a biblatex citation there if we want to use a source in the code.

```
let mysource1993 = create.source("a3idzk", "BiblatexKey1993")
    .defined("L7aY6D", Unknown, &treewidth, "..., the treewidth of an undirected graph ...")
    .showed("RgLQ2P", Pp(21), &pathwidth, &treewidth, UpperBound(Linear), "Theorem 4")
    .showed("", Unknown, &treewidth, &pathwidth, Exclusion, "Observation 3")
    .cited("pJxHVS", Unknown, othersource1991, "Based on the work by [Sa19] ...")
    .done();
```

Methods require values that we discuss next

* The 6-alphanumeric id can be omitted (put `""` in its place) in contributions.
* Page number is `Pp(page)` or `Unknown` or `NotApplicable`.
* Reference to variable of the parameter in question, e.g. `&treewidth` (note the ampersand). The relations are given in `superset, subset` order (discussed later). Note that each pair of parameters have a relation in both directions, these are treated separately.
* In parameter blowup we indicate how much the parameter can change. Examples follow.
    * If parameter A bounds parameter B then in a typical case we input `A, B, UpperBound(asymptotic_blowup)`.
    * If parameter A is equivalent to B then `A, B, Equivalence`
    * If bounded parameter A does *not* mean B is bounded then we put `A, B, Exclusion`
    * If A bounds B and we know some lower bound we put e.g. `A, B, Bounds(Polynomial, Exponential)`
* Context of the fact either lists the entire definition/proof (if short) or can just refer to where we can find that fact in the cited publication. This string can use latex however each backslash `\` must be escaped, meaning instead of one we input two backslashes `\\`.

