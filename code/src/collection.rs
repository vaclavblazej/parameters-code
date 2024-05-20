//! Collects the information we have about parameterized complexity classes, their inclusions,
//! and related graph classes, topics, bibliographic sources, theorems, proofs, and so on.

use crate::general::enums::{Cpx::*, CpxTime::*, Page::*};
use crate::input::{raw::RawData, build::Builder};

pub fn build_collection() -> RawData {
    let mut create = Builder::new();

    let connected = create.graph_class("KlMP0i", "connected");
    let bipartite = create.graph_class("cLHJkW", "bipartite");
    let block = create.graph_class("QrxQsH", "block");
    let chordal = create.graph_class("Cv1PaJ", "chordal");
    let cluster = create.graph_class("WAU7vf", "cluster");
    let co_cluster = create.graph_class("7HR4uV", "co-cluster");
    let cograph = create.graph_class("9Qd0Mx", "cograph");
    let bounded_degree = create.graph_class("yeKPCw", "bounded degree");
    let parameter_degree = create.graph_class("PUSZhY", "parameter degree");
    let complete = create.intersection("EhdXNA", &connected, &cluster, "complete");
    let const_components = create.graph_class("FJ8gmU", "constant components");
    let forest = create.graph_class("JngPPm", "forest");
    let tree = create.intersection("rJyICu", &connected, &forest, "tree");
    let interval = create.graph_class("p5skoj", "interval");
    let edgeless = create.graph_class("LsiBbX", "edgeless");
    let linear_forest = create.graph_class("skQuFN", "linear forest");
    let path = create.intersection("ryPlqz", &connected, &linear_forest, "path");
    let outerplanar = create.graph_class("0oCyaG", "outerplanar");
    let perfect = create.graph_class("RmssrZ", "perfect");
    let planar = create.graph_class("loZ5LD", "planar");
    let stars = create.graph_class("10JR3F", "stars");
    let cycles = create.graph_class("2iJr52", "cycles");
    let cycle = create.intersection("Ti0asF", &connected, &cycles, "cycle");
    let disjoint_cycles = create.graph_class("AGnF5Z", "disjoint cycles");
    let grid = create.graph_class("lfYXuK", "grid");

    create.isgci(&bipartite, 69)
        .isgci(&block, 93)
        .isgci(&chordal, 32)
        .isgci(&cluster, 1237)
        .isgci(&co_cluster, 1248)
        .isgci(&cograph, 151)
        .isgci(&bounded_degree, 1053)
        .isgci(&complete, 1241)
        .isgci(&forest, 342)
        .isgci(&interval, 234)
        .isgci(&edgeless, 1247)
        .isgci(&outerplanar, 110)
        .isgci(&perfect, 56)
        .isgci(&planar, 43)
        .isgci(&stars, 1297)
        .isgci(&grid, 464)
        ;

    create.source("myit4D", "unknown")
        .defined("2kG0kY", Unknown, &block, "Every block (maximal 2-connected subgraph) is a clique.")
        .defined("roSFzV", Unknown, &cluster, "Disjoint union of complete graphs.")
        .defined("", Unknown, &cluster, "Every connected component is a complete graph.")
        .defined("", Unknown, &cluster, "Does not include path on three vertices as an induced subgraph.")
        .defined("FDbIDy", Unknown, &co_cluster, "Complete multipartite graph.")
        .defined("ivxOm1", Unknown, &parameter_degree, "Degree is upper bounded by the parameter")
        .defined("FvgORV", Unknown, &const_components, "Disjoint union of components of constant size.")
        .defined("51KDFn", Unknown, &stars, "Disjoint union of stars.")
        .defined("sAZHF4", Unknown, &cycles, "Every component is a cycle.")
        .defined("cBDurK", Unknown, &disjoint_cycles, "All cycles in the graph are disjoint. Can contain arbitrary trees attached to and between the cycles.")
        .defined("sp6LGE", Unknown, &grid, "Cartesian product of two paths.")
        .proper_graph_inclusion("piRTZw", &chordal, &perfect)
        .proper_graph_inclusion("stwHRi", &cograph, &perfect)
        .proper_graph_inclusion("ogyvLp", &bipartite, &perfect)
        .proper_graph_inclusion("FM1wVJ", &cluster, &interval)
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
        .proper_graph_inclusion("yWSq1V", &edgeless, &const_components)
        .proper_graph_inclusion("HtdoRP", &edgeless, &co_cluster)
        .proper_graph_inclusion("1PLbSg", &grid, &planar)
        .proper_graph_inclusion("RQcVkC", &grid, &bipartite)
        .proper_graph_inclusion("KxMj5k", &grid, &bounded_degree)
        .proper_graph_inclusion("VnTIL0", &bounded_degree, &parameter_degree)
        .proper_graph_inclusion("rSYFkG", &const_components, &bounded_degree)
        .proper_graph_inclusion("ZEEhCr", &linear_forest, &bounded_degree)
        .proper_graph_inclusion("CJ76wg", &cycles, &disjoint_cycles)
        .proper_graph_inclusion("a3JKzR", &cycles, &bounded_degree)
        .proper_graph_inclusion("CTwA2j", &grid, &connected)
        .proper_graph_inclusion("wTugFB", &edgeless, &cluster)
        ;

    let top_metric = create.topic("wpYsel", "metric", "Typically used in metric spaces.");
    let top_drawing = create.topic("lJJaYb", "drawing", "Closely tied to drawing the graph onto a topological space.");

    let vertex_cover = create.parameter("4lp9Yj", "vertex cover");
    let max_matching = create.parameter("veU7Jf", "maximum matching");
    let vertex_integrity = create.parameter("KVhJFB", "vertex integrity");
    let treedepth = create.parameter("KEP2qM", "treedepth");
    let clique_cover_num = create.parameter("VomShB", "clique cover number");
    let max_independent_set = create.parameter("mHtXUU", "maximum independent set");
    let domination_num = create.parameter("Gq0onN", "domination number");
    let twin_cover_num = create.parameter("MUnHA0", "twin-cover number");
    let edge_clique_cover = create.parameter("nYQDv6", "edge clique cover number");
    let neighborhood_diversity = create.parameter("vMs3RS", "neighborhood diversity");
    let modular_width = create.parameter("4bj71L", "modular-width");
    let max_leaf_num = create.parameter("BN92vX", "maximum leaf number");
    let feedback_edge_set = create.parameter("HTk9PZ", "feedback edge set");
    let genus = create.parameter("gbaHdw", "genus"); // top_drawing
    let cutwidth = create.parameter("TLx1pz", "cutwidth");
    let carving_width = create.parameter("dS6OgO", "carving-width");
    let bandwidth = create.parameter("aP5a38", "bandwidth");
    let topological_bandwidth = create.parameter("SnA7Eq", "topological bandwidth");
    let bisection_bandwidth = create.parameter("wUdmUb", "bisection bandwidth");
    let max_degree = create.parameter("UyQ5yM", "maximum degree");
    let c_closure = create.parameter("ou9VU1", "c-closure");
    let feedback_vertex_set = create.parameter("GNOiyB", "feedback vertex set");
    let shrub_depth = create.parameter("NTgNzT", "shrub-depth");
    let linear_clique_width = create.parameter("fQj3wU", "linear clique-width");
    let pathwidth = create.parameter("VHClqR", "pathwidth");
    let treewidth = create.parameter("5Q7fuR", "treewidth");
    let branch_width = create.parameter("lIcmuR", "branch width");
    let clique_width = create.parameter("wg5HuV", "clique width");
    let rank_width = create.parameter("fojquT", "rank width");
    let linear_rank_width = create.parameter("cHugsk", "linear rank width");
    let boolean_width = create.parameter("A2jPWT", "boolean width");
    let inf_flip_width = create.parameter("nYXiuT", "inf-flip-width");
    let twin_width = create.parameter("MNTjuW", "twin width");
    let r_flip_width = create.parameter("4DIiH0", "radius-r flip-width");
    let book_thickness = create.parameter("doijTS", "book thickness"); // top_drawing
    let hindex = create.parameter("GNTwUS", "h-index");
    let acyclic_chromatic_number = create.parameter("QGZuUW", "acyclic chromatic number");
    let odd_cycle_transversal = create.parameter("Ve5ruW", "odd cycle transversal");
    let degeneracy = create.parameter("VowkuW", "degeneracy");
    let chromatic_num = create.parameter("w7MmyW", "chromatic number");
    let average_degree = create.parameter("z0y4TW", "average degree");
    let min_degree = create.parameter("GPmOeT", "minimum degree");
    let max_clique = create.parameter("q7zHeT", "maximum clique");
    let edge_connectivity = create.parameter("JbqZoT", "edge connectivity");
    let boxicity = create.parameter("a7MpiT", "boxicity");
    let chordality = create.parameter("fTqo40", "chordality");
    let max_induced_matching = create.parameter("GzMYlT", "maximum induced matching");
    let diameter = create.parameter("p4bTjp", "diameter");
    let average_distance = create.parameter("zH8PpT", "average distance");
    let girth = create.parameter("BCwUeT", "girth");
    let domatic_num = create.parameter("KRV6tI", "domatic number");
    let connected_component_size = create.parameter("t7c4mp", "connected component size");

    let dist_to_complete = create.distance_to(&complete);
    let dist_to_co_cluster = create.distance_to(&co_cluster);
    let dist_to_cograph = create.distance_to(&cograph);
    let dist_to_cluster = create.distance_to(&cluster);
    let dist_to_linear_forest = create.distance_to(&linear_forest);
    let dist_to_outerplanar = create.distance_to(&outerplanar);
    let dist_to_block = create.distance_to(&block);
    let dist_to_edgeless = create.distance_to(&edgeless);
    let dist_to_forest = create.distance_to(&forest);
    let dist_to_bipartite = create.distance_to(&bipartite);
    let dist_to_planar = create.distance_to(&planar);
    let dist_to_chordal = create.distance_to(&chordal);
    let dist_to_stars = create.distance_to(&stars);
    let dist_to_const_components = create.distance_to(&const_components);
    let dist_to_perfect = create.distance_to(&perfect);
    let dist_to_cycles = create.distance_to(&cycles);
    let dist_to_interval = create.distance_to(&interval);
    let dist_to_max_degree = create.distance_to(&max_degree);
    let dist_to_connected_component_size = create.distance_to(&connected_component_size);

    create.isgci(&vertex_cover, 2)
        .isgci(&max_matching, 13)
        .isgci(&treedepth, 18)
        .isgci(&dist_to_complete, 1)
        .isgci(&max_independent_set, 8)
        .isgci(&domination_num, 5)
        .isgci(&dist_to_co_cluster, 3)
        .isgci(&dist_to_cograph, 7)
        .isgci(&dist_to_cluster, 29)
        .isgci(&max_leaf_num, 22)
        .isgci(&genus, 23)
        .isgci(&cutwidth, 15)
        .isgci(&carving_width, 16)
        .isgci(&bandwidth, 25)
        .isgci(&max_degree, 28)
        .isgci(&dist_to_linear_forest, 24)
        .isgci(&dist_to_outerplanar, 26)
        .isgci(&pathwidth, 9)
        .isgci(&treewidth, 10)
        .isgci(&branch_width, 11)
        .isgci(&clique_width, 12)
        .isgci(&rank_width, 20)
        .isgci(&boolean_width, 21)
        .isgci(&book_thickness, 32)
        .isgci(&acyclic_chromatic_number, 31)
        .isgci(&degeneracy, 17)
        .isgci(&chromatic_num, 19)
        .isgci(&max_clique, 27)
        .isgci(&dist_to_block, 30)
        .isgci(&max_induced_matching, 14)
        .isgci(&diameter, 6);

    let by_definition = "By definition";
    let cliques_make_it_unbounded = "Parameter is unbounded for the graph class of cliques.";

    create.source("pQlQN7", "unknown")
        .defined("XK5Xxy", Unknown, &linear_forest, "Disjoint union of paths.")
        .showed("H1gQ6m", Unknown, &feedback_vertex_set, &dist_to_forest, Equivalence, "")
        // .showed("8Mm5qJ", Unknown, &vertex_cover, &max_matching, Equivalence, "Kőnig's theorem") // only on bipartite graphs
        .showed("", Unknown, &vertex_cover, &max_matching, UpperBound(Linear), "") // only on bipartite graphs
        // Cite(id="gBA7dc", url="https://en.wikipedia.org/wiki/K%C5%91nig%27s_theorem_(graph_theory)", text="Kőnig's theorem"),
        .showed("U14yX4", Unknown, &odd_cycle_transversal, &dist_to_bipartite, Equivalence, "Bipartite graphs is the graph class without any odd cycles.")
        // Note(id="lqOY3G", text="Bipartite graphs is the graph class without any odd cycles."),
        .showed("pohj2V", Unknown, &const_components, &connected_component_size, UpperBound(Linear), "")
        .showed("5sq1SD", Unknown, &feedback_edge_set, &feedback_vertex_set, UpperBound(Linear), "Given solution to feedback edge set one can remove one vertex incident to the solution edges to obtain feedback vertex set.")
        .showed("8dQ8Us", Unknown, &feedback_edge_set, &genus, UpperBound(Linear), "Removing $k$ edges creates a forest that is embeddable into the plane. We now add one handle for each of the $k$ edges to get embedding into $k$-handle genus.")
        .showed("K0Bc61", Unknown, &chromatic_num, &max_clique, UpperBound(Linear), "Unbounded clique implies the number of needed colors is unbounded.")
        .showed("uKFrrb", Unknown, &degeneracy, &chromatic_num, UpperBound(Linear), "Greedily color the vertices in order of the degeneracy ordering. As each vertex has at most $k$ colored predecesors we use at most $k+1$ colors.")
        .showed("gLjejq", Unknown, &degeneracy, &average_degree, UpperBound(Linear), "Removing a vertex of degree $d$ increases the value added to the sum of all degrees by at most $2d$, hence, the average is no more than twice the degeneracy.")
        .showed("q5QDXg", Unknown, &max_degree, &hindex, UpperBound(Linear), "As h-index seeks $k$ vertices of degree $k$ it is trivially upper bound by maximum degree.")
        .showed("1MAoyr", Unknown, &min_degree, &edge_connectivity, UpperBound(Linear), "Removing edges incident to the minimum degree vertex disconnects the graph.")
        .showed("XOAOqw", Unknown, &linear_rank_width, &rank_width, Todo, "")
        .showed("SUEy4S", Unknown, &pathwidth, &linear_rank_width, Todo, "")
        .showed("UYpwYn", Unknown, &min_degree, &domatic_num, UpperBound(Linear), "The vertex of minimum degree needs to be dominated in each of the sets. As the sets cannot overlap there can be at most $k+1$ of them.")
        .showed("d2ZJIh", Unknown, &dist_to_linear_forest, &pathwidth, UpperBound(Linear), "After removal of $k$ vertices the remaining class has a bounded width $w$. So by including the removed vertices in every bag, we can achieve decomposition of width $w+k$")
        .showed("LyJWeW", Unknown, &topological_bandwidth, &bisection_bandwidth, UpperBound(Linear), "Order vertices by their bandwidth integer. We split the graph in the middle of this ordering. There are at most roughly $k^2/2$ edges over this split.")
        .showed("waxvtz", Unknown, &bandwidth, &max_degree, UpperBound(Linear), "Each vertex has an integer $i$ and may be connected only to vertices whose difference from $i$ is at most $k$. There are at most $k$ bigger and $k$ smaller such neighbors.")
        .showed("d2ZJIh", Unknown, &dist_to_linear_forest, &pathwidth, UpperBound(Linear), "After removal of $k$ vertices the remaining class has a bounded width $w$. So by including the removed vertices in every bag, we can achieve decomposition of width $w+k$")
        .showed("d2ZJIh", Unknown, &dist_to_outerplanar, &treewidth, UpperBound(Linear), "After removal of $k$ vertices the remaining class has a bounded width $w$. So by including the removed vertices in every bag, we can achieve decomposition of width $w+k$")
        .showed("VS44M7", Unknown, &vertex_integrity, &treedepth, UpperBound(Linear), "First, treedepth removes vertices of the modulator, then it iterates through remaining components one by one.")
        .showed("D0wE7A", Unknown, &dist_to_const_components, &vertex_integrity, UpperBound(Linear), "The remaining components in vertex integrity are parameter-sized which can be always made bigger than constant-sized components.")
        .showed("rmLeo2", Unknown, &dist_to_stars, &treedepth, UpperBound(Linear), "First, treedepth removes vertices of the modulator, remainder has treedepth $2$")
        .showed("bYybsT", Unknown, &dist_to_complete, &clique_cover_num, UpperBound(Linear), "We cover the $k$ vertices of the modulator by cliques of size $1$ and cover the remaining clique by another one.")
        .showed("h8nG9p", Unknown, &clique_cover_num, &max_independent_set, UpperBound(Linear), "Every clique of the clique cover partition may contain at most one vertex of the independent set.")
        .showed("gGtTUf", Unknown, &max_independent_set, &domination_num, UpperBound(Linear), "Every maximal independent set is also a dominating set because any undominated vertex could be added to the independent set.")
        .showed("J0jyXi", Unknown, &domination_num, &diameter, UpperBound(Linear), "An unbounded diameter implies a long path where no vertices that are more than $3$ apart may be dominated by the same dominating vertex, otherwise we could shorten the path. Hence, the number of dominating vertices is also unbounded.")
        .showed("xrVJqb", Unknown, &dist_to_bipartite, &chromatic_num, UpperBound(Linear), "Removed vertices get one color each and we need only $2$ colors for the rest.")
        .showed("5wc1ir", Unknown, &edge_clique_cover, &neighborhood_diversity, UpperBound(Exponential), "Label vertices by the cliques they are contained in, each label is its own group in the neighborhood diversity, connect accordingly.")
        .showed("RnkWvT", Unknown, &dist_to_complete, &edge_clique_cover, UpperBound(Polynomial), "Cover the remaining clique, cover each modulator vertex and its neighborhood outside of it with another clique, cover each edge within the modulator by its own edge.")
        .showed("FY0U1r", Unknown, &treewidth, &book_thickness, UpperBound(Exists), "")
        .showed("rQBO3K", Unknown, &chromatic_num, &chordality, UpperBound(Exists), "")
        .showed("GCGHcz", Unknown, &boxicity, &chordality, UpperBound(Exists), "")
        .showed("ghgPz2", Unknown, &max_leaf_num, &cutwidth, UpperBound(Exists), "")
        .showed("BKCgft", Unknown, &max_leaf_num, &dist_to_linear_forest, UpperBound(Exists), "")
        .showed("ECnpoM", Unknown, &genus, &book_thickness, UpperBound(Exists), "")
        .showed("CyAMhs", Unknown, &acyclic_chromatic_number, &boxicity, UpperBound(Exists), "")
        .showed("rVEmFt", Unknown, &acyclic_chromatic_number, &degeneracy, UpperBound(Exists), "")
        .showed("thTXGX", Unknown, &hindex, &acyclic_chromatic_number, UpperBound(Exists), "")
        .showed("pUfoGn", Unknown, &hindex, &dist_to_max_degree, UpperBound(Linear), "Remove the $h$ vertices of degree at least $h$ to get a graph that has maximum degree $h$.")
        .showed("8ZzI5w", Unknown, &dist_to_max_degree, &hindex, UpperBound(Linear), "Removal of $k$ vertices yielding a graph with maximum degree $c$ means that there were $k$ vertices of arbitrary degree and the remaining vertices had degree at most $k+c$. Hence, $h$-index is no more than $k+c$.")
        .showed("16XW6a", Unknown, &dist_to_chordal, &chordality, UpperBound(Exists), "")
        .showed("Bzw7GY", Unknown, &dist_to_cograph, &clique_width, UpperBound(Exists), "")
        .showed("fedm1t", Unknown, &dist_to_cograph, &chordality, UpperBound(Exists), "")
        .showed("rGMb0t", Unknown, &dist_to_cograph, &diameter, UpperBound(Exists), "")
        .showed("Er0L5w", Unknown, &book_thickness, &acyclic_chromatic_number, UpperBound(Exists), "")
        .showed("03kKbA", Unknown, &dist_to_planar, &acyclic_chromatic_number, UpperBound(Exists), "")
        .showed("wJkzlI", Unknown, &average_distance, &girth, UpperBound(Exists), "")
        .showed("JfSGx1", Unknown, &max_leaf_num, &feedback_edge_set, UpperBound(Exists), "")
        .showed("LJQHKw", Unknown, &max_induced_matching, &diameter, UpperBound(Exists), "")
        .showed("unkZhD", Unknown, &max_independent_set, &max_induced_matching, UpperBound(Exists), "")
        .showed("RqDij1", Unknown, &vertex_cover, &neighborhood_diversity, UpperBound(Exponential), "")
        .showed("a2DTDH", Unknown, &twin_cover_num, &neighborhood_diversity, Exclusion, "")
        .showed("Pinlr2", Unknown, &linear_clique_width, &clique_width, UpperBound(Exists), "")
        .showed("OUUh3y", Unknown, &clique_width, &boolean_width, UpperBound(Linear), "")
        .showed("hgUvsR", Unknown, &boolean_width, &clique_width, UpperBound(Exponential), "")
        .showed("V9Pisv", Unknown, &branch_width, &boolean_width, UpperBound(Linear), "")
        .showed("0zGd6N", Unknown, &branch_width, &rank_width, UpperBound(Linear), "")
        .showed("QWXYYb", Unknown, &treewidth, &boolean_width, UpperBound(Exists), "")
        .showed("mD6cvS", Unknown, &cutwidth, &bandwidth, UpperBound(Exists), "")
        .showed("8rtBjc", Unknown, &twin_cover_num, &modular_width, UpperBound(Exists), "") // J. Gajarsky, M. Lampis, and S. Ordyniak. Parameterized Algorithms for Modular-Width
        .showed("Vv11bW", Unknown, &modular_width, &twin_cover_num, Exclusion, "") // J. Gajarsky, M. Lampis, and S. Ordyniak. Parameterized Algorithms for Modular-Width
        .showed("NeMJtU", Unknown, &neighborhood_diversity, &modular_width, UpperBound(Exists), "") // J. Gajarsky, M. Lampis, and S. Ordyniak. Parameterized Algorithms for Modular-Width
        .showed("NTZE4R", Unknown, &modular_width, &clique_width, UpperBound(Exists), "")
        .showed("Vq2BBF", Unknown, &modular_width, &diameter, UpperBound(Exists), "")
        .showed("cEEX99", Unknown, &neighborhood_diversity, &boxicity, UpperBound(Exists), "")
        .showed("3iR4qs", Unknown, &genus, &twin_width, UpperBound(Exists), "")
        .showed("TA2EZd", Unknown, &dist_to_planar, &twin_width, UpperBound(Exists), "")
        .showed("qB1OMb", Unknown, &max_degree, &c_closure, UpperBound(Exists), "")
        .showed("fmiQlU", Unknown, &feedback_edge_set, &c_closure, UpperBound(Exists), "")
        .showed("LTyhoG", Unknown, &vertex_integrity, &dist_to_connected_component_size, UpperBound(Exists), "")
        .showed("SyGwqT", NotApplicable, &bandwidth, &topological_bandwidth, UpperBound(Linear), by_definition)
        .showed("ebAUEu", NotApplicable, &twin_cover_num, &dist_to_cluster, UpperBound(Linear), by_definition)
        .showed("2XN8ux", NotApplicable, &vertex_cover, &twin_cover_num, UpperBound(Linear), by_definition)
        .showed("XTPNkl", NotApplicable, &average_degree, &min_degree, UpperBound(Linear), by_definition)
        .showed("TezCU1", NotApplicable, &diameter, &average_distance, UpperBound(Linear), by_definition)
        .showed("qy7Xdi", NotApplicable, &max_matching, &max_induced_matching, UpperBound(Linear), by_definition)
        .showed("2gTckj", NotApplicable, &dist_to_interval, &boxicity, UpperBound(Linear), by_definition)
        .showed("LAc0Ur", NotApplicable, &bisection_bandwidth, &edge_connectivity, UpperBound(Linear), by_definition)
// Bound(fr=vertex_cover, to=neighborhood_diversity, notes=[
    // Cite(id="YgTRtT", url="https://link.springer.com/article/10.1007/s00453-011-9554-x", text="Construct $k$ singleton sets, one for each vertex in the vertex cover and at most $2^k$ additional sets, one for each subset of vertices of the vertex cover. ...", range=Range(EXPONENTIAL)),
    // ])
        .defined("aQQnbF", Unknown, &vertex_integrity, "Minimum $k$ such that there exists $k$ vertices whose removal results in connected components of sizes at most $k$.")
        .defined("nTIDMU", Unknown, &twin_cover_num, "Distance to cluster where all vertices of each clique are siblings.")
        .defined("8tk4SI", Unknown, &max_degree, "maximum degree of graph's vertices")
        .defined("81zlqB", Unknown, &feedback_vertex_set, "can be thought of as a *distance to forest*")
        .defined("CKNuj2", Unknown, &min_degree, "minimum degree of graph's vertices")
        .showed("q3qJkr", Unknown, &treedepth, &pathwidth, UpperBound(Exists), "Saving the set of open vertices in a DFS over the tree treedepth after every step yields bags of a nice path decomposition.")
        .showed("7tFsi6", Unknown, &treedepth, &diameter, UpperBound(Exists), "An unbounded diameter implies the class contains paths as subgraphs. Minimum treedepth to embed a path of length $n$ in a treedepth tree is $\\log n$.")
        // ;
        .showed("H1gQ6m", Unknown, &feedback_vertex_set, &dist_to_forest, Equivalence, "")
        .showed("hDNUsi", Unknown, &vertex_cover, &dist_to_edgeless, Equivalence, "")
        .showed("U14yX4", Unknown, &odd_cycle_transversal, &dist_to_bipartite, Equivalence, "Bipartite graphs is the graph class without any odd cycles.")
        .showed("Jyi5e3", NotApplicable, &complete, &max_clique, Exclusion, cliques_make_it_unbounded)
        .showed("t9mJyF", NotApplicable, &complete, &domatic_num, Exclusion, cliques_make_it_unbounded)
        .showed("KnGxdS", NotApplicable, &complete, &edge_connectivity, Exclusion, cliques_make_it_unbounded)
        .showed("fQjK7z", Unknown, &co_cluster, &dist_to_chordal, Exclusion, "")
        .showed("cOXKlo", Unknown, &cluster, &twin_cover_num, UpperBound(Constant), "")
        .showed("jIxF3A", Unknown, &cluster, &domination_num, Exclusion, "")
        .showed("OjWb8I", Unknown, &bipartite, &girth, Exclusion, "")
        .showed("d1qoN7", Unknown, &bipartite, &edge_connectivity, Exclusion, "")
        .showed("Z335lf", Unknown, &forest, &feedback_edge_set, UpperBound(Constant), "")
        .showed("5pJxbA", Unknown, &forest, &girth, Exclusion, "")
        .showed("AdhtOR", Unknown, &forest, &pathwidth, Exclusion, "")
        .showed("k18Pyk", Unknown, &forest, &dist_to_interval, Exclusion, "")
        .showed("2QZo3T", Unknown, &edgeless, &vertex_cover, UpperBound(Constant), "")
        .showed("cq2q83", Unknown, &edgeless, &domination_num, Exclusion, "")
        .showed("ipo6rm", Unknown, &grid, &clique_width, Exclusion, "")
        .showed("TOJxXi", Unknown, &grid, &dist_to_chordal, Exclusion, "")
        .showed("MRucBP", Unknown, &grid, &average_distance, Exclusion, "")
        .showed("MYM6Ye", Unknown, &grid, &bisection_bandwidth, Exclusion, "")
        .showed("gaJIvr", Unknown, &disjoint_cycles, &bisection_bandwidth, UpperBound(Constant), "")
        .showed("VJwjbX", Unknown, &outerplanar, &bisection_bandwidth, UpperBound(Constant), "")
        .showed("fQjK7z", Unknown, &grid, &max_degree, UpperBound(Constant), "")
        .showed("HOULS0", Unknown, &disjoint_cycles, &girth, Exclusion, "")
        .showed("OjWb8I", Unknown, &interval, &average_distance, Exclusion, "")
        .showed("967lJ2", Unknown, &path, &treedepth, Exclusion, "")
        .showed("ne07p3", Unknown, &linear_forest, &average_distance, Exclusion, "")
        .showed("kkMeCO", Unknown, &planar, &genus, UpperBound(Constant), "")
        .showed("EZdonY", Unknown, &planar, &girth, Exclusion, "")
        .showed("cIAr80", Unknown, &planar, &max_degree, Exclusion, "")
        .showed("DxmXhS", Unknown, &planar, &dist_to_perfect, Exclusion, "")
        .showed("QeiwSR", Unknown, &const_components, &cutwidth, UpperBound(Constant), "")
        .showed("EjGaM8", Unknown, &const_components, &dist_to_perfect, Exclusion, "")
        .showed("bQLN2O", Unknown, &const_components, &dist_to_planar, Exclusion, "")
        .showed("CoBOm0", Unknown, &stars, &hindex, Exclusion, "")
        .showed("Ei8B1H", Unknown, &stars, &vertex_integrity, Exclusion, "")
        .showed("r2VydG", Unknown, &outerplanar, &dist_to_cycles, Exclusion, "")
        .showed("ORlCs0", Unknown, &disjoint_cycles, &dist_to_perfect, Exclusion, "")
        .showed("tZrOta", Unknown, &cycle, &max_leaf_num, UpperBound(Constant), "")
        .showed("cYF2KU", Unknown, &cycle, &girth, Exclusion, "")
        // .showed("5iynqA", Unknown, &cycles, &dist_to_cycles, Exclusion, "") // probably nonsense
        ;

    create.source("ez07Er", "https://en.wikipedia.org/wiki/Vertex_cover")
        .defined("l20H0G", Unknown, &vertex_cover, "... set of vertices that includes at least one endpoint of every edge of the graph.");
    create.source("f3q99d", "https://www.graphclasses.org/classes/par_13.html")
        .defined("wiwa6x", Unknown, &max_matching, "A matching in a graph is a subset of pairwise disjoint edges (any two edges that do not share an endpoint). The parameter maximum matching of a graph $G$ is the largest size of a matching in $G$.");
    create.source("QHJ1Kl", "https://en.wikipedia.org/wiki/Tree-depth")
        .defined("E9GMDZ", Unknown, &treedepth, "The tree-depth of a graph $G$ may be defined as the minimum height of a forest $F$ with the property that every edge of $G$ connects a pair of nodes that have an ancestor-descendant relationship to each other in $F$.");
    create.source("jh0OIZ", "https://en.wikipedia.org/wiki/Clique_cover")
        .defined("p0NZrl", Unknown, &clique_cover_num, "... is a partition of the vertices into cliques ... A minimum clique cover is a clique cover that uses as few cliques as possible. The minimum $k$ for which a clique cover exists is called the clique cover number of the given graph.");
    create.source("0cYayY", "https://en.wikipedia.org/wiki/Maximal_independent_set")
        .defined("2xRnhJ", Unknown, &max_independent_set, "For a graph $G=(V,E)$, an independent set $S$ is a maximal independent set if for $v \\in V$, one of the following is true: 1) $v \\in S$ 2), $N(v) \\cap S \\ne \\emptyset$ where $N(v)$ denotes the neighbors of $v$. ... the largest maximum independent set of a graph is called a maximum independent set.");
    create.source("82RsGb", "https://mathworld.wolfram.com/DominationNumber.html")
        .defined("7XYxB4", Unknown, &domination_num, "The domination number $\\gamma(G)$ of a graph $G$ is the minimum size of a dominating set of vertices in $G$ ...");
    create.source("L2KX25", "https://link.springer.com/article/10.1007/s00453-011-9554-x")
        .defined("ljbw1n", Unknown, &neighborhood_diversity, "We will say that two vertices $v, v'$ of a graph $G(V, E)$ have the same *type* iff they have the same colors and $N(v) \\setminus \\{v\\}=N(v') \\setminus \\{v\\}$, where $N(v)$ denotes the set of neighbors of $v$. ... A colored graph $G(V, E)$ has neighborhood diversity at most $w$, if there exists a partition of $V$ into at most $w$ sets, such that all the vertices in each set have the same type.");
    create.source("Q3HJs5", "https://mathworld.wolfram.com/MaximumLeafNumber.html")
        .defined("rBWwFy", Unknown, &max_leaf_num, "... the largest number of tree leaves in any of its spanning trees.");
    create.source("WP7pFA", "https://stackoverflow.com/questions/10791689/how-to-find-feedback-edge-set-in-undirected-graph")
        .defined("eYijvL", Unknown, &feedback_edge_set, "Let $G=(V,E)$ be an undirected graph. A set $F \\subseteq E$ of edges is called a feedback-edge set if every cycle of $G$ has at least one edge in $F$.");
    create.source("8ryhNq", "https://en.wikipedia.org/wiki/Genus_(mathematics)#Graph_theory")
        .defined("3qF6Zm", Unknown, &genus, "The genus of a graph is the minimal integer $n$ such that the graph can be drawn without crossing itself on a sphere with $n$ handles.");
    create.source("bnOBjM", "https://link.springer.com/article/10.1007/bf01215352")
        .defined("gMC8t4", Unknown, &carving_width, "Let $V$ be a finite set with $|V| \\ge 2$. Two subsets $A,B\\subseteq V$ \\emph{cross} if $A\\cap B$, $A-B$, $B-A$, $V-(A\\cup B)$ are all non-empty. A \\emph{carving} in $V$ is a set $\\mathscr{C}$ of subsets of $V$ such that 1) $\\emptyset, V \\notin \\mathscr{C}$ 2) no two members of $\\mathscr{C}$ cross, and 3) $\\mathscr{C}$ is maximal subject to (1) and (2). ... For $A \\subseteq V(G)$, we denote by $\\delta(A)$ ... the set of all edges with an end in $A$ and an end in $V(G)-A$. For each $e \\in E(G)$, let $p(e) \\ge 0$ be an integer. For $X \\subseteq E(G)$ we denote $\\sum_{e \\in X}p(e)$ by $p(X)$, and if $|V(G)| \\ge 2$ we define the \\emph{$p$-carving-width} of $G$ to be the minimum, over all carvings $\\mathscr{C}$ in $V(G)$, of the maximum, over all $A \\in \\mathscr{C}$, of $p(\\delta(A))$. ... The \\emph{carving-width} of $G$ is the $p$-carving-width of $G$ where $p(e)=1$ for every edge $e$.");
    create.source("s11UF7", "https://en.wikipedia.org/wiki/Carving_width")
        .defined("LtcqRs", Unknown, &carving_width, "A carving can be described as an unrooted binary tree whose leaves are labeled with the vertices of the given graph. Removing any edge from this tree partitions the tree into two subtrees, and correspondingly partitions the vertices of the tree into two clusters. ... The width of a carving, defined in this way, is the maximum number of edges that connect two complementary clusters. The carving width of the graph is the minimum width of any hierarchical clustering.");
    create.source("s7OvjQ", "https://en.wikipedia.org/wiki/Graph_bandwidth")
        .defined("9n7dry", Unknown, &bandwidth, "(paraphrased) Label graph vertices with distinct integers. Bandwidth of this labelling is the maximum over label differences over all edges. Bandwidth of a graph is the minimum over all labellings.");
    create.source("iWUynL", "https://en.wikipedia.org/wiki/Bisection_bandwidth")
        .defined("Kj73IQ", Unknown, &bisection_bandwidth, "... bisected into two equal-sized partitions, the bisection bandwidth of a network topology is the bandwidth available between the two partitions.");
    create.source("AeRM2B", "http://parallelcomp.github.io/Lecture3.pdf")
        .defined("w15E7O", Unknown, &bisection_bandwidth, "(number of) links across smallest cut that divides nodes in two (nearly) equal parts");
    create.source("BJhqpe", "https://en.wikipedia.org/wiki/Feedback_vertex_set")
        .defined("xPcvEf", Unknown, &feedback_vertex_set, "... a feedback vertex set (FVS) of a graph is a set of vertices whose removal leaves a graph without cycles... . The feedback vertex set number of a graph is the size of a smallest feedback vertex set.");
    create.source("4Dua5N", "https://www.fi.muni.cz/~hlineny/papers/shrubdepth-warw18-slipp.pdf")
        .defined("zWFoL1", Unknown, &shrub_depth, "Tree-model of $m$ colors and depth $d$: a rooted tree $T$ of height $d$, leaves are the vertices of $G$, each leaf has one of $m$ colors, an associated signature determining the edge set of $G$ as follows: for $i=1,2,\\dots,d$, let $u$ and $v$ be leaves with the least common ancestor at height $i$ in $T$, then $uv \\in E(G)$ iff the color pair of $u,v$ is in the signature at height $i$.");
    create.source("dxaIhi", "https://mathworld.wolfram.com/Pathwidth.html")
        .defined("OivGaa", Unknown, &pathwidth, "The pathwidth of a graph $G$, also called the interval thickness, vertex separation number, and node searching number, is one less than the size of the largest set in a path decomposition G.");
    create.source("W4j934", "https://en.wikipedia.org/wiki/Treewidth")
        .defined("LNtnP9", Unknown, &treewidth, "..., the treewidth of an undirected graph is an integer number which specifies, informally, how far the graph is from being a tree.");
    create.source("xnhT1P", "https://www.mimuw.edu.pl/~malcin/book/parameterized-algorithms.pdf")
        .defined("96BXHn", Unknown, &treewidth, "Very roughly, treewidth captures how similar a graph is to a tree. There are many ways to define ``tree-likeness'' of a graph; ... it appears that the approach most useful from algorithmic and graph theoretical perspectives, is to view tree-likeness of a graph $G$ as the existence of a structural decomposition of $G$ into pieces of bounded size that are connected in a tree-like fashion. This intuitive concept is formalized via the notions of a *tree decomposition* and the *treewidth* of a graph; the latter is a quantitative measure of how good a tree decomposition we can possibly obtain.");
    create.source("ZhBkjd", "https://en.wikipedia.org/wiki/Branch-decomposition")
        .defined("0SLCxV", Unknown, &branch_width, "... branch-decomposition of an undirected graph $G$ is a hierarchical clustering of the edges of $G$, represented by an unrooted binary tree $T$ with the edges of $G$ as its leaves. Removing any edge from $T$ partitions the edges of $G$ into two subgraphs, and the width of the decomposition is the maximum number of shared vertices of any pair of subgraphs formed in this way. The branchwidth of $G$ is the minimum width of any branch-decomposition of $G$.");
    create.source("9Ckusi", "https://en.wikipedia.org/wiki/Clique-width")
        .defined("pLDACG", Unknown, &clique_width, "... the minimum number of labels needed to construct G by means of the following 4 operations: 1. Creation of a new vertex... 2. Disjoint union of two labeled graphs... 3. Joining by an edge every vertex labeled $i$ to every vertex labeled $j$, where $i \\ne j$ 4. Renaming label $i$ to label $j$");
    create.source("pjVGlR", "https://www.sciencedirect.com/science/article/pii/S0095895605001528")
        .defined("JTZTcU", Unknown, &rank_width, "see Section 6");
    create.source("nyaOye", "https://dl.acm.org/doi/10.1145/3486655")
        .defined("s5Ktq7", Unknown, &twin_width, "... we consider a sequence of graph $G_n,G_{n-1},\\dots,G_2,G_1$, where $G_n$ is the original graph $G$, $G_1$ is the one-vertex graph, $G_i$ has $i$ vertices, and $G_{i-1}$ is obtained from $G_i$ by performing a single contraction of two (non-necessarily adjacent) vertices. For every vertex $u \\in V(G_i)$, let us denote by $u(G)$ the vertices of $G$ which have been contracted to $u$ along the sequence $G_n,\\dots,G_i$. A pair of disjoint sets of vertices is homogeneous if, between these sets, there are either all possible edges or no edge at all. The red edges mentioned previously consist of all pairs $uv$ of vertices of $G_i$ such that $u(G)$ and $v(G)$ are not homogeneous in $G$. If the red degree of every $G_i$ is at most $d$, then $G_n,G_{n-1},\\dots,G_2,G_1$ is called a sequence of $d$-contractions, or $d$-sequence. The twin-width of $G$ is the minimum $d$ for which there exists a sequence of $d$-contractions.");
    create.source("YGmwCG", "https://en.wikipedia.org/wiki/Book_embedding")
        .defined("jiDWoN", Unknown, &book_thickness, "... a book embedding is a generalization of planar embedding of a graph to embeddings into a book, a collection of half-planes all having the same line as their boundary. Usually, the vertices of the graph are required to lie on this boundary line, called the spine, and the edges are required to stay within a single half-plane. The book thickness of a graph is the smallest possible number of half-planes for any book embedding of the graph.");
    create.source("WY6oNX", "https://link.springer.com/chapter/10.1007/978-3-642-03367-4_25")
        .defined("1juCAg", Unknown, &hindex, "... $h$ is the $h$-index of the graph, the maximum number such that the graph contains $h$ vertices of degree at least $h$.");
    create.source("cNSdgE", "https://www.graphclasses.org/classes/par_31.html")
        .defined("JpPGki", Unknown, &acyclic_chromatic_number, "The acyclic chromatic number of a graph $G$ is the smallest size of a vertex partition $V_1,\\dots,V_\\ell$ such that each $V_i$ is an independent set and for all $i,j$ that graph $G[V_i \\cup V_j]$ does not contain a cycle.");
    create.source("rj2m4h", "https://en.wikipedia.org/wiki/Acyclic_coloring")
        .defined("PQ9STH", Unknown, &acyclic_chromatic_number, "... an acyclic coloring is a (proper) vertex coloring in which every 2-chromatic subgraph is acyclic.");
    create.source("6LCwBu", "https://en.wikipedia.org/wiki/Degeneracy_(graph_theory)")
        .defined("TYABmf", Unknown, &degeneracy, "... the least $k$ for which there exists an ordering of the vertices of $G$ in which each vertex has fewer than $k$ neighbors that are earlier in the ordering.");
    create.source("VqwUmp", "https://mathworld.wolfram.com/ChromaticNumber.html")
        .defined("VLEw7q", Unknown, &chromatic_num, "The chromatic number of a graph G is the smallest number of colors needed to color the vertices of G so that no two adjacent vertices share the same color (Skiena 1990, p. 210), ...");
    create.source("o6tFCJ", "https://bookdown.org/omarlizardo/_main/2-7-average-degree.html")
        .defined("PUQ3kt", Unknown, &average_degree, "Average degree is simply the average number of edges per node in the graph. ... Total Edges/Total Nodes=Average Degree");
    create.source("PVi4lL", "https://mathworld.wolfram.com/MaximumClique.html")
        .defined("Nm1F3M", Unknown, &max_clique, "A maximum clique of a graph $G$ is a clique (i.e., complete subgraph) of maximum possible size for $G$.");
    create.source("ZunX1e", "https://mathworld.wolfram.com/EdgeConnectivity.html")
        .defined("2gQP1W", Unknown, &edge_connectivity, "The edge connectivity, also called the line connectivity, of a graph is the minimum number of edges $\\lambda(G)$ whose deletion from a graph $G$ disconnects $G$.");
    create.source("XWbXPm", "https://en.wikipedia.org/wiki/Boxicity")
        .defined("PgaxqR", Unknown, &boxicity, "The boxicity of a graph is the minimum dimension in which a given graph can be represented as an intersection graph of axis-parallel boxes.");
    create.source("8eXjAy", "https://mathworld.wolfram.com/DomaticNumber.html")
        .defined("TG2BEi", Unknown, &domination_num, "The maximum number of disjoint dominating sets in a domatic partition of a graph $G$ is called its domatic number $d(G)$. ");
    create.source("zYzUJ5", "https://onlinelibrary.wiley.com/doi/abs/10.1002/jgt.3190170210")
        .defined("Xdg7Hv", Unknown, &chordality, "The \\emph{chordality} of a graph $G=(V,E)$ is defined as the minimum $k$ such that we can write $E=E_1,\\cap\\dots\\cap E_k$ with each $(V,E_i)$ a chordal graph.");
    create.source("TKnuNP", "https://www.sciencedirect.com/science/article/pii/0166218X9290275F?via%3Dihub")
        .defined("zqGB0p", Unknown, &max_induced_matching, "An induced matching in a graph G is a set of edges, no two of which meet a common node or are joined by an edge of G;");
    create.source("055mG5", "https://en.wikipedia.org/wiki/Distance_(graph_theory)#Related_concepts")
        .defined("OaKBaL", Unknown, &diameter, "... [diameter] is the greatest distance between any pair of vertices ...");
    create.source("GfSsR4", "https://onlinelibrary.wiley.com/doi/abs/10.1002/jgt.3190120309")
        .defined("sBhhEO", Unknown, &average_degree, "The average distance in a graph is defined as the average length of a shortest path between two vertices, taken over all pairs of vertices.");
    create.source("u13WN1", "https://en.wikipedia.org/wiki/Girth_(graph_theory)")
        .defined("INk53D", Unknown, &girth, "In graph theory, the girth of an undirected graph is the length of a shortest cycle contained in the graph.");
    create.source("8eXjAy", "https://mathworld.wolfram.com/DomaticNumber.html")
        .defined("oTPnV8", Unknown, &domatic_num, "The maximum number of disjoint dominating sets in a domatic partition of a graph $G$ is called its domatic number $d(G)$. ");
    create.source("7K6dAT", "https://dl.acm.org/doi/10.1145/3486655")
        .showed("08lETp", Unknown, &boolean_width, &twin_width, UpperBound(Exponential), "Theorem 3: Every graph with boolean-width $k$ has twin-width at most $2^{k+1}-1$.");

    let bandwidth_on_trees = create.intersection("Iu05N3", &tree, &bandwidth, "tree+bandwidth");
    let cutwidth_on_trees = create.intersection("peyWzt", &tree, &cutwidth, "tree+cutwidth");
    let pathwidth_maxdeg = create.intersection("6BWcgd", &pathwidth, &max_degree, "pathwidth+maxdegree");

    let chung1985 = create.source("DkY1Aq", "Chung1985")
        .showed("YgJVvi", Unknown, &bandwidth_on_trees, &cutwidth_on_trees, UpperBound(Linear), "")
        .showed("pRjX8u", Unknown, &cutwidth_on_trees, &bandwidth_on_trees, UpperBound(Linear), "")
        .todo();
    let robertson_seymour1986 = create.source("", "RobertsonSymour1986")
        .defined("", Pp(1), &treewidth, "A \\emph{tree-decomposition} of $G$ is a family $(X_i \\colon i\\in I)$ of subsets of $V(G)$, together with a tree $T$ with $V(T)=I$, with the following properties. (W1) $\\bigcup(X_i \\colon i \\in I)=V(G)$. (W2) Every edge of $G$ has both its ends in some $X_i$ ($i \\in I$). (W3) For $i,j,k \\in I$, if $j$ lies on the path of $T$ from $i$ to $k$ then $X_i \\cap X_k \\subseteq X_j$. The \\emph{width} of the tree-decomposition is $\\max(|X_i|-1 \\colon i \\in I)$. The tree-width of $G$ is the minimum $w \\ge 0$ such that $G$ has a tree-decomposition of width $\\le w$.")
        .defined("", Pp(1), &treewidth, "Equivalently, the tree-width of $G$ is the minimum $w \\ge 0$ such that $G$ is a subgraph of a ``chordal'' graph with all cliques of size at most $w + 1$.")
        // .showed("", Pp(2), "(1.2) For any fixed integer $w$, there is a polynomial algorithm to decide if the input graph has tree-width $\\le w$.") // non-constructive
        // .showed("", Pp(6), treewidth, minor_closed, "(2.7) If $G$ has tree-width $< w$, so does ever minor of $G$.")
        .done();
    let robertson_seymour1991 = create.source("", "RobertsonSymour1991")
        .defined("", Pp(12), &branch_width, "A \\emph{branch-width} of a hypergraph $G$ is a pair $(T,\\tau)$, where $T$ is a ternary tree and $\\tau$ is a bijection from the set of leaves of $T$ to $E(G)$. The \\emph{order} of an edge $e$ of $T$ is the number of vertices $v$ of $G$ such that there are leaves $t_1,t_2$ of $T$ in different components of $T \\setminus e$, with $\\tau(t_1),\\tau(t_2)$ both incident with $v$. The \\emph{width} of $(T,\\tau)$ is the maximum order of the edges of $T$, and the \\emph{branch-width} $\\beta(G)$ of $G$ is the minimum width of all branch-decompositions of $G$ (or 0 if $|E(G)| \\le 1$, when $G$ has no branch-decompositions).")
        // .showed("", Pp(12), "(4.1) If $H$ is a minor of a graph $G$, then $\\beta(H) \\le \\beta(G)$.")
        .showed("8ewSpI", Pp(16), &treewidth, &branch_width, UpperBound(Linear), "(5.1) For any hypergraph $G$, $\\max(\\beta(G), \\gamma(G)) \\le \\omega(G) + 1 \\le \\max(\\lfloor(3/2)\\beta(G)\\rfloor, \\gamma(G), 1)$.")
        .showed("wrBAYk", Pp(16), &branch_width, &treewidth, UpperBound(Linear), "(5.1) For any hypergraph $G$, $\\max(\\beta(G), \\gamma(G)) \\le \\omega(G) + 1 \\le \\max(\\lfloor(3/2)\\beta(G)\\rfloor, \\gamma(G), 1)$.") // todo join with previous
        .done();
    let bodlaender_mohring1993 = create.source("a3yKzk", "BodlaenderMohring1993")
        .showed("cIAr80", Pp(4), &complete, &treewidth, Exclusion, "Lemma 3.1 (\"clique containment lemma\"). Let $(\\{X_i\\mid u\\in I\\},T=(I,F))$ be a tree-decomposition of $G=(V,E)$ and let $W \\subseteq V$ be a clique in $G$. Then there exists $i \\in I$ with $W \\subseteq X_i$.")
        .showed("", Pp(4), &bipartite, &treewidth, Exclusion, "Lemma 3.2 (\"complete bipartite subgraph containment lemma\").")
        // .showed("", Pp(5), &create.intersection("", &cograph, &treewidth, ""), &create.intersection("", &cograph, &pathwidth, ""), Equivalence, "Theorem 3.2. For every cograph $G = (V,E)$, $treewidth(G) = pathwidth(G)$.")
        // .showed(Theorem 4.1. The pathwidth and treewidth of a cograph given with a corresponding cotree can be computed in $O(n)$ time.)
        .done();
    let bodlaender1998 = create.source("BOFCWg", "Bodlaender1998")
        .showed("uHJAUo", Pp(4), &pathwidth, &treewidth, UpperBound(Linear), "Lemma 3. (a) For all graphs $G$, $pathwidth(G) \\ge treewidth(G)$. ...")
        .defined("", Pp(5), &branch_width, "A \\emph{branch decomposition} of a graph $G=(V,E)$ is a pair $(T=(I,F),\\sigma)$, where $T$ is a tree with every node in $T$ of degree one of three, and $\\sigma$ is a bijection from $E$ to the set of leaves in $T$. The \\emph{order} of an edge $f \\in F$ is the number of vertices $v \\in V$, for which there exist adjacent edges $(v,w),(v,x) \\in E$, such that the path in $T$ from $\\sigma(v,w)$ to $\\sigma(v,x)$ uses $f$. The \\emph{width} of branch decomposition $(T=(I,F),\\sigma)$, is the maximum order over all edges $f \\in F$. The \\emph{branchwidth} of $G$ is the minimum width over all branch decompositions of $G$.")
        // page 6, tw and pw do not rise for subgraphs
        // mark
        .defined("hajrD0", Pp(22), &bandwidth, "Let $G=(V,E)$ be a graph, and let $f\\colon V\\to \\{1,2,\\dots,n\\}$ be a linear ordering of $G$. 1. The \\emph{bandwidth} of $f$ is $\\max\\{|f(v)-f(w)| \\mid (v,w) \\in E\\}$. ... The bandwidth ... is the minimum bandwidth ... over all possible linear orderings of $G$.")
        .defined("c6Hdu3", Pp(22), &cutwidth, "Let $G=(V,E)$ be a graph, and let $f\\colon V\\to \\{1,2,\\dots,n\\}$ be a linear ordering of $G$. ... 2. The \\emph{cutwidth} of $f$ is $\\max_{1\\le i\\le n} |\\{(u,v)\\in E \\mid f(u) \\le i < f(v) \\}|$. ... [cutwidth] of a graph $G$ is the minimum [cutwidth] ... over all possible linear orderings of $G$.")
        .defined("H3lAh2", Pp(22), &topological_bandwidth, "The \\emph{topological bandwidth} of a graph $G$ is the minimum [bandwidth](../aP5a38) over all graphs $G'$ which are obtained by addition of an arbitrary number of vertices along edges of $G$.")
        .showed("kiza4J", Pp(23), &bandwidth, &pathwidth, UpperBound(Linear), "Theorem 44. For every graph $G$, the pathwidth of $G$ is at most the bandwidth of $G$, ... Proof. Let $f \\colon V\\to \\{1,\\dots,n\\}$ be a linear ordering of $G$ with bandwidth $k$. Then $(X_1,\\dots,X_{n-k})$ with $X_i=\\{f^{-1}(i), f^{-1}(i+1), \\dots, f^{-1}(i+k)\\} is a path decomposition of $G$ with pathwidth $k$. ...")
        .showed("RQriva", Pp(23), &topological_bandwidth, &pathwidth, UpperBound(Linear), "Theorem 45. For every graph $G$, the pathwidth of $G$ is at most the topological band-width of $G$.")
        .showed("iiE5jo", Pp(24), &cutwidth, &pathwidth, UpperBound(Linear), "Theorem 47. For every graph $G$, the pathwidth of $G$ is at most the cutwidth of $G$.")
        .showed("RgLQ2P", Pp(24), &pathwidth_maxdeg, &cutwidth, UpperBound(Linear), "Theorem 49.")
        .showed("", Pp(24), &cutwidth, &pathwidth_maxdeg, UpperBound(Linear), "Theorem 49.") // join with the above
        .showed("", Pp(34), &outerplanar, &treewidth, UpperBound(Constant), "Lemma 78. Every outerplanar graph $G=(V,E)$ has treewidth at most 2.") // check whether dist_to_outerplanar bounding treewidth infered from this?
        .showed("", Pp(37), &grid, &treewidth, Exclusion, "Lemma 88. The treewidth of an $n \\times n$ grid graph ... is at least $n$.")
        .showed("", Pp(38), &treewidth, &min_degree, UpperBound(Linear), "Lemma 90 (Scheffler [94]). Every graph of treewidth at most $k$ contains a vertex of degree at most $k$.") // todo Schemer, Die Baumweite von Graphen als ein Ma8 Rir die Kompliziertheit algorithmischer Probleme, Ph.D. Thesis, Akademie der Wissenschafien der DDR, Berlin, 1989.
        .done();
    let corneil2005 = create.source("HCGunF", "Corneil2005")
        .showed("sGBrPC", Unknown, &treewidth, &clique_width, UpperBound(Exponential), "... the clique-width of $G$ is at most $3 \\cdot 2k - 1$ and, more importantly, that there is an exponential lower bound on this relationship. In particular, for any $k$, there is a graph $G$ with treewidth equal to $k$, where the clique-width of $G$ is at least $2\\lfloor k/2\\rfloor-1$.")
        .todo();
    let oum2006 = create.source("1ZTWBd", "Oum2006")
        .showed("yLdAHe", Unknown, &rank_width, &clique_width, Exactly(Exponential), "Proposition 6.3")
        .showed("uEUXMq", Unknown, &clique_width, &rank_width, UpperBound(Linear), "Proposition 6.3")
        .todo();
    let sasak2010 = create.source("XlBXyo", "Sasak2010")
        .showed("WiiQn4", Pp(38), &cutwidth, &carving_width, UpperBound(Linear), "Theorem 4.3 (carw $\\prec$ cutw) Carving-width is bounded by cut-width.")
        .showed("u5VPeX", Pp(49), &carving_width, &treewidth, UpperBound(Linear), "Theorem 5.5 (tw $\\prec$ carw) Tree-width is bounded by carving-width.")
        .showed("BwIc79", Pp(28), &cutwidth, &max_degree, UpperBound(Linear), "Lemma 2.18. For any graph $G$ and any vertex $v \\in V(G), cutw(g) \\ge \\lceil \\frac{deg(v)}2 \\rceil$.")
        .todo();
    let bui_xuan2011 = create.source("cNjhWx", "BuiXuan2011")
        .defined("L7aY6D", Unknown, &boolean_width, "\\textbf{Definition 1.} A decomposition tree of a graph $G$ is a pair $(T,\\delta)$ where $T$ is a tree having internal nodes of degree three and $\\delta$ a bijection between the leaf set of $T$ and the vertex set of $G$. Removing an edge from $T$ results in two subtrees, and in a cut $\\{A,\\comp{A}\\}$ of $G$ given by the two subsets of $V(G)$ in bijection $\\delta$ with the leaves of the two subtrees. Let $f\\colon w^V \\to \\mathbb{R}$ be a symmetric function that is also called a cut function: $f(A)=f(\\comp{A})$ for all $A\\subseteq V(G)$. The $f$-width of $(T,\\delta)$ is the maximum value of $f(A)$ over all cuts $\\{A,\\comp{A}\\}$ of $G$ given by the removal of an edge of $T$. ... \\textbf{Definition 2.} Let $G$ be a graph and $A \\subseteq V(G)$. Define the set of unions of neighborhoods of $A$ across the cut $\\{A,\\comp{A}\\}$ as $U(A) = \\{Y \\subseteq \\comp{A} \\mid \\exists X \\subseteq A \\land Y=N(X)\\cap \\comp{A}\\}. The \\emph{bool-dim}$\\colon 2^{V(G)} \\to \\mathbb{R}$ function of a graph $G$ is defined as $\\mathrm{bool-dim}(A)=\\log_2 |U(A)|$. Using Definition 1 with $f=\\mathrm{bool-dim}$ we define the boolean-width of a decomposition tree, denoted by $boolw(T,\\delta)$, and the boolean-width of a graph, denoted by $boolw(G)$.")
        .showed("AdNkCy", Unknown, &boolean_width, &rank_width, UpperBound(Exponential), "\\textbf{Corollary 1.} For any graph $G$ and decomposition tree $(T,\\gamma)$ of $G$ it holds that ... $\\log_2 rw(G) \\le boolw(G)$ ...")
        .showed("cIWQDn", Unknown, &rank_width, &boolean_width, UpperBound(Polynomial), "\\textbf{Corollary 1.} For any graph $G$ and decomposition tree $(T,\\gamma)$ of $G$ it holds that ... $boolw(G) \\le \\frac 14 rw^2(G)+O(rw(G))$.")
        .todo();
    let ganian_twin_cover2012 = create.source("7UoBR6", "GanianTwinCover2012")
        .defined("k6ApS2", Pp(262), &twin_cover_num, "Definition 3.1. $X \\subseteq V(G)$ is a twin-cover of $G$ if for every edge $e=\\{a,b\\} \\in E(G)$ either 1. $a \\in X$ or $b \\in X$, or 2. $a$ and $b$ are twins, i.e. all other vertices are either adjacent to both $a$ and $b$ or none. We then say that $G$ has twin-cover number $k$ if $k$ is the minimum possible size of a twin-cover of $G$.")
        .defined("pFk5uY", Pp(262), &twin_cover_num, "Definition 3.2. $X \\subseteq V(G)$ is a twin-cover of $G$ if there exists a subgraph $G'$ of $G$ such that 1. $X \\subseteq V(G')$ and $X$ is a vertex cover of $G'$. 2. $G$ can be obtained by iteratively adding twins to non-cover vertices in $G'$.")
        .showed("oxtaEE", Pp(263), &complete, &twin_cover_num, UpperBound(Constant), "We note that complete graphs indeed have a twin-cover of zero.")
        .showed("nkOAMh", Pp(263), &twin_cover_num, &vertex_cover, Exclusion, "The vertex cover of graphs of bounded twin-cover may be arbitrarily large.")
        .showed("gmsOd4", Pp(263), &twin_cover_num, &treewidth, Exclusion, "There exists graphs with arbitrarily large twin-cover and bounded tree-width and vice-versa.")
        .showed("iG3eGq", Pp(263), &treewidth, &twin_cover_num, Exclusion, "There exists graphs with arbitrarily large twin-cover and bounded tree-width and vice-versa.") // todo simplify to Incomparable
        .showed("E8oHKm", Pp(263), &twin_cover_num, &clique_width, UpperBound(Linear), "The clique-width of graphs of twin-cover $k$ is at most $k+2$.")
        .showed("qB058E", Pp(263), &twin_cover_num, &rank_width, UpperBound(Linear), "The rank-width and linaer rank-width of graph of twin-cover $k$ are at most $k+1$.")
        .showed("WZcIOW", Pp(263), &twin_cover_num, &linear_rank_width, UpperBound(Linear), "The rank-width and linaer rank-width of graph of twin-cover $k$ are at most $k+1$.")
        // .tractable("XwMEnS", Pp(263), &twin_cover_num, &twin_cover_num, FPT, "Theorem 3.4. It is possible to find a twin-cover of size $k$ in time $O(|E||V|+k|V|+1.2738^k)$.")
        // .tractable("PxaiDG", Pp(267), &twin_cover_num, &boxicity, FPT, "Theorem 4.6. The Boxicity
        // problem can be solved in time $2^{O(2^kk^2)}|V|$ on graph of twin-cover at most $k$.")
        .done();
    let belmonte2013 = create.source("sJ476m", "Belmonte2013")
        .showed("ZHXKjC", Unknown, &carving_width, &max_degree, UpperBound(Linear), "Observation 1. Let $G$ be a graph. Then $cw(G) \\ge \\Delta(G)$.")
        .todo();
    let jansen2013 = create.source("FLOjic", "Jansen2013")
        .defined("PV6tGG", Unknown, &topological_bandwidth, "The \\emph{topological bandwidth} of a graph $G$ is the minimum [bandwidth](../aP5a38) over all subdivisions of $G$")
        .todo();
    let parameterized_algorithms2015 = create.source("Xlsyce", "ParameterizedAlgorithms2015")
        .todo();
    let diestel2017 = create.source("r2Lwky", "Diestel2017")
        .defined("hxpfbI", Pp(3), &complete, "If all the vertices of $G$ are pairwise adjacent, then $G$ is \\emph{complete}.")
        .defined("T8RJcC", Pp(5), &edgeless, "A vertex of degree $0$ is \\emph{isolated}.")
        .defined("8XlBpy", Pp(13), &forest, "An acyclic graph, one not containing any cycles, is called a \\emph{forest}.")
        .defined("P1ExcE", Pp(17), &bipartite, "Instead of `2-partite' one usually says bipartite.")
        .defined("eMZCoY", Pp(89), &planar, "When we draw a graph on a piece of paper, we naturally try to do this as transparently as possible. One obvious way to limit the mess created by all the lines is to avoid intersections. ... Graphs drawn in this way are called \\emph{plane graphs}; abstract graphs that can be drawn in this way are called \\emph{planar}.")
        .defined("6Q0kuL", Pp(115), &outerplanar, "A graph is called outerplanar if it has a drawing in which every vertex lies on the boundary of the outer face.")
        .defined("wkrz7h", Pp(135), &chordal, "... a graph is chordal (or triangulated) if each of its cycles of length at least $4$ has a chord, i.e. if it contains no induced cycles other than triangles.")
        .defined("54XChb", Pp(135), &perfect, "A graph is called perfect if every induced subgraph $H \\subseteq G$ has chromatic number $\\chi(H)=\\omega(H)$, i.e. if the trivial lower bound of $\\omega(H)$ colours always suffices to colour the vertices of $H$.")
        .defined("pMo8dB", Pp(145), &interval, "A graph $G$ is called an \\emph{interval graph} if there exists a set $\\{ I_v \\mid v \\in V(G) \\}$ of real intervals such that $I_u \\cap I_v \\ne \\emptyset$ if and only if $uv \\in E(G)$.")
        .todo();
    let froemmrich2018 = create.source("45xW87", "Froemmrich2018")
        .todo();
    let ganian2019 = create.source("Scw7zm", "Ganian2019")
        .showed("TUftFh", Unknown, &shrub_depth, &linear_clique_width, UpperBound(Linear), "Proposition 3.4. Let $\\mathcal G$ be a graph class and $d$ an integer. Then: ... b) If $\\mathcal G$ is of bounded shrub-depth, then $\\mathcal G$ is of bounded linear clique-width.")
        .showed("EG7vp6", Unknown, &neighborhood_diversity, &shrub_depth, UpperBound(Exists), "$\\mathcal{TM}_m(1)$ is exactly the class of graphs of neighborhood diversity at most $m$.")
        .showed("sq0brL", Unknown, &treedepth, &shrub_depth, UpperBound(Linear), "Proposition 3.2. If $G$ is of tree-depth $d$, then $G \\in \\mathcal{TM}_{2^d}(d)$. ...")
        .todo();
    let sorge2019 = create.source("VnTIL0", "Sorge2019")
        .defined("ddviDI", Pp(3), &acyclic_chromatic_number, "The \\emph{acyclic chromatic number} of a graph $G = (V,E)$ is the smallest size of a vertex partition $P=\\{V_1,\\dots,V_\\ell\\}$ such that each $V_i$ is an independent set and for all $V_i,V_j$ the graph $G[V_i \\cup V_j]$ does not contain a cycle.")
        // .cited("KrMa3o", Pp(3), &grunbaum1973, "Introduced by Grünbaum [18]")
        // .defined("aUvKTa", Pp(3), &path_number, "The \\emph{path number} of a graph $G$ is the minimum number of paths the edges of $G$ can be partitioned into [2].")
        // .defined("EedT02", Pp(3), &arboricity, "The \\emph{arboricity} of a graph $G$ is the minimum number of forests the edges of $G$ can be partitioned into.") // todo covering by trees
        // .defined("iUqFsf", Pp(3), &vertex_arboricity, "The \\emph{vertex arboricity} (or ``point arboricity'') of a graph $G$ is the minimum number of vertex subsets $V_i$ of $G$ such that $G[V_i]$ induces a forest for each $i$. ... [2]")
        .defined("77Klw8", Pp(3), &average_degree, "The \\emph{average degree} of a graph $G = (V,E)$ is $2|E|/|V|$.")
        .todo();
    let schroder_thesis = create.source("DYGiYb", "SchroderThesis")
        .cited("pJxHVS", Unknown, sorge2019, "Based on the work by [Sa19] as well as [Fr8], we investigate unknown connections between graph parameters to continue the work on the graph parameter hierarchy")
        .cited("bybFgo", Unknown, froemmrich2018, "Based on the work by [Sa19] as well as [Fr8], we investigate unknown connections between graph parameters to continue the work on the graph parameter hierarchy")
        .showed("R9eI61", Unknown, &treedepth, &diameter, UpperBound(Linear), "Proposition 3.1")
        .showed("dohKmq", Unknown, &dist_to_linear_forest, &hindex, UpperBound(Linear), "Proposition 3.2")
        .todo();
    let tran2022 = create.source("uXViPE", "Tran2022")
        .defined("J1sHj8", Pp(15), &twin_cover_num, "An edge $\\{v,w\\}$ is a twin edge if vertices $v$ and $w$ have the same neighborhood excluding each other. The twin cover number $tcn(G)$ of a graph $G$ is the size of a smallest set $V' \\subseteq V(G)$ of vertices such that every edge in $E(G)$ is either a twin edge or incident to a vertex in $V'$")
        .defined("MlTT7n", Pp(15), &edge_clique_cover, "The edge clique cover number $eccn(G)$ of a graph $G$ is the minimum number of complete subgraphs required such that each edge is contained in at least one of them.")
        .defined("iAkCJ3", Pp(15), &neighborhood_diversity, "The neighborhood diversity $nd(G)$ of a graph $G$ is the smallest number $k$ such that there is a $k$-partition $(V_1,\\dots,V_k)$ of $G$, where each subset $V_i$, $i \\in [k]$ is a module and is either a complete set or an independent set.")
        .defined("i3su9L", Pp(15), &modular_width, "The modular-width $mw(G)$ of a graph $G$ is the smallest number $h$ such that a $k$-partition $(V_1,\\dots,V_k)$ of $G$ exists, where $k \\le h$ and each subset $V_i$, $i \\in [k]$ is a module and either contains a single vertex or for which the modular-subgraph $G[V_i]$ has a modular-width of $h$.")
        .defined("Fhp3Dr", Pp(15), &c_closure, "The c-closure $\\mathrm{cc}(G)$ of a graph $G$ is the smallest number $c$ such that any pair of vertices $v,w \\in V(G)$ with $|N_G(v) \\cap N_G(w)| \\ge c$ is adjacent. ...")
        .defined("eGC0vH", Pp(17), &boxicity, "The boxicity of a graph $G$ is the minimum amount of interval graphs required, such that their intersecten results in $G$")
        .defined("gZtk6B", Pp(17), &chordality, "The chordality of a graph $G$ is the minimum amount of chordal graphs required, such that their intersecten results in $G$")
        .showed("L0BALz", Pp(19), &vertex_cover, &twin_cover_num, UpperBound(Linear), by_definition)
        .showed("e1r4xS", Pp(19), &complete, &twin_cover_num, UpperBound(Constant), "Note that a clique of size $n$ has a twin cover number of $0$...")
        .showed("kcsO0r", Pp(19), &complete, &vertex_cover, Exclusion, "Note that a clique of size $n$ has ... a vertex cover number of $n-1$")
        // .showed("am05hY", Pp(19), &twin_cover_num, &dist_to_cluster, Exclusion, "")
        // .showed("XEruuV", Pp(20), &twin_cover_num, &dist_to_complete, Exclusion, "")
        // .showed("jCROUz", Pp(20), &dist_to_complete, &twin_cover_num, Exclusion, "")
        .showed("mttPgQ", Pp(20), &twin_cover_num, &domatic_num, Exclusion, cliques_make_it_unbounded)
        .showed("39CEgf", Pp(20), &twin_cover_num, &max_clique, Exclusion, cliques_make_it_unbounded)
        .showed("DaQuOK", Pp(20), &twin_cover_num, &edge_connectivity, Exclusion, cliques_make_it_unbounded)
        // .showed("Jh68En", Pp(22), &twin_cover_num, &dist_to_co_cluster, Exclusion, "")
        // .showed("YSMSiS", Pp(22), &dist_to_co_cluster, &twin_cover_num, Exclusion, "")
        .showed("TddK24", Pp(23), &edge_clique_cover, &neighborhood_diversity, UpperBound(Linear), "Theorem 4.1. Edge Clique Cover Number strictly upper bounds Neighborhood Diversity.").showed("UAYKMq", Pp(23), &neighborhood_diversity, &edge_clique_cover, Exclusion, "Theorem 4.1. Edge Clique Cover Number strictly upper bounds Neighborhood Diversity.") // todo simplify strict relations
        .showed("O9TmJs", Pp(24), &dist_to_complete, &edge_clique_cover, UpperBound(Linear), "Proposition 4.2. Disatnce to Clique strictly upper bounds Edge Clique Cover Number.").showed("CKyJfe", Pp(24), &edge_clique_cover, &dist_to_complete, Exclusion, "Proposition 4.2. Disatnce to Clique strictly upper bounds Edge Clique Cover Number.")
        .showed("gKXcun", Pp(25), &path, &modular_width, Exclusion, "The Modular-width of a path $P$ with length $n > 3$ is $n$.")
        // .showed("wFtd4d", Pp(25), &modular_width, &complement(modular_width), Equivalence, "Given any graph $G$, $\\mathrm{mw}(G) = \\mathrm{mw}(\\bar G)$.")
        .showed("FeMtBe", Pp(26), &modular_width, &clique_width, UpperBound(Linear), "Proposition 4.6. Modular-width strictly upper bounds Clique-width.").showed("mKSql8", Pp(26), &clique_width, &modular_width, Exclusion, "Proposition 4.6. Modular-width strictly upper bounds Clique-width.")
        // Pp(26), "Theorem 4.7. Modular-width strictly upper bounds Max Diameter of Components."
        // .showed("78NYVs", Pp(26), &modular_width, &max_diameter_of_components, StrictlyUpperBounds, "Theorem 4.7. Modular-width strictly upper bounds Max Diameter of Components.")
        .todo(); // todo 27 onwards
    let torunczyk2023 = create.source("KpkMZB", "Torunczyk2023")
        .defined("9VHraO", Unknown, &inf_flip_width, "See radius-r flip-width for $r=\\infty$.")
        .defined("gxeVOT", Unknown, &r_flip_width, "The radius-$r$ flip-width of a graph $G$, denoted $fwr (G)$, is the smallest number $k \\in \\mathbb{N}$ such that the cops have a winning strategy in the flipper game of radius $r$ and width $k$ on $G$")
        .showed("9DTyeJ", Unknown, &inf_flip_width, &rank_width, UpperBound(Linear), "For every graph $G$, we have $\\mathrm{rankwidth}(G) \\le 3 \\mathrm{fw}_\\infty(G)+1 ...")
        .showed("zYQZyB", Unknown, &rank_width, &inf_flip_width, UpperBound(Exponential), "For every graph $G$, we have ... $3 \\mathrm{fw}_\\infty(G)+1 \\le O(2^{\\mathrm{rankwidth(G)}}).")
        .showed("OdbuZP", Unknown, &twin_width, &r_flip_width, UpperBound(Exponential), "Theorem 7.1. Fix $r \\in \\mathbb N$. For every graph $G$ of twin-width $d$ we have: $\\mathrm{fw}_r(G) \\le 2^d \\cdot d^{O(r)}.$")
        .showed("gvSCeQ", Unknown, &inf_flip_width, &r_flip_width, UpperBound(Linear), by_definition)
        .todo();

    create.build()
}
