//! Collects the information we have about parameterized complexity classes, their inclusions,
//! and related graph classes, tags, bibliographic sources, theorems, proofs, and so on.

use crate::general::enums::{Cpx::*, CpxTime::*, Page::*};
use crate::input::build::{graph_class, parameter, provider, source};
use crate::input::{build::Builder, raw::RawData};

pub fn build_collection() -> RawData {
    let mut create = Builder::new();

    let tag_topology = create.tag("lJJaYb", "topology", "A graph is planar if it can be embedded into a plane without edges crossings. Related sets include those which restrict the embedding (e.g. geometric intersection graphs), consider other surfaces (e.g. toroid), or allow crossings (e.g. upper bound the total number of crossings).");
    let tag_coloring = create.tag("szWO2M", "coloring", "Coloring of a graph is its partition where each part is an independent set. Each part is considered to be a color so in a properly colored graph every edge goes between two differently colored vertices.");
    let tag_vertex_removal = create.tag("e26LM8", "vertex removal", "Some vertices of the graph may completely describe the complex structure of the graph, and the remainder may fall into a simple graph class. This includes variants of removing the vertices such as removing a set of vertices all at once (vertex cover), or removing vertices one by one (treedepth), resulting in a simple or a complex graph class (e.g. distance to X and elimination distance to X).");
    let tag_edge_removal = create.tag("hnX8tG", "edge removal", "The complexity of a graph may be described by the number of edges that we need to remove to get a simple graph class. This often means removing edges to that a forbidden structure is not contained in the graph, e.g., a cycle ([[HTk9PZ]]) or an odd cycle ([[Ve5ruW]]). Hitting each structure X is also called X-transversal.");
    let tag_edge_cover = create.tag("tK4S1r", "covering edges", "Take a graph and create a cover of its edges by graphs from a well-understood graph class.");
    let tag_vertex_cover = create.tag(
        "TgTiqK",
        "covering vertices",
        "Partition vertices of the graph into smaller graph from a well-understood graph class.",
    );
    let tag_vertex_order = create.tag("O1poSV", "vertex order", "Take an appropriate order of graph vertices to get a definition or a decomposition. Typical for some graph parameters (e.g. degeneracy) and graphs (e.g. chordal).");
    let tag_modules = create.tag("ZtdvKW", "module", "Module of a graph is its vertex set which has the same neighborhood with all vertices outside of it. As modules have a very regular structure they were used to create useful graph decompositions.");
    let tag_linear = create.tag("nAjQi4", "linear variant", "Graph decompositions often end up being trees, but still, some problems remain hard on those decompositions. Restricting the decomposition to be a path (or to be linearized in some other way) gives bigger power to the decomposition and makes some problems tractable.");
    let tree_decomposition = create.tag("bzffn0", "tree decomposition", "The classical tree decomposition is tied to the treewidth which measures maximum size of a bag. There are parameters use different bag measures and there are graphs that are closely tied to tree decompositions.");
    // let branch_decomposition = create.tag("KaLXjx", "branch decomposition", "");

    let connected = graph_class("KlMP0i", "connected", 2)
        .main_definition("mIBYMD", "There is a path between any pair of vertices.")
        .done(&mut create);
    let bipartite = graph_class("cLHJkW", "bipartite", 8)
        .main_definition("1iQ54v", "Can be partitioned into two independent sets.")
        .tag(&tag_coloring)
        .done(&mut create);
    let block = graph_class("QrxQsH", "block", 4)
        .main_definition("2kG0kY", "Every block (maximal 2-connected subgraph) is a clique.")
        .done(&mut create);
    let chordal = graph_class("Cv1PaJ", "chordal", 5)
        .main_definition("TU8uiD", "Contains no induced cycles on 4 or more vertices.")
        .tag(&tag_vertex_order)
        .tag(&tree_decomposition)
        .done(&mut create);
    let cluster = graph_class("WAU7vf", "cluster", 6)
        .main_definition("roSFzV", "Disjoint union of complete graphs.")
        .main_definition("pUnhFT", "Every connected component is a complete graph.")
        .main_definition("4aKorn", "Does not include path on three vertices as an induced subgraph.")
        .done(&mut create);
    let co_cluster = graph_class("7HR4uV", "co-cluster", 6)
        .main_definition("FDbIDy", "Complete multipartite graph.")
        .tag(&tag_coloring)
        .done(&mut create);
    let cograph = graph_class("9Qd0Mx", "cograph", 7)
        .main_definition("ZWeaib", "Can be created from single vertices by disjoint unions and complements.")
        .main_definition("2szioJ", "$P_4$-free")
        .tag(&tag_modules)
        .done(&mut create);
    let complete = create.intersection("EhdXNA", &connected, &cluster, "complete", 9)
        .main_definition("ARCCvF", "Contains every edge.")
        .done(&mut create);
    let forest = graph_class("JngPPm", "forest", 9)
        .main_definition("NUxE1Q", "A graph with no cycle.")
        .done(&mut create);
    let tree = create
        .intersection("rJyICu", &connected, &forest, "tree", 7)
        .main_definition("tCAYae", "A connected graph without cycles.")
        .main_definition("npoYQB", "A connected graph with $n$ vertices and $n-1$ edges.")
        .done(&mut create);
    let interval = graph_class("p5skoj", "interval", 7)
        .main_definition("UHFdbP", "Intersection graph of intervals on the real line.")
        .done(&mut create);
    let edgeless = graph_class("LsiBbX", "edgeless", 1)
        .main_definition("vS7Zy8", "A graph with no edges.")
        .done(&mut create);
    let linear_forest = graph_class("skQuFN", "linear forest", 4)
        .main_definition("XK5Xxy", "A disjoint union of paths.")
        .done(&mut create);
    let path = create.intersection("ryPlqz", &connected, &linear_forest, "path", 3)
        .main_definition("55gdPP", "Vertices can be ordered such that exactly those next to each other in the order are connected by an edge.")
        .done(&mut create);
    let outerplanar = graph_class("0oCyaG", "outerplanar", 5)
        .main_definition("WeXBO1", "A planar graph that can be drawin in a way where one face is incident to all the vertices.")
        .tag(&tag_topology)
        .done(&mut create);
    let perfect = graph_class("RmssrZ", "perfect", 6)
        .main_definition("aWZgIz", "Has [[q7zHeT]] equal to [[w7MmyW]].")
        .tag(&tag_coloring)
        .done(&mut create);
    let planar = graph_class("loZ5LD", "planar", 8)
        .main_definition("CUPkWg", "Has an embedding with vertices being points, edges being curves between respective points, which is planar, i.e., no curves cross each other.")
        .tag(&tag_topology)
        .done(&mut create);
    let stars = graph_class("10JR3F", "stars", 4)
        .main_definition("51KDFn", "Disjoint union of stars.")
        .done(&mut create);
    let star = create.intersection("CortlU", &connected, &stars, "star", 3)
        .main_definition("TUqM0F", "Contains only edges that connect a vertex to every other vertex.")
        .done(&mut create);
    let cycles = graph_class("2iJr52", "cycles", 4)
        .main_definition("sAZHF4", "Every component is a cycle.")
        .done(&mut create);
    let cycle = create.intersection("Ti0asF", &connected, &cycles, "cycle", 2)
        .main_definition("EqavC7", "Has a cyclic vertex order such that the graph contains edges for each pair of vertices that are next to each other in the order.")
        .main_definition("oTRopc", "A connected graph with all vertices having degree two.")
        .done(&mut create);
    let disjoint_cycles = graph_class("AGnF5Z", "disjoint cycles", 4)
        .main_definition("cBDurK", "All cycles in the graph are disjoint. Can contain arbitrary trees attached to and between the cycles.")
        .done(&mut create);
    let grid = graph_class("lfYXuK", "grid", 6)
        .main_definition("sp6LGE", "Cartesian product of two paths.")
        .done(&mut create);
    let disconnected = graph_class("lA0K71", "disconnected", 1)
        .hide()
        .done(&mut create);
    // let all_graphs = graph_class("TDTA85", "all graphs")
    // .done(&mut create); // hide
    // let nowhere_dense = graph_class("BqY0YK", "nowhere dense", 5) // todo
    // .done(&mut create);
    // let bounded_expansion = graph_class("DDM0j3", "bounded expansion", 4)
    // .done(&mut create);

    let by_definition = "By definition";

    create
        .assumed_source()
        .proper_graph_inclusion("piRTZw", &chordal, &perfect)
        .proper_graph_inclusion("stwHRi", &cograph, &perfect)
        .proper_graph_inclusion("ogyvLp", &bipartite, &perfect)
        .proper_graph_inclusion("FM1wVJ", &cluster, &interval)
        .proper_graph_inclusion("zJbehb", &cluster, &cograph)
        .proper_graph_inclusion("rHotfs", &linear_forest, &interval)
        .proper_graph_inclusion("OT3dig", &stars, &interval)
        .proper_graph_inclusion("fKpyMg", &interval, &chordal)
        .proper_graph_inclusion("cZy5xs", &co_cluster, &cograph)
        .proper_graph_inclusion("AbAK8n", &forest, &bipartite)
        .proper_graph_inclusion("ZiCzGe", &outerplanar, &planar)
        .proper_graph_inclusion("6TFVVG", &complete, &co_cluster)
        .proper_graph_inclusion("2Jde0p", &block, &chordal)
        .proper_graph_inclusion("DxYTTn", &cluster, &block)
        .proper_graph_inclusion("lmKGuy", &linear_forest, &forest)
        .proper_graph_inclusion("hIuPAJ", &disjoint_cycles, &outerplanar)
        .proper_graph_inclusion("eruyce", &forest, &disjoint_cycles)
        .proper_graph_inclusion("WJHhf0", &forest, &block)
        .proper_graph_inclusion("VsrnoK", &edgeless, &linear_forest)
        .proper_graph_inclusion("E8B2Gj", &stars, &forest)
        .proper_graph_inclusion("BWJDZs", &edgeless, &stars)
        .proper_graph_inclusion("HtdoRP", &edgeless, &co_cluster)
        .proper_graph_inclusion("1PLbSg", &grid, &planar)
        .proper_graph_inclusion("RQcVkC", &grid, &bipartite)
        .proper_graph_inclusion("CJ76wg", &cycles, &disjoint_cycles)
        .proper_graph_inclusion("CTwA2j", &grid, &connected)
        .proper_graph_inclusion("wTugFB", &edgeless, &cluster)
        .proper_graph_inclusion("1pdarO", &star, &tree)
        .proper_graph_inclusion("gAlyjK", &path, &tree)
        .proper_graph_inclusion("1pdarO", &path, &grid);

    let size = parameter("F1NpDy", "size", 3)
        .main_definition("lKvvzN", "Total size of the graph: $|V(G)+E(G)|$.")
        .done(&mut create);
    let vertex_cover = parameter("4lp9Yj", "vertex cover", 9)
        .main_definition("qOc9n0", "The minimum number of vertices that have to be removed to get an independent set.")
        .abbr("vc")
        .tag(&tag_vertex_removal)
        .done(&mut create);
    let max_matching = parameter("veU7Jf", "maximum matching", 3)
        .main_definition("ErDFlH", "The size of a maximum independent edge set.")
        .done(&mut create);
    let vertex_integrity = parameter("KVhJFB", "vertex integrity", 6)
        .main_definition("aQQnbF", "The minimum $k$ such that there exists $d$ vertices whose removal results in connected components each of size at most $d-k$.")
        .abbr("vi")
        .tag(&tag_vertex_removal)
        .done(&mut create);
    let treedepth = parameter("KEP2qM", "treedepth", 7)
        .main_definition("cGN8qC", "Treedepth of a graph is height of an auxiliary rooted forest over graph's vertices such that all edges of the graph have ancestor-descendant relationship within the tree.")
        .main_definition("E9GMDZ", "For a graph treedepth is 1 if the graph is a single vertex. Otherwise, it is the minimum value obtained by removing some vertex and taking maximum over treedepths of each connected component.")
        .abbr("td")
        .tag(&tag_vertex_removal)
        .done(&mut create);
    let clique_cover_num = parameter("VomShB", "clique cover number", 5)
        .main_definition("p0NZrl", "Clique cover number is the minimum number of parts into which vertices of the graph can be partitioned so that each part induces a clique.")
        .tag(&tag_vertex_cover)
        .done(&mut create);
    let max_independent_set = parameter("mHtXUU", "maximum independent set", 2)
        .main_definition("2xRnhJ", "Is the cardinality of a maximum vertex set such that no pair of vertices in the set are connected by an edge.")
        .done(&mut create);
    let domination_num = parameter("Gq0onN", "domination number", 3)
        .main_definition("7XYxB4", "Is the cardinality of a minimum vertex set such that its closed neighborhood contains all vertices of the graph.")
        .tag(&tag_vertex_cover)
        .done(&mut create);
    let twin_cover_num = parameter("MUnHA0", "twin-cover number", 5)
        .main_definition("nTIDMU", "In graph $G$, twin-cover number is the minimum number $k$ such that there exists a set $M$ of size $k$ such that $G-M$ is a union of cliques where each pair of vertices from the same clique are true siblings in $G$.")
        .done(&mut create);
    let edge_clique_cover = parameter("nYQDv6", "edge clique cover number", 4)
        .tag(&tag_edge_cover)
        .done(&mut create);
    let neighborhood_diversity = parameter("vMs3RS", "neighborhood diversity", 6)
        .main_definition("A3QGjy", "Vertices can be partitioned into $\\mathrm{nd}$ parts, each consisting of only false or true twins.")
        .abbr("nd")
        .tag(&tag_modules)
        .done(&mut create);
    let modular_width = parameter("4bj71L", "modular-width", 7)
        .tag(&tag_modules)
        .done(&mut create);
    let iterated_type_partitions = parameter("G1Cwmc", "iterated type partitions", 3)
        .tag(&tag_modules)
        .done(&mut create);
    let max_leaf_num = parameter("BN92vX", "maximum leaf number", 6)
        .main_definition("rBWwFy", "Largest number of tree leaves in any of graph's spanning trees.")
        .tag(&tag_vertex_cover)
        .done(&mut create);
    let feedback_edge_set = parameter("HTk9PZ", "feedback edge set", 6)
        .main_definition("eYijvL", "In the cardinality of a minimum edge set where every cycle has at least one edge in the edge set.")
        .tag(&tag_edge_removal)
        .done(&mut create);
    let genus = parameter("gbaHdw", "genus", 2)
        .tag(&tag_topology)
        .done(&mut create);
    let cutwidth = parameter("TLx1pz", "cutwidth", 4)
        .tag(&tag_vertex_order)
        .done(&mut create);
    let carving_width = parameter("dS6OgO", "carving-width", 3)
        .done(&mut create);
    let bandwidth = parameter("aP5a38", "bandwidth", 5)
        .tag(&tag_vertex_order)
        .done(&mut create);
    let topological_bandwidth = parameter("SnA7Eq", "topological bandwidth", 4)
        .tag(&tag_vertex_order)
        .done(&mut create);
    let bisection_bandwidth = parameter("wUdmUb", "bisection bandwidth", 4)
        .done(&mut create);
    // let reduced_bandwidth = create.reduced("reduced bandwidth", &bandwidth, 2)
    // .done();
    let max_degree = parameter("UyQ5yM", "maximum degree", 8)
        .main_definition("8tk4SI", "Maximum degree over graph's vertices.")
        .done(&mut create);
    let c_closure = parameter("ou9VU1", "c-closure", 0)
        .done(&mut create);
    let feedback_vertex_set = parameter("GNOiyB", "feedback vertex set", 8)
        .main_definition("81zlqB", "The minimum set of edges $S$ such that every cycle in the graph contains at least one edge of $S$.")
        .abbr("fvs")
        .tag(&tag_vertex_removal)
        .done(&mut create);
    let shrub_depth = parameter("NTgNzT", "shrub-depth", 6)
        .done(&mut create);
    let linear_clique_width = parameter("fQj3wU", "linear clique-width", 5)
        .tag(&tag_linear)
        .done(&mut create);
    let pathwidth = parameter("VHClqR", "pathwidth", 8)
        .abbr("pw")
        .tag(&tag_linear)
        .done(&mut create);
    let degree_pathwidth = create
        .intersection("6BWcgd", &pathwidth, &max_degree, "pathwidth+maxdegree", 3)
        .tag(&tag_linear)
        .done(&mut create);
    let d_path_free = parameter("s4EiWI", "d-path-free", 2)
        .done(&mut create); // todo
    let treewidth = parameter("5Q7fuR", "treewidth", 9)
        .abbr("tw")
        .tag(&tree_decomposition)
        .done(&mut create);
    let mm_width = parameter("d7vRYU", "mm-width", 4)
        .done(&mut create);
    let degree_treewidth = create
        .intersection("nCWUh3", &max_degree, &treewidth, "degree treewidth", 6)
        .tag(&tree_decomposition)
        .done(&mut create);
    // let domino_treewidth = parameter("aEs5ap", "domino treewidth", 4)
        // .tag(&tree_decomposition)
        // .done(&mut create);
    // let treebandwidth = parameter("w3LxG1", "treebandwidth", 4)
        // .main_definition("JTZz7J", "A \\emph{tree-layout} of $G=(V,E)$ is a rooted tree $T$ whose nodes are the vertices of $V$, and such that, for every edge $xy \\in E$, $x$ is an ancestor of $y$ or vice-versa. The bandwidth of $T$ is then the maximum distance in $T$ between pairs of neighbors in $G$. We call \\emph{treebandwidth} of $G$, the minimum bandwidth over tree-layouts of $G$, and denote it by ${\\rm tbw}(G)$.")
        // .done(&mut create);
    let contraction_complexity = parameter("LlWzhg", "contraction complexity", 2)
        .done(&mut create);
    let branch_width = parameter("lIcmuR", "branch width", 5)
        .done(&mut create);
    let clique_width = parameter("wg5HuV", "clique-width", 7)
        .abbr("cw")
        .done(&mut create);
    let clique_tree_width = parameter("7P9WUz", "clique-tree-width", 2)
        .done(&mut create);
    let rank_width = parameter("fojquT", "rank-width", 7)
        .done(&mut create);
    let linear_rank_width = parameter("cHugsk", "linear rank-width", 2)
        .tag(&tag_linear)
        .done(&mut create);
    let boolean_width = parameter("A2jPWT", "boolean width", 5)
        .done(&mut create);
    let inf_flip_width = parameter("nYXiuT", "inf-flip-width", 3)
        .done(&mut create);
    let twin_width = parameter("OrH7et", "twin-width", 8)
        .abbr("tww")
        .done(&mut create);
    let r_flip_width = parameter("4DIiH0", "radius-r flip-width", 3)
        .done(&mut create);
    // let merge_width = parameter("UWmTKl", "merge-width", 5)
        // .done(&mut create);
    let book_thickness = parameter("doijTS", "book thickness", 4)
        .aka("stacknumber")
        .aka("pagenumber")
        .aka("fixed outerthickness")
        .tag(&tag_topology)
        .done(&mut create);
    // .showed("1IL2wn", NotApplicable, &book_thickness, &create.edge_cover_by(&outerplanar), Equal, "") //but with fixed vertices
    let hindex = parameter("GNTwUS", "h-index", 4)
        .main_definition("1juCAg", "Maximum $h$ for which a graph contains $h$ vertices of degree at least $h$.")
        .done(&mut create);
    let acyclic_chromatic_number = parameter("QGZuUW", "acyclic chromatic number", 5)
        .tag(&tag_coloring)
        .done(&mut create);
    let odd_cycle_transversal = parameter("Ve5ruW", "odd cycle transversal", 6)
        .abbr("oct")
        .tag(&tag_edge_removal)
        .done(&mut create);
    let degeneracy = parameter("VowkuW", "degeneracy", 6)
        .tag(&tag_vertex_order)
        .done(&mut create);
    let chromatic_num = parameter("w7MmyW", "chromatic number", 5)
        .tag(&tag_coloring)
        .done(&mut create);
    let average_degree = parameter("z0y4TW", "average degree", 2)
        .done(&mut create);
    let min_degree = parameter("GPmOeT", "minimum degree", 0)
        .main_definition("CKNuj2", "Minimum degree over graph's vertices.")
        .done(&mut create);
    let max_clique = parameter("q7zHeT", "maximum clique", 5)
        .done(&mut create);
    let edge_connectivity = parameter("JbqZoT", "edge connectivity", 2)
        .done(&mut create);
    let vertex_connectivity = parameter("OyLUe4", "vertex connectivity", 0)
        .hide()
        .done(&mut create);
    let boxicity = parameter("a7MpiT", "boxicity", 6)
        .tag(&tag_topology)
        .done(&mut create);
    let chordality = parameter("fTqo40", "chordality", 4)
        .tag(&tag_edge_cover)
        .done(&mut create);
    let max_induced_matching = parameter("GzMYlT", "maximum induced matching", 3)
        .done(&mut create);
    let diameter = parameter("p4bTjp", "diameter", 6)
        .main_definition("MlVCMG", "Maximum distance of two vertices that are in the same connected component.")
        .done(&mut create);
    let average_distance = parameter("zH8PpT", "average distance", 3)
        .done(&mut create);
    let girth = parameter("BCwUeT", "girth", 1)
        .done(&mut create);
    let domatic_num = parameter("KRV6tI", "domatic number", 3)
        .done(&mut create);
    let arboricity = parameter("zgMenA", "arboricity", 5)
        .done(&mut create);
    // let star_arboricity = parameter("Mvz8MX", "star-arboricity", 1)
    // .done(&mut create);
    let mim_width = parameter("WmIFB1", "mim-width", 6)
        .done(&mut create);
    let sim_width = parameter("aEGv5N", "sim-width", 5)
        .done(&mut create);
    let module_width = parameter("EV3FqL", "module-width", 6)
        .done(&mut create);
    let tree_independence = parameter("fNR6QK", "tree-independence number", 5)
        .tag(&tree_decomposition)
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
    let dist_to_bipartite = create.distance_to("1yW82F", &bipartite, 6)
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
    let dist_to_max_degree = create.distance_to("kRR8zx", &max_degree, 4)
        .done(&mut create);
    let dist_to_bounded_components = create.distance_to("RPTCxd", &bounded_components, 4)
        .done(&mut create);
    let dist_to_disconnected = create.distance_to("ZL7BOP", &disconnected, 2).hide()
        .done(&mut create);
    // let bip_free = parameter("Qme7wD", "$K_{t,t}$-free", 5)
    // .done(&mut create);
    // let bip_sub_free = parameter("LoQADQ", "$K_{t,t}$-subgraph-free", 5)
    // .done(&mut create);
    let bipartite_number = parameter("1dQQ87", "bipartite number", 2)
        .main_definition("QmlowC", "Bipartite number of $G$ is the maximum order of an induced bipartite subgraph.")
        .done(&mut create);
    let treelength = parameter("JA2nKw", "treelength", 6)
        .main_definition("H4YERL", "Treelength of a tree decomposition is the maxmimum distance of two vertices that appear in the same bag. Treelength of a graph is the minimum treelength over tree decompositions.")
        .tag(&tree_decomposition)
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
        .done(&mut create);

    provider("bS9nCY", "ISGCI",
        "https://www.graphclasses.org/",
        Box::new(|id: &str| format!(r"https://www.graphclasses.org/classes/par_{id}.html")),
        )
        .link(&vertex_cover, "2")
        .link(&max_matching, "13")
        .link(&treedepth, "18")
        .link(&dist_to_complete, "1")
        .link(&max_independent_set, "8")
        .link(&domination_num, "5")
        .link(&dist_to_co_cluster, "3")
        .link(&dist_to_cograph, "7")
        .link(&dist_to_cluster, "29")
        .link(&max_leaf_num, "22")
        .link(&genus, "23")
        .link(&cutwidth, "15")
        .link(&carving_width, "16")
        .link(&bandwidth, "25")
        .link(&max_degree, "28")
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
        .link(&chromatic_num, "19")
        .link(&max_clique, "27")
        .link(&dist_to_block, "30")
        .link(&max_induced_matching, "14")
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
        // .link(&twin_width, "2023")
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
        .intersection("ri9Seh", &diameter, &max_degree, "diameter+max degree", 5)
        .hide()
        .done(&mut create);

    create.assumed_source()
        .ref_showed("YOBod9", NotApplicable, &vertex_connectivity, &dist_to_disconnected, Equal, by_definition)
        .ref_showed("LTyhoG", NotApplicable, &vertex_integrity, &dist_to_bounded_components, UpperBound(Linear), by_definition)
        .ref_showed("wgnjMg", NotApplicable, &dist_to_bounded_components, &vertex_integrity, UpperBound(Linear), by_definition)
        .ref_showed("SyGwqT", NotApplicable, &bandwidth, &topological_bandwidth, UpperBound(Linear), by_definition)
        .ref_showed("ebAUEu", NotApplicable, &twin_cover_num, &dist_to_cluster, UpperBound(Linear), by_definition)
        .ref_showed("2XN8ux", NotApplicable, &vertex_cover, &twin_cover_num, UpperBound(Linear), by_definition)
        .ref_showed("XTPNkl", NotApplicable, &average_degree, &min_degree, UpperBound(Linear), by_definition)
        .ref_showed("TezCU1", NotApplicable, &diameter, &average_distance, UpperBound(Linear), by_definition)
        .ref_showed("qy7Xdi", NotApplicable, &max_matching, &max_induced_matching, UpperBound(Linear), by_definition)
        .ref_showed("2gTckj", NotApplicable, &dist_to_interval, &boxicity, UpperBound(Linear), by_definition)
        .ref_showed("LAc0Ur", NotApplicable, &bisection_bandwidth, &edge_connectivity, UpperBound(Linear), by_definition)
        .ref_showed("yWSq1V", NotApplicable, &edgeless, &bounded_components, UpperBound(Constant), by_definition)
        .ref_showed("KxMj5k", NotApplicable, &grid, &max_degree, UpperBound(Constant), by_definition)
        .ref_showed("TxxhnK", NotApplicable, &bounded_components, &max_degree, UpperBound(Linear), by_definition)
        .ref_showed("ZEEhCr", NotApplicable, &linear_forest, &max_degree, UpperBound(Constant), by_definition)
        .ref_showed("a3JKzR", NotApplicable, &cycles, &max_degree, UpperBound(Constant), by_definition)
        .ref_showed("G5i5Bz", NotApplicable, &diameter, &treelength, UpperBound(Linear), by_definition)
        .ref_showed("t0TUmk", NotApplicable, &edgeless, &connected, Exclusion, by_definition)
        .ref_showed("jkoObg", NotApplicable, &edgeless, &cycles, Exclusion, by_definition)
        .ref_showed("81rUKt", NotApplicable, &diam_maxdeg, &bounded_components, UpperBound(Exponential), "folklore")
        .ref_showed("Ktfezk", NotApplicable, &max_independent_set, &bipartite_number, UpperBound(Linear), "folklore")
        .ref_showed("rZeuh1", NotApplicable, &min_degree, &average_degree, Exclusion, "folklore")
        .ref_showed("lKvvzN", NotApplicable, &size, &vertex_cover, UpperBound(Linear), by_definition)
        .ref_showed("sXOMaO", NotApplicable, &size, &max_leaf_num, UpperBound(Linear), by_definition)
        .ref_showed("yzbP6z", NotApplicable, &size, &dist_to_complete, UpperBound(Linear), by_definition)
        .ref_showed("O4vak5", NotApplicable, &size, &bounded_components, UpperBound(Linear), by_definition)
        .ref_showed("Cgalgy", NotApplicable, &size, &planar, Exclusion, by_definition)
        .ref_showed("Bj3xlp", NotApplicable, &size, &perfect, Exclusion, by_definition)
        .ref_showed("nIqkYI", NotApplicable, &size, &connected, Exclusion, by_definition)
        ;

    let cliques_make_it_unbounded = "Parameter is unbounded for the graph class of cliques.";

    let tmp_8mm5qj = create.intersection("8Mm5qJ", &bipartite, &max_matching, "maximum matching on bipartite graphs", 0)
        .hide()
        .done(&mut create);

    create.unknown_source()
        .ref_showed("8Mm5qJ", NotApplicable, &tmp_8mm5qj, &vertex_cover, Exactly(Linear), "Kőnig's theorem")
        .ref_showed("kmCM7X", NotApplicable, &vertex_cover, &max_matching, Exactly(Linear), "Every edge of the matching needs to be covered by at least one vertex. Path shows that this relation is not better than linear.")
        .ref_showed("cGN8qC", NotApplicable, &max_matching, &vertex_cover, UpperBound(Linear), "A set of all vertices taking part in a maximum matching creates a vertex cover, hence $vc(G) \\le 2 \\cdot mm(G)$.")
        // Cite(id="gBA7dc", url="https://en.wikipedia.org/wiki/K%C5%91nig%27s_theorem_(graph_theory)", text="Kőnig's theorem"),
        .ref_showed("U14yX4", NotApplicable, &odd_cycle_transversal, &dist_to_bipartite, Equal, "Bipartite graphs is the graph class without any odd cycles.")
        // Note(id="lqOY3G", text="Bipartite graphs is the graph class without any odd cycles."),
        .ref_showed("5sq1SD", NotApplicable, &feedback_edge_set, &feedback_vertex_set, UpperBound(Linear), "Given solution to feedback edge set one can remove one vertex incident to the solution edges to obtain feedback vertex set.")
        .ref_showed("8dQ8Us", NotApplicable, &feedback_edge_set, &genus, UpperBound(Linear), "Removing $k$ edges creates a forest that is embeddable into the plane. We now add one handle for each of the $k$ edges to get embedding into $k$-handle genus.")
        .ref_showed("K0Bc61", NotApplicable, &chromatic_num, &max_clique, UpperBound(Linear), "Unbounded clique implies the number of needed colors is unbounded.")
        .ref_showed("uKFrrb", NotApplicable, &degeneracy, &chromatic_num, UpperBound(Linear), "Greedily color the vertices in order of the degeneracy ordering. As each vertex has at most $k$ colored predecesors we use at most $k+1$ colors.")
        .ref_showed("gLjejq", NotApplicable, &degeneracy, &average_degree, UpperBound(Linear), "Removing a vertex of degree $d$ increases the value added to the sum of all degrees by at most $2d$, hence, the average is no more than twice the degeneracy.")
        .ref_showed("q5QDXg", NotApplicable, &max_degree, &hindex, UpperBound(Linear), "As h-index seeks $k$ vertices of degree $k$ it is trivially upper bound by maximum degree.")
        .ref_showed("1MAoyr", NotApplicable, &min_degree, &edge_connectivity, UpperBound(Linear), "Removing edges incident to the minimum degree vertex disconnects the graph.")
        .ref_showed("XOAOqw", NotApplicable, &linear_rank_width, &rank_width, Todo, "")
        .ref_showed("SUEy4S", NotApplicable, &pathwidth, &linear_rank_width, Todo, "")
        .ref_showed("UYpwYn", NotApplicable, &min_degree, &domatic_num, UpperBound(Linear), "The vertex of minimum degree needs to be dominated in each of the sets. As the sets cannot overlap there can be at most $k+1$ of them.")
        .ref_showed("d2ZJIh", NotApplicable, &dist_to_linear_forest, &pathwidth, UpperBound(Linear), "After removal of $k$ vertices the remaining class has a bounded width $w$. So by including the removed vertices in every bag, we can achieve decomposition of width $w+k$")
        .ref_showed("LyJWeW", NotApplicable, &topological_bandwidth, &bisection_bandwidth, UpperBound(Linear), "Order vertices by their bandwidth integer. We split the graph in the middle of this ordering. There are at most roughly $k^2/2$ edges over this split.")
        .ref_showed("waxvtz", NotApplicable, &bandwidth, &max_degree, UpperBound(Linear), "Each vertex has an integer $i$ and may be connected only to vertices whose difference from $i$ is at most $k$. There are at most $k$ bigger and $k$ smaller such neighbors.")
        .ref_showed("d2ZJIh", NotApplicable, &dist_to_linear_forest, &pathwidth, UpperBound(Linear), "After removal of $k$ vertices the remaining class has a bounded width $w$. So by including the removed vertices in every bag, we can achieve decomposition of width $w+k$")
        .ref_showed("d2ZJIh", NotApplicable, &dist_to_outerplanar, &treewidth, UpperBound(Linear), "After removal of $k$ vertices the remaining class has a bounded width $w$. So by including the removed vertices in every bag, we can achieve decomposition of width $w+k$")
        .ref_showed("VS44M7", NotApplicable, &vertex_integrity, &treedepth, UpperBound(Linear), "First, treedepth removes vertices of the modulator, then it iterates through remaining components one by one.")
        .ref_showed("rmLeo2", NotApplicable, &dist_to_stars, &treedepth, UpperBound(Linear), "First, treedepth removes vertices of the modulator, remainder has treedepth $2$")
        .ref_showed("bYybsT", NotApplicable, &dist_to_complete, &clique_cover_num, UpperBound(Linear), "We cover the $k$ vertices of the modulator by cliques of size $1$ and cover the remaining clique by another one.")
        .ref_showed("gGtTUf", NotApplicable, &max_independent_set, &domination_num, UpperBound(Linear), "Every maximal independent set is also a dominating set because any undominated vertex could be added to the independent set.")
        .ref_showed("J0jyXi", NotApplicable, &domination_num, &diameter, UpperBound(Linear), "An unbounded diameter implies a long path where no vertices that are more than $3$ apart may be dominated by the same dominating vertex, otherwise we could shorten the path. Hence, the number of dominating vertices is also unbounded.")
        .ref_showed("xrVJqb", NotApplicable, &dist_to_bipartite, &chromatic_num, UpperBound(Linear), "Removed vertices get one color each and we need only $2$ colors for the rest.")
        .ref_showed("5wc1ir", NotApplicable, &edge_clique_cover, &neighborhood_diversity, UpperBound(Exponential), "Label vertices by the cliques they are contained in, each label is its own group in the neighborhood diversity, connect accordingly.")
        .ref_showed("RnkWvT", NotApplicable, &dist_to_complete, &edge_clique_cover, UpperBound(Polynomial), "Cover the remaining clique, cover each modulator vertex and its neighborhood outside of it with another clique, cover each edge within the modulator by its own edge.")
        // .ref_showed("8ouyNs", NotApplicable, &edge_clique_cover, &clique_cover_num, UpperBound(Linear), "Covering all edges ")
        .ref_showed("FY0U1r", NotApplicable, &treewidth, &book_thickness, UpperBound(Exists), "")
        .ref_showed("BKCgft", NotApplicable, &max_leaf_num, &dist_to_linear_forest, UpperBound(Exists), "")
        .ref_showed("CyAMhs", NotApplicable, &acyclic_chromatic_number, &boxicity, UpperBound(Exists), "")
        .ref_showed("pUfoGn", NotApplicable, &hindex, &dist_to_max_degree, UpperBound(Linear), "Remove the $h$ vertices of degree at least $h$ to get a graph that has maximum degree $h$.")
        .ref_showed("8ZzI5w", NotApplicable, &dist_to_max_degree, &hindex, UpperBound(Linear), "Removal of $k$ vertices yielding a graph with maximum degree $c$ means that there were $k$ vertices of arbitrary degree and the remaining vertices had degree at most $k+c$. Hence, $h$-index is no more than $k+c$.")
        .ref_showed("fedm1t", NotApplicable, &dist_to_cograph, &chordality, UpperBound(Linear), "")
        .ref_showed("rGMb0t", NotApplicable, &dist_to_cograph, &diameter, UpperBound(Linear), "")
        .ref_showed("Er0L5w", NotApplicable, &book_thickness, &acyclic_chromatic_number, UpperBound(Exists), "")
        // .ref_showed("03kKbA", NotApplicable, &dist_to_planar, &acyclic_chromatic_number, UpperBound(Exists), "") // idk
        .ref_showed("wJkzlI", NotApplicable, &average_distance, &girth, StrictUpperBound(Exists), "Small average distance implies a small cycle while adding a triangle makes the girth constant and minimally changes the average distance.")
        .ref_showed("gRJqnm", NotApplicable, &average_distance, &diameter, Exclusion, "join of a path and a complete bipartite graph")
        .ref_showed("JfSGx1", NotApplicable, &max_leaf_num, &feedback_edge_set, UpperBound(Exists), "")
        .ref_showed("LJQHKw", NotApplicable, &max_induced_matching, &diameter, UpperBound(Linear), "Diameter requires an induced path on $d$ edges, hence, maximum induced matching is at least $\\lfloor (d+1)/3 \\rfloor$.")
        .ref_showed("unkZhD", NotApplicable, &max_independent_set, &max_induced_matching, UpperBound(Linear), "Each edge of the induced matching can host at one vertex of the independent set.")
        .ref_showed("RqDij1", NotApplicable, &vertex_cover, &neighborhood_diversity, UpperBound(Exponential), "")
        .ref_showed("a2DTDH", NotApplicable, &twin_cover_num, &neighborhood_diversity, Exclusion, "")
        .ref_showed("Pinlr2", NotApplicable, &linear_clique_width, &clique_width, UpperBound(Exists), "")
        .ref_showed("OUUh3y", NotApplicable, &clique_width, &boolean_width, UpperBound(Linear), "")
        .ref_showed("hgUvsR", NotApplicable, &boolean_width, &clique_width, UpperBound(Exponential), "")
        .ref_showed("V9Pisv", NotApplicable, &branch_width, &boolean_width, UpperBound(Linear), "")
        .ref_showed("Q3Bz8d", NotApplicable, &module_width, &clique_width, Equivalent(Exists, Exists), "")
        .ref_showed("yPnIog", NotApplicable, &treewidth, &mm_width, Equivalent(Exists, Exists), "")
        .ref_showed("0zGd6N", NotApplicable, &branch_width, &rank_width, UpperBound(Linear), "")
        .ref_showed("QWXYYb", NotApplicable, &treewidth, &boolean_width, UpperBound(Exists), "")
        .ref_showed("mD6cvS", NotApplicable, &bandwidth, &cutwidth, Exactly(Polynomial), "Any bandwidth bound cutwidth quadratically. An example where this happens is $(P_n)^k$ which has bandwidth $k$ and cutwidth $O(k^2)$; both seem to be optimal.")
        .ref_showed("NTZE4R", NotApplicable, &modular_width, &clique_width, UpperBound(Exists), "")
        .ref_showed("Vq2BBF", NotApplicable, &modular_width, &diameter, UpperBound(Exists), "")
        // .ref_showed("TA2EZd", NotApplicable, &dist_to_planar, &twin_width, UpperBound(Exists), "") // dist may not, even if planar has bounded twin-width
        .ref_showed("qB1OMb", NotApplicable, &max_degree, &c_closure, UpperBound(Exists), "")
        .ref_showed("fmiQlU", NotApplicable, &feedback_edge_set, &c_closure, UpperBound(Exists), "")
// Bound(fr=vertex_cover, to=neighborhood_diversity, notes=[
    // Cite(id="YgTRtT", url="https://link.springer.com/article/10.1007/s00453-011-9554-x", text="Construct $k$ singleton sets, one for each vertex in the vertex cover and at most $2^k$ additional sets, one for each subset of vertices of the vertex cover. ...", range=Range(EXPONENTIAL)),
    // ])
        .ref_showed("H1gQ6m", NotApplicable, &feedback_vertex_set, &dist_to_forest, Equal, "")
        .ref_showed("hDNUsi", NotApplicable, &vertex_cover, &dist_to_edgeless, Equal, "")
        .ref_showed("Jyi5e3", NotApplicable, &complete, &max_clique, Exclusion, cliques_make_it_unbounded)
        .ref_showed("t9mJyF", NotApplicable, &complete, &domatic_num, Exclusion, cliques_make_it_unbounded)
        .ref_showed("KnGxdS", NotApplicable, &complete, &edge_connectivity, Exclusion, cliques_make_it_unbounded)
        .ref_showed("fQjK7z", NotApplicable, &co_cluster, &dist_to_chordal, Exclusion, "")
        .ref_showed("cOXKlo", NotApplicable, &cluster, &twin_cover_num, UpperBound(Constant), "")
        .ref_showed("jIxF3A", NotApplicable, &cluster, &domination_num, Exclusion, "")
        .ref_showed("OjWb8I", NotApplicable, &bipartite, &girth, Exclusion, "")
        .ref_showed("d1qoN7", NotApplicable, &bipartite, &edge_connectivity, Exclusion, "")
        .ref_showed("Z335lf", NotApplicable, &forest, &feedback_edge_set, UpperBound(Constant), "")
        .ref_showed("5pJxbA", NotApplicable, &forest, &girth, Exclusion, "")
        .ref_showed("k18Pyk", NotApplicable, &forest, &dist_to_interval, Exclusion, "")
        .ref_showed("2QZo3T", NotApplicable, &edgeless, &vertex_cover, UpperBound(Constant), "")
        .ref_showed("cq2q83", NotApplicable, &edgeless, &domination_num, Exclusion, "")
        .ref_showed("TOJxXi", NotApplicable, &grid, &dist_to_chordal, Exclusion, "")
        .ref_showed("MRucBP", NotApplicable, &grid, &average_distance, Exclusion, "")
        .ref_showed("MYM6Ye", NotApplicable, &grid, &bisection_bandwidth, Exclusion, "")
        .ref_showed("gaJIvr", NotApplicable, &disjoint_cycles, &bisection_bandwidth, UpperBound(Constant), "")
        .ref_showed("VJwjbX", NotApplicable, &outerplanar, &bisection_bandwidth, UpperBound(Constant), "")
        .ref_showed("fQjK7z", NotApplicable, &grid, &max_degree, UpperBound(Constant), "")
        .ref_showed("HOULS0", NotApplicable, &disjoint_cycles, &girth, Exclusion, "")
        .ref_showed("OjWb8I", NotApplicable, &interval, &average_distance, Exclusion, "")
        .ref_showed("967lJ2", NotApplicable, &path, &treedepth, Exclusion, "")
        .ref_showed("ne07p3", NotApplicable, &linear_forest, &average_distance, Exclusion, "")
        .ref_showed("kkMeCO", NotApplicable, &planar, &genus, UpperBound(Constant), "")
        .ref_showed("EZdonY", NotApplicable, &planar, &girth, Exclusion, "")
        .ref_showed("cIAr80", NotApplicable, &planar, &max_degree, Exclusion, "")
        .ref_showed("DxmXhS", NotApplicable, &planar, &dist_to_perfect, Exclusion, "")
        .ref_showed("VAAXVv", NotApplicable, &vertex_integrity, &neighborhood_diversity, Exclusion, "")
        .ref_showed("CoBOm0", NotApplicable, &stars, &hindex, Exclusion, "")
        .ref_showed("Ei8B1H", NotApplicable, &stars, &vertex_integrity, Exclusion, "")
        .ref_showed("ORlCs0", NotApplicable, &cycles, &dist_to_perfect, Exclusion, "")
        .ref_showed("tZrOta", NotApplicable, &cycle, &max_leaf_num, UpperBound(Constant), "")
        .ref_showed("cYF2KU", NotApplicable, &cycle, &girth, Exclusion, "")
        .ref_showed("CkDe7e", NotApplicable, &max_leaf_num, &feedback_edge_set, UpperBound(Polynomial), "M. Bentert (personal communication)") // todo not unknown
        .ref_showed("QeiwSR", NotApplicable, &bounded_components, &cutwidth, UpperBound(Polynomial), "By greedily placing one component after another.")
        .ref_showed("EjGaM8", NotApplicable, &bounded_components, &dist_to_perfect, Exclusion, "By a disjoint union of small components with distance to perfect at least 1.")
        .ref_showed("bQLN2O", NotApplicable, &bounded_components, &dist_to_planar, Exclusion, "By a disjoint union of many $K_5$ graphs.")
        .ref_showed("MQ0K6A", NotApplicable, &star, &vertex_cover, UpperBound(Constant), "trivially")
        .ref_showed("btFVbS", NotApplicable, &star, &hindex, UpperBound(Constant), "trivially")
        .ref_showed("A2vYf3", NotApplicable, &tree, &hindex, Exclusion, "trivially")
        .ref_showed("vPk1LG", NotApplicable, &path, &dist_to_cluster, Exclusion, "trivially")
        .ref_showed("dy8lvH", NotApplicable, &path, &diameter, Exclusion, "trivially")
        .ref_showed("DsZGLl", NotApplicable, &cycles, &pathwidth, UpperBound(Constant), "trivially")
        .ref_showed("K9z178", NotApplicable, &star, &max_degree, Exclusion, "trivially")
        .ref_showed("f1nTaY", NotApplicable, &complete, &max_matching, Exclusion, "")
        .ref_showed("8io8sJ", NotApplicable, &path, &max_matching, Exclusion, "")
        .ref_showed("GiDjOm", NotApplicable, &star, &max_matching, UpperBound(Constant), "")
        .ref_showed("rmHBsY", NotApplicable, &edgeless, &max_matching, UpperBound(Constant), "")
        // .ref_showed("D2YglK", Unknown, &create.intersection("QrYeIw", &treewidth, &max_degree, "treewidth+maxdegree"), &create.intersection("hljuu8", &clique_width, &max_degree, "cliquewidth+maxdegree"), UpperBound(Linear), "")
        // .ref_showed("JJTNMl", Unknown, &create.intersection("nP3xBv", &clique_width, &max_degree, "cliquewidth+maxdegree"), &create.intersection("iPgGur", &treewidth, &max_degree, "treewidth+maxdegree"), UpperBound(Linear), "")
        // clique-width = fusing width (operation to merge a color class to a single vertex)
        .ref_showed("tiEYdy", NotApplicable, &clique_width, &mim_width, UpperBound(Exists), "")
        .ref_showed("5vq7po", NotApplicable, &mim_width, &sim_width, UpperBound(Exists), "")
        .ref_showed("sWR5Yw", NotApplicable, &treewidth, &tree_independence, UpperBound(Exists), "")
        .ref_showed("xwLQQ8", NotApplicable, &tree_independence, &sim_width, UpperBound(Exists), "")
        .ref_showed("E7K0I5", NotApplicable, &clique_width, &twin_width, UpperBound(Exists), "")
        .ref_showed("KI6Jq6", NotApplicable, &min_degree, &dist_to_disconnected, UpperBound(Exists), "")
        .ref_showed("g20P5t", NotApplicable, &bisection_bandwidth, &dist_to_disconnected, UpperBound(Exists), "")
        .ref_showed("V01YhI", NotApplicable, &dist_to_cluster, &dist_to_cograph, UpperBound(Exists), "")
        .ref_showed("zJ6L6N", NotApplicable, &feedback_vertex_set, &dist_to_outerplanar, UpperBound(Exists), "")
        .ref_showed("aTjwcL", NotApplicable, &dist_to_planar, &acyclic_chromatic_number, UpperBound(Exists), "")
        .ref_showed("VYcUHd", NotApplicable, &max_independent_set, &clique_cover_num, Exclusion, "")
        .ref_showed("Zv5i0U", NotApplicable, &domination_num, &max_independent_set, Exclusion, "")
        .ref_showed("Mogls9", NotApplicable, &genus, &chromatic_num, UpperBound(Linear), "in fact, bounded by square root") // jansen mentions this is in Graphs, Colourings And The Four-Colour Theorem Get access Arrow by Robert A Wilson
        .ref_showed("pavXOg", NotApplicable, &dist_to_cluster, &shrub_depth, UpperBound(Constant), "J. Pokorný, personal communication: Assume the class of constant dtc we want to show it has constant sd as well. For each clique connect them in a star in the tree model T. Each vertex in the modulator connect to their own vertex in T. Add a root that is in distance 2 to all leaves. Now give each vertex in the modulator a unique colour. Each other vertex that is not in the modulator has as it's colour the set of neighbours from the modulator. In total there are $2^{dtc} + dtc$ colours that is a constant.")
        ;

    create.web_source("8ryhNq", "https://en.wikipedia.org/wiki/Genus_(mathematics)#Graph_theory")
        .defined("3qF6Zm", NotApplicable, &genus, "The genus of a graph is the minimal integer $n$ such that the graph can be drawn without crossing itself on a sphere with $n$ handles.");
    create.web_source("bnOBjM", "https://link.springer.com/article/10.1007/bf01215352")
        .defined("gMC8t4", NotApplicable, &carving_width, "Let $V$ be a finite set with $|V| \\ge 2$. Two subsets $A,B\\subseteq V$ \\emph{cross} if $A\\cap B$, $A-B$, $B-A$, $V-(A\\cup B)$ are all non-empty. A \\emph{carving} in $V$ is a set $\\mathscr{C}$ of subsets of $V$ such that 1) $\\emptyset, V \\notin \\mathscr{C}$ 2) no two members of $\\mathscr{C}$ cross, and 3) $\\mathscr{C}$ is maximal subject to (1) and (2). ... For $A \\subseteq V(G)$, we denote by $\\delta(A)$ ... the set of all edges with an end in $A$ and an end in $V(G)-A$. For each $e \\in E(G)$, let $p(e) \\ge 0$ be an integer. For $X \\subseteq E(G)$ we denote $\\sum_{e \\in X}p(e)$ by $p(X)$, and if $|V(G)| \\ge 2$ we define the \\emph{$p$-carving-width} of $G$ to be the minimum, over all carvings $\\mathscr{C}$ in $V(G)$, of the maximum, over all $A \\in \\mathscr{C}$, of $p(\\delta(A))$. ... The \\emph{carving-width} of $G$ is the $p$-carving-width of $G$ where $p(e)=1$ for every edge $e$.");
    create.web_source("s11UF7", "https://en.wikipedia.org/wiki/Carving_width")
        .defined("LtcqRs", NotApplicable, &carving_width, "A carving can be described as an unrooted binary tree whose leaves are labeled with the vertices of the given graph. Removing any edge from this tree partitions the tree into two subtrees, and correspondingly partitions the vertices of the tree into two clusters. ... The width of a carving, defined in this way, is the maximum number of edges that connect two complementary clusters. The carving width of the graph is the minimum width of any hierarchical clustering.");
    create.web_source("s7OvjQ", "https://en.wikipedia.org/wiki/Graph_bandwidth")
        .defined("9n7dry", NotApplicable, &bandwidth, "(paraphrased) Label graph vertices with distinct integers. Bandwidth of this labelling is the maximum over label differences over all edges. Bandwidth of a graph is the minimum over all labellings.");
    create.web_source("iWUynL", "https://en.wikipedia.org/wiki/Bisection_bandwidth")
        .defined("Kj73IQ", NotApplicable, &bisection_bandwidth, "... bisected into two equal-sized partitions, the bisection bandwidth of a network topology is the bandwidth available between the two partitions.");
    create
        .web_source("AeRM2B", "http://parallelcomp.github.io/Lecture3.pdf")
        .defined("w15E7O", NotApplicable, &bisection_bandwidth, "(number of) links across smallest cut that divides nodes in two (nearly) equal parts");
    create.web_source("BJhqpe", "https://en.wikipedia.org/wiki/Feedback_vertex_set")
        .defined("xPcvEf", NotApplicable, &feedback_vertex_set, "... a feedback vertex set (FVS) of a graph is a set of vertices whose removal leaves a graph without cycles... . The feedback vertex set number of a graph is the size of a smallest feedback vertex set.");
    create.web_source("4Dua5N", "https://www.fi.muni.cz/~hlineny/papers/shrubdepth-warw18-slipp.pdf")
        .defined("zWFoL1", Pp(7), &shrub_depth, "Tree-model of $m$ colors and depth $d$: a rooted tree $T$ of height $d$, leaves are the vertices of $G$, each leaf has one of $m$ colors, an associated signature determining the edge set of $G$ as follows: for $i=1,2,\\dots,d$, let $u$ and $v$ be leaves with the least common ancestor at height $i$ in $T$, then $uv \\in E(G)$ iff the color pair of $u,v$ is in the signature at height $i$.")
        .showed("X3CThx", Pp(9), &neighborhood_diversity, &shrub_depth, UpperBound(Constant), "Shrub-depth 1: e.g., cliques, stars, \\dots, gen. BND -- bounded neighborhood diversity.")
        .done(&mut create);
    create.web_source("dxaIhi", "https://mathworld.wolfram.com/Pathwidth.html")
        .defined("OivGaa", NotApplicable, &pathwidth, "The pathwidth of a graph $G$, also called the interval thickness, vertex separation number, and node searching number, is one less than the size of the largest set in a path decomposition G.");
    create.web_source("ZhBkjd", "https://en.wikipedia.org/wiki/Branch-decomposition")
        .defined("0SLCxV", NotApplicable, &branch_width, "... branch-decomposition of an undirected graph $G$ is a hierarchical clustering of the edges of $G$, represented by an unrooted binary tree $T$ with the edges of $G$ as its leaves. Removing any edge from $T$ partitions the edges of $G$ into two subgraphs, and the width of the decomposition is the maximum number of shared vertices of any pair of subgraphs formed in this way. The branchwidth of $G$ is the minimum width of any branch-decomposition of $G$.");
    create.web_source("9Ckusi", "https://en.wikipedia.org/wiki/Clique-width")
        .defined("pLDACG", NotApplicable, &clique_width, "... the minimum number of labels needed to construct G by means of the following 4 operations: 1. Creation of a new vertex... 2. Disjoint union of two labeled graphs... 3. Joining by an edge every vertex labeled $i$ to every vertex labeled $j$, where $i \\ne j$ 4. Renaming label $i$ to label $j$");
    create.web_source("YGmwCG", "https://en.wikipedia.org/wiki/Book_embedding")
        .defined("jiDWoN", NotApplicable, &book_thickness, "... a book embedding is a generalization of planar embedding of a graph to embeddings into a book, a collection of half-planes all having the same line as their boundary. Usually, the vertices of the graph are required to lie on this boundary line, called the spine, and the edges are required to stay within a single half-plane. The book thickness of a graph is the smallest possible number of half-planes for any book embedding of the graph.");
    create.web_source("cNSdgE", "https://www.graphclasses.org/classes/par_31.html")
        .defined("JpPGki", NotApplicable, &acyclic_chromatic_number, "The acyclic chromatic number of a graph $G$ is the smallest size of a vertex partition $V_1,\\dots,V_\\ell$ such that each $V_i$ is an independent set and for all $i,j$ that graph $G[V_i \\cup V_j]$ does not contain a cycle.");
    create.web_source("rj2m4h", "https://en.wikipedia.org/wiki/Acyclic_coloring")
        .defined("PQ9STH", NotApplicable, &acyclic_chromatic_number, "... an acyclic coloring is a (proper) vertex coloring in which every 2-chromatic subgraph is acyclic.");
    create.web_source("6LCwBu", "https://en.wikipedia.org/wiki/Degeneracy_(graph_theory)")
        .defined("TYABmf", NotApplicable, &degeneracy, "... the least $k$ for which there exists an ordering of the vertices of $G$ in which each vertex has fewer than $k$ neighbors that are earlier in the ordering.");
    create.web_source("VqwUmp", "https://mathworld.wolfram.com/ChromaticNumber.html")
        .defined("VLEw7q", NotApplicable, &chromatic_num, "The chromatic number of a graph G is the smallest number of colors needed to color the vertices of G so that no two adjacent vertices share the same color (Skiena 1990, p. 210), ...");
    create.web_source("o6tFCJ", "https://bookdown.org/omarlizardo/_main/2-7-average-degree.html")
        .defined("PUQ3kt", NotApplicable, &average_degree, "Average degree is simply the average number of edges per node in the graph. ... Total Edges/Total Nodes=Average Degree");
    create.web_source("PVi4lL", "https://mathworld.wolfram.com/MaximumClique.html")
        .defined("Nm1F3M", NotApplicable, &max_clique, "A maximum clique of a graph $G$ is a clique (i.e., complete subgraph) of maximum possible size for $G$.");
    create.web_source("ZunX1e", "https://mathworld.wolfram.com/EdgeConnectivity.html")
        .defined("2gQP1W", NotApplicable, &edge_connectivity, "The edge connectivity, also called the line connectivity, of a graph is the minimum number of edges $\\lambda(G)$ whose deletion from a graph $G$ disconnects $G$.");
    create.web_source("XWbXPm", "https://en.wikipedia.org/wiki/Boxicity")
        .defined("PgaxqR", NotApplicable, &boxicity, "The boxicity of a graph is the minimum dimension in which a given graph can be represented as an intersection graph of axis-parallel boxes.");
    create.web_source("8eXjAy", "https://mathworld.wolfram.com/DomaticNumber.html")
        .defined("TG2BEi", NotApplicable, &domination_num, "The maximum number of disjoint dominating sets in a domatic partition of a graph $G$ is called its domatic number $d(G)$. ");
    create.web_source("055mG5", "https://en.wikipedia.org/wiki/Distance_(graph_theory)#Related_concepts")
        .defined("OaKBaL", NotApplicable, &diameter, "... [diameter] is the greatest distance between any pair of vertices ...");
    create.web_source("GfSsR4", "https://onlinelibrary.wiley.com/doi/abs/10.1002/jgt.3190120309")
        .defined("sBhhEO", NotApplicable, &average_degree, "The average distance in a graph is defined as the average length of a shortest path between two vertices, taken over all pairs of vertices.");
    create.web_source("u13WN1", "https://en.wikipedia.org/wiki/Girth_(graph_theory)")
        .defined("INk53D", NotApplicable, &girth, "In graph theory, the girth of an undirected graph is the length of a shortest cycle contained in the graph.");
    create.web_source("8eXjAy", "https://mathworld.wolfram.com/DomaticNumber.html")
        .defined("oTPnV8", NotApplicable, &domatic_num, "The maximum number of disjoint dominating sets in a domatic partition of a graph $G$ is called its domatic number $d(G)$. ");

    // let bandwidth_on_trees = create.intersection("Iu05N3", &tree, &bandwidth, "tree+bandwidth");
    // let cutwidth_on_trees = create.intersection("peyWzt", &tree, &cutwidth, "tree+cutwidth");

    let chung1985 = source("DkY1Aq", "Chung1985", 1)
        // .showed("YgJVvi", Unknown, &bandwidth_on_trees, &cutwidth_on_trees, UpperBound(Linear), "")
        // .showed("pRjX8u", Unknown, &cutwidth_on_trees, &bandwidth_on_trees, UpperBound(Linear), "")
        .todo_rest(&mut create);
    let chung1988 = source("ePpmZt", "chung1988", 1)
        .showed("fccHmU", Unknown, &max_independent_set, &average_distance, UpperBound(Linear), "[ed. paraphrased from another source] Let $G$ be a graph. Then $\\bar{D} \\le \\alpha$, with equality holding if and only if $G$ is complete.")
        .todo_rest(&mut create);
    let robertson_seymour1986 = source("i56ihO", "RobertsonSymour1986", 8)
        .defined("HHHQZT", Pp(1), &treewidth, "A \\emph{tree-decomposition} of $G$ is a family $(X_i \\colon i\\in I)$ of subsets of $V(G)$, together with a tree $T$ with $V(T)=I$, with the following properties. (W1) $\\bigcup(X_i \\colon i \\in I)=V(G)$. (W2) Every edge of $G$ has both its ends in some $X_i$ ($i \\in I$). (W3) For $i,j,k \\in I$, if $j$ lies on the path of $T$ from $i$ to $k$ then $X_i \\cap X_k \\subseteq X_j$. The \\emph{width} of the tree-decomposition is $\\max(|X_i|-1 \\colon i \\in I)$. The tree-width of $G$ is the minimum $w \\ge 0$ such that $G$ has a tree-decomposition of width $\\le w$.")
        .defined("aYyqd4", Pp(1), &treewidth, "Equivalently, the tree-width of $G$ is the minimum $w \\ge 0$ such that $G$ is a subgraph of a ``chordal'' graph with all cliques of size at most $w + 1$.")
        // .showed("NqLFrC", Pp(2), "(1.2) For any fixed integer $w$, there is a polynomial algorithm to decide if the input graph has tree-width $\\le w$.") // non-constructive
        // .showed("a7nQ0N", Pp(6), treewidth, minor_closed, "(2.7) If $G$ has tree-width $< w$, so does ever minor of $G$.")
        .done(&mut create);
    // let excludingforest1991 = source("AyLnH4", "excludingforest1991")
    // .todo_rest(&mut create);
    let chordality1993 = source("IFY0Rw", "chordality1993", 4)
        .defined("Xdg7Hv", Pp(1), &chordality, "The \\emph{chordality} of a graph $G=(V,E)$ is defined as the minimum $k$ such that we can write $E=E_1,\\cap\\dots\\cap E_k$ with each $(V,E_i)$ a chordal graph.")
        .showed("rQBO3K", Pp(2), &chromatic_num, &chordality, UpperBound(Linear), "Corollary 4. For any graph $G$, $\\mathrm{Chord}(G) \\le \\chi(G)$, the chromatic number of $G$.")
        .showed("N0jfjr", Pp(5), &treewidth, &chordality, UpperBound(Linear), "Theorem 7. For any graph $G$, $\\mathrm{Chord}(G) \\le \\tau(G)$.")
        .done(&mut create);
    let malitz1994 = source("cCrsoK", "Malitz1994", 2)
        .showed("ECnpoM", Pp(24), &genus, &book_thickness, UpperBound(Linear), "Theorem 5.1. Genus $g$ graphs have pagenumber $O(\\sqrt{g})$.") // is optimal
        .done(&mut create);
    let robertson_seymour1986_5 = source("A82svt", "RobertsonSymour1986V", 3)
        // .showed("u4wtjE", Pp(2), &excluding any planar, &treewidth, "(1.5) For every planar graph $H$, there is a number $w$ such that every planar graph with no minor isomorphic to $H$ has tree-wdtih $\\le w$")
        .todo_rest(&mut create);
    let robertson_seymour1991 = source("1hPzXs", "RobertsonSymour1991", 7)
        .defined("gMAL5e", Pp(12), &branch_width, "A \\emph{branch-width} of a hypergraph $G$ is a pair $(T,\\tau)$, where $T$ is a ternary tree and $\\tau$ is a bijection from the set of leaves of $T$ to $E(G)$. The \\emph{order} of an edge $e$ of $T$ is the number of vertices $v$ of $G$ such that there are leaves $t_1,t_2$ of $T$ in different components of $T \\setminus e$, with $\\tau(t_1),\\tau(t_2)$ both incident with $v$. The \\emph{width} of $(T,\\tau)$ is the maximum order of the edges of $T$, and the \\emph{branch-width} $\\beta(G)$ of $G$ is the minimum width of all branch-decompositions of $G$ (or 0 if $|E(G)| \\le 1$, when $G$ has no branch-decompositions).")
        // .showed("FN4FJJ", Pp(12), "(4.1) If $H$ is a minor of a graph $G$, then $\\beta(H) \\le \\beta(G)$.")
        .collective(Pp(16), "(5.1) For any hypergraph $G$, $\\max(\\beta(G), \\gamma(G)) \\le \\omega(G) + 1 \\le \\max(\\lfloor(3/2)\\beta(G)\\rfloor, \\gamma(G), 1)$.")
            .showed("8ewSpI", &treewidth, &branch_width, UpperBound(Linear))
            .showed("wrBAYk", &branch_width, &treewidth, UpperBound(Linear))
            .done()
        .done(&mut create);
    let bodlaender_mohring1993 = source("a3yKzk", "BodlaenderMohring1993", 5)
        .showed("cIAr80", Pp(4), &complete, &treewidth, Exclusion, "Lemma 3.1 (\"clique containment lemma\"). Let $(\\{X_i\\mid u\\in I\\},T=(I,F))$ be a tree-decomposition of $G=(V,E)$ and let $W \\subseteq V$ be a clique in $G$. Then there exists $i \\in I$ with $W \\subseteq X_i$.")
        .showed("mIvbmU", Pp(4), &bipartite, &treewidth, Exclusion, "Lemma 3.2 (\"complete bipartite subgraph containment lemma\").")
        // .showed("LDCZyj", Pp(5), &create.intersection("NxM8Gc", &cograph, &treewidth, ""), &create.intersection("chwMbI", &cograph, &pathwidth, ""), Equal, "Theorem 3.2. For every cograph $G = (V,E)$, $treewidth(G) = pathwidth(G)$.")
        // .showed(Theorem 4.1. The pathwidth and treewidth of a cograph given with a corresponding cotree can be computed in $O(n)$ time.)
        .done(&mut create);
    // clique-width ideas in 'Handle-Rewriting Hypergraph Grammars'
    let wanke1994 = source("SQjcYg", "Wanke1994", 3)
        .defined("ENvDZb", Pp(3), &nlc_width, "Definition 2.1. Let $k \\in \\mathbb N$ be a positive integer. A \\emph{$k$-node label controlled (NLC) graph} is a $k$-NL graph defined as follows: ...")
        .defined("yNzt7o", Pp(4), &nlct_width, "Definition 2.2. Let $k \\in \\mathbb N$ be a positive integer. A \\emph{$k$-node label controlled (NLC) tree} is a $k$-NL graph defined as follows: ...")
        .showed("jBcoBD", Pp(5), &cograph, &nlc_width, UpperBound(Constant), "Fact 2.3. $G$ is a $1$-NLC graph if and only if $unlab(G)$ is a co-graph.")
        .showed("cXI1DK", Pp(6), &treewidth, &nlc_width, UpperBound(Exponential), "Theorem 2.5. For each partial $k$-tree $G$ there is a $(2^{k+1}-1)$-NLC tree $J$ with $G=unlab(J)$.")
        .done(&mut create);
    let bodlaender1998 = source("BOFCWg", "Bodlaender1998", 6)
        .showed("uHJAUo", Pp(4), &pathwidth, &treewidth, UpperBound(Linear), "Lemma 3. (a) For all graphs $G$, $pathwidth(G) \\ge treewidth(G)$. ...")
        .defined("oGAdW1", Pp(5), &branch_width, "A \\emph{branch decomposition} of a graph $G=(V,E)$ is a pair $(T=(I,F),\\sigma)$, where $T$ is a tree with every node in $T$ of degree one of three, and $\\sigma$ is a bijection from $E$ to the set of leaves in $T$. The \\emph{order} of an edge $f \\in F$ is the number of vertices $v \\in V$, for which there exist adjacent edges $(v,w),(v,x) \\in E$, such that the path in $T$ from $\\sigma(v,w)$ to $\\sigma(v,x)$ uses $f$. The \\emph{width} of branch decomposition $(T=(I,F),\\sigma)$, is the maximum order over all edges $f \\in F$. The \\emph{branchwidth} of $G$ is the minimum width over all branch decompositions of $G$.")
        // page 6, tw and pw do not rise for subgraphs
        // mark
        .defined("hajrD0", Pp(22), &bandwidth, "Let $G=(V,E)$ be a graph, and let $f\\colon V\\to \\{1,2,\\dots,n\\}$ be a linear ordering of $G$. 1. The \\emph{bandwidth} of $f$ is $\\max\\{|f(v)-f(w)| \\mid (v,w) \\in E\\}$. ... The bandwidth ... is the minimum bandwidth ... over all possible linear orderings of $G$.")
        .defined("c6Hdu3", Pp(22), &cutwidth, "Let $G=(V,E)$ be a graph, and let $f\\colon V\\to \\{1,2,\\dots,n\\}$ be a linear ordering of $G$. ... 2. The \\emph{cutwidth} of $f$ is $\\max_{1\\le i\\le n} |\\{(u,v)\\in E \\mid f(u) \\le i < f(v) \\}|$. ... [cutwidth] of a graph $G$ is the minimum [cutwidth] ... over all possible linear orderings of $G$.")
        .defined("H3lAh2", Pp(22), &topological_bandwidth, "The \\emph{topological bandwidth} of a graph $G$ is the minimum [bandwidth](../aP5a38) over all graphs $G'$ which are obtained by addition of an arbitrary number of vertices along edges of $G$.")
        .showed("kiza4J", Pp(23), &bandwidth, &pathwidth, UpperBound(Linear), "Theorem 44. For every graph $G$, the pathwidth of $G$ is at most the bandwidth of $G$, ... Proof. Let $f \\colon V\\to \\{1,\\dots,n\\}$ be a linear ordering of $G$ with bandwidth $k$. Then $(X_1,\\dots,X_{n-k})$ with $X_i=\\{f^{-1}(i), f^{-1}(i+1), \\dots, f^{-1}(i+k)\\}$ is a path decomposition of $G$ with pathwidth $k$. ...")
        .showed("RQriva", Pp(23), &topological_bandwidth, &pathwidth, UpperBound(Linear), "Theorem 45. For every graph $G$, the pathwidth of $G$ is at most the topological band-width of $G$.")
        .showed("iiE5jo", Pp(24), &cutwidth, &pathwidth, UpperBound(Linear), "Theorem 47. For every graph $G$, the pathwidth of $G$ is at most the cutwidth of $G$.")
        .collective(Pp(24), "Theorem 49.")
            .showed("RgLQ2P", &degree_pathwidth, &cutwidth, UpperBound(Linear))
            .showed("Bq4H8y", &cutwidth, &degree_pathwidth, UpperBound(Linear))
            .done()
        .showed("VdNTHZ", Pp(34), &outerplanar, &treewidth, UpperBound(Constant), "Lemma 78. Every outerplanar graph $G=(V,E)$ has treewidth at most 2.") // check whether dist_to_outerplanar bounding treewidth infered from this?
        .showed("oFitZo", Pp(37), &grid, &treewidth, Exclusion, "Lemma 88. The treewidth of an $n \\times n$ grid graph ... is at least $n$.")
        .showed("KoFslx", Pp(38), &treewidth, &min_degree, UpperBound(Linear), "Lemma 90 (Scheffler [94]). Every graph of treewidth at most $k$ contains a vertex of degree at most $k$.") // todo Schemer, Die Baumweite von Graphen als ein Ma8 Rir die Kompliziertheit algorithmischer Probleme, Ph.D. Thesis, Akademie der Wissenschafien der DDR, Berlin, 1989.
        .done(&mut create);
    let johansson1998 = source("W2nwG4", "Johansson1998", 3) // according to Gurski2005
        .showed("DBXQMa", Unknown, &clique_width, &nlc_width, UpperBound(Linear), "")
        .showed("BjlRwP", Unknown, &nlc_width, &clique_width, UpperBound(Linear), "")
        .showed("qy5Ojn", Unknown, &linear_clique_width, &linear_nlc_width, UpperBound(Linear), "")
        .showed("hI8Txh", Unknown, &linear_nlc_width, &linear_clique_width, UpperBound(Linear), "")
        .todo_rest(&mut create);
    // let domino_treewidth1999 = source("gcMYuX", "dominoTreewidth1999", 4)
        // .showed("vXf5Ie", Pp(4), &degree_treewidth, &domino_treewidth, UpperBound(Polynomial), "Theorem 3.1 Let $G=(V,E)$ be a graph with treewidth at most $k$ and maximum degree at most $d$. Then the domino treewidth of $G$ is at most $(9k+7)d(d+1)-1$.")
        // .showed("KRgpco", Pp(6), &degree_treewidth, &domino_treewidth, LowerBound(Polynomial), "Lemma 4.3 For all $d \\ge 5$, $k \\ge 2$, $k$ even, there exists a graph $G$ with treewidth at most $k$, maximum degree at most $d$, and domino treewidth at least $\\frac{1}{12} kd-2$.")
        // .todo_rest(&mut create);
    let courcelle_olariu_2000 = source("ZQrXS8", "courcelle2000", 5)
        // .defined("OL0McK", Unknown, &clique_width, "")
        .showed("sGBrPC", Pp(18), &treewidth, &clique_width, UpperBound(Exponential), "We will prove that for every undirected graph $G$, $cwd(G) \\le 2^{twd(G)+1}+1$ ...")
        .done(&mut create);
    let tack_layouts2004 = source("w7RVn9", "TackLayouts2004", 3)
        // .defined("bcdAXe", Pp(2), &track_number, "The track-number of $G$ is $\\mathrm{tn}_1(G)$, ...")
        // .showed("ZXhXax", Pp(12), &track_number, &acyclic_chromatic_number, UpperBound(Exponential), "Corollary 3. Acyclic chromatic number is bounded by track-number. In particular, every $(k,t)$-track graph $G$ has acyclic chromatic number $\\chi_a(G) \\le t \\cdot 4^{\\binom k2(t_1)}$.")
        .showed("v1Ygyr", Pp(14), &book_thickness, &acyclic_chromatic_number, UpperBound(Exponential), "Theorem 5. Acyclic chromatic number is bounded by stack-number (ed: a.k.a. book-thickness). In particular, every $k$-stack graph $G$ has acyclich chromatic number $\\chi_a(G) \\le 80^{k(2k-1)}$.")
        .done(&mut create);
    // let corneil2005 = source("HCGunF", "Corneil2005")
    // .showed("sGBrPC", Unknown, &treewidth, &clique_width, Exactly(Exponential), "... the clique-width of $G$ is at most $3 \\cdot 2k - 1$ and, more importantly, that there is an exponential lower bound on this relationship. In particular, for any $k$, there is a graph $G$ with treewidth equal to $k$, where the clique-width of $G$ is at least $2\\lfloor k/2\\rfloor-1$.")
    // .todo_rest(&mut create);
    let gurski2005 = source("FLSQsw", "Gurski2005", 3)
        .defined("umo10J", Pp(4), &linear_nlc_width, "Definition 3")
        .defined("ZPOCMc", Pp(4), &clique_tree_width, "Definition 5")
        .defined("q9qg89", Pp(5), &linear_clique_width, "Definition 6") // as noted in footnote of 10.1016/j.jctb.2007.04.001
        .showed("lY4S8K", Pp(8), &linear_nlc_width, &nlct_width, UpperBound(Linear), "")
        .showed("QBpUMV", Pp(8), &nlct_width, &nlc_width, UpperBound(Linear), "")
        .showed("CwlGA8", Pp(8), &linear_clique_width, &clique_tree_width, UpperBound(Linear), "")
        .showed("pY3u9l", Pp(8), &clique_tree_width, &clique_width, UpperBound(Linear), "")
        .showed("hxsxob", Pp(8), &clique_tree_width, &nlct_width, UpperBound(Linear), "")
        .showed("JXeEwu", Pp(8), &nlct_width, &clique_tree_width, UpperBound(Linear), "")
        .collective(Pp(8), "The results of [23] imply that each graph class of bounded path-width has bounded linear NLC-width and that each graph class of bounded tree-width has bounded NLCT-width.")
            .showed("mwTHcM", &pathwidth, &linear_nlc_width, UpperBound(Exists))
            .showed("BELFKR", &treewidth, &nlct_width, UpperBound(Exists))
            .done()
        // .showed("3udN1G", Pp(8), &treewidth, &nlct_width, UpperBound(Linear), "")
        .done(&mut create);
    let oum2006 = source("1ZTWBd", "Oum2006", 4)
        .defined("SGJJ1Y", Pp(9), &rank_width, "... and the \\emph{rank-width} $\\mathrm{rwd}(G)$ of $G$ is the branch-width of $\\mathrm{cutrk}_G$.")
        .showed("yLdAHe", Pp(9), &rank_width, &clique_width, Exactly(Exponential), "Proposition 6.3")
        .showed("uEUXMq", Pp(9), &clique_width, &rank_width, UpperBound(Linear), "Proposition 6.3")
        .done(&mut create);
    let geometric_thickness2007 = source("2q7m9E", "GeometricThickness2007", 4)
        // .defined("3p2P4E", Pp(3), &thickness, "The thickness of a graph $G$, ..., is the minimum number of planar subgraphs that partition (ed: edges of) $G$.") // defined by Tutte 1963
        // .defined("j9NrW9", Pp(3), &outerthickness, "The outerthickness of a graph $G$, ..., is the minimum number of outerplanar subgraphs that partition (ed: edges of) $G$.")
        // .showed("0B1cGr", Pp(4), &treewidth, &thickness, UpperBound(Linear), "Proposition 1. The maximum thickness of a graph in $\\mathcal T_k$ (ed: $k$-tree) is $\\lceil k/2 \\rceil$; ...")
        .showed("3zMwH9", Pp(5), &treewidth, &arboricity, UpperBound(Linear), "Proposition 2. The maximum arboricity of a graph in $\\mathcal T_k$ (ed: $k$-tree) is $k$; ...")
        // .showed("hXbdpU", Pp(5), &treewidth, &outerthickness, UpperBound(Linear), "Proposition 3. The maximum outerthickness of a graph in $\\mathcal T_k$ (ed: $k$-tree) is $k$; ...")
        // .showed("EKKZPJ", Pp(6), &treewidth, &star_arboricity, UpperBound(Linear), "Proposition 4. The maximum star-arboricity of a graph in $\\mathcal T_k$ (ed: $k$-tree) is $k+1$; ...")
        .defined("IxPMGS", Pp(7), &book_thickness, "A geometric drawing in which the vertices are in convex position is called a book embedding. The book thickness of a graph $G$, ..., is the minimum $k \\in \\mathbb N$ such that there is book embedding of $G$ with thickness $k$.")
        .showed("FY0U1r", Pp(8), &treewidth, &book_thickness, UpperBound(Linear), "The maximum book thickness ... of a graph $\\mathcal T_k$ (ed: $k$-tree) satisfy ... $=k$ for $k \\le 2$, $=k+1$ for $k \\ge 3$.")
        .todo_rest(&mut create);
    let contraction_complexity2008 = source("9JAQC7", "contractionComplexity2008", 2)
        .defined("4lmvZK", Pp(10), &contraction_complexity, "Definition 4.1. The contraction of an edge e removes e and replaces its end vertices (or vertex) with a single vertex. A contraction ordering π is an ordering of all the edges of G, π(1), π(2), . . ., π(|E(G)|). The complexity of π is the maximum degree of a merged vertex during the contraction process. The contraction complexity of G, denoted by cc(G), is the minimum complexity of a contraction ordering.")
        .showed("F0p61H", Pp(10), &contraction_complexity, &max_degree, UpperBound(Linear), "$cc(G) \\ge \\Delta(G) - 1$")
        .collective(Pp(11), "Proposition 4.2. ... $cc(G)=tw(G^*)$ ... Lemma 4.4. $(tw(G) - 1)/2 \\le tw(G^*) \\le \\Delta(G)(tw(G) + 1) - 1.$")
            .showed("YhbKPB", &contraction_complexity, &treewidth, UpperBound(Linear))
            .showed("YvvmJE", &degree_treewidth, &contraction_complexity, UpperBound(Polynomial))
            .done()
        .done(&mut create);
    let delavina_waller2008 = source("C5cBsd", "spanningTreesManyLeaves2008", 2)
        .showed("Pbg2ga", Pp(5), &average_distance, &bipartite_number, UpperBound(Linear), "Theorem 9 (Main Theorem). Let $G$ be a graph. Then $\\bar{D} < \\frac b2 + \\frac 12$. ...")
        // .showed("ZXINaY", Unknown, &max_leaf_num, &feedback_vertex_set, UpperBound(Linear), "")
        .done(&mut create);
    let gradnesetril2008 = source("kXDDmb", "gradnesetril2008", 3)
        .showed("VLpzhW", Unknown, &d_path_free, &treedepth, UpperBound(Polynomial), "") // todo
        .showed("Q7qpEp", Unknown, &treedepth, &d_path_free, UpperBound(Exponential), "") // todo
        // d_path_free
        .todo_rest(&mut create);
    let cliquewidthnpc2009 = source("zuhSo5", "cliquewidthnpc2009", 2)
        .showed("i1eBMN", Pp(8), &pathwidth, &linear_clique_width, UpperBound(Linear), "(5) $\\mathrm{lin-cwd}(G) \\le \\mathrm{pwd}(G)+2$.")
        .todo_rest(&mut create);
    let jelinek2010 = source("vIBI5v", "Jelinek2010", 1)
        .showed("ipo6rm", Pp(2), &grid, &rank_width, Exclusion, "The grid $G_{n,n}$ has rank-width equal to $n-1$.")
        .done(&mut create);
    let sasak2010 = source("XlBXyo", "Sasak2010", 7)
        .showed("Lp8I7N", Pp(16), &tree, &pathwidth, Exclusion, "Theorem 2.1") // cites excludingforest1991, cannot find there
        .showed("BtRWId", Pp(17), &complete, &treewidth, Exclusion, "Theorem 2.2")
        .showed("GN6dar", Pp(17), &grid, &treewidth, Exclusion, "Theorem 2.4") // cites Csaba Biró. Tree-width and grids, cannot find the paper
        // .cited("kXDDmb", Pp(21), jelinek2010, "Theorem 2.14 [12] Rank-width of a grid $\\sqrt{n} \\times \\sqrt{n}$ on $n$ vertices is $\\sqrt{n}-1$.")
        // Theorem 2.15 [4] Boolean-width of a grid √n×√n on n vertices lies between
        .showed("BwIc79", Pp(28), &cutwidth, &max_degree, UpperBound(Linear), "Lemma 2.18. For any graph $G$ and any vertex $v \\in V(G), cutw(g) \\ge \\lceil \\frac{deg(v)}2 \\rceil$.")
        .showed("h49cUu", Pp(30), &carving_width, &max_degree, UpperBound(Linear), "Lemma 2.20 Carving-width of a graph $G$ is at least $\\Delta(G)$ where $\\Delta(G)$ is the maximum degree of a graph $G$.")
        .showed("2Wk4AF", Pp(30), &star, &carving_width, Exclusion, "Corollary 2.21 Carving-width of a star is $n-1$.")
        .showed("6Ln8ux", Pp(30), &path, &carving_width, UpperBound(Constant), "... path with carving-width 2.")
        // .cited("7jmzab", Pp(32), &gradnesetril2008, "Theorem 2.23 [13] Let $l$ be the length of the longest path in a graph $G$. Then the tree-depth of $G$ is bounded as follows: $\\lceil \\log_2(l+2)\\rceil \\le td(G) \\le \\binom{l+3}2-1$ ...")
        .showed("MboUFT", Pp(32), &grid, &treedepth, Exclusion, "Corollary 2.24 Tree-depth of a grid is at least $\\lceil \\log_2(n+1)\\rceil$.")
        .showed("WiiQn4", Pp(38), &cutwidth, &carving_width, UpperBound(Linear), "Theorem 4.3 (carw $\\prec$ cutw) Carving-width is bounded by cut-width.")
        .showed("u5VPeX", Pp(49), &carving_width, &treewidth, UpperBound(Linear), "Theorem 5.5 (tw $\\prec$ carw) Tree-width is bounded by carving-width.")
        .todo_rest(&mut create);
    let bipboxicity2011 = source("Vkc4EU", "bipboxicity2011", 2)
        .showed("Yelk6V", Pp(9), &dist_to_bipartite, &boxicity, Exclusion, "Theorem 2 For any $b \\in \\mathbb N^+$, there exists a chordal bipartite graph $G$ (ed: i.e. bipartite graph with no induced cycle on more than 4 vertices) with $\\mathrm{box}(G) > b$.")
        .done(&mut create);
    let bui_xuan2011 = source("cNjhWx", "BuiXuan2011", 4)
        .defined("L7aY6D", Unknown, &boolean_width, "\\textbf{Definition 1.} A decomposition tree of a graph $G$ is a pair $(T,\\delta)$ where $T$ is a tree having internal nodes of degree three and $\\delta$ a bijection between the leaf set of $T$ and the vertex set of $G$. Removing an edge from $T$ results in two subtrees, and in a cut $\\{A,\\overline{A}\\}$ of $G$ given by the two subsets of $V(G)$ in bijection $\\delta$ with the leaves of the two subtrees. Let $f\\colon w^V \\to \\mathbb{R}$ be a symmetric function that is also called a cut function: $f(A)=f(\\overline{A})$ for all $A\\subseteq V(G)$. The $f$-width of $(T,\\delta)$ is the maximum value of $f(A)$ over all cuts $\\{A,\\overline{A}\\}$ of $G$ given by the removal of an edge of $T$. ... \\textbf{Definition 2.} Let $G$ be a graph and $A \\subseteq V(G)$. Define the set of unions of neighborhoods of $A$ across the cut $\\{A,\\overline{A}\\}$ as $U(A) = \\{Y \\subseteq \\overline{A} \\mid \\exists X \\subseteq A \\land Y=N(X)\\cap \\overline{A}\\}$. The \\emph{bool-dim}$\\colon 2^{V(G)} \\to \\mathbb{R}$ function of a graph $G$ is defined as $\\mathrm{bool-dim}(A)=\\log_2 |U(A)|$. Using Definition 1 with $f=\\mathrm{bool-dim}$ we define the boolean-width of a decomposition tree, denoted by $boolw(T,\\delta)$, and the boolean-width of a graph, denoted by $boolw(G)$.")
        .showed("AdNkCy", Unknown, &boolean_width, &rank_width, UpperBound(Exponential), "\\textbf{Corollary 1.} For any graph $G$ and decomposition tree $(T,\\gamma)$ of $G$ it holds that ... $\\log_2 rw(G) \\le boolw(G)$ ...")
        .showed("cIWQDn", Unknown, &rank_width, &boolean_width, UpperBound(Polynomial), "\\textbf{Corollary 1.} For any graph $G$ and decomposition tree $(T,\\gamma)$ of $G$ it holds that ... $boolw(G) \\le \\frac 14 rw^2(G)+O(rw(G))$.")
        .todo_rest(&mut create);
    let lampis2012 = source("0LYUEV", "lampis2012", 1)
        .defined("ljbw1n", NotApplicable, &neighborhood_diversity, "We will say that two vertices $v, v'$ of a graph $G(V, E)$ have the same *type* iff they have the same colors and $N(v) \\setminus \\{v\\}=N(v') \\setminus \\{v\\}$, where $N(v)$ denotes the set of neighbors of $v$. ... A colored graph $G(V, E)$ has neighborhood diversity at most $w$, if there exists a partition of $V$ into at most $w$ sets, such that all the vertices in each set have the same type.")
        .todo_rest(&mut create);
    let ganian_twin_cover2012 = source("7UoBR6", "GanianTwinCover2012", 4)
        .defined("k6ApS2", Pp(262), &twin_cover_num, "Definition 3.1. $X \\subseteq V(G)$ is a twin-cover of $G$ if for every edge $e=\\{a,b\\} \\in E(G)$ either 1. $a \\in X$ or $b \\in X$, or 2. $a$ and $b$ are twins, i.e. all other vertices are either adjacent to both $a$ and $b$ or none. We then say that $G$ has twin-cover number $k$ if $k$ is the minimum possible size of a twin-cover of $G$.")
        .defined("pFk5uY", Pp(262), &twin_cover_num, "Definition 3.2. $X \\subseteq V(G)$ is a twin-cover of $G$ if there exists a subgraph $G'$ of $G$ such that 1. $X \\subseteq V(G')$ and $X$ is a vertex cover of $G'$. 2. $G$ can be obtained by iteratively adding twins to non-cover vertices in $G'$.")
        .showed("oxtaEE", Pp(263), &complete, &twin_cover_num, UpperBound(Constant), "We note that complete graphs indeed have a twin-cover of zero.")
        .showed("nkOAMh", Pp(263), &twin_cover_num, &vertex_cover, Exclusion, "The vertex cover of graphs of bounded twin-cover may be arbitrarily large.")
        .collective(Pp(263), "There exists graphs with arbitrarily large twin-cover and bounded tree-width and vice-versa.")
            .showed("gmsOd4", &twin_cover_num, &treewidth, Exclusion)
            .showed("iG3eGq", &treewidth, &twin_cover_num, Exclusion)
            .done()
        .showed("E8oHKm", Pp(263), &twin_cover_num, &clique_width, UpperBound(Linear), "The clique-width of graphs of twin-cover $k$ is at most $k+2$.")
        .showed("qB058E", Pp(263), &twin_cover_num, &rank_width, UpperBound(Linear), "The rank-width and linaer rank-width of graph of twin-cover $k$ are at most $k+1$.")
        .showed("WZcIOW", Pp(263), &twin_cover_num, &linear_rank_width, UpperBound(Linear), "The rank-width and linaer rank-width of graph of twin-cover $k$ are at most $k+1$.")
        // .tractable("XwMEnS", Pp(263), &twin_cover_num, &twin_cover_num, FPT, "Theorem 3.4. It is possible to find a twin-cover of size $k$ in time $O(|E||V|+k|V|+1.2738^k)$.")
        // .tractable("PxaiDG", Pp(267), &twin_cover_num, &boxicity, FPT, "Theorem 4.6. The Boxicity
        // problem can be solved in time $2^{O(2^kk^2)}|V|$ on graph of twin-cover at most $k$.")
        .done(&mut create);
    let vatshelle2012 = source("nRO7AG", "Vatshelle2012", 3)
        .defined("Usp3Ca", Pp(33), &mim_width, "Definition 3.7.1 (MIM-width). For $G$ a graph and $A \\subseteq V(G)$ let $mim \\colon 2^{V(G)} \\to \\mathbb N$ be a function where $mim(A)$ is the size of a maximum induced matching in $G[A,\\bar A]$. Using Definition 3.1.3 with $f=mim$ we define $mimw(T,\\delta$ as the $f$-width of a binary decomposition tree $(T,\\delta)$ and $mimw(G)$ as the $f$-width of $G$, also called the MIM-width of $G$ or the maximum induced matching width.")
        .showed("77zJ2z", Pp(42), &boolean_width, &mim_width, UpperBound(Linear), "Theorem 4.2.10. Let $G$ be a graph, then $mimw(G) \\le boolw(G) \\le mimw(G) \\log_2(n)$")
        .todo_rest(&mut create);
    let modularwidth2013 = source("OH3sI3", "modularwidth2013", 3)
        .showed("NeMJtU", Pp(6), &neighborhood_diversity, &modular_width, StrictUpperBound(Linear), "Theorem 3. Let $G$ be a graph. Then $\\mathrm{mw}(G) \\le \\mathrm{nd}(G)$ and $\\mathrm{mw}(G) \\le 2\\mathrm{tc}(G) + \\mathrm{tc}(G)$. Furthermore, both inequalities are strict, ...")
        .showed("8rtBjc", Pp(6), &twin_cover_num, &modular_width, StrictUpperBound(Exponential), "Theorem 3. Let $G$ be a graph. Then $\\mathrm{mw}(G) \\le \\mathrm{nd}(G)$ and $\\mathrm{mw}(G) \\le 2\\mathrm{tc}(G) + \\mathrm{tc}(G)$. Furthermore, both inequalities are strict, ...")
        .showed("ppKqXp", Pp(8), &modular_width, &shrub_depth, Incomparable, "Theorem 4. There are classes of graphs with unbounded modular-width and bounded shrub-depth and vice versa.")
        .todo_rest(&mut create);
    let belmonte2013 = source("sJ476m", "Belmonte2013", 1)
        .showed("ZHXKjC", Unknown, &carving_width, &max_degree, UpperBound(Linear), "Observation 1. Let $G$ be a graph. Then $cw(G) \\ge \\Delta(G)$.")
        .todo_rest(&mut create);
    let jansen2013 = source("FLOjic", "Jansen2013", 1)
        .hasse("u6oAPX", Pp(46), &vec!["4lp9Yj", "BN92vX", "2LDMQ6", "yk7XP0", "TLx1pz", "aP5a38", "SnA7Eq", "GNOiyB", "OdZQna", "lPHVWU", "VHClqR", "Ve5ruW", "5Q7fuR", "gbaHdw", "kJZKgd", "w7MmyW"])
        .defined("PV6tGG", Unknown, &topological_bandwidth, "The \\emph{topological bandwidth} of a graph $G$ is the minimum [bandwidth](../aP5a38) over all subdivisions of $G$")
        .todo_rest(&mut create);
    let adler2015 = source("rhj9my", "Adler2015", 2)
        .collective(Pp(1),  "Linear rank-width is equivalent to linear clique-width in the sense that any graph class has bounded linear clique-width if and only if it has bounded linear rank-width.")
            .showed("3yUfrd", &linear_rank_width, &linear_clique_width, UpperBound(Exists))
            .showed("2dN9wh", &linear_clique_width, &linear_rank_width, UpperBound(Exists))
            .done()
        .showed("dvqfqQ", Pp(3), &pathwidth, &linear_rank_width, UpperBound(Linear), "Lemma 5. Any graph $G$ satisfies $\\mathrm{lrw}(G) \\le \\mathrm{pw}(G)$.")
        .todo_rest(&mut create);
    let twin_cover_2015 = source("VQLE2i", "ganianTwinCover2015", 4)
        .defined("J1sHj8", Pp(5), &twin_cover_num, "Definition 3 A set of vertices $X \\subseteq V(G)$ is a twin-cover of $G$ if for every edge $e = ab \\in E(G)$ either 1. $a \\in X$ or $b \\in X$, or 2. $a$ and $b$ are true twins. We then say that $G$ has twin-cover $k$ if the size of a minimum twin-cover of $G$ is $k$.")
        .showed("OoSnHu", Pp(20), &twin_cover_num, &shrub_depth, UpperBound(Constant), "Let $\\mathcal H_k$ be the class of graphs of twin-cover $k$. Then $\\mathcal H_k \\subseteq \\mathcal{TM}_{2^k+k}(2)$ and a tree-model of any $G \\in \\mathcal H_k$ may be constructed in single-exponential FPT time.")
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
        .showed("TUftFh", Unknown, &shrub_depth, &linear_clique_width, UpperBound(Linear), "Proposition 3.4. Let $\\mathcal G$ be a graph class and $d$ an integer. Then: ... b) If $\\mathcal G$ is of bounded shrub-depth, then $\\mathcal G$ is of bounded linear clique-width.")
        .showed("EG7vp6", Unknown, &neighborhood_diversity, &shrub_depth, UpperBound(Constant), "$\\mathcal{TM}_m(1)$ is exactly the class of graphs of neighborhood diversity at most $m$.")
        .showed("sq0brL", Unknown, &treedepth, &shrub_depth, UpperBound(Linear), "Proposition 3.2. If $G$ is of tree-depth $d$, then $G \\in \\mathcal{TM}_{2^d}(d)$. ...")
        .todo_rest(&mut create);
    let sorge2019 = source("VnTIL0", "Sorge2019", 7)
        .hasse("Im1xnN", Pp(2), &vec!["2LDMQ6", "4lp9Yj", "BN92vX", "aP5a38", "UyQ5yM", "wUdmUb", "gbaHdw", "HTk9PZ", "KEP2qM", "VHClqR", "5Q7fuR", "lPHVWU", "GNOiyB", "yk7XP0", "aXw3Co", "AVc2K6", "OdZQna", "hbfWwE", "uDXX2i", "VomShB", "mHtXUU", "Gq0onN", "p4bTjp", "zH8PpT", "BCwUeT", "kJZKgd", "1yW82F", "wg5HuV", "fTqo40", "a7MpiT", "QGZuUW", "VowkuW", "w7MmyW", "q7zHeT", "z0y4TW", "GPmOeT", "KRV6tI", "ZL7BOP", "GNTwUS"])
        .defined("ddviDI", Pp(3), &acyclic_chromatic_number, "The \\emph{acyclic chromatic number} of a graph $G = (V,E)$ is the smallest size of a vertex partition $P=\\{V_1,\\dots,V_\\ell\\}$ such that each $V_i$ is an independent set and for all $V_i,V_j$ the graph $G[V_i \\cup V_j]$ does not contain a cycle.")
        // .cited("KrMa3o", Pp(3), &grunbaum1973, "Introduced by Grünbaum [18]")
        // .defined("aUvKTa", Pp(3), &path_number, "The \\emph{path number} of a graph $G$ is the minimum number of paths the edges of $G$ can be partitioned into [2].")
        .defined("EedT02", Pp(3), &arboricity, "The \\emph{arboricity} of a graph $G$ is the minimum number of forests the edges of $G$ can be partitioned into.") // todo covering by trees
        // .defined("iUqFsf", Pp(3), &vertex_arboricity, "The \\emph{vertex arboricity} (or ``point arboricity'') of a graph $G$ is the minimum number of vertex subsets $V_i$ of $G$ such that $G[V_i]$ induces a forest for each $i$. ... [2]")
        .defined("77Klw8", Pp(3), &average_degree, "The \\emph{average degree} of a graph $G = (V,E)$ is $2|E|/|V|$.")
        .showed("LBLgZG", Pp(8), &arboricity, &degeneracy, UpperBound(Linear), "Lemma 4.5")
        .showed("RWlDuy", Pp(8), &degeneracy, &arboricity, UpperBound(Linear), "Lemma 4.5")
        .showed("mNvMUr", Pp(8), &max_degree, &acyclic_chromatic_number, UpperBound(Polynomial), "Lemma 4.6 ([15]). The acyclic chromatic number $\\chi_a$ is uppre bounded by the maximum degree $\\Delta$ (for every graph with $\\Delta > 4$). We have $\\chi_a \\le \\Delta(\\Delta-1)/2$.")
        .showed("thTXGX", Pp(8), &hindex, &acyclic_chromatic_number, UpperBound(Polynomial), "Lemma 4.7. The acyclic chromatic number $\\chi_a$ is upper bounded by the $h$-index $h$. We have $\\chi_a \\le h(h+1)/2$.")
        .showed("8X0RWp", Pp(8), &genus, &acyclic_chromatic_number, UpperBound(Linear), "Lemma 4.8 ([3]). The accylic chromatic number $\\chi_a$ is upper bounded by the genus $g$. We have $\\chi_a \\le 4g+4$.") // cites
        .showed("3z6jS1", Pp(8), &acyclic_chromatic_number, &boxicity, UpperBound(Polynomial), "Lemma 4.9. The boxicity $b$ is upper bounded by the acyclic chromatic number $\\chi_a$ (for every graph with $\\chi_a>1$). We have $b \\le \\chi_a(\\chi_a-1)$.") // cites
        .showed("WhwImQ", Pp(8), &max_leaf_num, &dist_to_linear_forest, UpperBound(Linear), "Lemma 4.10 ([14]). The max-leaf number $\\mathrm{ml}$ upper bounds the distance to disjoint paths $d$. We have $d \\le \\mathrm{ml}-1$.") // cites
        .showed("GCGHcz", Pp(9), &boxicity, &chordality, UpperBound(Linear), "Lemma 4.15 ([8,11]). The boxicity $b$ upper-bounds the chordality $c$. We have $c \\le b$.")
        .showed("1xFA4j", Pp(9), &dist_to_interval, &boxicity, UpperBound(Linear), "Lemma 4.16. The distance $i$ to an interval graph upper bounds the boxicity $b$. We have $b \\le i+1$.")
        .showed("16XW6a", Pp(9), &dist_to_chordal, &chordality, UpperBound(Linear), "(ed: apparently goes as the lemma for ddist to interval and boxicity) Lemma 4.16. The distance $i$ to an interval graph upper bounds the boxicity $b$. We have $b \\le i+1$.")
        .showed("OV1KKK", Pp(9), &dist_to_cograph, &clique_width, UpperBound(Exponential), "Lemma 4.17. The distance $c$ to a cograph upper bounds the cliquewidth $q$. We have $q \\le 2^{3+c}-1$.")
        .showed("rVEmFt", Pp(9), &acyclic_chromatic_number, &degeneracy, UpperBound(Polynomial), "Lemma 4.18. The acyclic chromatic number $a$ upper bounds the degeneracy $d$. We have $d \\le 2 \\binom a2 - 1$")
        .showed("mwNELs", Pp(10), &feedback_edge_set, &genus, UpperBound(Linear), "Lemma 4.19. The feedback edge set number $f$ upper bounds the genus $g$. We have $g \\le f$.")
        .showed("50WEZP", Pp(10), &feedback_vertex_set, &dist_to_chordal, UpperBound(Linear), "Lemma 4.20. The feedback edge set number $f$ upper bounds the distance to a chordal graph $c$. We have $c \\le f$.")
        .showed("ghgPz2", Pp(10), &max_leaf_num, &bandwidth, UpperBound(Linear), "Lemma 4.25. The max leaf number $\\mathrm{ml}$ strictly upper bounds the bandwidth $\\mathrm{bw}$.")
        .showed("h8nG9p", Pp(11), &clique_cover_num, &max_independent_set, UpperBound(Linear), "Lemma 4.26. The minimum clique cover number $c$ strictly upper bounds the independence number $\\alpha$.")
        .showed("q3qJkr", Pp(11), &treedepth, &pathwidth, UpperBound(Linear), "Lemma 4.27. The treedepth $t$ strictly upper bounds the pathwidth $p$. We have $p \\le t$.")
        .todo_rest(&mut create); // page 10
    let mimwidth2020 = source("BIQh3r", "mimwidth2020", 1)
        .todo_rest(&mut create);
    let schroder_parameter_list = vec![
        "2LDMQ6", "4lp9Yj", "BN92vX", "VomShB", "hbfWwE", "aXw3Co", "yk7XP0", "HTk9PZ", "aP5a38",
        "mHtXUU", "uDXX2i", "AVc2K6", "GNOiyB", "KEP2qM", "UyQ5yM", "Gq0onN", "lPHVWU", "VHClqR",
        "gbaHdw", "GNTwUS", "p4bTjp", "MLJMRH", "5Q7fuR", "zH8PpT", "OdZQna", "QGZuUW", "BCwUeT",
        "wg5HuV", "VowkuW", "1yW82F", "a7MpiT", "z0y4TW", "kJZKgd", "w7MmyW", "GPmOeT", "q7zHeT",
        "fTqo40", "KRV6tI", "ZL7BOP", "wUdmUb",
    ];
    let schroder_thesis = source("DYGiYb", "SchroderThesis", 7)
        // .cited("pJxHVS", Unknown, sorge2019, "Based on the work by [Sa19] as well as [Fr8], we investigate unknown connections between graph parameters to continue the work on the graph parameter hierarchy")
        // .cited("bybFgo", Unknown, froemmrich2018, "Based on the work by [Sa19] as well as [Fr8], we investigate unknown connections between graph parameters to continue the work on the graph parameter hierarchy")
        .hasse("DfHlFn", Pp(7), &schroder_parameter_list)
        .table("ONqedT", Pp(8), &schroder_parameter_list)
        .showed("R9eI61", Pp(11), &treedepth, &diameter, UpperBound(Exponential), "Proposition 3.1")
        .showed("dohKmq", Pp(12), &dist_to_linear_forest, &hindex, UpperBound(Linear), "Proposition 3.2")
        .showed("WY0T4I", Pp(13), &dist_to_cluster, &dist_to_co_cluster, Exclusion, "Proposition 3.3")
        .showed("9VEBot", Pp(14), &dist_to_co_cluster, &boxicity, Exclusion, "Proposition 3.4")
        .showed("s1Sx0j", Pp(15), &vertex_cover, &domination_num, Exclusion, "Proposition 3.5")
        .showed("Ysx42j", Pp(15), &clique_cover_num, &dist_to_perfect, Exclusion, "Proposition 3.6")
        .showed("VMDVbW", Pp(16), &dist_to_complete, &max_clique, Exclusion, "Proposition 3.7")
        .showed("3WQZ4g", Pp(16), &dist_to_complete, &domatic_num, Exclusion, "Proposition 3.7")
        .showed("rCDCaC", Pp(16), &dist_to_complete, &dist_to_disconnected, Exclusion, "Proposition 3.8")
        .showed("AbzdZf", Pp(16), &clique_cover_num, &clique_width, Exclusion, "Proposition 3.9")
        .showed("W2GU1L", Pp(19), &clique_cover_num, &chordality, Exclusion, "Proposition 3.11")
        .showed("3Tunx6", Pp(19), &dist_to_perfect, &chordality, Exclusion, "Proposition 3.11")
        .showed("OKKYpY", Pp(20), &dist_to_co_cluster, &dist_to_chordal, Exclusion, "Proposition 3.12")
        .showed("A18O6S", Pp(20), &dist_to_bipartite, &dist_to_chordal, Exclusion, "Proposition 3.12")
        .showed("TiiRaX", Pp(20), &dist_to_co_cluster, &dist_to_disconnected, Exclusion, "Proposition 3.12")
        .showed("uvXQGw", Pp(20), &dist_to_bipartite, &dist_to_disconnected, Exclusion, "Proposition 3.12")
        .showed("5jYT5W", Pp(20), &dist_to_co_cluster, &domatic_num, Exclusion, "Proposition 3.12")
        .showed("U6hu68", Pp(20), &dist_to_bipartite, &domatic_num, Exclusion, "Proposition 3.12")
        .showed("hu6cvG", Pp(21), &bandwidth, &dist_to_planar, Exclusion, "Proposition 3.13")
        .showed("0gaHWl", Pp(21), &treedepth, &dist_to_planar, Exclusion, "Proposition 3.13")
        .showed("rVwCp7", Pp(21), &max_leaf_num, &girth, Exclusion, "Proposition 3.14")
        .showed("LEiVG6", Pp(23), &feedback_edge_set, &pathwidth, Exclusion, "Proposition 3.16")
        .showed("G5p3ht", Pp(23), &genus, &clique_width, Exclusion, "Proposition 3.17")
        .showed("Ydrtul", Pp(23), &dist_to_planar, &clique_width, Exclusion, "Proposition 3.17")
        .showed("6n7Mlc", Pp(24), &vertex_cover, &genus, Exclusion, "Proposition 3.18")
        .showed("AB8NTk", Pp(24), &vertex_cover, &max_degree, Exclusion, "Proposition 3.19")
        .showed("hUe750", Pp(24), &vertex_cover, &bisection_bandwidth, Exclusion, "Proposition 3.20")
        .showed("s1H8GJ", Pp(25), &feedback_edge_set, &dist_to_interval, Exclusion, "Proposition 3.21")
        .showed("PPqzfo", Pp(25), &treedepth, &hindex, Exclusion, "Proposition 3.22")
        .showed("2bGE0b", Pp(25), &feedback_edge_set, &hindex, Exclusion, "Proposition 3.22")
        .showed("xNaCQ3", Pp(26), &dist_to_outerplanar, &dist_to_perfect, Exclusion, "Proposition 3.23")
        .showed("MkoHxT", Pp(26), &bandwidth, &dist_to_perfect, Exclusion, "Proposition 3.24")
        .showed("hpzbh8", Pp(26), &genus, &dist_to_perfect, Exclusion, "Proposition 3.24")
        .showed("nrJVzy", Pp(26), &treedepth, &dist_to_perfect, Exclusion, "Proposition 3.24")
        .showed("8lk9o6", Pp(27), &dist_to_chordal, &boxicity, Exclusion, "Proposition 3.25")
        .showed("QvBfYR", Pp(28), &max_degree, &clique_width, Exclusion, "Proposition 3.26")
        .showed("mTfYrt", Pp(28), &max_degree, &bisection_bandwidth, Exclusion, "Proposition 3.26")
        .showed("NZGZCw", Pp(28), &dist_to_bipartite, &clique_width, Exclusion, "Proposition 3.26")
        .showed("uVhLEQ", Pp(28), &dist_to_bipartite, &bisection_bandwidth, Exclusion, "Proposition 3.26")
        .showed("Hm9nY3", Pp(30), &bandwidth, &genus, Exclusion, "Proposition 3.27")
        .showed("6olZjM", Pp(30), &bisection_bandwidth, &domatic_num, Exclusion, "Proposition 3.28")
        .showed("W6h5ZK", Pp(30), &feedback_edge_set, &bisection_bandwidth, Exclusion, "Proposition 3.29")
        .showed("KYpom9", Pp(31), &domatic_num, &dist_to_disconnected, Exclusion, "Proposition 3.30")
        .showed("2nmdxu", Pp(33), &bisection_bandwidth, &chordality, Exclusion, "Proposition 3.31")
        .showed("bxep24", Pp(33), &bisection_bandwidth, &clique_width, Exclusion, "Proposition 3.32")
        .showed("bhJsnM", Pp(33), &bisection_bandwidth, &max_clique, Exclusion, "Proposition 3.33")
        .showed("SynFuK", Pp(33), &genus, &dist_to_planar, Exclusion, "Proposition 3.34")
        .showed("hoJGBX", Pp(35), &average_degree, &max_clique, Exclusion, "Proposition 3.35")
        .showed("JRqAlT", Pp(36), &average_degree, &chordality, Exclusion, "Proposition 3.36")
        .todo_rest(&mut create);
    // let reduced_edgeless = create.reduced("reduced edgeless", &edgeless, 0);
    let twin_width_1_2021 = source("nyaOye", "twinWidthI2021", 6)
        .defined("s5Ktq7", Pp(2), &twin_width, "... we consider a sequence of graphs $G_n,G_{n-1},\\dots,G_2,G_1$, where $G_n$ is the original graph $G$, $G_1$ is the one-vertex graph, $G_i$ has $i$ vertices, and $G_{i-1}$ is obtained from $G_i$ by performing a single contraction of two (non-necessarily adjacent) vertices. For every vertex $u \\in V(G_i)$, let us denote by $u(G)$ the vertices of $G$ which have been contracted to $u$ along the sequence $G_n,\\dots,G_i$. A pair of disjoint sets of vertices is \\emph{homogeneous} if, between these sets, there are either all possible edges or no edge at all. The red edges ... consist of all pairs $uv$ of vertices of $G_i$ such that $u(G)$ and $v(G)$ are not homogeneous in $G$. If the red degree of every $G_i$ is at most $d$, then $G_n,G_{n-1},\\dots,G_2,G_1$ is called a \\emph{sequence of $d$-contractions}, or \\emph{$d$-sequence}. The twin-width of $G$ is the minimum $d$ for which there exists a sequence of $d$-contractions.")
        .showed("08lETp", Pp(14), &boolean_width, &twin_width, UpperBound(Exponential), "Theorem 3: Every graph with boolean-width $k$ has twin-width at most $2^{k+1}-1$.")
        .showed("0RiLv2", Pp(15), &grid, &twin_width, UpperBound(Constant), "Theorem 4.3. For every positive integers $d$ and $n$, the $d$-dimensional $n$-grid has twin-width at most $3d$.")
        // .showed("7p2TWN", Unknown, &cograph, &reduced_edgeless, Equal, "") // todo
        .todo_rest(&mut create);
    // let reduced_star = &create.reduced(&star, 0);
    // let twin_width_beyond_2022 = source("3B7Kvt", "twinWidthBeyond2022")
    // .showed("AwGkfi", Pp(3), &all_graphs, &reduced_star, UpperBound(Constant), "Every graph has a reduction sequence in which every red graph is a star ...")
    // // .defined("M6H2kI", , &reduced_bandwidth, "")
    // .todo_rest(&mut create);
    let tran2022 = source("uXViPE", "Tran2022", 7)
        // .defined("J1sHj8", Pp(14), &twin_cover_num, "An edge $\\{v,w\\}$ is a twin edge if vertices $v$ and $w$ have the same neighborhood excluding each other. The twin cover number $tcn(G)$ of a graph $G$ is the size of a smallest set $V' \\subseteq V(G)$ of vertices such that every edge in $E(G)$ is either a twin edge or incident to a vertex in $V'$")
        .defined("MlTT7n", Pp(14), &edge_clique_cover, "The edge clique cover number $eccn(G)$ of a graph $G$ is the minimum number of complete subgraphs required such that each edge is contained in at least one of them.")
        .defined("iAkCJ3", Pp(14), &neighborhood_diversity, "The neighborhood diversity $nd(G)$ of a graph $G$ is the smallest number $k$ such that there is a $k$-partition $(V_1,\\dots,V_k)$ of $G$, where each subset $V_i$, $i \\in [k]$ is a module and is either a complete set or an independent set.")
        .defined("i3su9L", Pp(14), &modular_width, "The modular-width $mw(G)$ of a graph $G$ is the smallest number $h$ such that a $k$-partition $(V_1,\\dots,V_k)$ of $G$ exists, where $k \\le h$ and each subset $V_i$, $i \\in [k]$ is a module and either contains a single vertex or for which the modular-subgraph $G[V_i]$ has a modular-width of $h$.")
        .defined("Fhp3Dr", Pp(14), &c_closure, "The c-closure $\\mathrm{cc}(G)$ of a graph $G$ is the smallest number $c$ such that any pair of vertices $v,w \\in V(G)$ with $|N_G(v) \\cap N_G(w)| \\ge c$ is adjacent. ...")
        .defined("eGC0vH", Pp(16), &boxicity, "The boxicity of a graph $G$ is the minimum amount of interval graphs required, such that their intersection (ed: fixed typo) results in $G$.")
        .defined("gZtk6B", Pp(16), &chordality, "The chordality of a graph $G$ is the minimum amount of chordal graphs required, such that their intersection (ed: fixed typo) results in $G$.")
        .showed("L0BALz", Pp(18), &vertex_cover, &twin_cover_num, UpperBound(Linear), by_definition)
        .showed("kcsO0r", Pp(18), &complete, &vertex_cover, Exclusion, "Note that a clique of size $n$ has ... a vertex cover number of $n-1$")
        .showed("RUjcaV", Pp(18), &complete, &twin_cover_num, UpperBound(Constant), "Note that a clique of size $n$ has a twin cover number of 0 ...")
        .showed("am05hY", Pp(18), &twin_cover_num, &dist_to_cluster, UpperBound(Linear), "... graph $H$ with a twin cover of size $k$ has a distance to cluster of at most $k$.")
        .showed("tNZjb1", Pp(18), &dist_to_cluster, &twin_cover_num, Exclusion, "We show that twin cover number is not upper bounded by distance to cluster.")
        .showed("ikkDSL", Pp(18), &twin_cover_num, &dist_to_complete, Incomparable, "Observation 3.3. Twin Cover Number is incomparable to Distance to Clique.")
        .showed("39CEgf", Pp(19), &twin_cover_num, &max_clique, Incomparable, "Observation 3.4. Twin Cover Number is incomparable to Maximum Clique, Domatic Number and Distance to Disconnected.")
        .showed("mttPgQ", Pp(19), &twin_cover_num, &domatic_num, Incomparable, "Observation 3.4. Twin Cover Number is incomparable to Maximum Clique, Domatic Number and Distance to Disconnected.")
        .showed("DaQuOK", Pp(19), &twin_cover_num, &edge_connectivity, Incomparable, "Observation 3.4. Twin Cover Number is incomparable to Maximum Clique, Domatic Number and Distance to Disconnected.")
        .showed("o5JorW", Pp(21), &twin_cover_num, &dist_to_co_cluster, Incomparable, "Proposition 3.5. Twin Cover Number is incomparable to Distance to Co-Cluster.")
        .showed("TddK24", Pp(22), &edge_clique_cover, &neighborhood_diversity, StrictUpperBound(Exponential), "Theorem 4.1. Edge Clique Cover Number strictly upper bounds Neighborhood Diversity.")
        .showed("O9TmJs", Pp(23), &dist_to_complete, &edge_clique_cover, StrictUpperBound(Polynomial), "Proposition 4.2. Disatnce to Clique strictly upper bounds Edge Clique Cover Number.")
        .showed("YGEkmM", Pp(23), &vertex_cover, &neighborhood_diversity, StrictUpperBound(Exponential), "Proposition 4.3. Vertex Cover Number strictly upper bounds Neighborhood Diversity.")
        .showed("gKXcun", Pp(24), &path, &modular_width, Exclusion, "The Modular-width of a path $P$ with length $n > 3$ is $n$.")
        // .showed("wFtd4d", Pp(24), &modular_width, &complement(modular_width), Equal, "Given any graph $G$, $\\mathrm{mw}(G) = \\mathrm{mw}(\\bar G)$.")
        .showed("FeMtBe", Pp(25), &modular_width, &clique_width, StrictUpperBound(Linear), "Proposition 4.6. Modular-width strictly upper bounds Clique-width.")
        .showed("rNb4Qe", Pp(25), &modular_width, &diameter, StrictUpperBound(Linear), "Theorem 4.7. Modular-width strictly upper bounds Max Diameter of Components.")
        .showed("SmUw6p", Pp(26), &path, &neighborhood_diversity, Exclusion, "The Neighborhood Diversity of a Path $P$ with length $n > 3$ is $n$.")
        .showed("cEEX99", Pp(26), &neighborhood_diversity, &boxicity, StrictUpperBound(Polynomial), "Note that given a path of length $n > 3$. The boxicity is 1 while ... neighborhood diversity is $n$. ... a graph ... with neighborhood diversity of $k$, its boxicity is at most $k+k^2$. ")
        .showed("78NYVs", Pp(28), &modular_width, &dist_to_cluster, Incomparable, "Proposition 4.10. Modular-width is incomparable to Distance to Cluster.")
        .showed("o64vSn", Pp(28), &modular_width, &dist_to_co_cluster, Incomparable, "Proposition 4.11. Modular-width is incomparable to Distance to Co-Cluster.")
        .showed("szCcet", Pp(28), &neighborhood_diversity, &twin_cover_num, Incomparable, "Proposition 4.12. Modular-width is incomparable to Distance to Twin Cover Number.")
        .showed("x3RM6x", Pp(29), &edge_clique_cover, &vertex_cover, Incomparable, "Proposition 4.13. Edge Clique Cover Number is incomparable to Vertex Cover Number.")
        .showed("4yZbJp", Pp(29), &edge_clique_cover, &domination_num, Incomparable, "Proposition 4.14. Edge Clique Cover Number is incomparable to Domination Number.")
        .showed("4QIldV", Pp(29), &edge_clique_cover, &dist_to_perfect, Incomparable, "Proposition 4.15. Edge Clique Cover Number is incomparable to Distance to Perfect.")
        .showed("Asq2IA", Pp(30), &modular_width, &chordality, Incomparable, "Theorem 4.16. Modular-width is incomparable to Chordality.")
        .showed("QLeIsq", Pp(32), &max_degree, &c_closure, StrictUpperBound(Linear), "Proposition 5.1. Maximum Degree strictly upper bounds $c$-Closure.")
        .showed("jNHLqN", Pp(32), &feedback_edge_set, &c_closure, StrictUpperBound(Linear), "Theorem 5.2. Feedback Edge Number strictly upper bounds $c$-Closure.")
        .showed("7vteay", Pp(34), &c_closure, &vertex_cover, Incomparable, "Proposition 5.3. $c$-Closure is incomparable to Vertex Cover Number.")
        .showed("Yvkf8u", Pp(34), &c_closure, &dist_to_complete, Incomparable, "Proposition 5.4. $c$-Closure is incomparable to Distance to Clique.")
        .showed("49lTWH", Pp(34), &c_closure, &bisection_bandwidth, Incomparable, "Proposition 5.5. $c$-Closure is incomparable to Bisection Width.") // todo check bisection bandwidth = bisection width
        .showed("LCe3uI", Pp(34), &c_closure, &genus, Incomparable, "Proposition 5.6. $c$-Closure is incomparable to Genus.")
        .showed("XjB6Cy", Pp(34), &c_closure, &vertex_connectivity, Incomparable, "Observation 5.7. $c$-Closure is incomparable to Distance to Disconnected, Domatic Number and Maximum Clique.")
        .showed("XaXtqm", Pp(34), &c_closure, &domatic_num, Incomparable, "Observation 5.7. $c$-Closure is incomparable to Distance to Disconnected, Domatic Number and Maximum Clique.")
        .showed("lXXajO", Pp(34), &c_closure, &max_clique, Incomparable, "Observation 5.7. $c$-Closure is incomparable to Distance to Disconnected, Domatic Number and Maximum Clique.")
        .showed("OinVkl", Pp(35), &c_closure, &boxicity, Incomparable, "Proposition 5.8. $c$-Closure is incomparable to Boxicity.")
        // .showed("wFtd4d", Pp(36), &twin_width, &complement(twin_width), Equal, "")
        .showed("PK4H2R", Pp(36), &clique_width, &twin_width, StrictUpperBound(Exponential), "Proposition 6.2. Clique-width strictly upper bounds Twin-width.")
        .showed("2F5Zr8", Pp(37), &genus, &twin_width, StrictUpperBound(Linear), "Proposition 6.3. Genus strictly upper bounds Twin-width.")
        .showed("xsiECz", Pp(37), &dist_to_planar, &twin_width, StrictUpperBound(Exponential), "Theorem 6.4. Distance to Planar strictly upper bounds Twin-width.") // cite
        .showed("aa0bCE", Pp(38), &twin_width, &dist_to_interval, Incomparable, "Observation 6.5. Twin-width is incomparable to Distance to Interval.")
        .showed("YqjLQa", Pp(38), &twin_width, &dist_to_bipartite, Incomparable, "Proposition 6.6. Twin-width is incomparable to Distance to Bipartite.")
        .showed("n3WK3H", Pp(40), &twin_width, &clique_cover_num, Incomparable, "Proposition 6.7. Twin-width is incomparable to Clique Cover Number.")
        .showed("52S4T0", Pp(40), &twin_width, &max_degree, Incomparable, "Proposition 6.8. Twin-width is incomparable to Maximum Degree.") // cites
        .showed("1lfJWG", Pp(40), &twin_width, &bisection_bandwidth, Incomparable, "Observation 6.9. Twin-width is incomparable to Bisection Width.") // check with bisection width
        .showed("UN2Lbu", Pp(42), &degeneracy, &boxicity, Incomparable, "Proposition 7.1. Degeneracy is incomparable to Boxicity.")
        .done(&mut create);
    let torunczyk2023 = source("KpkMZB", "Torunczyk2023", 7)
        .defined("9VHraO", Unknown, &inf_flip_width, "See radius-r flip-width for $r=\\infty$.")
        .defined("gxeVOT", Unknown, &r_flip_width, "The radius-$r$ flip-width of a graph $G$, denoted $fwr (G)$, is the smallest number $k \\in \\mathbb{N}$ such that the cops have a winning strategy in the flipper game of radius $r$ and width $k$ on $G$")
        .showed("9DTyeJ", Unknown, &inf_flip_width, &rank_width, UpperBound(Linear), "For every graph $G$, we have $\\mathrm{rankwidth}(G) \\le 3 \\mathrm{fw}_\\infty(G)+1$ ...")
        .showed("zYQZyB", Unknown, &rank_width, &inf_flip_width, UpperBound(Exponential), "For every graph $G$, we have ... $3 \\mathrm{fw}_\\infty(G)+1 \\le O(2^{\\mathrm{rankwidth(G)}})$.")
        .showed("OdbuZP", Unknown, &twin_width, &r_flip_width, UpperBound(Exponential), "Theorem 7.1. Fix $r \\in \\mathbb N$. For every graph $G$ of twin-width $d$ we have: $\\mathrm{fw}_r(G) \\le 2^d \\cdot d^{O(r)}$.")
        .showed("gvSCeQ", Unknown, &inf_flip_width, &r_flip_width, UpperBound(Linear), by_definition)
        .todo_rest(&mut create);
    // let merge_width_2024 = source("9exguJ", "MergeWidth2024", 7)
        // .defined("CJMyrW", Pp(4), &merge_width, "")
        // .todo_rest(&mut create);
    source("47Xy7Z", "munaroAlgorithmicApplicationsSimwidth2023", 1)
        .todo_rest(&mut create);
    // source("YFKpDU", "brettell2023comparingwidthparametersgraph")
    // .collective(Pp(3), "Theorem 1. For every $s \\ge 3$ and $t \\ge 2$, when restricted to $(K_s, K_{t,t})$-free graphs, sim-width, mim-width, clique-width, treewidth and tree-independence number are equivalent, whereas twin-width is more powerful than any of these parameters.")
    // .done()
    // .todo_rest(&mut create);
    let twwsurfaces2024 = source("lgJ2j7", "twwsurfaces2024", 2)
        .showed("3iR4qs", Pp(18), &genus, &twin_width, UpperBound(Linear), "The twin-width of every graph $G$ of Euler genus $g \\ge 1$ is at most ... $18 \\sqrt{47g}+O(1)$.") // todo sqrt
        .todo_rest(&mut create);
    let itp_introduced2024 = source("oBcMqr", "iteratedTypePartitions2024", 3)
        .defined("q3zZx7", Pp(3), &iterated_type_partitions, "two nodes have the same type iff $N(v) \\setminus \\{u\\} = N(u) \\setminus \\{v\\}$ ... [ed. paraphrased] let $\\mathcal V = \\{V_1,\\dots,V_t\\}$ be a partition of graph vertices such that each $V_i$ is a clique or an independent set and $t$ is minimized ... we can see each element of $\\mathcal V$ as a \\emph{metavertex} of a new graph $H$, called \\emph{type graph} of $G$ ... We say that $G$ is a \\emph{prime graph} if it matches its type graph ... let $H^{(0)}=G$ and $H^{(i)}$ denote the type graph of $H^{(i-1)}$, for $i \\ge 1$. Let $d$ be the smallest integer such that $H^{(d)}$ is a \\emph{prime graph}. The \\emph{iterated type partition} number of $G$, denoted by $\\mathrm{itp}(G)$, is the number of nodes of $H^{(d)}$.")
        .showed("LUQLaI", Pp(3), &neighborhood_diversity, &iterated_type_partitions, StrictUpperBound(Linear), "... $itp(G) \\le nd(G)$. Actually $itp(G)$ can be arbitrarily smaller than $nd(G)$.")
        .showed("Lh05uc", Pp(3), &iterated_type_partitions, &modular_width, UpperBound(Linear), by_definition)
        .todo_rest(&mut create);
    // let treebandwidth2025 = source("EImlRb", "treebandwidth2025", 4)
        // .hasse("l1oyAq", Pp(36), &vec!["F1NpDy", "4lp9Yj", "t7c4mp", "KEP2qM", "VHClqR", "GNOiyB", "5Q7fuR", "UyQ5yM", "TLx1pz", "aEs5ap", "w3LxG1"]) // also: slim tree-cut width, edge-treewidth, tree-cut width, tree-partition-width, overlap width, fan number, dipole number, biconnected maximum degree
        // .defined("u2JciR", Pp(1), &treebandwidth, "A \\emph{tree-layout} of $G=(V,E)$ is a rooted tree $T$ whose nodes are the vertices of $V$, and such that, for every edge $xy \\in E$, $x$ is an ancestor of $y$ or vice-versa. The bandwidth of $T$ is then the maximum distance in $T$ between pairs of neighbors in $G$. We call \\emph{treebandwidth} of $G$, the minimum bandwidth over tree-layouts of $G$, and denote it by ${\\rm tbw}(G)$.")
        // // Pp(2), "Theorem 2. Graphs of bounded treebandwidth are exactly graphs which exclude large walls and large fans as topological minors."
        // .todo_rest(&mut create);

    create.build()
}

// h03faf
// EXGHbI
// 727EaU
// NPiFw1
// W4Oo7f
// F9TWg2
// 2FM8hj
// vgpxF1
// P8yP3M
// pKi2tL
// oFvl4c
// 8CgU0P
// tInJY1
// QP01gs
// 5qgR1y
// Ie3w0s
// l230dC
// 8l33Gi
// opjGKG
// 5xOuoQ
// hhkOKk
// 45mNcy
// ssMLQE
// 3e5jLj
// IalVsc
// nvlJEd
// VLbSmM
// yOZQM5
// JIAD2R
// 9C3Gwi
// zZ1ZCa
// IbKkUQ
// 3Pff25
// DhGqJM
// lFz6Ci
