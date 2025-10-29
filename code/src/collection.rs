//! Collects the information we have about parameterized complexity classes, their inclusions,
//! and related graph classes, tags, bibliographic sources, theorems, proofs, and so on.

use crate::general::enums::{Cpx::*, CpxTime::*, Page::*};
use crate::input::build::{graph_class, parametric_graph_class, parameter, parameter_sequence, property, provider, source, copyvec};
use crate::input::{build::Builder, raw::RawData, raw::RawOwn::{Is, Has}, raw::RawNotedSource::*, sequence::ParameterType};

pub fn build_collection() -> RawData {
    let mut create = Builder::new();

    let tag_topology = create.tag("lJJaYb", "topology", "A graph is planar if it can be embedded into a plane without edges crossings. Related sets include those which restrict the embedding (e.g. geometric intersection graphs), consider other surfaces (e.g. toroid), or allow crossings (e.g. upper bound the total number of crossings).");
    let tag_coloring = create.tag("szWO2M", "coloring", "Coloring of a graph is its partition where each part is an independent set. Each part is considered to be a color so in a properly colored graph every edge goes between two differently colored vertices.");
    let tag_vertex_removal = create.tag("e26LM8", "vertex removal", "Some vertices of the graph may completely describe the complex structure of the graph, and the remainder may fall into a simple graph class. This includes variants of removing the vertices such as removing a set of vertices all at once (vertex cover), or removing vertices one by one (treedepth), resulting in a simple or a complex graph class (e.g. distance to X and elimination distance to X).");
    let tag_edge_removal = create.tag("hnX8tG", "edge removal", "The complexity of a graph may be described by the number of edges that we need to remove to get a simple graph class. This often means removing edges to that a forbidden structure is not contained in the graph, e.g., a cycle ([[HTk9PZ]]) or an odd cycle ([[Ve5ruW]]). Hitting each structure X is also called X-transversal.");
    let tag_edge_cover = create.tag("tK4S1r", "covering edges", "Take a graph and create a cover of its edges by graphs from a well-understood graph class.");
    let tag_vertex_cover = create.tag("TgTiqK", "covering vertices", "Partition vertices of the graph into smaller graph from a well-understood graph class.");
    let tag_vertex_order = create.tag("O1poSV", "vertex order", "Take an appropriate order of graph vertices to get a definition or a decomposition. Typical for some graph parameters (e.g. degeneracy) and graphs (e.g. chordal).");
    let tag_modules = create.tag("ZtdvKW", "module", "Module of a graph is its vertex set which has the same neighborhood with all vertices outside of it. As modules have a very regular structure they were used to create useful graph decompositions.");
    let tag_linear = create.tag("nAjQi4", "linear variant", "Graph decompositions often end up being trees, but still, some problems remain hard on those decompositions. Restricting the decomposition to be a path (or to be linearized in some other way) gives bigger power to the decomposition and makes some problems tractable.");
    let tag_tree_decomposition = create.tag("bzffn0", "tree decomposition", "The classical tree decomposition is tied to the treewidth which measures maximum size of a bag. There are parameters use different bag measures and there are graphs that are closely tied to tree decompositions.");
    let tag_branch_decomposition = create.tag("KaLXjx", "branch decomposition", "Branch decomposition is an unrooted tree with inner vertices of degree 3. Leaves represent (typically) edges of the graph. Every edge of this tree then implies a partition of the leaves into two sets -- which reflects in partition of the graph. We have some measure of how this partition os good and the decomposition which minimizes the measure over all its edges.");
    let tag_intersection = create.tag("QqAXvX", "intersection", "Graphs can be defined to have a vertex for some object and edges whenever two objects intersect.");
    let tag_structural = create.tag("P0XOsk", "structural", "Structural class (e.g. structurally bounded treewidth) contains all graphs that can be obtained from graphs of the class via transduction.");

    let connected = property("KlMP0i", "connected", Is, 2)
        .displayed_definition("mIBYMD", "There is a path between any pair of vertices.")
        .done(&mut create);
    let bipartite = graph_class("cLHJkW", "bipartite", 8)
        .displayed_definition("1iQ54v", "Can be partitioned into two independent sets.")
        .aka("odd-cycle-free")
        .tag(&tag_coloring)
        .done(&mut create);
    let block = graph_class("QrxQsH", "block", 3)
        .displayed_definition("2kG0kY", "Every block (maximal 2-connected subgraph) is a clique.")
        .done(&mut create);
    let chordal = graph_class("Cv1PaJ", "chordal", 6)
        .displayed_definition("TU8uiD", "Contains no induced cycle on 4 or more vertices.")
        .displayed_definition("RWtC9I", "Has a vertex order, called perfect elimination scheme, such that future neighborhoods are always cliques.")
        .displayed_definition("VBeQux", "Has a tree-decomposition such that every bag is a clique.")
        .displayed_definition("4pavjf", "Is an intersection graph of subtrees of a tree.")
        .tag(&tag_vertex_order)
        .tag(&tag_tree_decomposition)
        .tag(&tag_intersection)
        .done(&mut create);
    let cluster = graph_class("WAU7vf", "cluster", 6)
        .displayed_definition("roSFzV", "Disjoint union of complete graphs.")
        .displayed_definition("4aKorn", "$P_3$-induced-free.")
        .done(&mut create);
    let co_cluster = graph_class("7HR4uV", "co-cluster", 6)
        .displayed_definition("FDbIDy", "Complete multipartite graph.")
        .tag(&tag_coloring)
        .done(&mut create);
    let cograph = graph_class("9Qd0Mx", "cograph", 7)
        .displayed_definition("ZWeaib", "Created from single vertices by disjoint unions and complements.")
        .displayed_definition("2szioJ", "$P_4$-free")
        .tag(&tag_modules)
        .done(&mut create);
    let complete = create.intersection("EhdXNA", &connected, &cluster, "complete", 9)
        .displayed_definition("ARCCvF", "Contains all the edges.")
        .done(&mut create);
    let forest = graph_class("JngPPm", "forest", 9)
        .displayed_definition("NUxE1Q", "A graph with no cycle.")
        .done(&mut create);
    let tree = create
        .intersection("rJyICu", &connected, &forest, "tree", 7)
        .displayed_definition("tCAYae", "A connected graph without cycles.")
        .displayed_definition("npoYQB", "A connected graph with $n$ vertices and $n-1$ edges.")
        .done(&mut create);
    let interval = graph_class("p5skoj", "interval", 7)
        .displayed_definition("UHFdbP", "Intersection graph of intervals on the real line.")
        .tag(&tag_intersection)
        .done(&mut create);
    let edgeless = property("LsiBbX", "edgeless", Is, 1)
        .displayed_definition("vS7Zy8", "Contains no edges.")
        .done(&mut create);
    let linear_forest = graph_class("skQuFN", "linear forest", 4)
        .displayed_definition("XK5Xxy", "A disjoint union of paths.")
        .done(&mut create);
    let path = create.intersection("ryPlqz", &connected, &linear_forest, "path", 3)
        .displayed_definition("55gdPP", "Vertices can be ordered such that exactly those next to each other in the order are connected by an edge.")
        .done(&mut create);
    let outerplanar = graph_class("0oCyaG", "outerplanar", 5)
        .displayed_definition("WeXBO1", "A planar graph that can be drawin in a way where one face is incident to all the vertices.")
        .tag(&tag_topology)
        .done(&mut create);
    let perfect = graph_class("RmssrZ", "perfect", 6)
        .displayed_definition("aWZgIz", "Has [[q7zHeT]] equal to [[w7MmyW]].")
        .tag(&tag_coloring)
        .done(&mut create);
    let planar = graph_class("loZ5LD", "planar", 8)
        .displayed_definition("CUPkWg", "Has an embedding with vertices being points, edges being curves between respective points, which is planar, i.e., no curves cross each other.")
        .tag(&tag_topology)
        .done(&mut create);
    let stars = graph_class("10JR3F", "stars", 4)
        .displayed_definition("51KDFn", "Disjoint union of stars.")
        .done(&mut create);
    let star = create.intersection("CortlU", &connected, &stars, "star", 3)
        .displayed_definition("TUqM0F", "Contains only edges that connect a vertex to every other vertex.")
        .done(&mut create);
    let cycles = graph_class("2iJr52", "cycles", 4)
        .displayed_definition("sAZHF4", "Every component is a cycle.")
        .done(&mut create);
    let cycle = create.intersection("Ti0asF", &connected, &cycles, "cycle", 2)
        .displayed_definition("EqavC7", "Has a cyclic vertex order such that the graph contains edges for each pair of vertices that are next to each other in the order.")
        .displayed_definition("oTRopc", "A connected graph with all vertices having degree two.")
        .done(&mut create);
    let grid = parametric_graph_class("lfYXuK", "grid", 6)
        .displayed_definition("sp6LGE", "Cartesian product of two paths, each of length $k$.")
        .done(&mut create);
    let series_parallel = graph_class("eW1Gic", "series-parallel", 6)
        .displayed_definition("b0JuKf", "A (multi-)graph created from a forest by repeated edge subdivisions and edge duplication.")
        .done(&mut create);

    create.assumed_source()
        .assumed_proper_inclusion("piRTZw", &chordal, &perfect)
        .assumed_proper_inclusion("stwHRi", &cograph, &perfect)
        .assumed_proper_inclusion("ogyvLp", &bipartite, &perfect)
        .assumed_proper_inclusion("FM1wVJ", &cluster, &interval)
        .assumed_proper_inclusion("zJbehb", &cluster, &cograph)
        .assumed_proper_inclusion("rHotfs", &linear_forest, &interval)
        .assumed_proper_inclusion("OT3dig", &stars, &interval)
        .assumed_proper_inclusion("fKpyMg", &interval, &chordal)
        .assumed_proper_inclusion("cZy5xs", &co_cluster, &cograph)
        .assumed_proper_inclusion("AbAK8n", &forest, &bipartite)
        .assumed_proper_inclusion("ZiCzGe", &outerplanar, &planar)
        .assumed_proper_inclusion("APz0OO", &outerplanar, &series_parallel)
        .assumed_proper_inclusion("6TFVVG", &complete, &co_cluster)
        .assumed_proper_inclusion("2Jde0p", &block, &chordal)
        .assumed_proper_inclusion("DxYTTn", &cluster, &block)
        .assumed_proper_inclusion("lmKGuy", &linear_forest, &forest)
        .assumed_proper_inclusion("WJHhf0", &forest, &block)
        .assumed_proper_inclusion("VsrnoK", &edgeless, &linear_forest)
        .assumed_proper_inclusion("E8B2Gj", &stars, &forest)
        .assumed_proper_inclusion("BWJDZs", &edgeless, &stars)
        .assumed_proper_inclusion("HtdoRP", &edgeless, &co_cluster)
        .assumed_proper_inclusion("1PLbSg", &grid, &planar)
        .assumed_proper_inclusion("RQcVkC", &grid, &bipartite)
        .assumed_proper_inclusion("CTwA2j", &grid, &connected)
        .assumed_proper_inclusion("wTugFB", &edgeless, &cluster)
        .assumed_proper_inclusion("1pdarO", &star, &tree)
        .assumed_proper_inclusion("gAlyjK", &path, &tree)
        .assumed_proper_inclusion("1pdarO", &path, &grid)
        ;

    let size = parameter("F1NpDy", "size", 3)
        .displayed_definition("lKvvzN", "Total size of the graph $|V(G)+E(G)|$.")
        .done(&mut create);
    let vsize = parameter("Z7LoDd", "vsize", 2)
        .displayed_definition("lKvvzN", "Number of vertices in the graph $|V(G)|$.")
        .done(&mut create);
    let esize = parameter("a8glpM", "esize", 2)
        .displayed_definition("lKvvzN", "Number of edges in the graph $|E(G)|$.")
        .done(&mut create);
    let vertex_cover = parameter("4lp9Yj", "vertex cover", 9)
        .displayed_definition("qOc9n0", "The minimum number of vertices that have to be removed to get an independent set.")
        .abbr("vc")
        .tag(&tag_vertex_removal)
        .done(&mut create);
    let maximum_matching = parameter("veU7Jf", "maximum matching", 3)
        .displayed_definition("ErDFlH", "The size of a maximum independent edge set.")
        .done(&mut create);
    let vertex_integrity = parameter("KVhJFB", "vertex integrity", 6)
        .displayed_definition("aQQnbF", "The minimum $k$ such that there exists $d$ vertices whose removal results in connected components each of size at most $d-k$.")
        .abbr("vi")
        .tag(&tag_vertex_removal)
        .done(&mut create);
    let treedepth = parameter("KEP2qM", "treedepth", 7)
        .displayed_definition("cGN8qC", "Treedepth of a graph is height of an auxiliary rooted forest over graph's vertices such that all edges of the graph have ancestor-descendant relationship within the tree.")
        .displayed_definition("E9GMDZ", "For a graph treedepth is 1 if the graph is a single vertex. Otherwise, it is the minimum value obtained by removing some vertex and taking maximum over treedepths of each connected component.")
        .abbr("td")
        .tag(&tag_vertex_removal)
        .done(&mut create);
    let clique_cover_num = parameter("VomShB", "clique cover number", 5)
        .displayed_definition("p0NZrl", "Clique cover number is the minimum number of parts into which vertices of the graph can be partitioned so that each part induces a clique.")
        .tag(&tag_vertex_cover)
        .done(&mut create);
    let maximum_independent_set = parameter("mHtXUU", "maximum independent set", 2)
        .displayed_definition("2xRnhJ", "Is the cardinality of a maximum vertex set such that no pair of vertices in the set are connected by an edge.")
        .done(&mut create);
    let domination_num = parameter("Gq0onN", "domination number", 3)
        .displayed_definition("7XYxB4", "Is the cardinality of a minimum vertex set such that its closed neighborhood contains all vertices of the graph.")
        .tag(&tag_vertex_cover)
        .done(&mut create);
    let twin_cover_num = parameter("MUnHA0", "twin-cover number", 5)
        .displayed_definition("nTIDMU", "In graph $G$, twin-cover number is the minimum number $k$ such that there exists a set $M$ of size $k$ such that $G-M$ is a union of cliques where each pair of vertices from the same clique are true siblings in $G$.")
        .done(&mut create);
    let edge_clique_cover = parameter("nYQDv6", "edge clique cover number", 4)
        .displayed_definition("tiSkXE", "Edge clique cover number is the minimum number of clique subgraphs that cover every edge of the graph.")
        .tag(&tag_edge_cover)
        .done(&mut create);
    let neighborhood_diversity = parameter("vMs3RS", "neighborhood diversity", 6)
        .displayed_definition("A3QGjy", "Vertices can be partitioned into $\\mathrm{nd}$ parts, each consisting of only false or true twins.")
        .abbr("nd")
        .tag(&tag_modules)
        .done(&mut create);
    let modular_width = parameter("4bj71L", "modular-width", 7)
        .displayed_definition("RYhQuU", "Modular-width is the minimum integer $k$ such that there is a partition into at most $k$ modules where each module either contains a single vertex or the graph induced by the module has modular-width $k$.")
        .tag(&tag_modules)
        .done(&mut create);
    let modular_treewidth = parameter("VJrEcv", "modular-treewidth", 5)
        .displayed_definition("Eh2fg0", "Modular-treewidth is the minimum integer $k$ such that every quotient graph of graph's modular decomposition has treewidth at most $k$.")
        .tag(&tag_modules)
        .done(&mut create);
    let iterated_type_partitions = parameter("G1Cwmc", "iterated type partitions", 3)
        .displayed_definition("74t1tj", "see [[oBcMqr]]")
        .tag(&tag_modules)
        .done(&mut create);
    let maximum_leaf_num = parameter("BN92vX", "maximum leaf number", 6)
        .displayed_definition("rBWwFy", "Largest number of tree leaves in any of graph's spanning trees.")
        .tag(&tag_vertex_cover)
        .done(&mut create);
    let feedback_edge_set = parameter("HTk9PZ", "feedback edge set", 6)
        .displayed_definition("eYijvL", "In the cardinality of a minimum edge set where every cycle has at least one edge in the edge set.")
        .tag(&tag_edge_removal)
        .done(&mut create);
    let genus = parameter("gbaHdw", "genus", 6)
        .displayed_definition("eYijvL", "Genus is the minimum integer $k$ such that the graph can be embedded on a surface with $k$ handles without edge crossings.")
        .tag(&tag_topology)
        .done(&mut create);
    let cutwidth = parameter("TLx1pz", "cutwidth", 4)
        .displayed_definition("7gyJmF", "Minimum over vertex orders, maximum over positions (between vertices), number of edges that have endpoints on different sides of the position.")
        .tag(&tag_vertex_order)
        .done(&mut create);
    let carving_width = parameter("dS6OgO", "carving-width", 3)
        .displayed_definition("itJKBf", "Minimum over carving decompositions, maximum over decomposition edges, size of the respective edge cut.")
        .done(&mut create);
    let bandwidth = parameter("aP5a38", "bandwidth", 5)
        .displayed_definition("YvIT76", "Minimum over vertex labeling by distinct integers, maximum over edges, absolute difference of labels of edge's endpoints.")
        .tag(&tag_vertex_order)
        .done(&mut create);
    let topological_bandwidth = parameter("SnA7Eq", "topological bandwidth", 4)
        .displayed_definition("B3uOWz", "Minimum over grpah subdivisions, [[aP5a38]].")
        .tag(&tag_vertex_order)
        .done(&mut create);
    let bisection_bandwidth = parameter("wUdmUb", "bisection bandwidth", 4)
        .displayed_definition("AaJSw5", "Minimum over bi-partitions into (almost) equal-sized parts, the number of edges between parts.")
        .done(&mut create);
    let maximum_degree = parameter("UyQ5yM", "maximum degree", 8)
        .displayed_definition("8tk4SI", "Maximum degree over graph's vertices.")
        .done(&mut create);
    let c_closure = parameter("ou9VU1", "c-closure", 0)
        .displayed_definition("8tk4SI", "Minimum $c$ such that if vertices share at least $c$ neighbors, then they are adjacent.")
        .done(&mut create);
    let feedback_vertex_set = parameter("GNOiyB", "feedback vertex set", 8)
        .displayed_definition("81zlqB", "The minimum set of vertices $S$ such that every cycle in the graph contains at least one vertex of $S$.")
        .abbr("fvs")
        .tag(&tag_vertex_removal)
        .done(&mut create);
    let shrub_depth = parameter("NTgNzT", "shrub-depth", 6)
        .displayed_definition("09z8Sn", "see [[4Dua5N]]")
        .done(&mut create);
    let linear_clique_width = parameter("fQj3wU", "linear clique-width", 4)
        .displayed_definition("EBCy7E", "Minimum number of labels (colors) required to construct the graph using the following operations for constructing labeled graphs: 1) create a new labeled vertex, 2) union with a single labeled vertex, 3) complete join between two labels, and 4) change all vertices from one to another label.")
        .tag(&tag_linear)
        .done(&mut create);
    let pathwidth = parameter("VHClqR", "pathwidth", 8)
        .displayed_definition("gW3oo6", "Same as treewidth but restricted to have a tree decomposition that is a path.")
        .abbr("pw")
        .tag(&tag_linear)
        .done(&mut create);
    let degree_pathwidth = create
        .intersection("6BWcgd", &pathwidth, &maximum_degree, "pathwidth+maxdegree", 3)
        .tag(&tag_linear)
        .done(&mut create);
    let d_path_free = parameter("s4EiWI", "d-path-free", 2)
        .displayed_definition("cDzIUa", "Minimum $k$ such that the graph contains no path on $k$ vertices.")
        .done(&mut create); // todo - H-free classes and parameters
    let treewidth = parameter("5Q7fuR", "treewidth", 9)
        .displayed_definition("xKSFFn", "see [[i56ihO]]") // todo - point to the definition
        .abbr("tw")
        .tag(&tag_tree_decomposition)
        .done(&mut create);
    let mm_width = parameter("d7vRYU", "mm-width", 4)
        .displayed_definition("pLA7Pd", "see [[nRO7AG]]")
        .done(&mut create);
    let tree_partition_width = parameter("QP01gs", "tree-partition-width", 5)
        .displayed_definition("JWIK50", "see [[p00uyg]]")
        .tag(&tag_tree_decomposition)
        .done(&mut create);
    // DEFINITIONS WIP /////////////////////////////////////////////////////////////
    let edge_cut_width = parameter("ZNqIlN", "edge-cut width", 2)
        .done(&mut create);
    let tree_cut_width = parameter("8CgU0P", "tree-cut width", 2)
        .done(&mut create);
    let slim_tree_cut_width = parameter("oFvl4c", "slim tree-cut width", 2)
        .done(&mut create);
    let edge_treewidth = parameter("pKi2tL", "edge-treewidth", 2)
        .done(&mut create);
    let overlap_treewidth = parameter("P8yP3M", "overlap treewidth", 2)
        .done(&mut create);
    let degree_treewidth = create
        .intersection("nCWUh3", &maximum_degree, &treewidth, "degree treewidth", 6)
        .tag(&tag_tree_decomposition)
        .done(&mut create);
    let domino_treewidth = parameter("aEs5ap", "domino treewidth", 3)
        .displayed_definition("qPlUi0", "Minimum width of tree decompositions where every vertex is in at most 2 bags.")
        .tag(&tag_tree_decomposition)
        .done(&mut create);
    let treespan = parameter("IbKkUQ", "treespan", 3)
        .done(&mut create);
    let treebandwidth = parameter("w3LxG1", "treebandwidth", 4)
        .displayed_definition("JTZz7J", "A \\emph{tree-layout} of $G=(V,E)$ is a rooted tree $T$ whose nodes are the vertices of $V$, and such that, for every edge $xy \\in E$, $x$ is an ancestor of $y$ or vice-versa. The bandwidth of $T$ is then the maximum distance in $T$ between pairs of neighbors in $G$. We call \\emph{treebandwidth} of $G$, the minimum bandwidth over tree-layouts of $G$, and denote it by ${\\rm tbw}(G)$.")
        .done(&mut create);
    let contraction_complexity = parameter("LlWzhg", "contraction complexity", 2)
        .displayed_definition("Nim1uo", "see [[9JAQC7]]")
        .done(&mut create);
    let branch_width = parameter("lIcmuR", "branch width", 4)
        .displayed_definition("mHnMqv", "Minimum over branch decompositions, maximum over decomposition edges, number of vertices that are incident to edges in both parts of the bi-partition.")
        .tag(&tag_branch_decomposition)
        .done(&mut create);
    let clique_width = parameter("wg5HuV", "clique-width", 7)
        .displayed_definition("yAJh1T", "Minimum number of labels (colors) required to construct the graph using the following operations for constructing labeled graphs: 1) create a new labeled vertex, 2) disjoint union, 3) complete join between two labels, and 4) change all vertices from one to another label.")
        .abbr("cw")
        .done(&mut create);
    let clique_tree_width = parameter("7P9WUz", "clique-tree-width", 2)
        .done(&mut create);
    let rank_width = parameter("fojquT", "rank-width", 7)
        .displayed_definition("9nZyhu", "Let $T$ be a ternary tree and $\\tau$ be a bijection between graph's vertices and the set of leaves in $T$. Every edge of $T$ cuts $V(G)$ into two parts. Rank of such a cut is rank of a biadjacency matrix describing the cut edges. Rank-width of $G$ is minimum over all ternary trees $T$, maximum over ranks of cuts implied by edges in $T$.")
        .done(&mut create);
    let linear_rank_width = parameter("cHugsk", "linear rank-width", 2)
        .tag(&tag_linear)
        .done(&mut create);
    let boolean_width = parameter("A2jPWT", "boolean width", 5)
        .done(&mut create);
    let twin_width = parameter("OrH7et", "twin-width", 8)
        .displayed_definition("LTw98i", "A contraction sequence for a graph $G$ is a sequence of $n-1$ contractions -- identification of two not necessarily adjacent vertices. Note that at any point each vertex of a partially contracted graph represents a subset of vertices in the original graph. Vertices of a partially contracted graph are joined with an edge if there is a complete bipartite graph between the represented subsets of vertices in the original graph. Similarly, there is a non-edge if the two sets have no edges between them. Last, there is a red-edge is there are some edges and some non-edges. Red degree of a partially contracted graph is the maximum number of red edges adjacent to a single vertex. Twin-width is the minimum over contraction sequences of maximum over the sequence's red degree.")
        .abbr("tww")
        .done(&mut create);
    let mut flip_width_builder = parameter_sequence("4DIiH0");
    let inf_flip_width = flip_width_builder.parameter("nYXiuT", "radius-inf flip-width", ParameterType::Infinity, 3)
        .done(&mut create);
    let r_flip_width = flip_width_builder.parameter("4DIiH0", "radius-r flip-width", ParameterType::Any, 3)
        .done(&mut create);
    let flip_width = flip_width_builder.parameter("jYG7BR", "flip-width", ParameterType::All, 5)
        .done(&mut create);
    flip_width_builder.done(&mut create);
    let mut weak_coloring_number_builder = parameter_sequence("Klj5EI");
    let weak_d_coloring_number = weak_coloring_number_builder.parameter("3F3oc3", "weak d-coloring number", ParameterType::Any, 4)
        .done(&mut create);
    let weak_inf_coloring_number = weak_coloring_number_builder.parameter("DfwI9E", "weak inf-coloring number", ParameterType::Infinity, 3)
        .done(&mut create);
    let weak_coloring_number = weak_coloring_number_builder.parameter("KD6n2n", "weak coloring number", ParameterType::All, 5)
        .done(&mut create);
    weak_coloring_number_builder.done(&mut create);
    let mut strong_coloring_number_builder = parameter_sequence("cnlFUH");
    let strong_d_coloring_number = strong_coloring_number_builder.parameter("yihnem", "strong d-coloring number", ParameterType::Any, 4)
        .done(&mut create);
    let strong_inf_coloring_number = strong_coloring_number_builder.parameter("JQTHZS", "strong inf-coloring number", ParameterType::Infinity, 3)
        .done(&mut create);
    let strong_coloring_number = strong_coloring_number_builder.parameter("PxVh3F", "strong coloring number", ParameterType::All, 5)
        .done(&mut create);
    strong_coloring_number_builder.done(&mut create);
    let mut admissibility_builder = parameter_sequence("JNzE6K");
    let d_admissibility = admissibility_builder.parameter("Pqiy2C", "d-admissibility", ParameterType::Any, 4)
        .done(&mut create);
    // let admissibility_inf = admissibility_builder.parameter("UjWcOA", "inf-admissibility", ParameterType::Infinity, 3)
        // .done(&mut create);
    let admissibility = admissibility_builder.parameter("v4sLfO", "admissibility", ParameterType::All, 5)
        .done(&mut create);
    admissibility_builder.done(&mut create);
    let merge_width = parameter("UWmTKl", "merge-width", 5)
        .displayed_definition("JG4FIo", "see [[9exguJ]]")
        .done(&mut create);
    let book_thickness = parameter("doijTS", "book thickness", 4)
        .displayed_definition("0sKSfL", "Minimum $k$ such that for some order of vertices $\\prec$ there is a $k$-partition of edges so that no two edges $ab$ and $cd$ cross, i.e., $a \\prec c \\prec b \\prec d$.")
        .aka("stacknumber")
        .aka("pagenumber")
        .aka("fixed outerthickness")
        .tag(&tag_topology)
        .done(&mut create);
    // .proved("1IL2wn", NotApplicable, &book_thickness, &create.edge_cover_by(&outerplanar), Equal, "") //but with fixed vertices
    let hindex = parameter("GNTwUS", "h-index", 4)
        .displayed_definition("1juCAg", "Maximum $h$ for which a graph contains $h$ vertices of degree at least $h$.")
        .done(&mut create);
    let acyclic_chromatic_number = parameter("QGZuUW", "acyclic chromatic number", 5)
        .displayed_definition("TDX6hc", "Minimum number of colors such that there is a proper coloring and the graph induced on any two colors is acyclic.")
        .tag(&tag_coloring)
        .done(&mut create);
    let odd_cycle_transversal = parameter("Ve5ruW", "odd cycle transversal", 6)
        .abbr("oct")
        .displayed_definition("ShPKzs", "Minimum $k$ such that there is a set of $k$ vertices that hit every cycle of odd length.")
        .tag(&tag_edge_removal)
        .done(&mut create);
    let degeneracy = parameter("VowkuW", "degeneracy", 6)
        .displayed_definition("9ei8a0", "Minimum $k$ so that there is a vertex order such that for each vertex at most $k$ of its neighbors are in the order before it.")
        .tag(&tag_vertex_order)
        .done(&mut create);
    let chromatic_number = parameter("w7MmyW", "chromatic number", 5)
        .tag(&tag_coloring)
        .done(&mut create);
    let average_degree = parameter("z0y4TW", "average degree", 2)
        .displayed_definition("Cqlbok", "Average degree of graph's vertices.")
        .done(&mut create);
    let minimum_degree = parameter("GPmOeT", "minimum degree", 0)
        .displayed_definition("CKNuj2", "Minimum degree over graph's vertices.")
        .done(&mut create);
    let maximum_clique = parameter("q7zHeT", "maximum clique", 5)
        .displayed_definition("rwsCEM", "Maximum size of a clique in the graph")
        .done(&mut create);
    let edge_connectivity = parameter("JbqZoT", "edge connectivity", 2)
        .done(&mut create);
    let vertex_connectivity = parameter("OyLUe4", "vertex connectivity", 0)
        .hide()
        .done(&mut create);
    let boxicity = parameter("a7MpiT", "boxicity", 6)
        .displayed_definition("1a2qcc", "*Boxicity* is the minimum dimension $k$ such that $G$ is an intersection graph of $k$ dimensional axis-parallel boxes.")
        .tag(&tag_topology)
        .done(&mut create);
    let chordality = parameter("fTqo40", "chordality", 4)
        .tag(&tag_edge_cover)
        .done(&mut create);
    let maximum_induced_matching = parameter("GzMYlT", "maximum induced matching", 3)
        .done(&mut create);
    let diameter = parameter("p4bTjp", "diameter", 6)
        .displayed_definition("MlVCMG", "Maximum distance of two vertices that are in the same connected component.")
        .done(&mut create);
    let average_distance = parameter("zH8PpT", "average distance", 3)
        .done(&mut create);
    // let girth = parameter("BCwUeT", "girth", 1) // tentattively removed as it is not closed under taking induced subgraphs
        // .done(&mut create);
    let domatic_num = parameter("KRV6tI", "domatic number", 3)
        .done(&mut create);
    let arboricity = parameter("zgMenA", "arboricity", 5)
        .done(&mut create);
    // let star_arboricity = parameter("Mvz8MX", "star-arboricity", 1)
    // .done(&mut create);
    let mim_width = parameter("WmIFB1", "mim-width", 6)
        .displayed_definition("2yzjqV", "Let $T$ be a ternary tree and $\\tau$ be a bijection between graph's vertices and the set of leaves in $T$. Every edge of $T$ cuts $V(G)$ into two parts. $mim$ of such a cut is the maximum induced matching of the bipartite graph induced by the cut edges. MIM-width of $G$ is minimum over all ternary trees $T$, maximum over $mim$ of cuts implied by edges in $T$.")
        .done(&mut create);
    let sm_width = parameter("A9hzWC", "sm-width", 5)
        .done(&mut create);
    let sim_width = parameter("aEGv5N", "sim-width", 5)
        .done(&mut create);
    let module_width = parameter("EV3FqL", "module-width", 4)
        .done(&mut create);
    let tree_independence = parameter("fNR6QK", "tree-independence number", 5)
        .tag(&tag_tree_decomposition)
        .done(&mut create);
    let nlc_width = parameter("Xrpbv7", "NLC-width", 4)
        .done(&mut create);
    let nlct_width = parameter("mOri44", "NLCT-width", 2)
        .done(&mut create);
    let linear_nlc_width = parameter("v09DMY", "linear NLC-width", 2)
        .tag(&tag_linear)
        .done(&mut create);
    // let crossing_number = parameter("zUv8EU", "crossing number", 4)
    // .done(&mut create);
    // let thickness = parameter("sxTPYj", "thickness", 3)
    // .done(&mut create);
    // let outerthickness = parameter("MIeOwU", "outerthickness", 1)
    // .done(&mut create);
    let bounded_components = parameter("t7c4mp", "bounded components", 3)
        .done(&mut create);
    let dist_to_complete = create.distance_to("2LDMQ6", &complete, 6)
        .done(&mut create);
    let dist_to_co_cluster = create.distance_to("hbfWwE", &co_cluster, 5)
        .done(&mut create);
    let dist_to_cograph = create.distance_to("uDXX2i", &cograph, 5)
        .done(&mut create);
    let dist_to_cluster = create.distance_to("aXw3Co", &cluster, 5)
        .done(&mut create);
    let dist_to_linear_forest = create
        .distance_to("yk7XP0", &linear_forest, 4)
        .tag(&tag_linear)
        .done(&mut create);
    let dist_to_outerplanar = create
        .distance_to("lPHVWU", &outerplanar, 3)
        .tag(&tag_topology)
        .done(&mut create);
    let dist_to_block = create.distance_to("xNJnFb", &block, 4)
        .done(&mut create);
    let dist_to_edgeless = create.distance_to("4INs10", &edgeless, 1)
        .done(&mut create);
    let dist_to_forest = create.distance_to("hQZlLU", &forest, 5)
        .done(&mut create);
    let dist_to_bipartite = create.distance_to("1yW82F", &bipartite, 5)
        .done(&mut create);
    let dist_to_planar = create
        .distance_to("MLJMRH", &planar, 4)
        .tag(&tag_topology)
        .done(&mut create);
    let dist_to_chordal = create.distance_to("OdZQna", &chordal, 4)
        .done(&mut create);
    let dist_to_stars = create.distance_to("Z10jME", &stars, 3)
        .done(&mut create);
    let dist_to_perfect = create.distance_to("kJZKgd", &perfect, 4)
        .done(&mut create);
    let dist_to_interval = create.distance_to("AVc2K6", &interval, 3)
        .done(&mut create);
    let dist_to_maximum_degree = create.distance_to("kRR8zx", &maximum_degree, 4)
        .done(&mut create);
    let dist_to_bounded_components = create.distance_to("RPTCxd", &bounded_components, 4)
        .done(&mut create);
    // let dist_to_disconnected = create.distance_to("ZL7BOP", &disconnected, 2).hide()
        // .done(&mut create);
    let bipartite_number = parameter("1dQQ87", "bipartite number", 2)
        .displayed_definition("QmlowC", "Bipartite number of $G$ is the maximum order of an induced bipartite subgraph.")
        .done(&mut create);
    let treelength = parameter("JA2nKw", "treelength", 6)
        .displayed_definition("H4YERL", "Treelength of a tree decomposition is the maxmimum distance of two vertices that appear in the same bag. Treelength of a graph is the minimum treelength over tree decompositions.")
        .tag(&tag_tree_decomposition)
        .done(&mut create);

    provider(
        "OLlwnV",
        "ISGCI",
        "https://www.graphclasses.org/",
        Box::new(|id: &str| format!(r"https://www.graphclasses.org/classes/gc_{id}.html")),
        )
        .link(&bipartite, "69")
        .link(&block, "93")
        .link(&chordal, "32")
        .link(&cluster, "1237")
        .link(&co_cluster, "1248")
        .link(&cograph, "151")
        .link(&complete, "1241")
        .link(&forest, "342")
        .link(&interval, "234")
        .link(&edgeless, "1247")
        .link(&outerplanar, "110")
        .link(&perfect, "56")
        .link(&planar, "43")
        .link(&stars, "1297")
        .link(&grid, "464")
        .link(&series_parallel, "275")
        .done(&mut create);

    provider("bS9nCY", "ISGCI",
        "https://www.graphclasses.org/",
        Box::new(|id: &str| format!(r"https://www.graphclasses.org/classes/par_{id}.html")),
        )
        .link(&vertex_cover, "2")
        .link(&maximum_matching, "13")
        .link(&treedepth, "18")
        .link(&dist_to_complete, "1")
        .link(&maximum_independent_set, "8")
        .link(&domination_num, "5")
        .link(&dist_to_co_cluster, "3")
        .link(&dist_to_cograph, "7")
        .link(&dist_to_cluster, "29")
        .link(&maximum_leaf_num, "22")
        .link(&genus, "23")
        .link(&cutwidth, "15")
        .link(&carving_width, "16")
        .link(&bandwidth, "25")
        .link(&maximum_degree, "28")
        .link(&dist_to_linear_forest, "24")
        .link(&dist_to_outerplanar, "26")
        .link(&pathwidth, "9")
        .link(&treewidth, "10")
        .link(&branch_width, "11")
        .link(&clique_width, "12")
        .link(&rank_width, "20")
        .link(&boolean_width, "21")
        .link(&book_thickness, "32")
        .link(&acyclic_chromatic_number, "31")
        .link(&degeneracy, "17")
        .link(&chromatic_number, "19")
        .link(&maximum_clique, "27")
        .link(&dist_to_block, "30")
        .link(&maximum_induced_matching, "14")
        .link(&diameter, "6")
        .done(&mut create);

    provider(
        "D8GB7n",
        "PACE",
        "https://pacechallenge.org/",
        Box::new(|id: &str| format!(r"https://pacechallenge.org/{id}/")),
        )
        .link(&domination_num, "2025")
        // .link(&hitting_set, "2025")
        // .link(&one_sided_crossing_minimization, "2024")
        .link(&twin_width, "2023")
        // .link(&directed_feedback_vertex_set, "2022")
        // .link(&cluster_editing, "2021")
        .link(&treedepth, "2020")
        .link(&vertex_cover, "2019")
        // .link(&hypertree_width, "2019")
        // .link(&steiner_tree, "2018")
        .link(&treewidth, "2017")
        // .link(&add_edges_to_chordal, "2017")
        .link(&treewidth, "2016")
        .link(&feedback_vertex_set, "2016")
        .done(&mut create);

    let diam_maxdeg = create
        .intersection("ri9Seh", &diameter, &maximum_degree, "diameter+max degree", 5)
        .hide()
        .done(&mut create);

    let by_definition = "By definition";

    create.assumed_source()
        // .ref_noted_relation("YOBod9", NotApplicable, &vertex_connectivity, &dist_to_disconnected, Equal, by_definition)
        .ref_noted_relation("LTyhoG", NotApplicable, &vertex_integrity, &dist_to_bounded_components, UpperBound(Linear), by_definition, SrcTodo)
        .ref_noted_relation("wgnjMg", NotApplicable, &dist_to_bounded_components, &vertex_integrity, UpperBound(Linear), by_definition, SrcTodo)
        .ref_noted_relation("SyGwqT", NotApplicable, &bandwidth, &topological_bandwidth, UpperBound(Linear), by_definition, SrcTodo)
        .ref_noted_relation("ebAUEu", NotApplicable, &twin_cover_num, &dist_to_cluster, UpperBound(Linear), by_definition, SrcTodo)
        .ref_noted_relation("2XN8ux", NotApplicable, &vertex_cover, &twin_cover_num, UpperBound(Linear), by_definition, SrcTodo)
        .ref_noted_relation("XTPNkl", NotApplicable, &average_degree, &minimum_degree, UpperBound(Linear), by_definition, SrcTodo)
        .ref_noted_relation("TezCU1", NotApplicable, &diameter, &average_distance, UpperBound(Linear), by_definition, SrcTodo)
        .ref_noted_relation("qy7Xdi", NotApplicable, &maximum_matching, &maximum_induced_matching, UpperBound(Linear), by_definition, SrcTodo)
        .ref_noted_relation("2gTckj", NotApplicable, &dist_to_interval, &boxicity, UpperBound(Linear), by_definition, SrcTodo)
        .ref_noted_relation("LAc0Ur", NotApplicable, &bisection_bandwidth, &edge_connectivity, UpperBound(Linear), by_definition, SrcTodo)
        .ref_noted_relation("yWSq1V", NotApplicable, &edgeless, &bounded_components, UpperBound(Constant), by_definition, SrcTodo)
        .ref_noted_relation("KxMj5k", NotApplicable, &grid, &maximum_degree, UpperBound(Constant), by_definition, SrcTodo)
        .ref_noted_relation("TxxhnK", NotApplicable, &bounded_components, &maximum_degree, UpperBound(Linear), by_definition, SrcTodo)
        .ref_noted_relation("ZEEhCr", NotApplicable, &linear_forest, &maximum_degree, UpperBound(Constant), by_definition, SrcTodo)
        .ref_noted_relation("a3JKzR", NotApplicable, &cycles, &maximum_degree, UpperBound(Constant), by_definition, SrcTodo)
        .ref_noted_relation("G5i5Bz", NotApplicable, &diameter, &treelength, UpperBound(Linear), by_definition, SrcTodo)
        .ref_noted_relation("t0TUmk", NotApplicable, &edgeless, &connected, Exclusion, by_definition, SrcTodo)
        .ref_noted_relation("jkoObg", NotApplicable, &edgeless, &cycles, Exclusion, by_definition, SrcTodo)
        .ref_noted_relation("81rUKt", NotApplicable, &diam_maxdeg, &bounded_components, UpperBound(Exponential), "folklore", SrcTodo)
        .ref_noted_relation("Ktfezk", NotApplicable, &maximum_independent_set, &bipartite_number, UpperBound(Linear), "folklore: Each of the parts of the maximum induced bipartite subgraph is an independent set. Hence, the bipartite number is at most twice the size of the maximum independent set.", SrcTodo)
        .ref_noted_relation("F4Bsnu", NotApplicable, &bipartite_number, &maximum_independent_set, UpperBound(Linear), "As one can pick the maximum independent set as one side of an induced bipartite subgraph we know that the maximum one has size at least the size of the maximum independent set.", SrcTodo)
        .ref_noted_relation("rZeuh1", NotApplicable, &minimum_degree, &average_degree, Exclusion, "folklore", SrcTodo)
        .ref_noted_relation("lKvvzN", NotApplicable, &size, &vertex_cover, UpperBound(Linear), by_definition, SrcTodo)
        .ref_noted_relation("sXOMaO", NotApplicable, &size, &maximum_leaf_num, UpperBound(Linear), by_definition, SrcTodo)
        .ref_noted_relation("yzbP6z", NotApplicable, &size, &dist_to_complete, UpperBound(Linear), by_definition, SrcTodo)
        .ref_noted_relation("O4vak5", NotApplicable, &size, &bounded_components, UpperBound(Linear), by_definition, SrcTodo)
        .ref_noted_relation("Cgalgy", NotApplicable, &size, &planar, Exclusion, by_definition, SrcTodo)
        .ref_noted_relation("Bj3xlp", NotApplicable, &size, &perfect, Exclusion, by_definition, SrcTodo)
        .ref_noted_relation("nIqkYI", NotApplicable, &size, &connected, Exclusion, by_definition, SrcTodo)
        ;

    // let hereditary = property("0E0t0r", "hereditary", Is, 8)
        // .done(&mut create);
    let nowhere_dense = property("DhGqJM", "nowhere dense", Is, 5)
        .displayed_definition("2TzkL2", "A graph class $C$ is nowhere dense if for every $r \\in \\mathbb N$, the family of $r$-shallow minors does not include the family of all cliques.")
        .done(&mut create);
    let bounded_expansion = property("lFz6Ci", "bounded expansion", Has, 6)
        .displayed_definition("nvfldO", "A graph class $C$ has bounded expansion if for every $r \\in \\mathbb N$, the family of $r$-shallow minors does not include the family of graphs with unbounded density ($|E(G)|/|V(G)|$).")
        .done(&mut create);
    let sparse_twin_width = create.intersection("ORm833", &twin_width, &bounded_expansion, "sparse twin-width", 4)
        .done(&mut create);
    let monadically_stable = property("jHXy6Y", "monadically stable", Is, 5)
        .done(&mut create);
    let monadically_dependent = property("dN1D3C", "monadically dependent", Is, 5)
        .done(&mut create);
    let excluded_minor = property("5xOuoQ", "excluded minor", Has, 4)
        .displayed_definition("0zszt7", "There is a finite family of graphs $H$ such that the graph class does not contain any graph of $H$ is its minor.")
        .done(&mut create);
    let excluded_planar_minor = property("Gt22Ik", "excluded planar minor", Has, 4)
        .displayed_definition("j4XYWe", "There is a finite family of graphs $H$, with at least one graph of $H$ being planar, such that the graph class does not contain any graph of $H$ is its minor.")
        .done(&mut create);
    let excluded_top_minor = property("yOZQM5", "excluded top-minor", Has, 3)
        .done(&mut create);
    let chi_bounded = property("Jb1we5", "chi-bounded", Is, 5)
        .displayed_definition("Cf7poI", "For $\\chi$ being [[w7MmyW]] and $\\omega$ being [[q7zHeT]] we say a graph class is $\\chi$-bounded if there exists a function $f$ such that $\\chi(G) \\le f(\\omega(G))$ for every $G$ from the class.")
        .done(&mut create);
    let weakly_sparse = property("Qme7wD", "weakly sparse", Is, 3)
        .aka("$K_{t,t}$-free")
        .done(&mut create);
    // let erdos_hajnal = property("HnbWle", "Erdős-Hajnal", Is, 6)
        // .displayed_definition("3e7tTq", "A graph class satisfies this property if it Erdős-Hajnal conjecture.")
        // .done(&mut create);
    // let bip_sub_free = property("LoQADQ", "$K_{t,t}$-subgraph-free", 5)
    // .done(&mut create);
    let vc_dimension = parameter("hhkOKk", "VC-dimension", 3)
        .displayed_definition("LaZaUD", "*VC-dimension* of a set system $\\mathcal F$ is the maximum size $d$ of a set $Y$ such that $\\mathcal F \\cap Y = 2^Y$, i.e., $Y$ is *shattered* by $\\mathcal F$. By VC-dimension of a graph we mean VC-dimension of the set sytem $(V(G),\\{N(v) \\mid v \\in V(G)\\}.")
        .done(&mut create);

    create.unknown_source()
        // $adm_d \le col_d \le wcol_d$
        .ref_noted_relation("gNeaGx", NotApplicable, &strong_d_coloring_number, &d_admissibility, UpperBound(Linear), "by definition", SrcTodo)
        .ref_noted_relation("NVfs8a", NotApplicable, &weak_coloring_number, &strong_coloring_number, UpperBound(Linear), "by definition", SrcTodo)
        .ref_noted_relation("nvlJEd", NotApplicable, &bounded_expansion, &degeneracy, UpperBound(Constant), "$wcol_1-1 = col_1-1=adm_1=degeneracy$", SrcTodo) // constant due to be being a property?
        .ref_noted_relation("LRdQPE", NotApplicable, &d_admissibility, &strong_d_coloring_number, UpperBound(Tower), "$col_d \\le 1+(adm_d)^d$", SrcTodo)
        .ref_noted_relation("4sBB4p", NotApplicable, &strong_d_coloring_number, &weak_d_coloring_number, UpperBound(Tower), "$wcol_d \\le \\sum_{i=0}^d(col_d-1)^i$", SrcTodo)
        .ref_noted_relation("q5bjVP", NotApplicable, &weak_inf_coloring_number, &treedepth, Equivalent(Exists, Exists), "", SrcTodo)
        .ref_noted_relation("hvT4IE", NotApplicable, &strong_inf_coloring_number, &treewidth, Equivalent(Exists, Exists), "", SrcTodo)
        .ref_noted_relation("eDhK9F", NotApplicable, &weak_inf_coloring_number, &weak_coloring_number, UpperBound(Exists), "", SrcTodo)
        .ref_noted_relation("r3b2YQ", NotApplicable, &weak_coloring_number, &weak_d_coloring_number, UpperBound(Exists), "", SrcTodo)
        .ref_noted_relation("yH0x6l", NotApplicable, &strong_inf_coloring_number, &strong_coloring_number, UpperBound(Exists), "", SrcTodo)
        .ref_noted_relation("wrA0YA", NotApplicable, &strong_coloring_number, &strong_d_coloring_number, UpperBound(Exists), "", SrcTodo)
        .ref_noted_relation("EhPEVZ", NotApplicable, &bounded_expansion, &weak_coloring_number, Equivalent(Exists, Exists), "", SrcTodo) // https://www.sciencedirect.com/science/article/pii/S0012365X08001982
        .ref_noted_relation("KtfFkP", NotApplicable, &bounded_expansion, &strong_coloring_number, Equivalent(Exists, Exists), "", SrcTodo)
        .ref_noted_relation("rFt35p", NotApplicable, &bounded_expansion, &admissibility, Equivalent(Exists, Exists), "", SrcTodo)
        .ref_noted_relation("3Pff25", NotApplicable, &bounded_expansion, &nowhere_dense, UpperBound(Constant), by_definition, SrcTodo)
        .ref_noted_relation("VLbSmM", NotApplicable, &excluded_top_minor, &bounded_expansion, UpperBound(Constant), "", SrcTodo)
        .ref_noted_relation("3e5jLj", NotApplicable, &degeneracy, &weakly_sparse, UpperBound(Constant), "", SrcTodo)
        .ref_noted_relation("ssMLQE", NotApplicable, &nowhere_dense, &weakly_sparse, UpperBound(Constant), "", SrcTodo)
        .ref_noted_relation("45mNcy", NotApplicable, &nowhere_dense, &vc_dimension, UpperBound(Constant), "", SrcTodo)
        .ref_noted_relation("Ie3w0s", NotApplicable, &clique_width, &vc_dimension, UpperBound(Constant), "", SrcTodo)
        .ref_noted_relation("cF8esN", NotApplicable, &outerplanar, &excluded_planar_minor, UpperBound(Constant), "", SrcTodo)
        .ref_noted_relation("opjGKG", NotApplicable, &excluded_minor, &excluded_top_minor, UpperBound(Constant), "", SrcTodo)
        .ref_noted_relation("I8lPXq", NotApplicable, &excluded_planar_minor, &excluded_minor, UpperBound(Constant), "", SrcTodo)
        .ref_noted_relation("8l33Gi", NotApplicable, &maximum_degree, &excluded_top_minor, UpperBound(Constant), "", SrcTodo)
        .ref_noted_relation("l230dC", NotApplicable, &treewidth, &excluded_top_minor, UpperBound(Constant), "", SrcTodo)
        .ref_noted_relation("5qgR1y", NotApplicable, &planar, &excluded_minor, UpperBound(Constant), "", SrcTodo)
        ;

    let cliques_make_it_unbounded = "Parameter is unbounded for the graph class of cliques.";

    let tmp_8mm5qj = create.intersection("8Mm5qJ", &bipartite, &maximum_matching, "maximum matching on bipartite graphs", 0)
        .hide()
        .done(&mut create);

    let tmp_ws_mw = create.intersection("HJjpOL", &weakly_sparse, &merge_width, "weakly sparse and merge width", 3)
        .hide()
        .done(&mut create);

    create.unknown_source()
        .ref_proved("8Mm5qJ", NotApplicable, &tmp_8mm5qj, &vertex_cover, Exactly(Linear), "Kőnig's theorem")
        // Cite(id="gBA7dc", url="https://en.wikipedia.org/wiki/K%C5%91nig%27s_theorem_(graph_theory)", text="Kőnig's theorem"),
        .ref_noted_relation("U14yX4", NotApplicable, &odd_cycle_transversal, &dist_to_bipartite, Equal, "Bipartite graphs is the graph class without any odd cycles.", SrcTodo)
        // Note(id="lqOY3G", text="Bipartite graphs is the graph class without any odd cycles."),
        .ref_noted_relation("5sq1SD", NotApplicable, &feedback_edge_set, &feedback_vertex_set, UpperBound(Linear), "Given solution to feedback edge set one can remove one vertex incident to the solution edges to obtain feedback vertex set.", SrcTodo)
        .ref_noted_relation("8dQ8Us", NotApplicable, &feedback_edge_set, &genus, UpperBound(Linear), "Removing $k$ edges creates a forest that is embeddable into the plane. We now add one handle for each of the $k$ edges to get embedding into $k$-handle genus.", SrcTodo)
        .ref_noted_relation("K0Bc61", NotApplicable, &chromatic_number, &maximum_clique, UpperBound(Linear), "Unbounded clique implies the number of needed colors is unbounded.", SrcTodo)
        .ref_noted_relation("uKFrrb", NotApplicable, &degeneracy, &chromatic_number, UpperBound(Linear), "Greedily color the vertices in order of the degeneracy ordering. As each vertex has at most $k$ colored predecesors we use at most $k+1$ colors.", SrcTodo)
        .ref_noted_relation("gLjejq", NotApplicable, &degeneracy, &average_degree, UpperBound(Linear), "Removing a vertex of degree $d$ increases the value added to the sum of all degrees by at most $2d$, hence, the average is no more than twice the degeneracy.", SrcTodo)
        .ref_noted_relation("q5QDXg", NotApplicable, &maximum_degree, &hindex, UpperBound(Linear), "As h-index seeks $k$ vertices of degree $k$ it is trivially upper bound by maximum degree.", SrcTodo)
        .ref_noted_relation("1MAoyr", NotApplicable, &minimum_degree, &edge_connectivity, UpperBound(Linear), "Removing edges incident to the minimum degree vertex disconnects the graph.", SrcTodo)
        .ref_noted_relation("XOAOqw", NotApplicable, &linear_rank_width, &rank_width, Todo, "", SrcTodo)
        .ref_noted_relation("SUEy4S", NotApplicable, &pathwidth, &linear_rank_width, Todo, "", SrcTodo)
        .ref_noted_relation("UYpwYn", NotApplicable, &minimum_degree, &domatic_num, UpperBound(Linear), "The vertex of minimum degree needs to be dominated in each of the sets. As the sets cannot overlap there can be at most $k+1$ of them.", SrcTodo)
        .ref_noted_relation("d2ZJIh", NotApplicable, &dist_to_linear_forest, &pathwidth, UpperBound(Linear), "After removal of $k$ vertices the remaining class has a bounded width $w$. So by including the removed vertices in every bag, we can achieve decomposition of width $w+k$", SrcTodo)
        .ref_noted_relation("LyJWeW", NotApplicable, &topological_bandwidth, &bisection_bandwidth, UpperBound(Linear), "Order vertices by their bandwidth integer. We split the graph in the middle of this ordering. There are at most roughly $k^2/2$ edges over this split.", SrcTodo)
        .ref_noted_relation("waxvtz", NotApplicable, &bandwidth, &maximum_degree, UpperBound(Linear), "Each vertex has an integer $i$ and may be connected only to vertices whose difference from $i$ is at most $k$. There are at most $k$ bigger and $k$ smaller such neighbors.", SrcTodo)
        .ref_noted_relation("d2ZJIh", NotApplicable, &dist_to_linear_forest, &pathwidth, UpperBound(Linear), "After removal of $k$ vertices the remaining class has a bounded width $w$. So by including the removed vertices in every bag, we can achieve decomposition of width $w+k$", SrcTodo)
        .ref_noted_relation("d2ZJIh", NotApplicable, &dist_to_outerplanar, &treewidth, UpperBound(Linear), "After removal of $k$ vertices the remaining class has a bounded width $w$. So by including the removed vertices in every bag, we can achieve decomposition of width $w+k$", SrcTodo)
        .ref_noted_relation("VS44M7", NotApplicable, &vertex_integrity, &treedepth, UpperBound(Linear), "First, treedepth removes vertices of the modulator, then it iterates through remaining components one by one.", SrcTodo)
        .ref_noted_relation("rmLeo2", NotApplicable, &dist_to_stars, &treedepth, UpperBound(Linear), "First, treedepth removes vertices of the modulator, remainder has treedepth $2$", SrcTodo)
        .ref_noted_relation("bYybsT", NotApplicable, &dist_to_complete, &clique_cover_num, UpperBound(Linear), "We cover the $k$ vertices of the modulator by cliques of size $1$ and cover the remaining clique by another one.", SrcTodo)
        .ref_noted_relation("gGtTUf", NotApplicable, &maximum_independent_set, &domination_num, UpperBound(Linear), "Every maximal independent set is also a dominating set because any undominated vertex could be added to the independent set.", SrcTodo)
        .ref_noted_relation("J0jyXi", NotApplicable, &domination_num, &diameter, UpperBound(Linear), "An unbounded diameter implies a long path where no vertices that are more than $3$ apart may be dominated by the same dominating vertex, otherwise we could shorten the path. Hence, the number of dominating vertices is also unbounded.", SrcTodo)
        .ref_noted_relation("xrVJqb", NotApplicable, &dist_to_bipartite, &chromatic_number, UpperBound(Linear), "Removed vertices get one color each and we need only $2$ colors for the rest.", SrcTodo)
        .ref_noted_relation("5wc1ir", NotApplicable, &edge_clique_cover, &neighborhood_diversity, UpperBound(Exponential), "Label vertices by the cliques they are contained in, each label is its own group in the neighborhood diversity, connect accordingly.", SrcTodo)
        .ref_noted_relation("RnkWvT", NotApplicable, &dist_to_complete, &edge_clique_cover, UpperBound(Polynomial), "Cover the remaining clique, cover each modulator vertex and its neighborhood outside of it with another clique, cover each edge within the modulator by its own edge.", SrcTodo)
        // .ref_noted_relation("8ouyNs", NotApplicable, &edge_clique_cover, &clique_cover_num, UpperBound(Linear), "Covering all edges ")
        .ref_noted_relation("FY0U1r", NotApplicable, &treewidth, &book_thickness, UpperBound(Exists), "", SrcTodo)
        .ref_noted_relation("BKCgft", NotApplicable, &maximum_leaf_num, &dist_to_linear_forest, UpperBound(Exists), "", SrcTodo)
        .ref_noted_relation("CyAMhs", NotApplicable, &acyclic_chromatic_number, &boxicity, UpperBound(Exists), "", SrcTodo)
        .ref_noted_relation("pUfoGn", NotApplicable, &hindex, &dist_to_maximum_degree, UpperBound(Linear), "Remove the $h$ vertices of degree at least $h$ to get a graph that has maximum degree $h$.", SrcTodo)
        .ref_noted_relation("8ZzI5w", NotApplicable, &dist_to_maximum_degree, &hindex, UpperBound(Linear), "Removal of $k$ vertices yielding a graph with maximum degree $c$ means that there were $k$ vertices of arbitrary degree and the remaining vertices had degree at most $k+c$. Hence, $h$-index is no more than $k+c$.", SrcTodo)
        .ref_noted_relation("fedm1t", NotApplicable, &dist_to_cograph, &chordality, UpperBound(Linear), "", SrcTodo)
        .ref_noted_relation("rGMb0t", NotApplicable, &dist_to_cograph, &diameter, UpperBound(Linear), "", SrcTodo)
        .ref_noted_relation("Er0L5w", NotApplicable, &book_thickness, &acyclic_chromatic_number, UpperBound(Exists), "", SrcTodo)
        // .ref_noted_relation("03kKbA", NotApplicable, &dist_to_planar, &acyclic_chromatic_number, UpperBound(Exists), "") // idk
        // .ref_noted_relation("wJkzlI", NotApplicable, &average_distance, &girth, StrictUpperBound(Exists), "Small average distance implies a small cycle while adding a triangle makes the girth constant and minimally changes the average distance.")
        .ref_noted_relation("gRJqnm", NotApplicable, &average_distance, &diameter, Exclusion, "join of a path and a complete bipartite graph", SrcTodo)
        .ref_noted_relation("JfSGx1", NotApplicable, &maximum_leaf_num, &feedback_edge_set, UpperBound(Exists), "", SrcTodo)
        .ref_noted_relation("LJQHKw", NotApplicable, &maximum_induced_matching, &diameter, UpperBound(Linear), "Diameter requires an induced path on $d$ edges, hence, maximum induced matching is at least $\\lfloor (d+1)/3 \\rfloor$.", SrcTodo)
        .ref_noted_relation("unkZhD", NotApplicable, &maximum_independent_set, &maximum_induced_matching, UpperBound(Linear), "Each edge of the induced matching can host at one vertex of the independent set.", SrcTodo)
        .ref_noted_relation("RqDij1", NotApplicable, &vertex_cover, &neighborhood_diversity, UpperBound(Exponential), "", SrcTodo)
        .ref_noted_relation("a2DTDH", NotApplicable, &twin_cover_num, &neighborhood_diversity, Exclusion, "", SrcTodo)
        .ref_noted_relation("Pinlr2", NotApplicable, &linear_clique_width, &clique_width, UpperBound(Exists), "", SrcTodo)
        .ref_noted_relation("OUUh3y", NotApplicable, &clique_width, &boolean_width, UpperBound(Linear), "", SrcTodo)
        .ref_noted_relation("hgUvsR", NotApplicable, &boolean_width, &clique_width, UpperBound(Exponential), "", SrcTodo)
        .ref_noted_relation("V9Pisv", NotApplicable, &branch_width, &boolean_width, UpperBound(Linear), "", SrcTodo)
        .ref_noted_relation("Q3Bz8d", NotApplicable, &module_width, &clique_width, Equivalent(Exists, Exists), "", SrcTodo)
        .ref_noted_relation("0zGd6N", NotApplicable, &branch_width, &rank_width, UpperBound(Linear), "", SrcTodo)
        .ref_noted_relation("QWXYYb", NotApplicable, &treewidth, &boolean_width, UpperBound(Exists), "", SrcTodo)
        .ref_noted_relation("mD6cvS", NotApplicable, &bandwidth, &cutwidth, Exactly(Polynomial), "Any bandwidth bound cutwidth quadratically. An example where this happens is $(P_n)^k$ which has bandwidth $k$ and cutwidth $O(k^2)$; both seem to be optimal.", SrcTodo)
        .ref_noted_relation("NTZE4R", NotApplicable, &modular_width, &clique_width, UpperBound(Exists), "", SrcTodo)
        .ref_noted_relation("Vq2BBF", NotApplicable, &modular_width, &diameter, UpperBound(Exists), "", SrcTodo)
        // .ref_noted_relation("TA2EZd", NotApplicable, &dist_to_planar, &twin_width, UpperBound(Exists), "") // dist may not, even if planar has bounded twin-width
        .ref_noted_relation("qB1OMb", NotApplicable, &maximum_degree, &c_closure, UpperBound(Exists), "", SrcTodo)
        .ref_noted_relation("fmiQlU", NotApplicable, &feedback_edge_set, &c_closure, UpperBound(Exists), "", SrcTodo)
// Bound(fr=vertex_cover, to=neighborhood_diversity, notes=[
    // Cite(id="YgTRtT", url="https://link.springer.com/article/10.1007/s00453-011-9554-x", text="Construct $k$ singleton sets, one for each vertex in the vertex cover and at most $2^k$ additional sets, one for each subset of vertices of the vertex cover. ...", range=Range(EXPONENTIAL)),
    // ])
        .ref_noted_relation("H1gQ6m", NotApplicable, &feedback_vertex_set, &dist_to_forest, Equal, "", SrcTodo)
        .ref_noted_relation("hDNUsi", NotApplicable, &vertex_cover, &dist_to_edgeless, Equal, "", SrcTodo)
        .ref_noted_relation("Jyi5e3", NotApplicable, &complete, &maximum_clique, Exclusion, cliques_make_it_unbounded, SrcTodo)
        .ref_noted_relation("t9mJyF", NotApplicable, &complete, &domatic_num, Exclusion, cliques_make_it_unbounded, SrcTodo)
        .ref_noted_relation("KnGxdS", NotApplicable, &complete, &edge_connectivity, Exclusion, cliques_make_it_unbounded, SrcTodo)
        .ref_noted_relation("fQjK7z", NotApplicable, &co_cluster, &dist_to_chordal, Exclusion, "", SrcTodo)
        .ref_noted_relation("cOXKlo", NotApplicable, &cluster, &twin_cover_num, UpperBound(Constant), "", SrcTodo)
        .ref_noted_relation("jIxF3A", NotApplicable, &cluster, &domination_num, Exclusion, "", SrcTodo)
        // .ref_noted_relation("OjWb8I", NotApplicable, &bipartite, &girth, Exclusion, "")
        .ref_noted_relation("d1qoN7", NotApplicable, &bipartite, &edge_connectivity, Exclusion, "", SrcTodo)
        .ref_noted_relation("Z335lf", NotApplicable, &forest, &feedback_edge_set, UpperBound(Constant), "", SrcTodo)
        // .ref_noted_relation("5pJxbA", NotApplicable, &forest, &girth, Exclusion, "")
        .ref_noted_relation("k18Pyk", NotApplicable, &forest, &dist_to_interval, Exclusion, "", SrcTodo)
        .ref_noted_relation("2QZo3T", NotApplicable, &edgeless, &vertex_cover, UpperBound(Constant), "", SrcTodo)
        .ref_noted_relation("cq2q83", NotApplicable, &edgeless, &domination_num, Exclusion, "", SrcTodo)
        .ref_noted_relation("TOJxXi", NotApplicable, &grid, &dist_to_chordal, Exclusion, "", SrcTodo)
        .ref_noted_relation("MRucBP", NotApplicable, &grid, &average_distance, Exclusion, "", SrcTodo)
        .ref_noted_relation("MYM6Ye", NotApplicable, &grid, &bisection_bandwidth, Exclusion, "", SrcTodo)
        .ref_noted_relation("VJwjbX", NotApplicable, &outerplanar, &bisection_bandwidth, UpperBound(Constant), "", SrcTodo)
        .ref_noted_relation("fQjK7z", NotApplicable, &grid, &maximum_degree, UpperBound(Constant), "", SrcTodo)
        .ref_noted_relation("OjWb8I", NotApplicable, &interval, &average_distance, Exclusion, "", SrcTodo)
        .ref_noted_relation("967lJ2", NotApplicable, &path, &treedepth, Exclusion, "", SrcTodo)
        .ref_noted_relation("ne07p3", NotApplicable, &linear_forest, &average_distance, Exclusion, "", SrcTodo)
        .ref_noted_relation("kkMeCO", NotApplicable, &planar, &genus, UpperBound(Constant), "", SrcTodo)
        // .ref_noted_relation("EZdonY", NotApplicable, &planar, &girth, Exclusion, "")
        .ref_noted_relation("cIAr80", NotApplicable, &planar, &maximum_degree, Exclusion, "", SrcTodo)
        .ref_noted_relation("DxmXhS", NotApplicable, &planar, &dist_to_perfect, Exclusion, "", SrcTodo)
        .ref_noted_relation("VAAXVv", NotApplicable, &vertex_integrity, &neighborhood_diversity, Exclusion, "", SrcTodo)
        .ref_noted_relation("CoBOm0", NotApplicable, &stars, &hindex, Exclusion, "", SrcTodo)
        .ref_noted_relation("Ei8B1H", NotApplicable, &stars, &vertex_integrity, Exclusion, "", SrcTodo)
        .ref_noted_relation("ORlCs0", NotApplicable, &cycles, &dist_to_perfect, Exclusion, "", SrcTodo)
        .ref_noted_relation("tZrOta", NotApplicable, &cycle, &maximum_leaf_num, UpperBound(Constant), "", SrcTodo)
        // .ref_noted_relation("cYF2KU", NotApplicable, &cycle, &girth, Exclusion, "")
        .ref_noted_relation("CkDe7e", NotApplicable, &maximum_leaf_num, &feedback_edge_set, UpperBound(Polynomial), "M. Bentert (personal communication)", SrcTodo) // todo not unknown
        .ref_noted_relation("QeiwSR", NotApplicable, &bounded_components, &cutwidth, UpperBound(Polynomial), "By greedily placing one component after another.", SrcTodo)
        .ref_noted_relation("EjGaM8", NotApplicable, &bounded_components, &dist_to_perfect, Exclusion, "By a disjoint union of small components with distance to perfect at least 1.", SrcTodo)
        .ref_noted_relation("bQLN2O", NotApplicable, &bounded_components, &dist_to_planar, Exclusion, "By a disjoint union of many $K_5$ graphs.", SrcTodo)
        .ref_noted_relation("MQ0K6A", NotApplicable, &star, &vertex_cover, UpperBound(Constant), "trivially", SrcTodo)
        .ref_noted_relation("btFVbS", NotApplicable, &star, &hindex, UpperBound(Constant), "trivially", SrcTodo)
        .ref_noted_relation("A2vYf3", NotApplicable, &tree, &hindex, Exclusion, "trivially", SrcTodo)
        .ref_noted_relation("vPk1LG", NotApplicable, &path, &dist_to_cluster, Exclusion, "trivially", SrcTodo)
        .ref_noted_relation("dy8lvH", NotApplicable, &path, &diameter, Exclusion, "trivially", SrcTodo)
        .ref_noted_relation("DsZGLl", NotApplicable, &cycles, &pathwidth, UpperBound(Constant), "trivially", SrcTodo)
        .ref_noted_relation("K9z178", NotApplicable, &star, &maximum_degree, Exclusion, "trivially", SrcTodo)
        .ref_noted_relation("f1nTaY", NotApplicable, &complete, &maximum_matching, Exclusion, "", SrcTodo)
        .ref_noted_relation("8io8sJ", NotApplicable, &path, &maximum_matching, Exclusion, "", SrcTodo)
        .ref_noted_relation("GiDjOm", NotApplicable, &star, &maximum_matching, UpperBound(Constant), "", SrcTodo)
        .ref_noted_relation("rmHBsY", NotApplicable, &edgeless, &maximum_matching, UpperBound(Constant), "", SrcTodo)
        // .ref_noted_relation("D2YglK", Unknown, &create.intersection("QrYeIw", &treewidth, &maximum_degree, "treewidth+maxdegree"), &create.intersection("hljuu8", &clique_width, &maximum_degree, "cliquewidth+maxdegree"), UpperBound(Linear), "")
        // .ref_noted_relation("JJTNMl", Unknown, &create.intersection("nP3xBv", &clique_width, &maximum_degree, "cliquewidth+maxdegree"), &create.intersection("iPgGur", &treewidth, &maximum_degree, "treewidth+maxdegree"), UpperBound(Linear), "")
        // clique-width = fusing width (operation to merge a color class to a single vertex)
        .ref_noted_relation("tiEYdy", NotApplicable, &clique_width, &mim_width, UpperBound(Exists), "", SrcTodo)
        .ref_noted_relation("5vq7po", NotApplicable, &mim_width, &sim_width, UpperBound(Exists), "", SrcTodo)
        .ref_noted_relation("sWR5Yw", NotApplicable, &treewidth, &tree_independence, UpperBound(Exists), "", SrcTodo)
        .ref_noted_relation("xwLQQ8", NotApplicable, &tree_independence, &sim_width, UpperBound(Exists), "", SrcTodo)
        .ref_noted_relation("E7K0I5", NotApplicable, &clique_width, &twin_width, UpperBound(Exists), "", SrcTodo)
        // .ref_noted_relation("KI6Jq6", NotApplicable, &minimum_degree, &dist_to_disconnected, UpperBound(Exists), "")
        // .ref_noted_relation("g20P5t", NotApplicable, &bisection_bandwidth, &dist_to_disconnected, UpperBound(Exists), "")
        .ref_noted_relation("V01YhI", NotApplicable, &dist_to_cluster, &dist_to_cograph, UpperBound(Exists), "", SrcTodo)
        .ref_noted_relation("zJ6L6N", NotApplicable, &feedback_vertex_set, &dist_to_outerplanar, UpperBound(Exists), "", SrcTodo)
        .ref_noted_relation("aTjwcL", NotApplicable, &dist_to_planar, &acyclic_chromatic_number, UpperBound(Exists), "", SrcTodo)
        .ref_noted_relation("VYcUHd", NotApplicable, &maximum_independent_set, &clique_cover_num, Exclusion, "", SrcTodo)
        .ref_noted_relation("Zv5i0U", NotApplicable, &domination_num, &maximum_independent_set, Exclusion, "", SrcTodo)
        .ref_noted_relation("Mogls9", NotApplicable, &genus, &chromatic_number, UpperBound(Linear), "in fact, bounded by square root", SrcTodo) // jansen mentions this is in Graphs, Colourings And The Four-Colour Theorem Get access Arrow by Robert A Wilson
        .ref_noted_relation("pavXOg", NotApplicable, &dist_to_cluster, &shrub_depth, UpperBound(Constant), "J. Pokorný, personal communication: Assume the class of constant dtc we want to show it has constant sd as well. For each clique connect them in a star in the tree model T. Each vertex in the modulator connect to their own vertex in T. Add a root that is in distance 2 to all leaves. Now give each vertex in the modulator a unique colour. Each other vertex that is not in the modulator has as it's colour the set of neighbours from the modulator. In total there are $2^{dtc} + dtc$ colours that is a constant.", SrcTodo)
        .ref_noted_relation("8dewYb", NotApplicable, &dist_to_co_cluster, &shrub_depth, UpperBound(Constant), "M. Dvořák, personal communication: The proof essentially follows the Reason why there's an arrow from cvdn (distance to cluster) to sd. Or note that distance to co-cluster is just complement of distance to cluster. And shrub-depth is closed under complemenetation.", SrcTodo)
        // noted in treebandwidth2025
        .ref_noted_relation("DOKD9Q", NotApplicable, &domino_treewidth, &slim_tree_cut_width, UpperBound(Exists), "", SrcTodo)
        .ref_noted_relation("HMJodd", NotApplicable, &slim_tree_cut_width, &edge_treewidth, UpperBound(Exists), "", SrcTodo)
        .ref_noted_relation("h03faf", NotApplicable, &edge_treewidth, &overlap_treewidth, UpperBound(Exists), "", SrcTodo)
        .ref_noted_relation("EXGHbI", NotApplicable, &overlap_treewidth, &treewidth, UpperBound(Exists), "", SrcTodo)
        .ref_noted_relation("727EaU", NotApplicable, &slim_tree_cut_width, &tree_cut_width, UpperBound(Exists), "", SrcTodo)
        .ref_noted_relation("NPiFw1", NotApplicable, &tree_cut_width, &tree_partition_width, UpperBound(Exists), "", SrcTodo)
        .ref_noted_relation("W4Oo7f", NotApplicable, &edge_treewidth, &tree_partition_width, UpperBound(Exists), "", SrcTodo)
        // .ref_noted_relation("", NotApplicable, &edge_treewidth, &biconnected_maximum_degree, UpperBound(Exists), "")
        // .ref_noted_relation("", NotApplicable, &maximum_degree, &biconnected_maximum_degree, UpperBound(Exists), "")
        // .ref_noted_relation("", NotApplicable, &biconnected_maximum_degree, &dipole_number, UpperBound(Exists), "")
        // .ref_noted_relation("", NotApplicable, &biconnected_maximum_degree, &fan_number, UpperBound(Exists), "")
        // .ref_noted_relation("", NotApplicable, &treebandwidth, &fan_number, UpperBound(Exists), "")
        // .ref_noted_relation("", NotApplicable, &overlap_treewidth, &dipole_number, UpperBound(Exists), "")
        .ref_noted_relation("F9TWg2", NotApplicable, &treewidth, &sparse_twin_width, UpperBound(Exists), "", SrcTodo)
        .ref_noted_relation("JCI2An", NotApplicable, &sparse_twin_width, &bounded_expansion, UpperBound(Exists), "", SrcTodo)
        .ref_noted_relation("Go6gbV", NotApplicable, &nowhere_dense, &monadically_stable, UpperBound(Constant), "", SrcTodo)
        .ref_noted_relation("MNJN0n", NotApplicable, &monadically_stable, &monadically_dependent, UpperBound(Constant), "", SrcTodo)
        .ref_noted_relation("uK0Bkd", NotApplicable, &twin_width, &monadically_dependent, UpperBound(Constant), "", SrcTodo)
        .ref_noted_relation("Lx2PDS", NotApplicable, &genus, &excluded_minor, UpperBound(Constant), "", SrcTodo)
        .ref_noted_relation("BpYOJt", NotApplicable, &sparse_twin_width, &twin_width, UpperBound(Exists), "", SrcTodo)
        .ref_noted_relation("D7mRIW", NotApplicable, &perfect, &chi_bounded, UpperBound(Constant), "", SrcTodo)
        .ref_noted_relation("jTiJAO", NotApplicable, &clique_width, &chi_bounded, UpperBound(Constant), "", SrcTodo)
        .ref_noted_relation("RliMGK", NotApplicable, &series_parallel, &chi_bounded, UpperBound(Constant), "", SrcTodo)
        .ref_noted_relation("KjnuD1", NotApplicable, &dist_to_chordal, &tree_independence, UpperBound(Linear), "Put the modulator to every bag of the natural chodal graph tree decomposition which contains a clique in every bag. The biggest independent set can contain the modulator and no more than a single vertex of the clique.", SrcTodo)
        .ref_noted_relation("E9szyw", NotApplicable, &treewidth, &modular_treewidth, UpperBound(Linear), "", SrcTodo)
        .ref_noted_relation("hreVoq", NotApplicable, &modular_treewidth, &clique_width, UpperBound(Exponential), "", SrcTodo)
        .ref_noted_relation("8r1GEU", NotApplicable, &branch_width, &sm_width, UpperBound(Exists), "", SrcTodo)
        .ref_noted_relation("DTLpQH", NotApplicable, &sm_width, &clique_width, UpperBound(Exists), "", SrcTodo)
        // .ref_noted_relation("", NotApplicable, &, &, UpperBound(Exists), "", SrcTodo)
        // .ref_noted_relation("", NotApplicable, &, &, UpperBound(Exists), "", SrcTodo)
        // .ref_noted_relation("", NotApplicable, &, &, UpperBound(Exists), "", SrcTodo)
        // .ref_noted_relation("", NotApplicable, &, &, UpperBound(Exists), "", SrcTodo)
        // .ref_noted_relation("", NotApplicable, &, &, UpperBound(Exists), "", SrcTodo)
        ;

    create.web_source("8ryhNq", "https://en.wikipedia.org/wiki/Genus_(mathematics)#Graph_theory")
        .redefined("3qF6Zm", NotApplicable, &genus, "The genus of a graph is the minimal integer $n$ such that the graph can be drawn without crossing itself on a sphere with $n$ handles.");
    create.web_source("bnOBjM", "https://link.springer.com/article/10.1007/bf01215352")
        .redefined("gMC8t4", NotApplicable, &carving_width, "Let $V$ be a finite set with $|V| \\ge 2$. Two subsets $A,B\\subseteq V$ \\emph{cross} if $A\\cap B$, $A-B$, $B-A$, $V-(A\\cup B)$ are all non-empty. A \\emph{carving} in $V$ is a set $\\mathscr{C}$ of subsets of $V$ such that 1) $\\emptyset, V \\notin \\mathscr{C}$ 2) no two members of $\\mathscr{C}$ cross, and 3) $\\mathscr{C}$ is maximal subject to (1) and (2). ... For $A \\subseteq V(G)$, we denote by $\\delta(A)$ ... the set of all edges with an end in $A$ and an end in $V(G)-A$. For each $e \\in E(G)$, let $p(e) \\ge 0$ be an integer. For $X \\subseteq E(G)$ we denote $\\sum_{e \\in X}p(e)$ by $p(X)$, and if $|V(G)| \\ge 2$ we define the \\emph{$p$-carving-width} of $G$ to be the minimum, over all carvings $\\mathscr{C}$ in $V(G)$, of the maximum, over all $A \\in \\mathscr{C}$, of $p(\\delta(A))$. ... The \\emph{carving-width} of $G$ is the $p$-carving-width of $G$ where $p(e)=1$ for every edge $e$.");
    create.web_source("s11UF7", "https://en.wikipedia.org/wiki/Carving_width")
        .redefined("LtcqRs", NotApplicable, &carving_width, "A carving can be described as an unrooted binary tree whose leaves are labeled with the vertices of the given graph. Removing any edge from this tree partitions the tree into two subtrees, and correspondingly partitions the vertices of the tree into two clusters. ... The width of a carving, defined in this way, is the maximum number of edges that connect two complementary clusters. The carving width of the graph is the minimum width of any hierarchical clustering.");
    create.web_source("s7OvjQ", "https://en.wikipedia.org/wiki/Graph_bandwidth")
        .redefined("9n7dry", NotApplicable, &bandwidth, "(paraphrased) Label graph vertices with distinct integers. Bandwidth of this labelling is the maximum over label differences over all edges. Bandwidth of a graph is the minimum over all labellings.");
    create.web_source("iWUynL", "https://en.wikipedia.org/wiki/Bisection_bandwidth")
        .redefined("Kj73IQ", NotApplicable, &bisection_bandwidth, "... bisected into two equal-sized partitions, the bisection bandwidth of a network topology is the bandwidth available between the two partitions.");
    create.web_source("AeRM2B", "http://parallelcomp.github.io/Lecture3.pdf")
        .redefined("w15E7O", NotApplicable, &bisection_bandwidth, "(number of) links across smallest cut that divides nodes in two (nearly) equal parts");
    create.web_source("BJhqpe", "https://en.wikipedia.org/wiki/Feedback_vertex_set")
        .redefined("xPcvEf", NotApplicable, &feedback_vertex_set, "... a feedback vertex set (FVS) of a graph is a set of vertices whose removal leaves a graph without cycles... . The feedback vertex set number of a graph is the size of a smallest feedback vertex set.");
    create.web_source("4Dua5N", "https://www.fi.muni.cz/~hlineny/papers/shrubdepth-warw18-slipp.pdf")
        .redefined("zWFoL1", Pp(7), &shrub_depth, "Tree-model of $m$ colors and depth $d$: a rooted tree $T$ of height $d$, leaves are the vertices of $G$, each leaf has one of $m$ colors, an associated signature determining the edge set of $G$ as follows: for $i=1,2,\\dots,d$, let $u$ and $v$ be leaves with the least common ancestor at height $i$ in $T$, then $uv \\in E(G)$ iff the color pair of $u,v$ is in the signature at height $i$.")
        .done(&mut create);
    create.web_source("dxaIhi", "https://mathworld.wolfram.com/Pathwidth.html")
        .redefined("OivGaa", NotApplicable, &pathwidth, "The pathwidth of a graph $G$, also called the interval thickness, vertex separation number, and node searching number, is one less than the size of the largest set in a path decomposition G.");
    create.web_source("ZhBkjd", "https://en.wikipedia.org/wiki/Branch-decomposition")
        .redefined("0SLCxV", NotApplicable, &branch_width, "... branch-decomposition of an undirected graph $G$ is a hierarchical clustering of the edges of $G$, represented by an unrooted binary tree $T$ with the edges of $G$ as its leaves. Removing any edge from $T$ partitions the edges of $G$ into two subgraphs, and the width of the decomposition is the maximum number of shared vertices of any pair of subgraphs formed in this way. The branchwidth of $G$ is the minimum width of any branch-decomposition of $G$.");
    create.web_source("9Ckusi", "https://en.wikipedia.org/wiki/Clique-width")
        .redefined("pLDACG", NotApplicable, &clique_width, "... the minimum number of labels needed to construct G by means of the following 4 operations: 1. Creation of a new vertex... 2. Disjoint union of two labeled graphs... 3. Joining by an edge every vertex labeled $i$ to every vertex labeled $j$, where $i \\ne j$ 4. Renaming label $i$ to label $j$");
    create.web_source("YGmwCG", "https://en.wikipedia.org/wiki/Book_embedding")
        .redefined("jiDWoN", NotApplicable, &book_thickness, "... a book embedding is a generalization of planar embedding of a graph to embeddings into a book, a collection of half-planes all having the same line as their boundary. Usually, the vertices of the graph are required to lie on this boundary line, called the spine, and the edges are required to stay within a single half-plane. The book thickness of a graph is the smallest possible number of half-planes for any book embedding of the graph.");
    create.web_source("cNSdgE", "https://www.graphclasses.org/classes/par_31.html")
        .redefined("JpPGki", NotApplicable, &acyclic_chromatic_number, "The acyclic chromatic number of a graph $G$ is the smallest size of a vertex partition $V_1,\\dots,V_\\ell$ such that each $V_i$ is an independent set and for all $i,j$ that graph $G[V_i \\cup V_j]$ does not contain a cycle.");
    create.web_source("rj2m4h", "https://en.wikipedia.org/wiki/Acyclic_coloring")
        .redefined("PQ9STH", NotApplicable, &acyclic_chromatic_number, "... an acyclic coloring is a (proper) vertex coloring in which every 2-chromatic subgraph is acyclic.");
    create.web_source("6LCwBu", "https://en.wikipedia.org/wiki/Degeneracy_(graph_theory)")
        .redefined("TYABmf", NotApplicable, &degeneracy, "... the least $k$ for which there exists an ordering of the vertices of $G$ in which each vertex has fewer than $k$ neighbors that are earlier in the ordering.");
    create.web_source("VqwUmp", "https://mathworld.wolfram.com/ChromaticNumber.html")
        .redefined("VLEw7q", NotApplicable, &chromatic_number, "The chromatic number of a graph G is the smallest number of colors needed to color the vertices of G so that no two adjacent vertices share the same color (Skiena 1990, p. 210), ...");
    create.web_source("o6tFCJ", "https://bookdown.org/omarlizardo/_main/2-7-average-degree.html")
        .redefined("PUQ3kt", NotApplicable, &average_degree, "Average degree is simply the average number of edges per node in the graph. ... Total Edges/Total Nodes=Average Degree");
    create.web_source("PVi4lL", "https://mathworld.wolfram.com/MaximumClique.html")
        .redefined("Nm1F3M", NotApplicable, &maximum_clique, "A maximum clique of a graph $G$ is a clique (i.e., complete subgraph) of maximum possible size for $G$.");
    create.web_source("ZunX1e", "https://mathworld.wolfram.com/EdgeConnectivity.html")
        .redefined("2gQP1W", NotApplicable, &edge_connectivity, "The edge connectivity, also called the line connectivity, of a graph is the minimum number of edges $\\lambda(G)$ whose deletion from a graph $G$ disconnects $G$.");
    create.web_source("XWbXPm", "https://en.wikipedia.org/wiki/Boxicity")
        .redefined("PgaxqR", NotApplicable, &boxicity, "The boxicity of a graph is the minimum dimension in which a given graph can be represented as an intersection graph of axis-parallel boxes.");
    create.web_source("8eXjAy", "https://mathworld.wolfram.com/DomaticNumber.html")
        .redefined("TG2BEi", NotApplicable, &domination_num, "The maximum number of disjoint dominating sets in a domatic partition of a graph $G$ is called its domatic number $d(G)$. ");
    create.web_source("055mG5", "https://en.wikipedia.org/wiki/Distance_(graph_theory)#Related_concepts")
        .redefined("OaKBaL", NotApplicable, &diameter, "... [diameter] is the greatest distance between any pair of vertices ...");
    create.web_source("GfSsR4", "https://onlinelibrary.wiley.com/doi/abs/10.1002/jgt.3190120309")
        .redefined("sBhhEO", NotApplicable, &average_degree, "The average distance in a graph is defined as the average length of a shortest path between two vertices, taken over all pairs of vertices.");
    // create.web_source("u13WN1", "https://en.wikipedia.org/wiki/Girth_(graph_theory)")
        // .redefined("INk53D", NotApplicable, &girth, "In graph theory, the girth of an undirected graph is the length of a shortest cycle contained in the graph.");
    create.web_source("8eXjAy", "https://mathworld.wolfram.com/DomaticNumber.html")
        .redefined("oTPnV8", NotApplicable, &domatic_num, "The maximum number of disjoint dominating sets in a domatic partition of a graph $G$ is called its domatic number $d(G)$. ");

    // let bandwidth_on_trees = create.intersection("Iu05N3", &tree, &bandwidth, "tree+bandwidth");
    // let cutwidth_on_trees = create.intersection("peyWzt", &tree, &cutwidth, "tree+cutwidth");

    let chung1985 = source("DkY1Aq", "Chung1985", 1)
        // .proved("YgJVvi", Unknown, &bandwidth_on_trees, &cutwidth_on_trees, UpperBound(Linear), "")
        // .proved("pRjX8u", Unknown, &cutwidth_on_trees, &bandwidth_on_trees, UpperBound(Linear), "")
        .todo_rest(&mut create);
    let chung1988 = source("ePpmZt", "chung1988", 1)
        .proved("fccHmU", Unknown, &maximum_independent_set, &average_distance, UpperBound(Linear), "[ed. paraphrased from another source] Let $G$ be a graph. Then $\\bar{D} \\le \\alpha$, with equality holding if and only if $G$ is complete.")
        .todo_rest(&mut create);
    let robertson_seymour1986 = source("i56ihO", "RobertsonSymour1986", 8)
        .defined("HHHQZT", Pp(1), &treewidth, "A \\emph{tree-decomposition} of $G$ is a family $(X_i \\colon i\\in I)$ of subsets of $V(G)$, together with a tree $T$ with $V(T)=I$, with the following properties. (W1) $\\bigcup(X_i \\colon i \\in I)=V(G)$. (W2) Every edge of $G$ has both its ends in some $X_i$ ($i \\in I$). (W3) For $i,j,k \\in I$, if $j$ lies on the path of $T$ from $i$ to $k$ then $X_i \\cap X_k \\subseteq X_j$. The \\emph{width} of the tree-decomposition is $\\max(|X_i|-1 \\colon i \\in I)$. The tree-width of $G$ is the minimum $w \\ge 0$ such that $G$ has a tree-decomposition of width $\\le w$.")
        .defined("aYyqd4", Pp(1), &treewidth, "Equivalently, the tree-width of $G$ is the minimum $w \\ge 0$ such that $G$ is a subgraph of a ``[[Cv1PaJ]]'' graph with all cliques of size at most $w + 1$.")
        // .proved("NqLFrC", Pp(2), "(1.2) For any fixed integer $w$, there is a polynomial algorithm to decide if the input graph has tree-width $\\le w$.") // non-constructive
        // .proved("a7nQ0N", Pp(6), treewidth, minor_closed, "(2.7) If $G$ has tree-width $< w$, so does ever minor of $G$.")
        .done(&mut create);
    let cozzens1989 = source("hEmGQO", "cozzens1989", 3)
        // todo file:///home/blazeva1/Downloads/BF01788656.pdf
        .todo_rest(&mut create);
    // let excludingforest1991 = source("AyLnH4", "excludingforest1991")
    // .todo_rest(&mut create);
    let chordality1993 = source("IFY0Rw", "chordality1993", 4)
        .redefined("Xdg7Hv", Pp(1), &chordality, "The \\emph{chordality} of a graph $G=(V,E)$ is defined as the minimum $k$ such that we can write $E=E_1,\\cap\\dots\\cap E_k$ with each $(V,E_i)$ a chordal graph.")
        .proved("D5VlqV", Pp(2), &size, &chordality, UpperBound(Linear), "Corollary 3. For any graph $G$, $\\mathrm{Chord}(G) \\le |V(G)|/2$.")
        .proved("rQBO3K", Pp(2), &chromatic_number, &chordality, UpperBound(Linear), "Corollary 4. For any graph $G$, $\\mathrm{Chord}(G) \\le \\chi(G)$, the chromatic number of $G$.")
        .proved("N0jfjr", Pp(5), &treewidth, &chordality, UpperBound(Linear), "Theorem 7. For any graph $G$, $\\mathrm{Chord}(G) \\le \\tau(G)$.")
        .done(&mut create);
    let malitz1994 = source("cCrsoK", "Malitz1994", 2)
        .proved("ECnpoM", Pp(24), &genus, &book_thickness, UpperBound(Linear), "Theorem 5.1. Genus $g$ graphs have pagenumber $O(\\sqrt{g})$.") // is optimal
        .done(&mut create);
    // ATTRIBUTIONS WIP ////////////////////////////////////////////////////////////
    let robertson_seymour1986_5 = source("A82svt", "RobertsonSymour1986V", 3)
        .proved("u4wtjE", Pp(2), &excluded_planar_minor, &treewidth, UpperBound(Constant), "(1.5) For every planar graph $H$, there is a number $w$ such that every planar graph with no minor isomorphic to $H$ has tree-wdtih $\\le w$")
        .todo_rest(&mut create);
    let robertson_seymour1991 = source("1hPzXs", "RobertsonSymour1991", 7)
        .defined("gMAL5e", Pp(12), &branch_width, "A \\emph{branch-width} of a hypergraph $G$ is a pair $(T,\\tau)$, where $T$ is a ternary tree and $\\tau$ is a bijection from the set of leaves of $T$ to $E(G)$. The \\emph{order} of an edge $e$ of $T$ is the number of vertices $v$ of $G$ such that there are leaves $t_1,t_2$ of $T$ in different components of $T \\setminus e$, with $\\tau(t_1),\\tau(t_2)$ both incident with $v$. The \\emph{width} of $(T,\\tau)$ is the maximum order of the edges of $T$, and the \\emph{branch-width} $\\beta(G)$ of $G$ is the minimum width of all branch-decompositions of $G$ (or 0 if $|E(G)| \\le 1$, when $G$ has no branch-decompositions).")
        // .proved("FN4FJJ", Pp(12), "(4.1) If $H$ is a minor of a graph $G$, then $\\beta(H) \\le \\beta(G)$.")
        .collective(Pp(16), "(5.1) For any hypergraph $G$, $\\max(\\beta(G), \\gamma(G)) \\le \\omega(G) + 1 \\le \\max(\\lfloor(3/2)\\beta(G)\\rfloor, \\gamma(G), 1)$.")
            .proved("8ewSpI", &treewidth, &branch_width, UpperBound(Linear))
            .proved("wrBAYk", &branch_width, &treewidth, UpperBound(Linear))
            .done()
        .done(&mut create);
    let bodlaender_mohring1993 = source("a3yKzk", "BodlaenderMohring1993", 5)
        .proved("cIAr80", Pp(4), &complete, &treewidth, Exclusion, "Lemma 3.1 (\"clique containment lemma\"). Let $(\\{X_i\\mid u\\in I\\},T=(I,F))$ be a tree-decomposition of $G=(V,E)$ and let $W \\subseteq V$ be a clique in $G$. Then there exists $i \\in I$ with $W \\subseteq X_i$.")
        .proved("mIvbmU", Pp(4), &bipartite, &treewidth, Exclusion, "Lemma 3.2 (\"complete bipartite subgraph containment lemma\").")
        // .proved("LDCZyj", Pp(5), &create.intersection("NxM8Gc", &cograph, &treewidth, ""), &create.intersection("chwMbI", &cograph, &pathwidth, ""), Equal, "Theorem 3.2. For every cograph $G = (V,E)$, $treewidth(G) = pathwidth(G)$.")
        // .proved(Theorem 4.1. The pathwidth and treewidth of a cograph given with a corresponding cotree can be computed in $O(n)$ time.)
        .done(&mut create);
    // clique-width ideas in 'Handle-Rewriting Hypergraph Grammars'
    let wanke1994 = source("SQjcYg", "Wanke1994", 3)
        .defined("ENvDZb", Pp(3), &nlc_width, "Definition 2.1. Let $k \\in \\mathbb N$ be a positive integer. A \\emph{$k$-node label controlled (NLC) graph} is a $k$-NL graph defined as follows: ...")
        .defined("yNzt7o", Pp(4), &nlct_width, "Definition 2.2. Let $k \\in \\mathbb N$ be a positive integer. A \\emph{$k$-node label controlled (NLC) tree} is a $k$-NL graph defined as follows: ...")
        .proved("jBcoBD", Pp(5), &cograph, &nlc_width, UpperBound(Constant), "Fact 2.3. $G$ is a $1$-NLC graph if and only if $unlab(G)$ is a co-graph.")
        .proved("cXI1DK", Pp(6), &treewidth, &nlc_width, UpperBound(Exponential), "Theorem 2.5. For each partial $k$-tree $G$ there is a $(2^{k+1}-1)$-NLC tree $J$ with $G=unlab(J)$.")
        .done(&mut create);
    let domino_treewidth1997 = source("SN1KnV", "dominoTreewidth1997", 5)
        .defined("Aew4hT", Pp(3), &domino_treewidth, "A tree-decomposition ... is a domino tree-decomposition, if ... every vertex belongs to at most two sets $X_i$. The domino treewidth of a graph $G$ is the minimum width over all domino tree-decompositions of $G$.")
        .todo_rest(&mut create);
    let bodlaender1998 = source("BOFCWg", "Bodlaender1998", 6)
        .proved("uHJAUo", Pp(4), &pathwidth, &treewidth, UpperBound(Linear), "Lemma 3. (a) For all graphs $G$, $pathwidth(G) \\ge treewidth(G)$. ...")
        .defined("oGAdW1", Pp(5), &branch_width, "A \\emph{branch decomposition} of a graph $G=(V,E)$ is a pair $(T=(I,F),\\sigma)$, where $T$ is a tree with every node in $T$ of degree one of three, and $\\sigma$ is a bijection from $E$ to the set of leaves in $T$. The \\emph{order} of an edge $f \\in F$ is the number of vertices $v \\in V$, for which there exist adjacent edges $(v,w),(v,x) \\in E$, such that the path in $T$ from $\\sigma(v,w)$ to $\\sigma(v,x)$ uses $f$. The \\emph{width} of branch decomposition $(T=(I,F),\\sigma)$, is the maximum order over all edges $f \\in F$. The \\emph{branchwidth} of $G$ is the minimum width over all branch decompositions of $G$.")
        // page 6, tw and pw do not rise for subgraphs
        // mark
        .defined("hajrD0", Pp(22), &bandwidth, "Let $G=(V,E)$ be a graph, and let $f\\colon V\\to \\{1,2,\\dots,n\\}$ be a linear ordering of $G$. 1. The \\emph{bandwidth} of $f$ is $\\max\\{|f(v)-f(w)| \\mid (v,w) \\in E\\}$. ... The bandwidth ... is the minimum bandwidth ... over all possible linear orderings of $G$.")
        .defined("c6Hdu3", Pp(22), &cutwidth, "Let $G=(V,E)$ be a graph, and let $f\\colon V\\to \\{1,2,\\dots,n\\}$ be a linear ordering of $G$. ... 2. The \\emph{cutwidth} of $f$ is $\\max_{1\\le i\\le n} |\\{(u,v)\\in E \\mid f(u) \\le i < f(v) \\}|$. ... [cutwidth] of a graph $G$ is the minimum [cutwidth] ... over all possible linear orderings of $G$.")
        .defined("H3lAh2", Pp(22), &topological_bandwidth, "The \\emph{topological bandwidth} of a graph $G$ is the minimum [bandwidth](../aP5a38) over all graphs $G'$ which are obtained by addition of an arbitrary number of vertices along edges of $G$.")
        .proved("kiza4J", Pp(23), &bandwidth, &pathwidth, UpperBound(Linear), "Theorem 44. For every graph $G$, the pathwidth of $G$ is at most the bandwidth of $G$, ... Proof. Let $f \\colon V\\to \\{1,\\dots,n\\}$ be a linear ordering of $G$ with bandwidth $k$. Then $(X_1,\\dots,X_{n-k})$ with $X_i=\\{f^{-1}(i), f^{-1}(i+1), \\dots, f^{-1}(i+k)\\}$ is a path decomposition of $G$ with pathwidth $k$. ...")
        .proved("RQriva", Pp(23), &topological_bandwidth, &pathwidth, UpperBound(Linear), "Theorem 45. For every graph $G$, the pathwidth of $G$ is at most the topological band-width of $G$.")
        .proved("iiE5jo", Pp(24), &cutwidth, &pathwidth, UpperBound(Linear), "Theorem 47. For every graph $G$, the pathwidth of $G$ is at most the cutwidth of $G$.")
        .proved("RgLQ2P", Pp(24), &degree_pathwidth, &cutwidth, Equivalent(Linear, Linear), "Theorem 49.")
        .proved("VdNTHZ", Pp(34), &outerplanar, &treewidth, UpperBound(Constant), "Lemma 78. Every outerplanar graph $G=(V,E)$ has treewidth at most 2.") // check whether dist_to_outerplanar bounding treewidth infered from this?
        .proved("oFitZo", Pp(37), &grid, &treewidth, Exclusion, "Lemma 88. The treewidth of an $n \\times n$ grid graph ... is at least $n$.")
        .proved("KoFslx", Pp(38), &treewidth, &minimum_degree, UpperBound(Linear), "Lemma 90 (Scheffler [94]). Every graph of treewidth at most $k$ contains a vertex of degree at most $k$.") // todo Schemer, Die Baumweite von Graphen als ein Ma8 Rir die Kompliziertheit algorithmischer Probleme, Ph.D. Thesis, Akademie der Wissenschafien der DDR, Berlin, 1989.
        .done(&mut create);
    let johansson1998 = source("W2nwG4", "Johansson1998", 3) // according to Gurski2005
        .proved("DBXQMa", Unknown, &clique_width, &nlc_width, UpperBound(Linear), "")
        .proved("BjlRwP", Unknown, &nlc_width, &clique_width, UpperBound(Linear), "")
        .proved("qy5Ojn", Unknown, &linear_clique_width, &linear_nlc_width, UpperBound(Linear), "")
        .proved("hI8Txh", Unknown, &linear_nlc_width, &linear_clique_width, UpperBound(Linear), "")
        .todo_rest(&mut create);
    let domino_treewidth1999 = source("gcMYuX", "dominoTreewidth1999", 4)
        .proved("vXf5Ie", Pp(4), &degree_treewidth, &domino_treewidth, UpperBound(Polynomial), "Theorem 3.1 Let $G=(V,E)$ be a graph with treewidth at most $k$ and maximum degree at most $d$. Then the domino treewidth of $G$ is at most $(9k+7)d(d+1)-1$.")
        .proved("KRgpco", Pp(7), &degree_treewidth, &domino_treewidth, LowerBound(Polynomial), "Lemma 4.3 For all $d \\ge 5$, $k \\ge 2$, $k$ even, there exists a graph $G$ with treewidth at most $k$, maximum degree at most $d$, and domino treewidth at least $\\frac{1}{12} kd-2$.")
        // .proved("YXCJHm", Pp(9), &create.intersection(&tree, &maximum_degree), &domino_treewidth, UpperBound(Linear), "Theorem 5.1 The domino treewidth of a tree is at most its maximum degree.")
        .done(&mut create);
    let courcelle_olariu_2000 = source("ZQrXS8", "courcelle2000", 5)
        // .defined("OL0McK", Unknown, &clique_width, "")
        .proved("sGBrPC", Pp(18), &treewidth, &clique_width, UpperBound(Exponential), "We will prove that for every undirected graph $G$, $cwd(G) \\le 2^{twd(G)+1}+1$ ...")
        .done(&mut create);
    let tack_layouts2004 = source("w7RVn9", "TackLayouts2004", 3)
        // .defined("bcdAXe", Pp(2), &track_number, "The track-number of $G$ is $\\mathrm{tn}_1(G)$, ...")
        // .proved("ZXhXax", Pp(12), &track_number, &acyclic_chromatic_number, UpperBound(Exponential), "Corollary 3. Acyclic chromatic number is bounded by track-number. In particular, every $(k,t)$-track graph $G$ has acyclic chromatic number $\\chi_a(G) \\le t \\cdot 4^{\\binom k2(t_1)}$.")
        .proved("v1Ygyr", Pp(14), &book_thickness, &acyclic_chromatic_number, UpperBound(Exponential), "Theorem 5. Acyclic chromatic number is bounded by stack-number (ed: a.k.a. book-thickness). In particular, every $k$-stack graph $G$ has acyclich chromatic number $\\chi_a(G) \\le 80^{k(2k-1)}$.")
        .done(&mut create);
    // let corneil2005 = source("HCGunF", "Corneil2005")
    // .proved("sGBrPC", Unknown, &treewidth, &clique_width, Exactly(Exponential), "... the clique-width of $G$ is at most $3 \\cdot 2k - 1$ and, more importantly, that there is an exponential lower bound on this relationship. In particular, for any $k$, there is a graph $G$ with treewidth equal to $k$, where the clique-width of $G$ is at least $2\\lfloor k/2\\rfloor-1$.")
    // .todo_rest(&mut create);
    let treespan2005 = source("mIg9Mh", "treespan2005", 4)
        .defined("ytHbX3", Pp(4), &treespan, "Definitions 2.1 and 2.2")
        .todo_rest(&mut create);
    let gurski2005 = source("FLSQsw", "Gurski2005", 3)
        .defined("umo10J", Pp(4), &linear_nlc_width, "Definition 3")
        .defined("ZPOCMc", Pp(4), &clique_tree_width, "Definition 5")
        .defined("q9qg89", Pp(5), &linear_clique_width, "Definition 6") // as noted in footnote of 10.1016/j.jctb.2007.04.001
        .proved("lY4S8K", Pp(8), &linear_nlc_width, &nlct_width, UpperBound(Linear), "")
        .proved("QBpUMV", Pp(8), &nlct_width, &nlc_width, UpperBound(Linear), "")
        .proved("CwlGA8", Pp(8), &linear_clique_width, &clique_tree_width, UpperBound(Linear), "")
        .proved("pY3u9l", Pp(8), &clique_tree_width, &clique_width, UpperBound(Linear), "")
        .proved("hxsxob", Pp(8), &clique_tree_width, &nlct_width, UpperBound(Linear), "")
        .proved("JXeEwu", Pp(8), &nlct_width, &clique_tree_width, UpperBound(Linear), "")
        .collective(Pp(8), "The results of [23] imply that each graph class of bounded path-width has bounded linear NLC-width and that each graph class of bounded tree-width has bounded NLCT-width.")
            .proved("mwTHcM", &pathwidth, &linear_nlc_width, UpperBound(Exists))
            .proved("BELFKR", &treewidth, &nlct_width, UpperBound(Exists))
            .done()
        // .proved("3udN1G", Pp(8), &treewidth, &nlct_width, UpperBound(Linear), "")
        .done(&mut create);
    let oum2006 = source("1ZTWBd", "Oum2006", 4)
        .defined("SGJJ1Y", Pp(9), &rank_width, "... and the \\emph{rank-width} $\\mathrm{rwd}(G)$ of $G$ is the branch-width of $\\mathrm{cutrk}_G$.")
        .proved("yLdAHe", Pp(9), &rank_width, &clique_width, Exactly(Exponential), "Proposition 6.3")
        .proved("uEUXMq", Pp(9), &clique_width, &rank_width, UpperBound(Linear), "Proposition 6.3")
        .done(&mut create);
    let geometric_thickness2007 = source("2q7m9E", "GeometricThickness2007", 4)
        // .defined("3p2P4E", Pp(3), &thickness, "The thickness of a graph $G$, ..., is the minimum number of planar subgraphs that partition (ed: edges of) $G$.") // defined by Tutte 1963
        // .defined("j9NrW9", Pp(3), &outerthickness, "The outerthickness of a graph $G$, ..., is the minimum number of outerplanar subgraphs that partition (ed: edges of) $G$.")
        // .proved("0B1cGr", Pp(4), &treewidth, &thickness, UpperBound(Linear), "Proposition 1. The maximum thickness of a graph in $\\mathcal T_k$ (ed: $k$-tree) is $\\lceil k/2 \\rceil$; ...")
        .proved("3zMwH9", Pp(5), &treewidth, &arboricity, UpperBound(Linear), "Proposition 2. The maximum arboricity of a graph in $\\mathcal T_k$ (ed: $k$-tree) is $k$; ...")
        // .proved("hXbdpU", Pp(5), &treewidth, &outerthickness, UpperBound(Linear), "Proposition 3. The maximum outerthickness of a graph in $\\mathcal T_k$ (ed: $k$-tree) is $k$; ...")
        // .proved("EKKZPJ", Pp(6), &treewidth, &star_arboricity, UpperBound(Linear), "Proposition 4. The maximum star-arboricity of a graph in $\\mathcal T_k$ (ed: $k$-tree) is $k+1$; ...")
        .defined("IxPMGS", Pp(7), &book_thickness, "A geometric drawing in which the vertices are in convex position is called a book embedding. The book thickness of a graph $G$, ..., is the minimum $k \\in \\mathbb N$ such that there is book embedding of $G$ with thickness $k$.")
        .proved("FY0U1r", Pp(8), &treewidth, &book_thickness, UpperBound(Linear), "The maximum book thickness ... of a graph $\\mathcal T_k$ (ed: $k$-tree) satisfy ... $=k$ for $k \\le 2$, $=k+1$ for $k \\ge 3$.")
        .todo_rest(&mut create);
    let contraction_complexity2008 = source("9JAQC7", "contractionComplexity2008", 2)
        .defined("4lmvZK", Pp(10), &contraction_complexity, "Definition 4.1. The contraction of an edge e removes e and replaces its end vertices (or vertex) with a single vertex. A contraction ordering π is an ordering of all the edges of G, π(1), π(2), . . ., π(|E(G)|). The complexity of π is the maximum degree of a merged vertex during the contraction process. The contraction complexity of G, denoted by cc(G), is the minimum complexity of a contraction ordering.")
        .proved("F0p61H", Pp(10), &contraction_complexity, &maximum_degree, UpperBound(Linear), "$cc(G) \\ge \\Delta(G) - 1$")
        .collective(Pp(11), "Proposition 4.2. ... $cc(G)=tw(G^*)$ ... Lemma 4.4. $(tw(G) - 1)/2 \\le tw(G^*) \\le \\Delta(G)(tw(G) + 1) - 1.$")
            .proved("YhbKPB", &contraction_complexity, &treewidth, UpperBound(Linear))
            .proved("YvvmJE", &degree_treewidth, &contraction_complexity, UpperBound(Polynomial))
            .done()
        .done(&mut create);
    let delavina_waller2008 = source("C5cBsd", "spanningTreesManyLeaves2008", 2)
        .proved("Pbg2ga", Pp(5), &bipartite_number, &average_distance, UpperBound(Linear), "Theorem 9 (Main Theorem). Let $G$ be a graph. Then $\\bar{D} < \\frac b2 + \\frac 12$. ...")
        // .proved("ZXINaY", Unknown, &maximum_leaf_num, &feedback_vertex_set, UpperBound(Linear), "")
        .done(&mut create);
    let gradnesetril2008 = source("kXDDmb", "gradnesetril2008", 3)
        .proved("VLpzhW", Unknown, &d_path_free, &treedepth, UpperBound(Polynomial), "") // todo
        .proved("Q7qpEp", Unknown, &treedepth, &d_path_free, UpperBound(Exponential), "") // todo
        // d_path_free
        .todo_rest(&mut create);
    let cliquewidthnpc2009 = source("zuhSo5", "cliquewidthnpc2009", 2)
        .proved("i1eBMN", Pp(8), &pathwidth, &linear_clique_width, UpperBound(Linear), "(5) $\\mathrm{lin-cwd}(G) \\le \\mathrm{pwd}(G)+2$.")
        .todo_rest(&mut create);
    let wood_partition2009 = source("p00uyg", "WoodPartition2009", 3)
        .redefined("AKfiZY", Pp(1), &tree_partition_width, "A graph $H$ is a partition of a graph $G$ if: each vertex of $H$ is a set of vertices of $G$ (called a bag), every evrtex of $G$ is in exactly one bag of $H$, and distinct bags $A$ and $B$ are adjacent in $H$ if and only if there is an edge of $G$ with one endpoint in $A$ and the other endpoint in $B$. The width of a partition is the maximum number of vertices in a bag. ... If a forest $T$ is a partition of a graph $G$, then $T$ is a tree-partition of $G$. The tree-partition-width of $G$ ... is the minimum width of a tree-partition of $G$.")
        .noted_relation("Pu8yGH", Pp(2), &tree_partition_width, &treewidth, UpperBound(Linear), "$2twp(G) \\ge tw(G)+1$", Text("Detlef Seese, Tree-partite graphs and the complexity of algorithms, in: Lothar Budach (Ed.), Proc. International Conf. on Fundamentals of Computation Theory, in: Lecture Notes in Comput. Sci., vol. 199, Springer, 1985, pp. 412–421.".into()))
        .noted_relation("jXe8W6", Pp(2), &degree_treewidth, &tree_partition_width, UpperBound(Polynomial), "$twp(G) \\le 24tw(G)\\Delta(G)$", Text("Guoli Ding, Bogdan Oporowski, Some results on tree decomposition of graphs, J. Graph Theory 20 (4) (1995) 481–499.".into()))
        .proved("H0DqDz", Pp(2), &degree_treewidth, &tree_partition_width, UpperBound(Polynomial), "Theorem 1. ... $twp(G) < \\frac 52 (tw(G)+1)(\\frac 72 \\Delta-1)$")
        .proved("IR39i5", Pp(2), &degree_treewidth, &tree_partition_width, LowerBound(Polynomial), "Theorem 2. ... there is a chordal graph $G$ ... $twp(G) \\ge (\\frac 18 - \\epsilon)tw(G)\\Delta(G).$")
        .done(&mut create);
    let jelinek2010 = source("vIBI5v", "Jelinek2010", 1)
        .proved("ipo6rm", Pp(2), &grid, &rank_width, Exclusion, "The grid $G_{n,n}$ has rank-width equal to $n-1$.")
        .done(&mut create);
    let sasak2010 = source("XlBXyo", "Sasak2010", 7)
        .proved("Lp8I7N", Pp(16), &tree, &pathwidth, Exclusion, "Theorem 2.1") // cites excludingforest1991, cannot find there
        .proved("BtRWId", Pp(17), &complete, &treewidth, Exclusion, "Theorem 2.2")
        .proved("GN6dar", Pp(17), &grid, &treewidth, Exclusion, "Theorem 2.4") // cites Csaba Biró. Tree-width and grids, cannot find the paper
        // .cited("kXDDmb", Pp(21), jelinek2010, "Theorem 2.14 [12] Rank-width of a grid $\\sqrt{n} \\times \\sqrt{n}$ on $n$ vertices is $\\sqrt{n}-1$.")
        // Theorem 2.15 [4] Boolean-width of a grid √n×√n on n vertices lies between
        .proved("BwIc79", Pp(28), &cutwidth, &maximum_degree, UpperBound(Linear), "Lemma 2.18. For any graph $G$ and any vertex $v \\in V(G), cutw(g) \\ge \\lceil \\frac{deg(v)}2 \\rceil$.")
        .proved("h49cUu", Pp(30), &carving_width, &maximum_degree, UpperBound(Linear), "Lemma 2.20 Carving-width of a graph $G$ is at least $\\Delta(G)$ where $\\Delta(G)$ is the maximum degree of a graph $G$.")
        .proved("2Wk4AF", Pp(30), &star, &carving_width, Exclusion, "Corollary 2.21 Carving-width of a star is $n-1$.")
        .proved("6Ln8ux", Pp(30), &path, &carving_width, UpperBound(Constant), "... path with carving-width 2.")
        // .cited("7jmzab", Pp(32), &gradnesetril2008, "Theorem 2.23 [13] Let $l$ be the length of the longest path in a graph $G$. Then the tree-depth of $G$ is bounded as follows: $\\lceil \\log_2(l+2)\\rceil \\le td(G) \\le \\binom{l+3}2-1$ ...")
        .proved("MboUFT", Pp(32), &grid, &treedepth, Exclusion, "Corollary 2.24 Tree-depth of a grid is at least $\\lceil \\log_2(n+1)\\rceil$.")
        .proved("WiiQn4", Pp(38), &cutwidth, &carving_width, UpperBound(Linear), "Theorem 4.3 (carw $\\prec$ cutw) Carving-width is bounded by cut-width.")
        .proved("u5VPeX", Pp(49), &carving_width, &treewidth, UpperBound(Linear), "Theorem 5.5 (tw $\\prec$ carw) Tree-width is bounded by carving-width.")
        .todo_rest(&mut create);
    let bipboxicity2011 = source("Vkc4EU", "bipboxicity2011", 2)
        .proved("Yelk6V", Pp(9), &dist_to_bipartite, &boxicity, Exclusion, "Theorem 2 For any $b \\in \\mathbb N^+$, there exists a chordal bipartite graph $G$ (ed: i.e. bipartite graph with no induced cycle on more than 4 vertices) with $\\mathrm{box}(G) > b$.")
        .done(&mut create);
    let bui_xuan2011 = source("cNjhWx", "BuiXuan2011", 4)
        .defined("L7aY6D", Unknown, &boolean_width, "\\textbf{Definition 1.} A decomposition tree of a graph $G$ is a pair $(T,\\delta)$ where $T$ is a tree having internal nodes of degree three and $\\delta$ a bijection between the leaf set of $T$ and the vertex set of $G$. Removing an edge from $T$ results in two subtrees, and in a cut $\\{A,\\overline{A}\\}$ of $G$ given by the two subsets of $V(G)$ in bijection $\\delta$ with the leaves of the two subtrees. Let $f\\colon w^V \\to \\mathbb{R}$ be a symmetric function that is also called a cut function: $f(A)=f(\\overline{A})$ for all $A\\subseteq V(G)$. The $f$-width of $(T,\\delta)$ is the maximum value of $f(A)$ over all cuts $\\{A,\\overline{A}\\}$ of $G$ given by the removal of an edge of $T$. ... \\textbf{Definition 2.} Let $G$ be a graph and $A \\subseteq V(G)$. Define the set of unions of neighborhoods of $A$ across the cut $\\{A,\\overline{A}\\}$ as $U(A) = \\{Y \\subseteq \\overline{A} \\mid \\exists X \\subseteq A \\land Y=N(X)\\cap \\overline{A}\\}$. The \\emph{bool-dim}$\\colon 2^{V(G)} \\to \\mathbb{R}$ function of a graph $G$ is defined as $\\mathrm{bool-dim}(A)=\\log_2 |U(A)|$. Using Definition 1 with $f=\\mathrm{bool-dim}$ we define the boolean-width of a decomposition tree, denoted by $boolw(T,\\delta)$, and the boolean-width of a graph, denoted by $boolw(G)$.")
        .proved("AdNkCy", Unknown, &boolean_width, &rank_width, UpperBound(Exponential), "\\textbf{Corollary 1.} For any graph $G$ and decomposition tree $(T,\\gamma)$ of $G$ it holds that ... $\\log_2 rw(G) \\le boolw(G)$ ...")
        .proved("cIWQDn", Unknown, &rank_width, &boolean_width, UpperBound(Polynomial), "\\textbf{Corollary 1.} For any graph $G$ and decomposition tree $(T,\\gamma)$ of $G$ it holds that ... $boolw(G) \\le \\frac 14 rw^2(G)+O(rw(G))$.")
        .todo_rest(&mut create);
    let lampis2012 = source("0LYUEV", "lampis2012", 1)
        .defined("ljbw1n", NotApplicable, &neighborhood_diversity, "We will say that two vertices $v, v'$ of a graph $G(V, E)$ have the same *type* iff they have the same colors and $N(v) \\setminus \\{v\\}=N(v') \\setminus \\{v\\}$, where $N(v)$ denotes the set of neighbors of $v$. ... A colored graph $G(V, E)$ has neighborhood diversity at most $w$, if there exists a partition of $V$ into at most $w$ sets, such that all the vertices in each set have the same type.")
        .todo_rest(&mut create);
    let rankwidth_chibounded2012 = source("DbAWDM", "rankwidthChibounded2012", 4)
        .proved("Rf7Twn", Pp(2), &rank_width, &chi_bounded, UpperBound(Exists), "Theorem 1. For any $k$, the class of graphs with rank-width at most $k$ is $\\chi$-bounded.")
        .done(&mut create);
    let ganian_twin_cover2012 = source("7UoBR6", "GanianTwinCover2012", 4)
        .defined("k6ApS2", Pp(262), &twin_cover_num, "Definition 3.1. $X \\subseteq V(G)$ is a twin-cover of $G$ if for every edge $e=\\{a,b\\} \\in E(G)$ either 1. $a \\in X$ or $b \\in X$, or 2. $a$ and $b$ are twins, i.e. all other vertices are either adjacent to both $a$ and $b$ or none. We then say that $G$ has twin-cover number $k$ if $k$ is the minimum possible size of a twin-cover of $G$.")
        .defined("pFk5uY", Pp(262), &twin_cover_num, "Definition 3.2. $X \\subseteq V(G)$ is a twin-cover of $G$ if there exists a subgraph $G'$ of $G$ such that 1. $X \\subseteq V(G')$ and $X$ is a vertex cover of $G'$. 2. $G$ can be obtained by iteratively adding twins to non-cover vertices in $G'$.")
        .proved("oxtaEE", Pp(263), &complete, &twin_cover_num, UpperBound(Constant), "We note that complete graphs indeed have a twin-cover of zero.")
        .proved("nkOAMh", Pp(263), &twin_cover_num, &vertex_cover, Exclusion, "The vertex cover of graphs of bounded twin-cover may be arbitrarily large.")
        .collective(Pp(263), "There exists graphs with arbitrarily large twin-cover and bounded tree-width and vice-versa.")
            .proved("gmsOd4", &twin_cover_num, &treewidth, Exclusion)
            .proved("iG3eGq", &treewidth, &twin_cover_num, Exclusion)
            .done()
        .proved("E8oHKm", Pp(263), &twin_cover_num, &clique_width, UpperBound(Linear), "The clique-width of graphs of twin-cover $k$ is at most $k+2$.")
        .proved("qB058E", Pp(263), &twin_cover_num, &rank_width, UpperBound(Linear), "The rank-width and linaer rank-width of graph of twin-cover $k$ are at most $k+1$.")
        .proved("WZcIOW", Pp(263), &twin_cover_num, &linear_rank_width, UpperBound(Linear), "The rank-width and linaer rank-width of graph of twin-cover $k$ are at most $k+1$.")
        // .tractable("XwMEnS", Pp(263), &twin_cover_num, &twin_cover_num, FPT, "Theorem 3.4. It is possible to find a twin-cover of size $k$ in time $O(|E||V|+k|V|+1.2738^k)$.")
        // .tractable("PxaiDG", Pp(267), &twin_cover_num, &boxicity, FPT, "Theorem 4.6. The Boxicity
        // problem can be solved in time $2^{O(2^kk^2)}|V|$ on graph of twin-cover at most $k$.")
        .done(&mut create);
    let vatshelle2012 = source("nRO7AG", "Vatshelle2012", 3)
        .defined("5sKQe5", Pp(33), &mm_width, "Definition 3.6.1 (MM-width). For $G$ a graph and $A \\subseteq V(G)$ let $mm \\colon 2^{V(G)} \\to \\mathbb N$ be a function where $mm(A)$ for $A \\subseteq V(G)$ is the size of a maximum matching in $G[A,\\bar A]$. Using Definition 3.1.3 with $f=mm$ we define $mmw(T,\\delta)$ as the $f$-width of a binary decomposition tree $(T,\\delta)$ and $mmw(G)$ as the $f$-width of $G$, also called the MM-width of $G$ or the maximum matching width.")
        .defined("Usp3Ca", Pp(33), &mim_width, "Definition 3.7.1 (MIM-width). For $G$ a graph and $A \\subseteq V(G)$ let $mim \\colon 2^{V(G)} \\to \\mathbb N$ be a function where $mim(A)$ is the size of a maximum induced matching in $G[A,\\bar A]$. Using Definition 3.1.3 with $f=mim$ we define $mimw(T,\\delta)$ as the $f$-width of a binary decomposition tree $(T,\\delta)$ and $mimw(G)$ as the $f$-width of $G$, also called the MIM-width of $G$ or the maximum induced matching width.")
        .proved("yPnIog", Pp(40), &treewidth, &mm_width, Equivalent(Linear, Linear), "Theorem 4.2.5. Let $G$ be a graph, then $\\frac 13 (tw(G)+1) \\le mmw(G) \\le \\max(brw(G),1) \\le tw(G)+1$")
        .proved("77zJ2z", Pp(42), &boolean_width, &mim_width, UpperBound(Linear), "Theorem 4.2.10. Let $G$ be a graph, then $mimw(G) \\le boolw(G) \\le mimw(G) \\log_2(n)$")
        .todo_rest(&mut create);
    let modularwidth2013 = source("OH3sI3", "modularwidth2013", 3)
        .proved("NeMJtU", Pp(6), &neighborhood_diversity, &modular_width, StrictUpperBound(Linear), "Theorem 3. Let $G$ be a graph. Then $\\mathrm{mw}(G) \\le \\mathrm{nd}(G)$ and $\\mathrm{mw}(G) \\le 2\\mathrm{tc}(G) + \\mathrm{tc}(G)$. Furthermore, both inequalities are strict, ...")
        .proved("8rtBjc", Pp(6), &twin_cover_num, &modular_width, StrictUpperBound(Exponential), "Theorem 3. Let $G$ be a graph. Then $\\mathrm{mw}(G) \\le \\mathrm{nd}(G)$ and $\\mathrm{mw}(G) \\le 2\\mathrm{tc}(G) + \\mathrm{tc}(G)$. Furthermore, both inequalities are strict, ...")
        .proved("ppKqXp", Pp(8), &modular_width, &shrub_depth, Incomparable, "Theorem 4. There are classes of graphs with unbounded modular-width and bounded shrub-depth and vice versa.")
        .todo_rest(&mut create);
    let belmonte2013 = source("sJ476m", "Belmonte2013", 1)
        .proved("ZHXKjC", Unknown, &carving_width, &maximum_degree, UpperBound(Linear), "Observation 1. Let $G$ be a graph. Then $cw(G) \\ge \\Delta(G)$.")
        .todo_rest(&mut create);
    let jansen2013 = source("FLOjic", "Jansen2013", 1)
        .hasse("u6oAPX", Pp(46), copyvec(vec![&vertex_cover, &maximum_leaf_num, &dist_to_complete, &dist_to_linear_forest, &cutwidth, &bandwidth, &topological_bandwidth, &feedback_vertex_set, &dist_to_chordal, &dist_to_outerplanar, &pathwidth, &odd_cycle_transversal, &treewidth, &genus, &dist_to_perfect, &chromatic_number]))
        .defined("PV6tGG", Unknown, &topological_bandwidth, "The \\emph{topological bandwidth} of a graph $G$ is the minimum [bandwidth](../aP5a38) over all subdivisions of $G$")
        .todo_rest(&mut create);
    let treecutwidth2015 = source("zbWWC6", "treecutwidth2015", 3)
        .todo_rest(&mut create);
    let adler2015 = source("rhj9my", "Adler2015", 2)
        .collective(Pp(1),  "Linear rank-width is equivalent to linear clique-width in the sense that any graph class has bounded linear clique-width if and only if it has bounded linear rank-width.")
            .proved("3yUfrd", &linear_rank_width, &linear_clique_width, UpperBound(Exists))
            .proved("2dN9wh", &linear_clique_width, &linear_rank_width, UpperBound(Exists))
            .done()
        .proved("dvqfqQ", Pp(3), &pathwidth, &linear_rank_width, UpperBound(Linear), "Lemma 5. Any graph $G$ satisfies $\\mathrm{lrw}(G) \\le \\mathrm{pw}(G)$.")
        .todo_rest(&mut create);
    let twin_cover_2015 = source("VQLE2i", "ganianTwinCover2015", 4)
        .defined("J1sHj8", Pp(5), &twin_cover_num, "Definition 3 A set of vertices $X \\subseteq V(G)$ is a twin-cover of $G$ if for every edge $e = ab \\in E(G)$ either 1. $a \\in X$ or $b \\in X$, or 2. $a$ and $b$ are true twins. We then say that $G$ has twin-cover $k$ if the size of a minimum twin-cover of $G$ is $k$.")
        .proved("OoSnHu", Pp(20), &twin_cover_num, &shrub_depth, UpperBound(Constant), "Let $\\mathcal H_k$ be the class of graphs of twin-cover $k$. Then $\\mathcal H_k \\subseteq \\mathcal{TM}_{2^k+k}(2)$ and a tree-model of any $G \\in \\mathcal H_k$ may be constructed in single-exponential FPT time.")
        .todo_rest(&mut create);
    let parameterized_algorithms2015 = source("Xlsyce", "ParameterizedAlgorithms2015", 9)
        .defined("96BXHn", NotApplicable, &treewidth, "Very roughly, treewidth captures how similar a graph is to a tree. There are many ways to define ``tree-likeness'' of a graph; ... it appears that the approach most useful from algorithmic and graph theoretical perspectives, is to view tree-likeness of a graph $G$ as the existence of a structural decomposition of $G$ into pieces of bounded size that are connected in a tree-like fashion. This intuitive concept is formalized via the notions of a *tree decomposition* and the *treewidth* of a graph; the latter is a quantitative measure of how good a tree decomposition we can possibly obtain.")
        .todo_rest(&mut create);
    let diestel2017 = source("r2Lwky", "Diestel2017", 7)
        .defined("hxpfbI", Pp(3), &complete, "If all the vertices of $G$ are pairwise adjacent, then $G$ is \\emph{complete}.")
        .defined("T8RJcC", Pp(5), &edgeless, "A vertex of degree $0$ is \\emph{isolated}.")
        .defined("8XlBpy", Pp(13), &forest, "An acyclic graph, one not containing any cycles, is called a \\emph{forest}.")
        .defined("P1ExcE", Pp(17), &bipartite, "Instead of `2-partite' one usually says bipartite.")
        .defined("eMZCoY", Pp(89), &planar, "When we draw a graph on a piece of paper, we naturally try to do this as transparently as possible. One obvious way to limit the mess created by all the lines is to avoid intersections. ... Graphs drawn in this way are called \\emph{plane graphs}; abstract graphs that can be drawn in this way are called \\emph{planar}.")
        .defined("6Q0kuL", Pp(115), &outerplanar, "A graph is called outerplanar if it has a drawing in which every vertex lies on the boundary of the outer face.")
        .defined("wkrz7h", Pp(135), &chordal, "... a graph is chordal (or triangulated) if each of its cycles of length at least $4$ has a chord, i.e. if it contains no induced cycles other than triangles.")
        .defined("54XChb", Pp(135), &perfect, "A graph is called perfect if every induced subgraph $H \\subseteq G$ has chromatic number $\\chi(H)=\\omega(H)$, i.e. if the trivial lower bound of $\\omega(H)$ colours always suffices to colour the vertices of $H$.")
        .defined("pMo8dB", Pp(145), &interval, "A graph $G$ is called an \\emph{interval graph} if there exists a set $\\{ I_v \\mid v \\in V(G) \\}$ of real intervals such that $I_u \\cap I_v \\ne \\emptyset$ if and only if $uv \\in E(G)$.")
        .todo_rest(&mut create);
    let froemmrich2018 = source("45xW87", "Froemmrich2018", 1)
        .todo_rest(&mut create);
    let ganian2019 = source("Scw7zm", "Ganian2019", 5)
        .proved("TUftFh", Unknown, &shrub_depth, &linear_clique_width, UpperBound(Linear), "Proposition 3.4. Let $\\mathcal G$ be a graph class and $d$ an integer. Then: ... b) If $\\mathcal G$ is of bounded shrub-depth, then $\\mathcal G$ is of bounded linear clique-width.")
        .proved("EG7vp6", Unknown, &neighborhood_diversity, &shrub_depth, UpperBound(Constant), "$\\mathcal{TM}_m(1)$ is exactly the class of graphs of neighborhood diversity at most $m$.")
        .proved("sq0brL", Unknown, &treedepth, &shrub_depth, UpperBound(Linear), "Proposition 3.2. If $G$ is of tree-depth $d$, then $G \\in \\mathcal{TM}_{2^d}(d)$. ...")
        .todo_rest(&mut create);
    let sorge2019 = source("VnTIL0", "Sorge2019", 7)
        .hasse("Im1xnN", Pp(2), copyvec(vec![&dist_to_complete, &vertex_cover, &maximum_leaf_num, &bandwidth, &maximum_degree, &bisection_bandwidth, &genus, &feedback_edge_set, &treedepth, &pathwidth, &treewidth, &dist_to_outerplanar, &feedback_vertex_set, &dist_to_linear_forest, &dist_to_cluster, &dist_to_interval, &dist_to_chordal, &dist_to_co_cluster, &dist_to_cograph, &clique_cover_num, &maximum_independent_set, &domination_num, &diameter, &average_distance, &/*"BCwUeT",*/ &dist_to_perfect, &dist_to_bipartite, &clique_width, &chordality, &boxicity, &acyclic_chromatic_number, &degeneracy, &perfect, &chromatic_number, &average_degree, &minimum_degree, &domatic_num, /*"&ZL7BOP",*/ &hindex]))
        .defined("ddviDI", Pp(3), &acyclic_chromatic_number, "The \\emph{acyclic chromatic number} of a graph $G = (V,E)$ is the smallest size of a vertex partition $P=\\{V_1,\\dots,V_\\ell\\}$ such that each $V_i$ is an independent set and for all $V_i,V_j$ the graph $G[V_i \\cup V_j]$ does not contain a cycle.")
        // .cited("KrMa3o", Pp(3), &grunbaum1973, "Introduced by Grünbaum [18]")
        // .defined("aUvKTa", Pp(3), &path_number, "The \\emph{path number} of a graph $G$ is the minimum number of paths the edges of $G$ can be partitioned into [2].")
        .defined("EedT02", Pp(3), &arboricity, "The \\emph{arboricity} of a graph $G$ is the minimum number of forests the edges of $G$ can be partitioned into.") // todo covering by trees
        // .defined("iUqFsf", Pp(3), &vertex_arboricity, "The \\emph{vertex arboricity} (or ``point arboricity'') of a graph $G$ is the minimum number of vertex subsets $V_i$ of $G$ such that $G[V_i]$ induces a forest for each $i$. ... [2]")
        .defined("77Klw8", Pp(3), &average_degree, "The \\emph{average degree} of a graph $G = (V,E)$ is $2|E|/|V|$.")
        .proved("LBLgZG", Pp(8), &arboricity, &degeneracy, UpperBound(Linear), "Lemma 4.5")
        .proved("RWlDuy", Pp(8), &degeneracy, &arboricity, UpperBound(Linear), "Lemma 4.5")
        .proved("mNvMUr", Pp(8), &maximum_degree, &acyclic_chromatic_number, UpperBound(Polynomial), "Lemma 4.6 ([15]). The acyclic chromatic number $\\chi_a$ is uppre bounded by the maximum degree $\\Delta$ (for every graph with $\\Delta > 4$). We have $\\chi_a \\le \\Delta(\\Delta-1)/2$.")
        .proved("thTXGX", Pp(8), &hindex, &acyclic_chromatic_number, UpperBound(Polynomial), "Lemma 4.7. The acyclic chromatic number $\\chi_a$ is upper bounded by the $h$-index $h$. We have $\\chi_a \\le h(h+1)/2$.")
        .proved("8X0RWp", Pp(8), &genus, &acyclic_chromatic_number, UpperBound(Linear), "Lemma 4.8 ([3]). The accylic chromatic number $\\chi_a$ is upper bounded by the genus $g$. We have $\\chi_a \\le 4g+4$.") // cites
        .proved("3z6jS1", Pp(8), &acyclic_chromatic_number, &boxicity, UpperBound(Polynomial), "Lemma 4.9. The boxicity $b$ is upper bounded by the acyclic chromatic number $\\chi_a$ (for every graph with $\\chi_a>1$). We have $b \\le \\chi_a(\\chi_a-1)$.") // cites
        .proved("WhwImQ", Pp(8), &maximum_leaf_num, &dist_to_linear_forest, UpperBound(Linear), "Lemma 4.10 ([14]). The max-leaf number $\\mathrm{ml}$ upper bounds the distance to disjoint paths $d$. We have $d \\le \\mathrm{ml}-1$.") // cites
        .proved("GCGHcz", Pp(9), &boxicity, &chordality, UpperBound(Linear), "Lemma 4.15 ([8,11]). The boxicity $b$ upper-bounds the chordality $c$. We have $c \\le b$.")
        .proved("1xFA4j", Pp(9), &dist_to_interval, &boxicity, UpperBound(Linear), "Lemma 4.16. The distance $i$ to an interval graph upper bounds the boxicity $b$. We have $b \\le i+1$.")
        .proved("16XW6a", Pp(9), &dist_to_chordal, &chordality, UpperBound(Linear), "(ed: apparently goes as the lemma for ddist to interval and boxicity) Lemma 4.16. The distance $i$ to an interval graph upper bounds the boxicity $b$. We have $b \\le i+1$.")
        .proved("OV1KKK", Pp(9), &dist_to_cograph, &clique_width, UpperBound(Exponential), "Lemma 4.17. The distance $c$ to a cograph upper bounds the cliquewidth $q$. We have $q \\le 2^{3+c}-1$.")
        .proved("rVEmFt", Pp(9), &acyclic_chromatic_number, &degeneracy, UpperBound(Polynomial), "Lemma 4.18. The acyclic chromatic number $a$ upper bounds the degeneracy $d$. We have $d \\le 2 \\binom a2 - 1$")
        .proved("mwNELs", Pp(10), &feedback_edge_set, &genus, UpperBound(Linear), "Lemma 4.19. The feedback edge set number $f$ upper bounds the genus $g$. We have $g \\le f$.")
        .proved("50WEZP", Pp(10), &feedback_vertex_set, &dist_to_chordal, UpperBound(Linear), "Lemma 4.20. The feedback edge set number $f$ upper bounds the distance to a chordal graph $c$. We have $c \\le f$.")
        .proved("ghgPz2", Pp(10), &maximum_leaf_num, &bandwidth, UpperBound(Linear), "Lemma 4.25. The max leaf number $\\mathrm{ml}$ strictly upper bounds the bandwidth $\\mathrm{bw}$.")
        .proved("h8nG9p", Pp(11), &clique_cover_num, &maximum_independent_set, UpperBound(Linear), "Lemma 4.26. The minimum clique cover number $c$ strictly upper bounds the independence number $\\alpha$.")
        .proved("q3qJkr", Pp(11), &treedepth, &pathwidth, UpperBound(Linear), "Lemma 4.27. The treedepth $t$ strictly upper bounds the pathwidth $p$. We have $p \\le t$.")
        .todo_rest(&mut create); // page 10
    let mimwidth2020 = source("BIQh3r", "mimwidth2020", 1)
        .todo_rest(&mut create);
    let schroder_parameter_list = copyvec(vec![&dist_to_complete, &vertex_cover, &maximum_leaf_num, &clique_cover_num, &dist_to_co_cluster, &dist_to_cluster, &dist_to_linear_forest, &feedback_edge_set, &bandwidth, &maximum_independent_set, &dist_to_cograph, &dist_to_interval, &feedback_vertex_set, &treedepth, &maximum_degree, &domination_num, &dist_to_outerplanar, &pathwidth, &genus, &hindex, &diameter, &dist_to_planar, &treewidth, &average_distance, &dist_to_chordal, &acyclic_chromatic_number, &/*"BCwUeT",*/ &clique_width, &degeneracy, &dist_to_bipartite, &boxicity, &average_degree, &dist_to_perfect, &perfect, &minimum_degree, &chromatic_number, &chordality, &domatic_num, &/*"ZL7BOP",*/ &bisection_bandwidth]);
    let schroder_thesis = source("DYGiYb", "SchroderThesis", 7)
        // .cited("pJxHVS", Unknown, sorge2019, "Based on the work by [Sa19] as well as [Fr8], we investigate unknown connections between graph parameters to continue the work on the graph parameter hierarchy")
        // .cited("bybFgo", Unknown, froemmrich2018, "Based on the work by [Sa19] as well as [Fr8], we investigate unknown connections between graph parameters to continue the work on the graph parameter hierarchy")
        .hasse("DfHlFn", Pp(7), schroder_parameter_list.clone())
        .table("ONqedT", Pp(8), schroder_parameter_list.clone())
        .proved("R9eI61", Pp(11), &treedepth, &diameter, UpperBound(Exponential), "Proposition 3.1")
        .proved("dohKmq", Pp(12), &dist_to_linear_forest, &hindex, UpperBound(Linear), "Proposition 3.2")
        .proved("WY0T4I", Pp(13), &dist_to_cluster, &dist_to_co_cluster, Exclusion, "Proposition 3.3")
        .proved("9VEBot", Pp(14), &dist_to_co_cluster, &boxicity, Exclusion, "Proposition 3.4")
        .proved("s1Sx0j", Pp(15), &vertex_cover, &domination_num, Exclusion, "Proposition 3.5")
        .proved("Ysx42j", Pp(15), &clique_cover_num, &dist_to_perfect, Exclusion, "Proposition 3.6")
        .proved("VMDVbW", Pp(16), &dist_to_complete, &maximum_clique, Exclusion, "Proposition 3.7")
        .proved("3WQZ4g", Pp(16), &dist_to_complete, &domatic_num, Exclusion, "Proposition 3.7")
        // .proved("rCDCaC", Pp(16), &dist_to_complete, &dist_to_disconnected, Exclusion, "Proposition 3.8")
        .proved("AbzdZf", Pp(16), &clique_cover_num, &clique_width, Exclusion, "Proposition 3.9")
        .proved("W2GU1L", Pp(19), &clique_cover_num, &chordality, Exclusion, "Proposition 3.11")
        .proved("3Tunx6", Pp(19), &dist_to_perfect, &chordality, Exclusion, "Proposition 3.11")
        .proved("OKKYpY", Pp(20), &dist_to_co_cluster, &dist_to_chordal, Exclusion, "Proposition 3.12")
        .proved("A18O6S", Pp(20), &dist_to_bipartite, &dist_to_chordal, Exclusion, "Proposition 3.12")
        // .proved("TiiRaX", Pp(20), &dist_to_co_cluster, &dist_to_disconnected, Exclusion, "Proposition 3.12")
        // .proved("uvXQGw", Pp(20), &dist_to_bipartite, &dist_to_disconnected, Exclusion, "Proposition 3.12")
        .proved("5jYT5W", Pp(20), &dist_to_co_cluster, &domatic_num, Exclusion, "Proposition 3.12")
        .proved("U6hu68", Pp(20), &dist_to_bipartite, &domatic_num, Exclusion, "Proposition 3.12")
        .proved("hu6cvG", Pp(21), &bandwidth, &dist_to_planar, Exclusion, "Proposition 3.13")
        .proved("0gaHWl", Pp(21), &treedepth, &dist_to_planar, Exclusion, "Proposition 3.13")
        // .proved("rVwCp7", Pp(21), &maximum_leaf_num, &girth, Exclusion, "Proposition 3.14")
        .proved("LEiVG6", Pp(23), &feedback_edge_set, &pathwidth, Exclusion, "Proposition 3.16")
        .proved("G5p3ht", Pp(23), &genus, &clique_width, Exclusion, "Proposition 3.17")
        .proved("Ydrtul", Pp(23), &dist_to_planar, &clique_width, Exclusion, "Proposition 3.17")
        .proved("6n7Mlc", Pp(24), &vertex_cover, &genus, Exclusion, "Proposition 3.18")
        .proved("AB8NTk", Pp(24), &vertex_cover, &maximum_degree, Exclusion, "Proposition 3.19")
        .proved("hUe750", Pp(24), &vertex_cover, &bisection_bandwidth, Exclusion, "Proposition 3.20")
        .proved("s1H8GJ", Pp(25), &feedback_edge_set, &dist_to_interval, Exclusion, "Proposition 3.21")
        .proved("PPqzfo", Pp(25), &treedepth, &hindex, Exclusion, "Proposition 3.22")
        .proved("2bGE0b", Pp(25), &feedback_edge_set, &hindex, Exclusion, "Proposition 3.22")
        .proved("xNaCQ3", Pp(26), &dist_to_outerplanar, &dist_to_perfect, Exclusion, "Proposition 3.23")
        .proved("MkoHxT", Pp(26), &bandwidth, &dist_to_perfect, Exclusion, "Proposition 3.24")
        .proved("hpzbh8", Pp(26), &genus, &dist_to_perfect, Exclusion, "Proposition 3.24")
        .proved("nrJVzy", Pp(26), &treedepth, &dist_to_perfect, Exclusion, "Proposition 3.24")
        .proved("8lk9o6", Pp(27), &dist_to_chordal, &boxicity, Exclusion, "Proposition 3.25")
        .proved("QvBfYR", Pp(28), &maximum_degree, &clique_width, Exclusion, "Proposition 3.26")
        .proved("mTfYrt", Pp(28), &maximum_degree, &bisection_bandwidth, Exclusion, "Proposition 3.26")
        .proved("NZGZCw", Pp(28), &dist_to_bipartite, &clique_width, Exclusion, "Proposition 3.26")
        .proved("uVhLEQ", Pp(28), &dist_to_bipartite, &bisection_bandwidth, Exclusion, "Proposition 3.26")
        .proved("Hm9nY3", Pp(30), &bandwidth, &genus, Exclusion, "Proposition 3.27")
        .proved("6olZjM", Pp(30), &bisection_bandwidth, &domatic_num, Exclusion, "Proposition 3.28")
        .proved("W6h5ZK", Pp(30), &feedback_edge_set, &bisection_bandwidth, Exclusion, "Proposition 3.29")
        // .proved("KYpom9", Pp(31), &domatic_num, &dist_to_disconnected, Exclusion, "Proposition 3.30")
        .proved("2nmdxu", Pp(33), &bisection_bandwidth, &chordality, Exclusion, "Proposition 3.31")
        .proved("bxep24", Pp(33), &bisection_bandwidth, &clique_width, Exclusion, "Proposition 3.32")
        .proved("bhJsnM", Pp(33), &bisection_bandwidth, &maximum_clique, Exclusion, "Proposition 3.33")
        .proved("SynFuK", Pp(33), &genus, &dist_to_planar, Exclusion, "Proposition 3.34")
        .proved("hoJGBX", Pp(35), &average_degree, &maximum_clique, Exclusion, "Proposition 3.35")
        .proved("JRqAlT", Pp(36), &average_degree, &chordality, Exclusion, "Proposition 3.36")
        .todo_rest(&mut create);
    // let reduced_edgeless = create.reduced("reduced edgeless", &edgeless, 0);
    let excl_ladders2021 = source("EdPIVj", "exclLadders2021", 6)
        .todo_rest(&mut create);
    let twin_width_1_2021 = source("nyaOye", "twinWidthI2021", 6)
        .defined("s5Ktq7", Pp(2), &twin_width, "... we consider a sequence of graphs $G_n,G_{n-1},\\dots,G_2,G_1$, where $G_n$ is the original graph $G$, $G_1$ is the one-vertex graph, $G_i$ has $i$ vertices, and $G_{i-1}$ is obtained from $G_i$ by performing a single contraction of two (non-necessarily adjacent) vertices. For every vertex $u \\in V(G_i)$, let us denote by $u(G)$ the vertices of $G$ which have been contracted to $u$ along the sequence $G_n,\\dots,G_i$. A pair of disjoint sets of vertices is \\emph{homogeneous} if, between these sets, there are either all possible edges or no edge at all. The red edges ... consist of all pairs $uv$ of vertices of $G_i$ such that $u(G)$ and $v(G)$ are not homogeneous in $G$. If the red degree of every $G_i$ is at most $d$, then $G_n,G_{n-1},\\dots,G_2,G_1$ is called a \\emph{sequence of $d$-contractions}, or \\emph{$d$-sequence}. The twin-width of $G$ is the minimum $d$ for which there exists a sequence of $d$-contractions.")
        .proved("08lETp", Pp(14), &boolean_width, &twin_width, UpperBound(Exponential), "Theorem 3: Every graph with boolean-width $k$ has twin-width at most $2^{k+1}-1$.")
        .proved("0RiLv2", Pp(15), &grid, &twin_width, UpperBound(Constant), "Theorem 4.3. For every positive integers $d$ and $n$, the $d$-dimensional $n$-grid has twin-width at most $3d$.")
        // .proved("7p2TWN", Unknown, &cograph, &reduced_edgeless, Equal, "") // todo
        .todo_rest(&mut create);
    // let reduced_star = &create.reduced(&star, 0);
    // let twin_width_beyond_2022 = source("3B7Kvt", "twinWidthBeyond2022")
    // .proved("AwGkfi", Pp(3), &all_graphs, &reduced_star, UpperBound(Constant), "Every graph has a reduction sequence in which every red graph is a star ...")
    // // .defined("M6H2kI", , &reduced_bandwidth, "")
    // .todo_rest(&mut create);
    let edgecutwidth2022 = source("xhGnnq", "edgecutwidth2022", 4)
        .todo_rest(&mut create);
    let tran2022 = source("uXViPE", "Tran2022", 7)
        // .redefined("J1sHj8", Pp(14), &twin_cover_num, "An edge $\\{v,w\\}$ is a twin edge if vertices $v$ and $w$ have the same neighborhood excluding each other. The twin cover number $tcn(G)$ of a graph $G$ is the size of a smallest set $V' \\subseteq V(G)$ of vertices such that every edge in $E(G)$ is either a twin edge or incident to a vertex in $V'$")
        .redefined("MlTT7n", Pp(14), &edge_clique_cover, "The edge clique cover number $eccn(G)$ of a graph $G$ is the minimum number of complete subgraphs required such that each edge is contained in at least one of them.")
        .redefined("iAkCJ3", Pp(14), &neighborhood_diversity, "The neighborhood diversity $nd(G)$ of a graph $G$ is the smallest number $k$ such that there is a $k$-partition $(V_1,\\dots,V_k)$ of $G$, where each subset $V_i$, $i \\in [k]$ is a module and is either a complete set or an independent set.")
        .redefined("i3su9L", Pp(14), &modular_width, "The modular-width $mw(G)$ of a graph $G$ is the smallest number $h$ such that a $k$-partition $(V_1,\\dots,V_k)$ of $G$ exists, where $k \\le h$ and each subset $V_i$, $i \\in [k]$ is a module and either contains a single vertex or for which the modular-subgraph $G[V_i]$ has a modular-width of $h$.")
        .redefined("Fhp3Dr", Pp(14), &c_closure, "The c-closure $\\mathrm{cc}(G)$ of a graph $G$ is the smallest number $c$ such that any pair of vertices $v,w \\in V(G)$ with $|N_G(v) \\cap N_G(w)| \\ge c$ is adjacent. ...")
        .redefined("eGC0vH", Pp(16), &boxicity, "The boxicity of a graph $G$ is the minimum amount of interval graphs required, such that their intersection (ed: fixed typo) results in $G$.")
        .redefined("gZtk6B", Pp(16), &chordality, "The chordality of a graph $G$ is the minimum amount of chordal graphs required, such that their intersection (ed: fixed typo) results in $G$.")
        .proved("L0BALz", Pp(18), &vertex_cover, &twin_cover_num, UpperBound(Linear), by_definition)
        .proved("kcsO0r", Pp(18), &complete, &vertex_cover, Exclusion, "Note that a clique of size $n$ has ... a vertex cover number of $n-1$")
        .proved("RUjcaV", Pp(18), &complete, &twin_cover_num, UpperBound(Constant), "Note that a clique of size $n$ has a twin cover number of 0 ...")
        .proved("am05hY", Pp(18), &twin_cover_num, &dist_to_cluster, UpperBound(Linear), "... graph $H$ with a twin cover of size $k$ has a distance to cluster of at most $k$.")
        .proved("tNZjb1", Pp(18), &dist_to_cluster, &twin_cover_num, Exclusion, "We show that twin cover number is not upper bounded by distance to cluster.")
        .proved("ikkDSL", Pp(18), &twin_cover_num, &dist_to_complete, Incomparable, "Observation 3.3. Twin Cover Number is incomparable to Distance to Clique.")
        .proved("39CEgf", Pp(19), &twin_cover_num, &maximum_clique, Incomparable, "Observation 3.4. Twin Cover Number is incomparable to Maximum Clique, Domatic Number and Distance to Disconnected.")
        .proved("mttPgQ", Pp(19), &twin_cover_num, &domatic_num, Incomparable, "Observation 3.4. Twin Cover Number is incomparable to Maximum Clique, Domatic Number and Distance to Disconnected.")
        .proved("DaQuOK", Pp(19), &twin_cover_num, &edge_connectivity, Incomparable, "Observation 3.4. Twin Cover Number is incomparable to Maximum Clique, Domatic Number and Distance to Disconnected.")
        .proved("o5JorW", Pp(21), &twin_cover_num, &dist_to_co_cluster, Incomparable, "Proposition 3.5. Twin Cover Number is incomparable to Distance to Co-Cluster.")
        .proved("TddK24", Pp(22), &edge_clique_cover, &neighborhood_diversity, StrictUpperBound(Exponential), "Theorem 4.1. Edge Clique Cover Number strictly upper bounds Neighborhood Diversity.")
        .proved("O9TmJs", Pp(23), &dist_to_complete, &edge_clique_cover, StrictUpperBound(Polynomial), "Proposition 4.2. Disatnce to Clique strictly upper bounds Edge Clique Cover Number.")
        .proved("YGEkmM", Pp(23), &vertex_cover, &neighborhood_diversity, StrictUpperBound(Exponential), "Proposition 4.3. Vertex Cover Number strictly upper bounds Neighborhood Diversity.")
        .proved("gKXcun", Pp(24), &path, &modular_width, Exclusion, "The Modular-width of a path $P$ with length $n > 3$ is $n$.")
        // .proved("wFtd4d", Pp(24), &modular_width, &complement(modular_width), Equal, "Given any graph $G$, $\\mathrm{mw}(G) = \\mathrm{mw}(\\bar G)$.")
        .proved("FeMtBe", Pp(25), &modular_width, &clique_width, StrictUpperBound(Linear), "Proposition 4.6. Modular-width strictly upper bounds Clique-width.")
        .proved("rNb4Qe", Pp(25), &modular_width, &diameter, StrictUpperBound(Linear), "Theorem 4.7. Modular-width strictly upper bounds Max Diameter of Components.")
        .proved("SmUw6p", Pp(26), &path, &neighborhood_diversity, Exclusion, "The Neighborhood Diversity of a Path $P$ with length $n > 3$ is $n$.")
        .proved("cEEX99", Pp(26), &neighborhood_diversity, &boxicity, StrictUpperBound(Polynomial), "Note that given a path of length $n > 3$. The boxicity is 1 while ... neighborhood diversity is $n$. ... a graph ... with neighborhood diversity of $k$, its boxicity is at most $k+k^2$. ")
        .proved("78NYVs", Pp(28), &modular_width, &dist_to_cluster, Incomparable, "Proposition 4.10. Modular-width is incomparable to Distance to Cluster.")
        .proved("o64vSn", Pp(28), &modular_width, &dist_to_co_cluster, Incomparable, "Proposition 4.11. Modular-width is incomparable to Distance to Co-Cluster.")
        .proved("szCcet", Pp(28), &neighborhood_diversity, &twin_cover_num, Incomparable, "Proposition 4.12. Modular-width is incomparable to Distance to Twin Cover Number.")
        .proved("x3RM6x", Pp(29), &edge_clique_cover, &vertex_cover, Incomparable, "Proposition 4.13. Edge Clique Cover Number is incomparable to Vertex Cover Number.")
        .proved("4yZbJp", Pp(29), &edge_clique_cover, &domination_num, Incomparable, "Proposition 4.14. Edge Clique Cover Number is incomparable to Domination Number.")
        .proved("4QIldV", Pp(29), &edge_clique_cover, &dist_to_perfect, Incomparable, "Proposition 4.15. Edge Clique Cover Number is incomparable to Distance to Perfect.")
        .proved("Asq2IA", Pp(30), &modular_width, &chordality, Incomparable, "Theorem 4.16. Modular-width is incomparable to Chordality.")
        .proved("QLeIsq", Pp(32), &maximum_degree, &c_closure, StrictUpperBound(Linear), "Proposition 5.1. Maximum Degree strictly upper bounds $c$-Closure.")
        .proved("jNHLqN", Pp(32), &feedback_edge_set, &c_closure, StrictUpperBound(Linear), "Theorem 5.2. Feedback Edge Number strictly upper bounds $c$-Closure.")
        .proved("7vteay", Pp(34), &c_closure, &vertex_cover, Incomparable, "Proposition 5.3. $c$-Closure is incomparable to Vertex Cover Number.")
        .proved("Yvkf8u", Pp(34), &c_closure, &dist_to_complete, Incomparable, "Proposition 5.4. $c$-Closure is incomparable to Distance to Clique.")
        .proved("49lTWH", Pp(34), &c_closure, &bisection_bandwidth, Incomparable, "Proposition 5.5. $c$-Closure is incomparable to Bisection Width.") // todo check bisection bandwidth = bisection width
        .proved("LCe3uI", Pp(34), &c_closure, &genus, Incomparable, "Proposition 5.6. $c$-Closure is incomparable to Genus.")
        .proved("XjB6Cy", Pp(34), &c_closure, &vertex_connectivity, Incomparable, "Observation 5.7. $c$-Closure is incomparable to Distance to Disconnected, Domatic Number and Maximum Clique.")
        .proved("XaXtqm", Pp(34), &c_closure, &domatic_num, Incomparable, "Observation 5.7. $c$-Closure is incomparable to Distance to Disconnected, Domatic Number and Maximum Clique.")
        .proved("lXXajO", Pp(34), &c_closure, &maximum_clique, Incomparable, "Observation 5.7. $c$-Closure is incomparable to Distance to Disconnected, Domatic Number and Maximum Clique.")
        .proved("OinVkl", Pp(35), &c_closure, &boxicity, Incomparable, "Proposition 5.8. $c$-Closure is incomparable to Boxicity.")
        // .proved("wFtd4d", Pp(36), &twin_width, &complement(twin_width), Equal, "")
        .proved("PK4H2R", Pp(36), &clique_width, &twin_width, StrictUpperBound(Exponential), "Proposition 6.2. Clique-width strictly upper bounds Twin-width.")
        .proved("2F5Zr8", Pp(37), &genus, &twin_width, StrictUpperBound(Linear), "Proposition 6.3. Genus strictly upper bounds Twin-width.")
        .proved("xsiECz", Pp(37), &dist_to_planar, &twin_width, StrictUpperBound(Exponential), "Theorem 6.4. Distance to Planar strictly upper bounds Twin-width.") // cite
        .proved("aa0bCE", Pp(38), &twin_width, &dist_to_interval, Incomparable, "Observation 6.5. Twin-width is incomparable to Distance to Interval.")
        .proved("YqjLQa", Pp(38), &twin_width, &dist_to_bipartite, Incomparable, "Proposition 6.6. Twin-width is incomparable to Distance to Bipartite.")
        .proved("n3WK3H", Pp(40), &twin_width, &clique_cover_num, Incomparable, "Proposition 6.7. Twin-width is incomparable to Clique Cover Number.")
        .proved("52S4T0", Pp(40), &twin_width, &maximum_degree, Incomparable, "Proposition 6.8. Twin-width is incomparable to Maximum Degree.") // cites
        .proved("1lfJWG", Pp(40), &twin_width, &bisection_bandwidth, Incomparable, "Observation 6.9. Twin-width is incomparable to Bisection Width.") // check with bisection width
        .proved("UN2Lbu", Pp(42), &degeneracy, &boxicity, Incomparable, "Proposition 7.1. Degeneracy is incomparable to Boxicity.")
        .done(&mut create);
    let torunczyk2023 = source("KpkMZB", "Torunczyk2023", 7)
        .defined("gxeVOT", Unknown, &r_flip_width, "The radius-$r$ flip-width of a graph $G$, denoted $fw_r(G)$, is the smallest number $k \\in \\mathbb{N}$ such that the cops have a winning strategy in the flipper game of radius $r$ and width $k$ on $G$")
        .proved("9DTyeJ", Unknown, &inf_flip_width, &rank_width, UpperBound(Linear), "For every graph $G$, we have $\\mathrm{rankwidth}(G) \\le 3 \\mathrm{fw}_\\infty(G)+1$ ...")
        .proved("zYQZyB", Unknown, &rank_width, &inf_flip_width, UpperBound(Exponential), "For every graph $G$, we have ... $3 \\mathrm{fw}_\\infty(G)+1 \\le O(2^{\\mathrm{rankwidth(G)}})$.")
        .proved("OdbuZP", Unknown, &twin_width, &r_flip_width, UpperBound(Exponential), "Theorem 7.1. Fix $r \\in \\mathbb N$. For every graph $G$ of twin-width $d$ we have: $\\mathrm{fw}_r(G) \\le 2^d \\cdot d^{O(r)}$.")
        .proved("gvSCeQ", Unknown, &inf_flip_width, &r_flip_width, UpperBound(Linear), by_definition)
        .todo_rest(&mut create);
    let slim_tcw2024 = source("7g1aTu", "SlimTCW2024", 5)
        .hasse("hROdkf", Pp(2716), copyvec(vec![&tree_cut_width, &slim_tree_cut_width, &degree_treewidth, &feedback_edge_set])) // also tcw0, tcw1, tcw2, edge-cut width
        .collective(Pp(2716), "Figure 1")
            .noted_relation("GreUNa", &slim_tree_cut_width, &tree_cut_width, UpperBound(Exists), SrcTodo)
            .noted_relation("9TXlI9", &edge_cut_width, &slim_tree_cut_width, UpperBound(Exists), SrcTodo)
            .noted_relation("Tj5lLU", &feedback_edge_set, &edge_cut_width, UpperBound(Exists), SrcTodo)
            .noted_relation("hZ7bvg", &degree_treewidth, &slim_tree_cut_width, UpperBound(Exists), SrcTodo)
            .done()
        .todo_rest(&mut create);
    let merge_width_2024 = source("9exguJ", "MergeWidth2024", 7)
        .defined("CJMyrW", Pp(4), &merge_width, "Merge-width. ...")
        .proved("2WRKzg", Pp(6), &twin_width, &merge_width, UpperBound(Exists), "Theorem 1.4. Graph classes of bounded twin-width have bounded merge-width.")
        .proved("Ez6JNM", Pp(7), &bounded_expansion, &merge_width, UpperBound(Exists), "Theorem 1.6. Graph classes of bounded expansion have bounded merge-width.")
        .proved("ZMcZ2n", Pp(7), &merge_width, &flip_width, UpperBound(Exists), "Theorem 1.7. Every class of bounded merge-width has bounded flip-width.")
        .proved("FIv3Jh", Pp(7), &tmp_ws_mw, &bounded_expansion, Equivalent(Exists, Exists), "Corollary 1.8. A graph class has bounded expansion if and only if it has bounded merge-width, and is weakly sparse (exclydes some biclique $K_{t,t}$ as a subgraph).")
        // Corollary 1.9. A class of ordered graphs has bounded twin-width if and only if it has bounded merge-width.
        // Corollary 1.10. A graph class has bounded twin-width if and only if it is obtained from some class of ordered graphs of bounded merge-width by forgetting the order.
        // Theorem 1.11. [par] assuming construction sequence FO model checking is FPT wrt width and formula quantifier rank
        // Corollary 1.13. Suppose C and D are graph classes, and that C transduces D. If C has bounded merge-width, then so does D.
        .todo_rest(&mut create);
    source("47Xy7Z", "munaroAlgorithmicApplicationsSimwidth2023", 1)
        .todo_rest(&mut create);
    // source("YFKpDU", "brettell2023comparingwidthparametersgraph")
    // .collective(Pp(3), "Theorem 1. For every $s \\ge 3$ and $t \\ge 2$, when restricted to $(K_s, K_{t,t})$-free graphs, sim-width, mim-width, clique-width, treewidth and tree-independence number are equivalent, whereas twin-width is more powerful than any of these parameters.")
    // .done()
    // .todo_rest(&mut create);
    let twwsurfaces2024 = source("lgJ2j7", "twwsurfaces2024", 2)
        .proved("3iR4qs", Pp(18), &genus, &twin_width, UpperBound(Linear), "The twin-width of every graph $G$ of Euler genus $g \\ge 1$ is at most ... $18 \\sqrt{47g}+O(1)$.") // todo sqrt
        .todo_rest(&mut create);
    let itp_introduced2024 = source("oBcMqr", "iteratedTypePartitions2024", 3)
        .defined("q3zZx7", Pp(3), &iterated_type_partitions, "two nodes have the same type iff $N(v) \\setminus \\{u\\} = N(u) \\setminus \\{v\\}$ ... [ed. paraphrased] let $\\mathcal V = \\{V_1,\\dots,V_t\\}$ be a partition of graph vertices such that each $V_i$ is a clique or an independent set and $t$ is minimized ... we can see each element of $\\mathcal V$ as a \\emph{metavertex} of a new graph $H$, called \\emph{type graph} of $G$ ... We say that $G$ is a \\emph{prime graph} if it matches its type graph ... let $H^{(0)}=G$ and $H^{(i)}$ denote the type graph of $H^{(i-1)}$, for $i \\ge 1$. Let $d$ be the smallest integer such that $H^{(d)}$ is a \\emph{prime graph}. The \\emph{iterated type partition} number of $G$, denoted by $\\mathrm{itp}(G)$, is the number of nodes of $H^{(d)}$.")
        .proved("LUQLaI", Pp(3), &neighborhood_diversity, &iterated_type_partitions, StrictUpperBound(Linear), "... $itp(G) \\le nd(G)$. Actually $itp(G)$ can be arbitrarily smaller than $nd(G)$.")
        .proved("Lh05uc", Pp(3), &iterated_type_partitions, &modular_width, UpperBound(Linear), by_definition)
        .todo_rest(&mut create);
    let treebandwidth2025 = source("EImlRb", "treebandwidth2025", 4)
        .hasse("l1oyAq", Pp(36), copyvec(vec![&size, &vertex_cover, &bounded_components, &treedepth, &pathwidth, &feedback_vertex_set, &treewidth, &maximum_degree, &cutwidth, &domino_treewidth, &treebandwidth, &tree_partition_width, &slim_tree_cut_width, &edge_treewidth, &tree_cut_width, &overlap_treewidth])) // also: fan number, dipole number, biconnected maximum degree
        .defined("u2JciR", Pp(1), &treebandwidth, "A \\emph{tree-layout} of $G=(V,E)$ is a rooted tree $T$ whose nodes are the vertices of $V$, and such that, for every edge $xy \\in E$, $x$ is an ancestor of $y$ or vice-versa. The bandwidth of $T$ is then the maximum distance in $T$ between pairs of neighbors in $G$. We call \\emph{treebandwidth} of $G$, the minimum bandwidth over tree-layouts of $G$, and denote it by ${\\rm tbw}(G)$.")
        // Pp(5), relation to graphclasses
        .proved("tInJY1", Pp(7), &tree_partition_width, &treebandwidth, StrictUpperBound(Linear), "By rooting the tree-partition arbitrarily and replacing each bag by an arbitrary linear ordering of its vertices one derives ${\\rm tbw}(G) \\le 2 \\cdot {\\rm tpw}(G)$. However, some graphs of treebandwidth 2 have unbounded tree-partition-width: ...")
        // Pp(2), "Theorem 2. Graphs of bounded treebandwidth are exactly graphs which exclude large walls and large fans as topological minors."
        // Pp(29), "Theorem 38. There is an algorithm that given a graph G and an integer k, in time kO(k2) ·n either computes a tree-layout of bandwidth at most g(k), or determines that tbw(G) > k"
        // Pp(38), Table 1, list of obstructions
        .collective(Pp(36), "... discovered independently by Fanica Gavril and Mihalis Yannakakis. Theorem 50. In a graph $G$, we have $\\mu(G) \\le {\\rm vc}(G) \\le 2\\mu(G)$, where $\\mu(G)$ is the maximum matching number of $G$.")
            .proved("kmCM7X", &vertex_cover, &maximum_matching, Exactly(Linear)) // , "Every edge of the matching needs to be covered by at least one vertex. Path shows that this relation is not better than linear.")
            .proved("cGN8qC", &maximum_matching, &vertex_cover, UpperBound(Linear)) //, "A set of all vertices taking part in a maximum matching creates a vertex cover, hence $vc(G) \\le 2 \\cdot mm(G)$.")
        .done()
        .proved("zZ1ZCa", Pp(37), &degree_treewidth, &domino_treewidth, UpperBound(Linear), "Theorem 53. ... $\\max({\\rm tw}(G),(\\Delta(G)-1)/2) \\le {\\rm dtw}(G)$ ... [this result is claimed to be in [[gcMYuX]], didn't see it there]") // noted
        .proved("9C3Gwi", Pp(37), &treespan, &degree_treewidth, UpperBound(Linear), "Claim 55. $\\max({\\rm tw}(G),\\Delta(G)/2) \\le {\\rm ts}(G)$")
        .proved("JIAD2R", Pp(38), &domino_treewidth, &treespan, UpperBound(Linear), "Claim 56. ${\\rm ts}(G) \\le 2 \\cdot {\\rm dtw}(G)$")
        // Pp(39), "Edge-treewidth is polynomially tied to the largest of the treewidth and the biconnected maximum degree."
        .proved("vgpxF1", Pp(38), &treebandwidth, &treewidth, UpperBound(Exists), "[implied through obstructions]")
        .done(&mut create);
    let str_treedepth = parameter("HbBTjI", "structurally treedepth", 3).tag(&tag_structural).done(&mut create);
    let str_pathwidth = parameter("ei5NQ3", "structurally pathwidth", 3).tag(&tag_structural).done(&mut create);
    let str_treewidth = parameter("WQQqcJ", "structurally treewidth", 3).tag(&tag_structural).done(&mut create);
    let str_excluded_minor= property("1lCekH", "structurally excluded minor", Has, 3).tag(&tag_structural).done(&mut create);
    let str_sparse_twin_width = parameter("qXI6la", "structurally sparse twin-width", 3).tag(&tag_structural).done(&mut create);
    let str_bounded_expansion = property("cjLkrr", "structurally bounded expansion", Has, 3).tag(&tag_structural).done(&mut create);
    let str_nowhere_dense = property("Qj1Dfw", "structurally nowhere dense", Is, 3).tag(&tag_structural).done(&mut create);
    let mon_shrubdepth = create.intersection("UuaLsM", &monadically_stable, &shrub_depth, "mon stable and shrubdepth", 3).done(&mut create);
    let mon_linear_clique_width = create.intersection("xn4gFR", &monadically_stable, &linear_clique_width, "mon stable and linear clique-width", 3).done(&mut create);
    let mon_clique_width = create.intersection("KF02zL", &monadically_stable, &clique_width, "mon stable and clique-width", 3).done(&mut create);
    let mon_twin_width = create.intersection("YRCrDm", &monadically_stable, &twin_width, "mon stable and twin-width", 3).done(&mut create);
    let mon_flip_width = create.intersection("UA5Ya8", &monadically_stable, &flip_width, "mon stable and flip-width", 3).done(&mut create);
    let pilipczuklens2025 = source("ROetOx", "pilipczuk2025graphclasseslenslogic", 7)
        .hasse("dBI3GJ", Pp(3), copyvec(vec![&treedepth, &pathwidth, &treewidth, &excluded_minor, &sparse_twin_width, &bounded_expansion, &nowhere_dense, &str_treedepth, &str_pathwidth, &str_treewidth, &str_sparse_twin_width, &str_bounded_expansion, &str_nowhere_dense, &mon_shrubdepth, &mon_linear_clique_width, &mon_clique_width, &mon_twin_width, &mon_flip_width, &monadically_stable, &shrub_depth, &linear_clique_width, &clique_width, &twin_width, &flip_width, &monadically_dependent]))
        .collective(Pp(3), "Figure 1")
            .noted_relation("Ao4G7v", &treedepth, &pathwidth, UpperBound(Exists), SrcTodo)
            .noted_relation("Tp3U1z", &pathwidth, &treewidth, UpperBound(Exists), SrcTodo)
            .noted_relation("fU9cEU", &treewidth, &excluded_minor, UpperBound(Exists), SrcTodo)
            .noted_relation("7wE0yR", &excluded_minor, &sparse_twin_width, UpperBound(Exists), SrcTodo)
            .noted_relation("7Q501R", &sparse_twin_width, &bounded_expansion, UpperBound(Exists), SrcTodo)
            .noted_relation("fyRYB7", &bounded_expansion, &nowhere_dense, UpperBound(Exists), SrcTodo)
            .noted_relation("PXRSVO", &shrub_depth, &linear_clique_width, UpperBound(Exists), SrcTodo)
            .noted_relation("gif7x1", &linear_clique_width, &clique_width, UpperBound(Exists), SrcTodo)
            .noted_relation("Mlvc3P", &clique_width, &twin_width, UpperBound(Exists), SrcTodo)
            .noted_relation("24EfrR", &twin_width, &flip_width, UpperBound(Exists), SrcTodo)
            .noted_relation("smZ1kW", &flip_width, &monadically_dependent, UpperBound(Exists), SrcTodo)
            .noted_relation("uGjLvP", &treedepth, &str_treedepth, UpperBound(Exists), SrcTodo)
            .noted_relation("9OwM3c", &pathwidth, &str_pathwidth, UpperBound(Exists), SrcTodo)
            .noted_relation("HYbckV", &treewidth, &str_treewidth, UpperBound(Exists), SrcTodo)
            .noted_relation("BakYIM", &excluded_minor, &str_excluded_minor, UpperBound(Exists), SrcTodo)
            .noted_relation("PxLo45", &sparse_twin_width, &str_sparse_twin_width, UpperBound(Linear), SrcTodo)
            .noted_relation("3pGIso", &bounded_expansion, &str_bounded_expansion, UpperBound(Exists), SrcTodo)
            .noted_relation("i6wmYD", &nowhere_dense, &str_nowhere_dense, UpperBound(Exists), SrcTodo)
            .noted_relation("PF8rH0", &str_treedepth, &str_pathwidth, UpperBound(Exists), SrcTodo)
            .noted_relation("o7wIYn", &str_pathwidth, &str_treewidth, UpperBound(Exists), SrcTodo)
            .noted_relation("wAPB3U", &str_treewidth, &str_excluded_minor, UpperBound(Exists), SrcTodo)
            .noted_relation("2CQtlT", &str_excluded_minor, &str_sparse_twin_width, UpperBound(Exists), SrcTodo)
            .noted_relation("dPFWZa", &str_sparse_twin_width, &str_bounded_expansion, UpperBound(Exists), SrcTodo)
            .noted_relation("H0I69L", &str_bounded_expansion, &str_nowhere_dense, UpperBound(Exists), SrcTodo)
            .noted_relation("QjLdGf", &str_treedepth, &mon_shrubdepth, Equivalent(Exists, Exists), SrcTodo)
            .noted_relation("yrocwH", &mon_shrubdepth, &shrub_depth, Equivalent(Exists, Exists), SrcTodo)
            .noted_relation("Ch3QtW", &str_pathwidth, &mon_linear_clique_width, Equivalent(Exists, Exists), SrcTodo)
            .noted_relation("5DcZ5i", &str_treewidth, &mon_clique_width, Equivalent(Exists, Exists), SrcTodo)
            .noted_relation("ASrJsu", &str_sparse_twin_width, &mon_twin_width, Equivalent(Exists, Exists), SrcTodo)
            .noted_relation("Qt21ld", &str_bounded_expansion, &mon_flip_width, UpperBound(Exists), SrcTodo)
            .noted_relation("RCekfo", &str_nowhere_dense, &monadically_stable, UpperBound(Exists), SrcTodo)
            // remainder should be inferred
        .done()
        .todo_rest(&mut create);

    create.build()
}

// HFiKAB
// u5veuG
// jzQCJB
// 3pNJO5
// HghAUP
// un2HUZ
// aTvOui
// jUzsNa
// GKsXHM
// Uge3PQ
// mSEa3b
// YMfs4G
// USErps
// 2iwQv0
// iu40QI
// vw9BjN
// EpQl9l
// tVmfaa
// zefzkx
// M5s3qX
// 9vGZD0
// 6ILlpF
// oXV04c
// iKSIcK
// k6mnu5
// wegmLA
// AMOwg2
// PEmv7E
// 5OW5Rh
// vyW251
// bUuzDK
// DbI81X
// NsdqoF
// OceQyn
// BOhiyO
// etmyMV
// I5PQO9
// QeIhyS
// yIdjIZ
// h2zYp6
// lCDpdr
// z57T9T
// M5VUAG
// lFyAXB
// EGiqY7
// Zz18EK
// 2ddMsw
// SAJBe6
// Oilaeg
// QCYh7l
// XF5B1J
// cGKOIy
// aqllwB
// jPBt4b
// slsHW1
// RcAY8E
// 4Oqat3
// CMOWPQ
// yVyItw
// 4HYR6F
// 8i8TPb
// Mnayro
// mRz7Lq
// 1rJUW9
// fYvpzp
// O0O2lu
// lomLxF
// fIoQjs
// eyumwo
// nwa2id
