//! Collects the information we have about parameterized complexity classes, their inclusions,
//! and related graph classes, topics, bibliographic sources, theorems, proofs, and so on.

use crate::general::enums::{Cpx::*, CpxTime::*, Page::*};
use crate::input::{raw::RawData, build::Builder};

pub fn build_collection() -> RawData {
    let mut create = Builder::new();

    let connected = create.graph_class("KlMP0i", "connected", 2);
    let bipartite = create.graph_class("cLHJkW", "bipartite", 8);
    let block = create.graph_class("QrxQsH", "block", 4);
    let chordal = create.graph_class("Cv1PaJ", "chordal", 5);
    let cluster = create.graph_class("WAU7vf", "cluster", 6);
    let co_cluster = create.graph_class("7HR4uV", "co-cluster", 6);
    let cograph = create.graph_class("9Qd0Mx", "cograph", 7);
    let complete = create.intersection("EhdXNA", &connected, &cluster, "complete", 9);
    let forest = create.graph_class("JngPPm", "forest", 9);
    let tree = create.intersection("rJyICu", &connected, &forest, "tree", 7);
    let interval = create.graph_class("p5skoj", "interval", 7);
    let edgeless = create.graph_class("LsiBbX", "edgeless", 1);
    let linear_forest = create.graph_class("skQuFN", "linear forest", 4);
    let path = create.intersection("ryPlqz", &connected, &linear_forest, "path", 3);
    let outerplanar = create.graph_class("0oCyaG", "outerplanar", 5);
    let perfect = create.graph_class("RmssrZ", "perfect", 6);
    let planar = create.graph_class("loZ5LD", "planar", 8);
    let stars = create.graph_class("10JR3F", "stars", 4);
    let star = create.intersection("CortlU", &connected, &stars, "star", 3);
    let cycles = create.graph_class("2iJr52", "cycles", 4);
    let cycle = create.intersection("Ti0asF", &connected, &cycles, "cycle", 2);
    let disjoint_cycles = create.graph_class("AGnF5Z", "disjoint cycles", 4);
    let grid = create.graph_class("lfYXuK", "grid", 6);
    // let disconnected = create.graph_class("lA0K71", "disconnected", 1);
    // let all_graphs = create.graph_class("TDTA85", "all graphs"); // hide

    create.unknown_source()
        .defined("2kG0kY", Unknown, &block, "Every block (maximal 2-connected subgraph) is a clique.")
        .defined("roSFzV", Unknown, &cluster, "Disjoint union of complete graphs.")
        .defined("pUnhFT", Unknown, &cluster, "Every connected component is a complete graph.")
        .defined("4aKorn", Unknown, &cluster, "Does not include path on three vertices as an induced subgraph.")
        .defined("FDbIDy", Unknown, &co_cluster, "Complete multipartite graph.")
        .defined("cBDurK", Unknown, &disjoint_cycles, "All cycles in the graph are disjoint. Can contain arbitrary trees attached to and between the cycles.")
        ;

    let by_definition = "By definition";

    create.assumed_source()
        .defined("51KDFn", Unknown, &stars, "Disjoint union of stars.")
        .defined("sp6LGE", Unknown, &grid, "Cartesian product of two paths.")
        .defined("sAZHF4", Unknown, &cycles, "Every component is a cycle.")
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
        .proper_graph_inclusion("HtdoRP", &edgeless, &co_cluster)
        .proper_graph_inclusion("1PLbSg", &grid, &planar)
        .proper_graph_inclusion("RQcVkC", &grid, &bipartite)
        .proper_graph_inclusion("CJ76wg", &cycles, &disjoint_cycles)
        .proper_graph_inclusion("CTwA2j", &grid, &connected)
        .proper_graph_inclusion("wTugFB", &edgeless, &cluster)
        .proper_graph_inclusion("1pdarO", &star, &tree)
        .proper_graph_inclusion("gAlyjK", &path, &tree)
        .proper_graph_inclusion("1pdarO", &path, &grid)
        ;

    let top_metric = create.topic("wpYsel", "metric", "Typically used in metric spaces.");
    let top_drawing = create.topic("lJJaYb", "drawing", "Closely tied to drawing the graph onto a topological space.");

    let vertex_cover = create.parameter("4lp9Yj", "vertex cover", 9);
    let max_matching = create.parameter("veU7Jf", "maximum matching", 3);
    let vertex_integrity = create.parameter("KVhJFB", "vertex integrity", 6);
    let treedepth = create.parameter("KEP2qM", "treedepth", 7);
    let clique_cover_num = create.parameter("VomShB", "clique cover number", 5);
    let max_independent_set = create.parameter("mHtXUU", "maximum independent set", 2);
    let domination_num = create.parameter("Gq0onN", "domination number", 3);
    let twin_cover_num = create.parameter("MUnHA0", "twin-cover number", 5);
    let edge_clique_cover = create.parameter("nYQDv6", "edge clique cover number", 4);
    let neighborhood_diversity = create.parameter("vMs3RS", "neighborhood diversity", 6);
    let modular_width = create.parameter("4bj71L", "modular-width", 7);
    let max_leaf_num = create.parameter("BN92vX", "maximum leaf number", 6);
    let feedback_edge_set = create.parameter("HTk9PZ", "feedback edge set", 6);
    let genus = create.parameter("gbaHdw", "genus", 2); // top_drawing
    let cutwidth = create.parameter("TLx1pz", "cutwidth", 4);
    let carving_width = create.parameter("dS6OgO", "carving-width", 3);
    let bandwidth = create.parameter("aP5a38", "bandwidth", 5);
    let topological_bandwidth = create.parameter("SnA7Eq", "topological bandwidth", 4);
    let bisection_bandwidth = create.parameter("wUdmUb", "bisection bandwidth", 4);
    // let reduced_bandwidth = create.reduced("reduced bandwidth", &bandwidth, 2);
    let max_degree = create.parameter("UyQ5yM", "maximum degree", 8);
    let c_closure = create.parameter("ou9VU1", "c-closure", 0);
    let feedback_vertex_set = create.parameter("GNOiyB", "feedback vertex set", 8);
    let shrub_depth = create.parameter("NTgNzT", "shrub-depth", 6);
    let linear_clique_width = create.parameter("fQj3wU", "linear clique-width", 5);
    let pathwidth = create.parameter("VHClqR", "pathwidth", 8);
    let pathwidth_maxdeg = create.intersection("6BWcgd", &pathwidth, &max_degree, "pathwidth+maxdegree", 3);
    let d_path_free = create.parameter("s4EiWI", "d-path-free", 2); // todo
    let treewidth = create.parameter("5Q7fuR", "treewidth", 9);
    let branch_width = create.parameter("lIcmuR", "branch width", 5);
    let clique_width = create.parameter("wg5HuV", "clique-width", 7);
    let clique_tree_width = create.parameter("7P9WUz", "clique-tree-width", 2);
    let rank_width = create.parameter("fojquT", "rank-width", 7);
    let linear_rank_width = create.parameter("cHugsk", "linear rank-width", 2);
    let boolean_width = create.parameter("A2jPWT", "boolean width", 5);
    let inf_flip_width = create.parameter("nYXiuT", "inf-flip-width", 3);
    let degree_treewidth = create.intersection("nCWUh3", &max_degree, &treewidth, "degree treewidth", 6);
    let twin_width = create.reduced("twin-width", &max_degree, 8);
    let r_flip_width = create.parameter("4DIiH0", "radius-r flip-width", 3);
    let book_thickness = create.parameter("doijTS", "book thickness", 4); // top_drawing; a.k.a. stacknumber, pagenumber, fixed outerthickness
    // .showed("1IL2wn", NotApplicable, &book_thickness, &create.edge_cover_by(&outerplanar), Equivalence, "") //but with fixed vertices
    let hindex = create.parameter("GNTwUS", "h-index", 4);
    let acyclic_chromatic_number = create.parameter("QGZuUW", "acyclic chromatic number", 5);
    let odd_cycle_transversal = create.parameter("Ve5ruW", "odd cycle transversal", 6);
    let degeneracy = create.parameter("VowkuW", "degeneracy", 6);
    let chromatic_num = create.parameter("w7MmyW", "chromatic number", 5);
    let average_degree = create.parameter("z0y4TW", "average degree", 2);
    let min_degree = create.parameter("GPmOeT", "minimum degree", 0);
    let max_clique = create.parameter("q7zHeT", "maximum clique", 5);
    let edge_connectivity = create.parameter("JbqZoT", "edge connectivity", 2);
    let vertex_connectivity = create.parameter("OyLUe4", "vertex connectivity", 2);
    let boxicity = create.parameter("a7MpiT", "boxicity", 6);
    let chordality = create.parameter("fTqo40", "chordality", 4);
    let max_induced_matching = create.parameter("GzMYlT", "maximum induced matching", 3);
    let diameter = create.parameter("p4bTjp", "diameter", 6);
    let average_distance = create.parameter("zH8PpT", "average distance", 3);
    let girth = create.parameter("BCwUeT", "girth", 1);
    let domatic_num = create.parameter("KRV6tI", "domatic number", 3);
    let arboricity = create.parameter("zgMenA", "arboricity", 5);
    let star_arboricity = create.parameter("Mvz8MX", "star-arboricity", 1);
    let mim_width = create.parameter("WmIFB1", "mim-width", 6);
    let nlc_width = create.parameter("Xrpbv7", "NLC-width", 4);
    let nlct_width = create.parameter("mOri44", "NLCT-width", 2);
    let linear_nlc_width = create.parameter("v09DMY", "linear NLC-width", 2);
    let thickness = create.parameter("sxTPYj", "thickness", 3);
    let outerthickness = create.parameter("MIeOwU", "outerthickness", 1);
    let bounded_components = create.parameter("t7c4mp", "bounded components", 3);
    let dist_to_complete = create.distance_to(&complete, 6);
    let dist_to_co_cluster = create.distance_to(&co_cluster, 5);
    let dist_to_cograph = create.distance_to(&cograph, 5);
    let dist_to_cluster = create.distance_to(&cluster, 5);
    let dist_to_linear_forest = create.distance_to(&linear_forest, 4);
    let dist_to_outerplanar = create.distance_to(&outerplanar, 3);
    let dist_to_block = create.distance_to(&block, 4);
    let dist_to_edgeless = create.distance_to(&edgeless, 1);
    let dist_to_forest = create.distance_to(&forest, 5);
    let dist_to_bipartite = create.distance_to(&bipartite, 6);
    let dist_to_planar = create.distance_to(&planar, 4);
    let dist_to_chordal = create.distance_to(&chordal, 4);
    let dist_to_stars = create.distance_to(&stars, 3);
    let dist_to_perfect = create.distance_to(&perfect, 4);
    let dist_to_interval = create.distance_to(&interval, 3);
    let dist_to_max_degree = create.distance_to(&max_degree, 4);
    let dist_to_bounded_components = create.distance_to(&bounded_components, 4);
    // let dist_to_disconnected = create.distance_to(&disconnected, 2);

    create.provider("ISGCI", "https://www.graphclasses.org/", Box::new(|id: &str|{
        format!(r"https://www.graphclasses.org/classes/par_{id}.html")
    }))
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
        .done();

    create.assumed_source().collective(NotApplicable, by_definition)
            // .showed("YOBod9", &vertex_connectivity, &dist_to_disconnected, Equivalence)
            .showed("LTyhoG", &vertex_integrity, &dist_to_bounded_components, UpperBound(Linear))
            .showed("wgnjMg", &dist_to_bounded_components, &vertex_integrity, UpperBound(Linear))
            .showed("SyGwqT", &bandwidth, &topological_bandwidth, UpperBound(Linear))
            .showed("ebAUEu", &twin_cover_num, &dist_to_cluster, UpperBound(Linear))
            .showed("2XN8ux", &vertex_cover, &twin_cover_num, UpperBound(Linear))
            .showed("XTPNkl", &average_degree, &min_degree, UpperBound(Linear))
            .showed("TezCU1", &diameter, &average_distance, UpperBound(Linear))
            .showed("qy7Xdi", &max_matching, &max_induced_matching, UpperBound(Linear))
            .showed("2gTckj", &dist_to_interval, &boxicity, UpperBound(Linear))
            .showed("LAc0Ur", &bisection_bandwidth, &edge_connectivity, UpperBound(Linear))
            .showed("yWSq1V", &edgeless, &bounded_components, UpperBound(Constant))
            .showed("KxMj5k", &grid, &max_degree, UpperBound(Constant))
            .showed("TxxhnK", &bounded_components, &max_degree, UpperBound(Linear))
            .showed("ZEEhCr", &linear_forest, &max_degree, UpperBound(Constant))
            .showed("a3JKzR", &cycles, &max_degree, UpperBound(Constant))
            .done();

    let cliques_make_it_unbounded = "Parameter is unbounded for the graph class of cliques.";

    let tmp_8mm5qj = create.intersection("8Mm5qJ", &bipartite, &max_matching, "maximum matching on bipartite graphs", 0);

    create.unknown_source()
        .defined("XK5Xxy", Unknown, &linear_forest, "Disjoint union of paths.")
        .showed("8Mm5qJ", Unknown, &tmp_8mm5qj, &vertex_cover, Exactly(Linear), "Kőnig's theorem")
        .showed("kmCM7X", Unknown, &vertex_cover, &max_matching, Exactly(Linear), "Every edge of the matching needs to be covered by at least one vertex. Path shows lower bound.")
        // Cite(id="gBA7dc", url="https://en.wikipedia.org/wiki/K%C5%91nig%27s_theorem_(graph_theory)", text="Kőnig's theorem"),
        .showed("U14yX4", Unknown, &odd_cycle_transversal, &dist_to_bipartite, Equivalence, "Bipartite graphs is the graph class without any odd cycles.")
        // Note(id="lqOY3G", text="Bipartite graphs is the graph class without any odd cycles."),
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
        .showed("rmLeo2", Unknown, &dist_to_stars, &treedepth, UpperBound(Linear), "First, treedepth removes vertices of the modulator, remainder has treedepth $2$")
        .showed("bYybsT", Unknown, &dist_to_complete, &clique_cover_num, UpperBound(Linear), "We cover the $k$ vertices of the modulator by cliques of size $1$ and cover the remaining clique by another one.")
        .showed("gGtTUf", Unknown, &max_independent_set, &domination_num, UpperBound(Linear), "Every maximal independent set is also a dominating set because any undominated vertex could be added to the independent set.")
        .showed("J0jyXi", Unknown, &domination_num, &diameter, UpperBound(Linear), "An unbounded diameter implies a long path where no vertices that are more than $3$ apart may be dominated by the same dominating vertex, otherwise we could shorten the path. Hence, the number of dominating vertices is also unbounded.")
        .showed("xrVJqb", Unknown, &dist_to_bipartite, &chromatic_num, UpperBound(Linear), "Removed vertices get one color each and we need only $2$ colors for the rest.")
        .showed("5wc1ir", Unknown, &edge_clique_cover, &neighborhood_diversity, UpperBound(Exponential), "Label vertices by the cliques they are contained in, each label is its own group in the neighborhood diversity, connect accordingly.")
        .showed("RnkWvT", Unknown, &dist_to_complete, &edge_clique_cover, UpperBound(Polynomial), "Cover the remaining clique, cover each modulator vertex and its neighborhood outside of it with another clique, cover each edge within the modulator by its own edge.")
        // .showed("8ouyNs", Unknown, &edge_clique_cover, &clique_cover_num, UpperBound(Linear), "Covering all edges ")
        .showed("FY0U1r", Unknown, &treewidth, &book_thickness, UpperBound(Exists), "")
        .showed("BKCgft", Unknown, &max_leaf_num, &dist_to_linear_forest, UpperBound(Exists), "")
        .showed("CyAMhs", Unknown, &acyclic_chromatic_number, &boxicity, UpperBound(Exists), "")
        .showed("pUfoGn", Unknown, &hindex, &dist_to_max_degree, UpperBound(Linear), "Remove the $h$ vertices of degree at least $h$ to get a graph that has maximum degree $h$.")
        .showed("8ZzI5w", Unknown, &dist_to_max_degree, &hindex, UpperBound(Linear), "Removal of $k$ vertices yielding a graph with maximum degree $c$ means that there were $k$ vertices of arbitrary degree and the remaining vertices had degree at most $k+c$. Hence, $h$-index is no more than $k+c$.")
        .showed("Bzw7GY", Unknown, &dist_to_cograph, &clique_width, UpperBound(Exists), "")
        .showed("fedm1t", Unknown, &dist_to_cograph, &chordality, UpperBound(Exists), "")
        .showed("rGMb0t", Unknown, &dist_to_cograph, &diameter, UpperBound(Exists), "")
        .showed("Er0L5w", Unknown, &book_thickness, &acyclic_chromatic_number, UpperBound(Exists), "")
        // .showed("03kKbA", Unknown, &dist_to_planar, &acyclic_chromatic_number, UpperBound(Exists), "") // idk
        .showed("wJkzlI", Unknown, &average_distance, &girth, UpperBound(Exists), "")
        .showed("JfSGx1", Unknown, &max_leaf_num, &feedback_edge_set, UpperBound(Exists), "")
        .showed("LJQHKw", Unknown, &max_induced_matching, &diameter, UpperBound(Linear), "Diameter requires an induced path on $d$ edges, hence, maximum induced matching is at least $\\lfloor (d+1)/3 \\rfloor$.")
        .showed("unkZhD", Unknown, &max_independent_set, &max_induced_matching, UpperBound(Linear), "Each edge of the induced matching can host at one vertex of the independent set.")
        .showed("RqDij1", Unknown, &vertex_cover, &neighborhood_diversity, UpperBound(Exponential), "")
        .showed("a2DTDH", Unknown, &twin_cover_num, &neighborhood_diversity, Exclusion, "")
        .showed("Pinlr2", Unknown, &linear_clique_width, &clique_width, UpperBound(Exists), "")
        .showed("OUUh3y", Unknown, &clique_width, &boolean_width, UpperBound(Linear), "")
        .showed("hgUvsR", Unknown, &boolean_width, &clique_width, UpperBound(Exponential), "")
        .showed("V9Pisv", Unknown, &branch_width, &boolean_width, UpperBound(Linear), "")
        .showed("0zGd6N", Unknown, &branch_width, &rank_width, UpperBound(Linear), "")
        .showed("QWXYYb", Unknown, &treewidth, &boolean_width, UpperBound(Exists), "")
        .showed("mD6cvS", Unknown, &bandwidth, &cutwidth, Exactly(Polynomial), "Any bandwidth bound cutwidth quadratically. An example where this happens is $(P_n)^k$ which has bandwidth $k$ and cutwidth $O(k^2)$; both seem to be optimal.")
        .showed("NTZE4R", Unknown, &modular_width, &clique_width, UpperBound(Exists), "")
        .showed("Vq2BBF", Unknown, &modular_width, &diameter, UpperBound(Exists), "")
        // .showed("TA2EZd", Unknown, &dist_to_planar, &twin_width, UpperBound(Exists), "") // dist may not, even if planar has bounded twin-width
        .showed("qB1OMb", Unknown, &max_degree, &c_closure, UpperBound(Exists), "")
        .showed("fmiQlU", Unknown, &feedback_edge_set, &c_closure, UpperBound(Exists), "")
// Bound(fr=vertex_cover, to=neighborhood_diversity, notes=[
    // Cite(id="YgTRtT", url="https://link.springer.com/article/10.1007/s00453-011-9554-x", text="Construct $k$ singleton sets, one for each vertex in the vertex cover and at most $2^k$ additional sets, one for each subset of vertices of the vertex cover. ...", range=Range(EXPONENTIAL)),
    // ])
        .defined("aQQnbF", Unknown, &vertex_integrity, "Minimum $k$ such that there exists $k$ vertices whose removal results in connected components of sizes at most $k$.")
        .defined("nTIDMU", Unknown, &twin_cover_num, "Distance to cluster where all vertices of each clique are siblings.")
        .defined("8tk4SI", Unknown, &max_degree, "maximum degree of graph's vertices")
        .defined("81zlqB", Unknown, &feedback_vertex_set, "can be thought of as a *distance to forest*")
        .defined("CKNuj2", Unknown, &min_degree, "minimum degree of graph's vertices")
        .defined("MlVCMG", Unknown, &diameter, "Maximum distance of two vertices that are in the same connected component.")
        .showed("H1gQ6m", Unknown, &feedback_vertex_set, &dist_to_forest, Equivalence, "")
        .showed("hDNUsi", Unknown, &vertex_cover, &dist_to_edgeless, Equivalence, "")
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
        .showed("k18Pyk", Unknown, &forest, &dist_to_interval, Exclusion, "")
        .showed("2QZo3T", Unknown, &edgeless, &vertex_cover, UpperBound(Constant), "")
        .showed("cq2q83", Unknown, &edgeless, &domination_num, Exclusion, "")
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
        .showed("VAAXVv", Unknown, &vertex_integrity, &neighborhood_diversity, Exclusion, "")
        .showed("CoBOm0", Unknown, &stars, &hindex, Exclusion, "")
        .showed("Ei8B1H", Unknown, &stars, &vertex_integrity, Exclusion, "")
        .showed("ORlCs0", Unknown, &cycles, &dist_to_perfect, Exclusion, "")
        .showed("tZrOta", Unknown, &cycle, &max_leaf_num, UpperBound(Constant), "")
        .showed("cYF2KU", Unknown, &cycle, &girth, Exclusion, "")
        .showed("CkDe7e", Unknown, &max_leaf_num, &feedback_edge_set, UpperBound(Polynomial), "M. Bentert (personal communication)") // todo not unknown
        .showed("QeiwSR", NotApplicable, &bounded_components, &cutwidth, UpperBound(Polynomial), "By greedily placing one component after another.")
        .showed("EjGaM8", NotApplicable, &bounded_components, &dist_to_perfect, Exclusion, "By a disjoint union of small components with distance to perfect at least 1.")
        .showed("bQLN2O", NotApplicable, &bounded_components, &dist_to_planar, Exclusion, "By a disjoint union of many $K_5$ graphs.")
        .showed("MQ0K6A", NotApplicable, &star, &vertex_cover, UpperBound(Constant), "trivially")
        .showed("btFVbS", NotApplicable, &star, &hindex, UpperBound(Constant), "trivially")
        .showed("A2vYf3", NotApplicable, &tree, &hindex, Exclusion, "trivially")
        .showed("vPk1LG", NotApplicable, &path, &dist_to_cluster, Exclusion, "trivially")
        .showed("dy8lvH", NotApplicable, &path, &diameter, Exclusion, "trivially")
        .showed("DsZGLl", NotApplicable, &cycles, &pathwidth, UpperBound(Constant), "trivially")
        .showed("K9z178", NotApplicable, &star, &max_degree, Exclusion, "trivially")
        // .showed("D2YglK", Unknown, &create.intersection("QrYeIw", &treewidth, &max_degree, "treewidth+maxdegree"), &create.intersection("hljuu8", &clique_width, &max_degree, "cliquewidth+maxdegree"), UpperBound(Linear), "")
        // .showed("JJTNMl", Unknown, &create.intersection("nP3xBv", &clique_width, &max_degree, "cliquewidth+maxdegree"), &create.intersection("iPgGur", &treewidth, &max_degree, "treewidth+maxdegree"), UpperBound(Linear), "")
        // clique-width = fusing width (operation to merge a color class to a single vertex)
        ;

    create.web_source("ez07Er", "https://en.wikipedia.org/wiki/Vertex_cover")
        .defined("l20H0G", NotApplicable, &vertex_cover, "... set of vertices that includes at least one endpoint of every edge of the graph.");
    create.web_source("f3q99d", "https://www.graphclasses.org/classes/par_13.html")
        .defined("wiwa6x", NotApplicable, &max_matching, "A matching in a graph is a subset of pairwise disjoint edges (any two edges that do not share an endpoint). The parameter maximum matching of a graph $G$ is the largest size of a matching in $G$.");
    create.web_source("QHJ1Kl", "https://en.wikipedia.org/wiki/Tree-depth")
        .defined("E9GMDZ", NotApplicable, &treedepth, "The tree-depth of a graph $G$ may be defined as the minimum height of a forest $F$ with the property that every edge of $G$ connects a pair of nodes that have an ancestor-descendant relationship to each other in $F$.");
    create.web_source("jh0OIZ", "https://en.wikipedia.org/wiki/Clique_cover")
        .defined("p0NZrl", NotApplicable, &clique_cover_num, "... is a partition of the vertices into cliques ... A minimum clique cover is a clique cover that uses as few cliques as possible. The minimum $k$ for which a clique cover exists is called the clique cover number of the given graph.");
    create.web_source("0cYayY", "https://en.wikipedia.org/wiki/Maximal_independent_set")
        .defined("2xRnhJ", NotApplicable, &max_independent_set, "For a graph $G=(V,E)$, an independent set $S$ is a maximal independent set if for $v \\in V$, one of the following is true: 1) $v \\in S$ 2), $N(v) \\cap S \\ne \\emptyset$ where $N(v)$ denotes the neighbors of $v$. ... the largest maximum independent set of a graph is called a maximum independent set.");
    create.web_source("82RsGb", "https://mathworld.wolfram.com/DominationNumber.html")
        .defined("7XYxB4", NotApplicable, &domination_num, "The domination number $\\gamma(G)$ of a graph $G$ is the minimum size of a dominating set of vertices in $G$ ...");
    create.web_source("L2KX25", "https://link.springer.com/article/10.1007/s00453-011-9554-x")
        .defined("ljbw1n", NotApplicable, &neighborhood_diversity, "We will say that two vertices $v, v'$ of a graph $G(V, E)$ have the same *type* iff they have the same colors and $N(v) \\setminus \\{v\\}=N(v') \\setminus \\{v\\}$, where $N(v)$ denotes the set of neighbors of $v$. ... A colored graph $G(V, E)$ has neighborhood diversity at most $w$, if there exists a partition of $V$ into at most $w$ sets, such that all the vertices in each set have the same type.");
    create.web_source("Q3HJs5", "https://mathworld.wolfram.com/MaximumLeafNumber.html")
        .defined("rBWwFy", NotApplicable, &max_leaf_num, "... the largest number of tree leaves in any of its spanning trees.");
    create.web_source("WP7pFA", "https://stackoverflow.com/questions/10791689/how-to-find-feedback-edge-set-in-undirected-graph")
        .defined("eYijvL", NotApplicable, &feedback_edge_set, "Let $G=(V,E)$ be an undirected graph. A set $F \\subseteq E$ of edges is called a feedback-edge set if every cycle of $G$ has at least one edge in $F$.");
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
    create.web_source("AeRM2B", "http://parallelcomp.github.io/Lecture3.pdf")
        .defined("w15E7O", NotApplicable, &bisection_bandwidth, "(number of) links across smallest cut that divides nodes in two (nearly) equal parts");
    create.web_source("BJhqpe", "https://en.wikipedia.org/wiki/Feedback_vertex_set")
        .defined("xPcvEf", NotApplicable, &feedback_vertex_set, "... a feedback vertex set (FVS) of a graph is a set of vertices whose removal leaves a graph without cycles... . The feedback vertex set number of a graph is the size of a smallest feedback vertex set.");
    create.web_source("4Dua5N", "https://www.fi.muni.cz/~hlineny/papers/shrubdepth-warw18-slipp.pdf")
        .defined("zWFoL1", Pp(7), &shrub_depth, "Tree-model of $m$ colors and depth $d$: a rooted tree $T$ of height $d$, leaves are the vertices of $G$, each leaf has one of $m$ colors, an associated signature determining the edge set of $G$ as follows: for $i=1,2,\\dots,d$, let $u$ and $v$ be leaves with the least common ancestor at height $i$ in $T$, then $uv \\in E(G)$ iff the color pair of $u,v$ is in the signature at height $i$.")
        .showed("X3CThx", Pp(9), &neighborhood_diversity, &shrub_depth, UpperBound(Constant), "Shrub-depth 1: e.g., cliques, stars, \\dots, gen. BND -- bounded neighborhood diversity.")
        .done();
    create.web_source("dxaIhi", "https://mathworld.wolfram.com/Pathwidth.html")
        .defined("OivGaa", NotApplicable, &pathwidth, "The pathwidth of a graph $G$, also called the interval thickness, vertex separation number, and node searching number, is one less than the size of the largest set in a path decomposition G.");
    create.web_source("W4j934", "https://en.wikipedia.org/wiki/Treewidth")
        .defined("LNtnP9", NotApplicable, &treewidth, "..., the treewidth of an undirected graph is an integer number which specifies, informally, how far the graph is from being a tree.");
    create.web_source("xnhT1P", "https://www.mimuw.edu.pl/~malcin/book/parameterized-algorithms.pdf")
        .defined("96BXHn", NotApplicable, &treewidth, "Very roughly, treewidth captures how similar a graph is to a tree. There are many ways to define ``tree-likeness'' of a graph; ... it appears that the approach most useful from algorithmic and graph theoretical perspectives, is to view tree-likeness of a graph $G$ as the existence of a structural decomposition of $G$ into pieces of bounded size that are connected in a tree-like fashion. This intuitive concept is formalized via the notions of a *tree decomposition* and the *treewidth* of a graph; the latter is a quantitative measure of how good a tree decomposition we can possibly obtain.");
    create.web_source("ZhBkjd", "https://en.wikipedia.org/wiki/Branch-decomposition")
        .defined("0SLCxV", NotApplicable, &branch_width, "... branch-decomposition of an undirected graph $G$ is a hierarchical clustering of the edges of $G$, represented by an unrooted binary tree $T$ with the edges of $G$ as its leaves. Removing any edge from $T$ partitions the edges of $G$ into two subgraphs, and the width of the decomposition is the maximum number of shared vertices of any pair of subgraphs formed in this way. The branchwidth of $G$ is the minimum width of any branch-decomposition of $G$.");
    create.web_source("9Ckusi", "https://en.wikipedia.org/wiki/Clique-width")
        .defined("pLDACG", NotApplicable, &clique_width, "... the minimum number of labels needed to construct G by means of the following 4 operations: 1. Creation of a new vertex... 2. Disjoint union of two labeled graphs... 3. Joining by an edge every vertex labeled $i$ to every vertex labeled $j$, where $i \\ne j$ 4. Renaming label $i$ to label $j$");
    create.web_source("pjVGlR", "https://www.sciencedirect.com/science/article/pii/S0095895605001528")
        .defined("JTZTcU", NotApplicable, &rank_width, "see Section 6");
    create.web_source("YGmwCG", "https://en.wikipedia.org/wiki/Book_embedding")
        .defined("jiDWoN", NotApplicable, &book_thickness, "... a book embedding is a generalization of planar embedding of a graph to embeddings into a book, a collection of half-planes all having the same line as their boundary. Usually, the vertices of the graph are required to lie on this boundary line, called the spine, and the edges are required to stay within a single half-plane. The book thickness of a graph is the smallest possible number of half-planes for any book embedding of the graph.");
    create.web_source("WY6oNX", "https://link.springer.com/chapter/10.1007/978-3-642-03367-4_25")
        .defined("1juCAg", NotApplicable, &hindex, "... $h$ is the $h$-index of the graph, the maximum number such that the graph contains $h$ vertices of degree at least $h$.");
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
    create.web_source("TKnuNP", "https://www.sciencedirect.com/science/article/pii/0166218X9290275F?via%3Dihub")
        .defined("zqGB0p", NotApplicable, &max_induced_matching, "An induced matching in a graph G is a set of edges, no two of which meet a common node or are joined by an edge of G;");
    create.web_source("055mG5", "https://en.wikipedia.org/wiki/Distance_(graph_theory)#Related_concepts")
        .defined("OaKBaL", NotApplicable, &diameter, "... [diameter] is the greatest distance between any pair of vertices ...");
    create.web_source("GfSsR4", "https://onlinelibrary.wiley.com/doi/abs/10.1002/jgt.3190120309")
        .defined("sBhhEO", NotApplicable, &average_degree, "The average distance in a graph is defined as the average length of a shortest path between two vertices, taken over all pairs of vertices.");
    create.web_source("u13WN1", "https://en.wikipedia.org/wiki/Girth_(graph_theory)")
        .defined("INk53D", NotApplicable, &girth, "In graph theory, the girth of an undirected graph is the length of a shortest cycle contained in the graph.");
    create.web_source("8eXjAy", "https://mathworld.wolfram.com/DomaticNumber.html")
        .defined("oTPnV8", NotApplicable, &domatic_num, "The maximum number of disjoint dominating sets in a domatic partition of a graph $G$ is called its domatic number $d(G)$. ");
    create.web_source("7K6dAT", "https://dl.acm.org/doi/10.1145/3486655")
        .showed("08lETp", NotApplicable, &boolean_width, &twin_width, UpperBound(Exponential), "Theorem 3: Every graph with boolean-width $k$ has twin-width at most $2^{k+1}-1$.");

    // let bandwidth_on_trees = create.intersection("Iu05N3", &tree, &bandwidth, "tree+bandwidth");
    // let cutwidth_on_trees = create.intersection("peyWzt", &tree, &cutwidth, "tree+cutwidth");

    let chung1985 = create.source("DkY1Aq", "Chung1985")
        // .showed("YgJVvi", Unknown, &bandwidth_on_trees, &cutwidth_on_trees, UpperBound(Linear), "")
        // .showed("pRjX8u", Unknown, &cutwidth_on_trees, &bandwidth_on_trees, UpperBound(Linear), "")
        .todo();
    let robertson_seymour1986 = create.source("i56ihO", "RobertsonSymour1986")
        .defined("HHHQZT", Pp(1), &treewidth, "A \\emph{tree-decomposition} of $G$ is a family $(X_i \\colon i\\in I)$ of subsets of $V(G)$, together with a tree $T$ with $V(T)=I$, with the following properties. (W1) $\\bigcup(X_i \\colon i \\in I)=V(G)$. (W2) Every edge of $G$ has both its ends in some $X_i$ ($i \\in I$). (W3) For $i,j,k \\in I$, if $j$ lies on the path of $T$ from $i$ to $k$ then $X_i \\cap X_k \\subseteq X_j$. The \\emph{width} of the tree-decomposition is $\\max(|X_i|-1 \\colon i \\in I)$. The tree-width of $G$ is the minimum $w \\ge 0$ such that $G$ has a tree-decomposition of width $\\le w$.")
        .defined("aYyqd4", Pp(1), &treewidth, "Equivalently, the tree-width of $G$ is the minimum $w \\ge 0$ such that $G$ is a subgraph of a ``chordal'' graph with all cliques of size at most $w + 1$.")
        // .showed("NqLFrC", Pp(2), "(1.2) For any fixed integer $w$, there is a polynomial algorithm to decide if the input graph has tree-width $\\le w$.") // non-constructive
        // .showed("a7nQ0N", Pp(6), treewidth, minor_closed, "(2.7) If $G$ has tree-width $< w$, so does ever minor of $G$.")
        .done();
    // let excludingforest1991 = create.source("AyLnH4", "excludingforest1991")
        // .todo();
    let chordality1993 = create.source("IFY0Rw", "chordality1993")
        .defined("Xdg7Hv", Pp(1), &chordality, "The \\emph{chordality} of a graph $G=(V,E)$ is defined as the minimum $k$ such that we can write $E=E_1,\\cap\\dots\\cap E_k$ with each $(V,E_i)$ a chordal graph.")
        .showed("rQBO3K", Pp(2), &chromatic_num, &chordality, UpperBound(Linear), "Corollary 4. For any graph $G$, $\\mathrm{Chord}(G) \\le \\chi(G)$, the chromatic number of $G$.")
        .showed("N0jfjr", Pp(5), &treewidth, &chordality, UpperBound(Linear), "Theorem 7. For any graph $G$, $\\mathrm{Chord}(G) \\le \\tau(G)$.")
        .done();
    let malitz1994 = create.source("cCrsoK", "Malitz1994")
        .showed("ECnpoM", Pp(24), &genus, &book_thickness, UpperBound(Linear), "Theorem 5.1. Genus $g$ graphs have pagenumber $O(\\sqrt{g})$.") // is optimal
        .done();
    let robertson_seymour1986_5 = create.source("A82svt", "RobertsonSymour1986V")
        // .showed("u4wtjE", Pp(2), &excluding any planar, &treewidth, "(1.5) For every planar graph $H$, there is a number $w$ such that every planar graph with no minor isomorphic to $H$ has tree-wdtih $\\le w$")
        .todo();
    let robertson_seymour1991 = create.source("1hPzXs", "RobertsonSymour1991")
        .defined("gMAL5e", Pp(12), &branch_width, "A \\emph{branch-width} of a hypergraph $G$ is a pair $(T,\\tau)$, where $T$ is a ternary tree and $\\tau$ is a bijection from the set of leaves of $T$ to $E(G)$. The \\emph{order} of an edge $e$ of $T$ is the number of vertices $v$ of $G$ such that there are leaves $t_1,t_2$ of $T$ in different components of $T \\setminus e$, with $\\tau(t_1),\\tau(t_2)$ both incident with $v$. The \\emph{width} of $(T,\\tau)$ is the maximum order of the edges of $T$, and the \\emph{branch-width} $\\beta(G)$ of $G$ is the minimum width of all branch-decompositions of $G$ (or 0 if $|E(G)| \\le 1$, when $G$ has no branch-decompositions).")
        // .showed("FN4FJJ", Pp(12), "(4.1) If $H$ is a minor of a graph $G$, then $\\beta(H) \\le \\beta(G)$.")
        .collective(Pp(16), "(5.1) For any hypergraph $G$, $\\max(\\beta(G), \\gamma(G)) \\le \\omega(G) + 1 \\le \\max(\\lfloor(3/2)\\beta(G)\\rfloor, \\gamma(G), 1)$.")
            .showed("8ewSpI", &treewidth, &branch_width, UpperBound(Linear))
            .showed("wrBAYk", &branch_width, &treewidth, UpperBound(Linear))
            .done()
        .done();
    let bodlaender_mohring1993 = create.source("a3yKzk", "BodlaenderMohring1993")
        .showed("cIAr80", Pp(4), &complete, &treewidth, Exclusion, "Lemma 3.1 (\"clique containment lemma\"). Let $(\\{X_i\\mid u\\in I\\},T=(I,F))$ be a tree-decomposition of $G=(V,E)$ and let $W \\subseteq V$ be a clique in $G$. Then there exists $i \\in I$ with $W \\subseteq X_i$.")
        .showed("mIvbmU", Pp(4), &bipartite, &treewidth, Exclusion, "Lemma 3.2 (\"complete bipartite subgraph containment lemma\").")
        // .showed("LDCZyj", Pp(5), &create.intersection("NxM8Gc", &cograph, &treewidth, ""), &create.intersection("chwMbI", &cograph, &pathwidth, ""), Equivalence, "Theorem 3.2. For every cograph $G = (V,E)$, $treewidth(G) = pathwidth(G)$.")
        // .showed(Theorem 4.1. The pathwidth and treewidth of a cograph given with a corresponding cotree can be computed in $O(n)$ time.)
        .done();
    // clique-width ideas in 'Handle-Rewriting Hypergraph Grammars'
    let wanke1994 = create.source("SQjcYg", "Wanke1994")
        .defined("ENvDZb", Pp(3), &nlc_width, "Definition 2.1. Let $k \\in \\mathbb N$ be a positive integer. A \\emph{$k$-node label controlled (NLC) graph} is a $k$-NL graph defined as follows: ...")
        .defined("yNzt7o", Pp(4), &nlct_width, "Definition 2.2. Let $k \\in \\mathbb N$ be a positive integer. A \\emph{$k$-node label controlled (NLC) tree} is a $k$-NL graph defined as follows: ...")
        .showed("jBcoBD", Pp(5), &cograph, &nlc_width, UpperBound(Constant), "Fact 2.3. $G$ is a $1$-NLC graph if and only if $unlab(G)$ is a co-graph.")
        .showed("cXI1DK", Pp(6), &treewidth, &nlc_width, UpperBound(Exponential), "Theorem 2.5. For each partial $k$-tree $G$ there is a $(2^{k+1}-1)$-NLC tree $J$ with $G=unlab(J)$.")
        .done();
    let bodlaender1998 = create.source("BOFCWg", "Bodlaender1998")
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
            .showed("RgLQ2P", &pathwidth_maxdeg, &cutwidth, UpperBound(Linear))
            .showed("Bq4H8y", &cutwidth, &pathwidth_maxdeg, UpperBound(Linear))
            .done()
        .showed("VdNTHZ", Pp(34), &outerplanar, &treewidth, UpperBound(Constant), "Lemma 78. Every outerplanar graph $G=(V,E)$ has treewidth at most 2.") // check whether dist_to_outerplanar bounding treewidth infered from this?
        .showed("oFitZo", Pp(37), &grid, &treewidth, Exclusion, "Lemma 88. The treewidth of an $n \\times n$ grid graph ... is at least $n$.")
        .showed("KoFslx", Pp(38), &treewidth, &min_degree, UpperBound(Linear), "Lemma 90 (Scheffler [94]). Every graph of treewidth at most $k$ contains a vertex of degree at most $k$.") // todo Schemer, Die Baumweite von Graphen als ein Ma8 Rir die Kompliziertheit algorithmischer Probleme, Ph.D. Thesis, Akademie der Wissenschafien der DDR, Berlin, 1989.
        .done();
    let johansson1998 = create.source("W2nwG4", "Johansson1998") // according to Gurski2005
        .showed("DBXQMa", Unknown, &clique_width, &nlc_width, UpperBound(Linear), "")
        .showed("BjlRwP", Unknown, &nlc_width, &clique_width, UpperBound(Linear), "")
        .showed("qy5Ojn", Unknown, &linear_clique_width, &linear_nlc_width, UpperBound(Linear), "")
        .showed("hI8Txh", Unknown, &linear_nlc_width, &linear_clique_width, UpperBound(Linear), "")
        .todo();
    let courcelle_olariu_2000 = create.source("ZQrXS8", "courcelle2000")
        // .defined("OL0McK", Unknown, &clique_width, "")
        .showed("sGBrPC", Pp(18), &treewidth, &clique_width, UpperBound(Exponential), "We will prove that for every undirected graph $G$, $cwd(G) \\le 2^{twd(G)+1}+1$ ...")
        .done();
    let tack_layouts2004 = create.source("w7RVn9", "TackLayouts2004")
        // .defined("bcdAXe", Pp(2), &track_number, "The track-number of $G$ is $\\mathrm{tn}_1(G)$, ...")
        // .showed("ZXhXax", Pp(12), &track_number, &acyclic_chromatic_number, UpperBound(Exponential), "Corollary 3. Acyclic chromatic number is bounded by track-number. In particular, every $(k,t)$-track graph $G$ has acyclic chromatic number $\\chi_a(G) \\le t \\cdot 4^{\\binom k2(t_1)}$.")
        .showed("v1Ygyr", Pp(14), &book_thickness, &acyclic_chromatic_number, UpperBound(Exponential), "Theorem 5. Acyclic chromatic number is bounded by stack-number (ed: a.k.a. book-thickness). In particular, every $k$-stack graph $G$ has acyclich chromatic number $\\chi_a(G) \\le 80^{k(2k-1)}$.")
        .done();
    // let corneil2005 = create.source("HCGunF", "Corneil2005")
        // .showed("sGBrPC", Unknown, &treewidth, &clique_width, Exactly(Exponential), "... the clique-width of $G$ is at most $3 \\cdot 2k - 1$ and, more importantly, that there is an exponential lower bound on this relationship. In particular, for any $k$, there is a graph $G$ with treewidth equal to $k$, where the clique-width of $G$ is at least $2\\lfloor k/2\\rfloor-1$.")
        // .todo();
    let gurski2005 = create.source("FLSQsw", "Gurski2005")
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
        .done();
    let oum2006 = create.source("1ZTWBd", "Oum2006")
        .defined("SGJJ1Y", Pp(9), &rank_width, "... and the \\emph{rank-width} $\\mathrm{rwd}(G)$ of $G$ is the branch-width of $\\mathrm{cutrk}_G$.")
        .showed("yLdAHe", Pp(9), &rank_width, &clique_width, Exactly(Exponential), "Proposition 6.3")
        .showed("uEUXMq", Pp(9), &clique_width, &rank_width, UpperBound(Linear), "Proposition 6.3")
        .done();
    let geometric_thickness2007 = create.source("2q7m9E", "GeometricThickness2007")
        .defined("3p2P4E", Pp(3), &thickness, "The thickness of a graph $G$, ..., is the minimum number of planar subgraphs that partition (ed: edges of) $G$.") // defined by Tutte 1963
        .defined("j9NrW9", Pp(3), &outerthickness, "The outerthickness of a graph $G$, ..., is the minimum number of outerplanar subgraphs that partition (ed: edges of) $G$.")
        .showed("0B1cGr", Pp(4), &treewidth, &thickness, UpperBound(Linear), "Proposition 1. The maximum thickness of a graph in $\\mathcal T_k$ (ed: $k$-tree) is $\\lceil k/2 \\rceil$; ...")
        .showed("3zMwH9", Pp(5), &treewidth, &arboricity, UpperBound(Linear), "Proposition 2. The maximum arboricity of a graph in $\\mathcal T_k$ (ed: $k$-tree) is $k$; ...")
        .showed("hXbdpU", Pp(5), &treewidth, &outerthickness, UpperBound(Linear), "Proposition 3. The maximum outerthickness of a graph in $\\mathcal T_k$ (ed: $k$-tree) is $k$; ...")
        .showed("EKKZPJ", Pp(6), &treewidth, &star_arboricity, UpperBound(Linear), "Proposition 4. The maximum star-arboricity of a graph in $\\mathcal T_k$ (ed: $k$-tree) is $k+1$; ...")
        .defined("IxPMGS", Pp(7), &book_thickness, "A geometric drawing in which the vertices are in convex position is called a book embedding. The book thickness of a graph $G$, ..., is the minimum $k \\in \\mathbb N$ such that there is book embedding of $G$ with thickness $k$.")
        .showed("FY0U1r", Pp(8), &treewidth, &book_thickness, UpperBound(Linear), "The maximum book thickness ... of a graph $\\mathcal T_k$ (ed: $k$-tree) satisfy ... $=k$ for $k \\le 2$, $=k+1$ for $k \\ge 3$.")
        .todo();
    let delavina_waller2008 = create.source("C5cBsd", "spanningTreesManyLeaves2008")
        // .showed("ZXINaY", Unknown, &max_leaf_num, &feedback_vertex_set, UpperBound(Linear), "")
        .todo();
    let gradnesetril2008 = create.source("kXDDmb", "gradnesetril2008")
        .showed("VLpzhW", Unknown, &d_path_free, &treedepth, UpperBound(Polynomial), "") // todo
        .showed("Q7qpEp", Unknown, &treedepth, &d_path_free, UpperBound(Exponential), "") // todo
        // d_path_free
        .todo();
    let cliquewidthnpc2009 = create.source("zuhSo5", "cliquewidthnpc2009")
        .showed("i1eBMN", Pp(8), &pathwidth, &linear_clique_width, UpperBound(Linear), "(5) $\\mathrm{lin-cwd}(G) \\le \\mathrm{pwd}(G)+2$.")
        .todo();
    let jelinek2010 = create.source("vIBI5v", "Jelinek2010")
        .showed("ipo6rm", Pp(2), &grid, &rank_width, Exclusion, "The grid $G_{n,n}$ has rank-width equal to $n-1$.")
        .done();
    let sasak2010 = create.source("XlBXyo", "Sasak2010")
        .showed("Lp8I7N", Pp(16), &tree, &pathwidth, Exclusion, "Theorem 2.1") // cites excludingforest1991, cannot find there
        .showed("BtRWId", Pp(17), &complete, &treewidth, Exclusion, "Theorem 2.2")
        .showed("GN6dar", Pp(17), &grid, &treewidth, Exclusion, "Theorem 2.4") // cites Csaba Biró. Tree-width and grids, cannot find the paper
        .cited("kXDDmb", Pp(21), jelinek2010, "Theorem 2.14 [12] Rank-width of a grid $\\sqrt{n} \\times \\sqrt{n}$ on $n$ vertices is $\\sqrt{n}-1$.")
        // Theorem 2.15 [4] Boolean-width of a grid √n×√n on n vertices lies between
        .showed("BwIc79", Pp(28), &cutwidth, &max_degree, UpperBound(Linear), "Lemma 2.18. For any graph $G$ and any vertex $v \\in V(G), cutw(g) \\ge \\lceil \\frac{deg(v)}2 \\rceil$.")
        .showed("h49cUu", Pp(30), &carving_width, &max_degree, UpperBound(Linear), "Lemma 2.20 Carving-width of a graph $G$ is at least $\\Delta(G)$ where $\\Delta(G)$ is the maximum degree of a graph $G$.")
        .showed("2Wk4AF", Pp(30), &star, &carving_width, Exclusion, "Corollary 2.21 Carving-width of a star is $n-1$.")
        .showed("6Ln8ux", Pp(30), &path, &carving_width, UpperBound(Constant), "... path with carving-width 2.")
        // .cited("7jmzab", Pp(32), &gradnesetril2008, "Theorem 2.23 [13] Let $l$ be the length of the longest path in a graph $G$. Then the tree-depth of $G$ is bounded as follows: $\\lceil \\log_2(l+2)\\rceil \\le td(G) \\le \\binom{l+3}2-1$ ...")
        .showed("MboUFT", Pp(32), &grid, &treedepth, Exclusion, "Corollary 2.24 Tree-depth of a grid is at least $\\lceil \\log_2(n+1)\\rceil$.")
        .showed("WiiQn4", Pp(38), &cutwidth, &carving_width, UpperBound(Linear), "Theorem 4.3 (carw $\\prec$ cutw) Carving-width is bounded by cut-width.")
        .showed("u5VPeX", Pp(49), &carving_width, &treewidth, UpperBound(Linear), "Theorem 5.5 (tw $\\prec$ carw) Tree-width is bounded by carving-width.")
        .todo();
    let bipboxicity2011 = create.source("Vkc4EU", "bipboxicity2011")
        .showed("Yelk6V", Pp(9), &dist_to_bipartite, &boxicity, Exclusion, "Theorem 2 For any $b \\in \\mathbb N^+$, there exists a chordal bipartite graph $G$ (ed: i.e. bipartite graph with no induced cycle on more than 4 vertices) with $\\mathrm{box}(G) > b$.")
        .done();
    let bui_xuan2011 = create.source("cNjhWx", "BuiXuan2011")
        .defined("L7aY6D", Unknown, &boolean_width, "\\textbf{Definition 1.} A decomposition tree of a graph $G$ is a pair $(T,\\delta)$ where $T$ is a tree having internal nodes of degree three and $\\delta$ a bijection between the leaf set of $T$ and the vertex set of $G$. Removing an edge from $T$ results in two subtrees, and in a cut $\\{A,\\comp{A}\\}$ of $G$ given by the two subsets of $V(G)$ in bijection $\\delta$ with the leaves of the two subtrees. Let $f\\colon w^V \\to \\mathbb{R}$ be a symmetric function that is also called a cut function: $f(A)=f(\\comp{A})$ for all $A\\subseteq V(G)$. The $f$-width of $(T,\\delta)$ is the maximum value of $f(A)$ over all cuts $\\{A,\\comp{A}\\}$ of $G$ given by the removal of an edge of $T$. ... \\textbf{Definition 2.} Let $G$ be a graph and $A \\subseteq V(G)$. Define the set of unions of neighborhoods of $A$ across the cut $\\{A,\\comp{A}\\}$ as $U(A) = \\{Y \\subseteq \\comp{A} \\mid \\exists X \\subseteq A \\land Y=N(X)\\cap \\comp{A}\\}. The \\emph{bool-dim}$\\colon 2^{V(G)} \\to \\mathbb{R}$ function of a graph $G$ is defined as $\\mathrm{bool-dim}(A)=\\log_2 |U(A)|$. Using Definition 1 with $f=\\mathrm{bool-dim}$ we define the boolean-width of a decomposition tree, denoted by $boolw(T,\\delta)$, and the boolean-width of a graph, denoted by $boolw(G)$.")
        .showed("AdNkCy", Unknown, &boolean_width, &rank_width, UpperBound(Exponential), "\\textbf{Corollary 1.} For any graph $G$ and decomposition tree $(T,\\gamma)$ of $G$ it holds that ... $\\log_2 rw(G) \\le boolw(G)$ ...")
        .showed("cIWQDn", Unknown, &rank_width, &boolean_width, UpperBound(Polynomial), "\\textbf{Corollary 1.} For any graph $G$ and decomposition tree $(T,\\gamma)$ of $G$ it holds that ... $boolw(G) \\le \\frac 14 rw^2(G)+O(rw(G))$.")
        .todo();
    let lampis2012 = create.source("0LYUEV", "lampis2012")
        .todo();
    let ganian_twin_cover2012 = create.source("7UoBR6", "GanianTwinCover2012")
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
        .done();
    let vatshelle2012 = create.source("nRO7AG", "Vatshelle2012")
        .defined("Usp3Ca", Pp(33), &mim_width, "Definition 3.7.1 (MIM-width). For $G$ a graph and $A \\subseteq V(G)$ let $mim \\colon 2^{V(G)} \\to \\mathbb N$ be a function where $mim(A)$ is the size of a maximum induced matching in $G[A,\\bar A]$. Using Definition 3.1.3 with $f=mim$ we define $mimw(T,\\delta$ as the $f$-width of a binary decomposition tree $(T,\\delta)$ and $mimw(G)$ as the $f$-width of $G$, also called the MIM-width of $G$ or the maximum induced matching width.")
        .showed("77zJ2z", Pp(42), &boolean_width, &mim_width, UpperBound(Linear), "Theorem 4.2.10. Let $G$ be a graph, then $mimw(G) \\le boolw(G) \\le mimw(G) \\log_2(n)$")
        .todo();
    let modularwidth2013 = create.source("OH3sI3", "modularwidth2013")
        .showed("NeMJtU", Pp(6), &neighborhood_diversity, &modular_width, StrictUpperBound(Linear), "Theorem 3. Let $G$ be a graph. Then $\\mathrm{mw}(G) \\le \\mathrm{nd}(G)$ and $\\mathrm{mw}(G) \\le 2\\mathrm{tc}(G) + \\mathrm{tc}(G)$. Furthermore, both inequalities are strict, ...")
        .showed("8rtBjc", Pp(6), &twin_cover_num, &modular_width, StrictUpperBound(Exponential), "Theorem 3. Let $G$ be a graph. Then $\\mathrm{mw}(G) \\le \\mathrm{nd}(G)$ and $\\mathrm{mw}(G) \\le 2\\mathrm{tc}(G) + \\mathrm{tc}(G)$. Furthermore, both inequalities are strict, ...")
        .showed("ppKqXp", Pp(8), &modular_width, &shrub_depth, Incomparable, "Theorem 4. There are classes of graphs with unbounded modular-width and bounded shrub-depth and vice versa.")
        .todo();
    let belmonte2013 = create.source("sJ476m", "Belmonte2013")
        .showed("ZHXKjC", Unknown, &carving_width, &max_degree, UpperBound(Linear), "Observation 1. Let $G$ be a graph. Then $cw(G) \\ge \\Delta(G)$.")
        .todo();
    let jansen2013 = create.source("FLOjic", "Jansen2013")
        .defined("PV6tGG", Unknown, &topological_bandwidth, "The \\emph{topological bandwidth} of a graph $G$ is the minimum [bandwidth](../aP5a38) over all subdivisions of $G$")
        .todo();
    let adler2015 = create.source("rhj9my", "Adler2015")
        .showed("3yUfrd", Pp(1), &linear_rank_width, &linear_clique_width, UpperBound(Exists), "Linear rank-width is equivalent to linear clique-width in the sense that any graph class has bounded linear clique-width if and only if it has bounded linear rank-width.").showed("2dN9wh", Pp(1), &linear_clique_width, &linear_rank_width, UpperBound(Exists), "Linear rank-width is equivalent to linear clique-width in the sense that any graph class has bounded linear clique-width if and only if it has bounded linear rank-width.")
        .showed("dvqfqQ", Pp(3), &pathwidth, &linear_rank_width, UpperBound(Linear), "Lemma 5. Any graph $G$ satisfies $\\mathrm{lrw}(G) \\le \\mathrm{pw}(G)$.")
        .todo();
    let twin_cover_2015 = create.source("VQLE2i", "ganianTwinCover2015")
        .defined("J1sHj8", Pp(5), &twin_cover_num, "Definition 3 A set of vertices $X \\subseteq V(G)$ is a twin-cover of $G$ if for every edge $e = ab \\in E(G)$ either 1. $a \\in X$ or $b \\in X$, or 2. $a$ and $b$ are true twins. We then say that $G$ has twin-cover $k$ if the size of a minimum twin-cover of $G$ is $k$.")
        .showed("OoSnHu", Pp(20), &twin_cover_num, &shrub_depth, UpperBound(Constant), "Let $\\mathcal H_k$ be the class of graphs of twin-cover $k$. Then $\\mathcal H_k \\subseteq \\mathcal{TM}_{2^k+k}(2)$ and a tree-model of any $G \\in \\mathcal H_k$ may be constructed in single-exponential FPT time.")
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
        .showed("EG7vp6", Unknown, &neighborhood_diversity, &shrub_depth, UpperBound(Constant), "$\\mathcal{TM}_m(1)$ is exactly the class of graphs of neighborhood diversity at most $m$.")
        .showed("sq0brL", Unknown, &treedepth, &shrub_depth, UpperBound(Linear), "Proposition 3.2. If $G$ is of tree-depth $d$, then $G \\in \\mathcal{TM}_{2^d}(d)$. ...")
        .todo();
    let sorge2019 = create.source("VnTIL0", "Sorge2019")
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
        .todo(); // page 10
    let mimwidth2020 = create.source("BIQh3r", "mimwidth2020")
        .todo();
    let schroder_parameter_list = vec!["distance_to_EhdXNA", "4lp9Yj", "BN92vX", "VomShB", "distance_to_7HR4uV", "distance_to_WAU7vf", "distance_to_skQuFN", "HTk9PZ", "aP5a38", "mHtXUU", "distance_to_9Qd0Mx", "distance_to_p5skoj", "GNOiyB", "KEP2qM", "UyQ5yM", "Gq0onN", "distance_to_0oCyaG", "VHClqR", "gbaHdw", "GNTwUS", "p4bTjp", "distance_to_loZ5LD", "5Q7fuR", "zH8PpT", "distance_to_Cv1PaJ", "QGZuUW", "BCwUeT", "wg5HuV", "VowkuW", "distance_to_cLHJkW", "a7MpiT", "z0y4TW", "distance_to_RmssrZ", "w7MmyW", "GPmOeT", "q7zHeT", "fTqo40", "KRV6tI", "distance_to_lA0K71"];
    let schroder_thesis = create.source("DYGiYb", "SchroderThesis")
        .cited("pJxHVS", Unknown, sorge2019, "Based on the work by [Sa19] as well as [Fr8], we investigate unknown connections between graph parameters to continue the work on the graph parameter hierarchy")
        .cited("bybFgo", Unknown, froemmrich2018, "Based on the work by [Sa19] as well as [Fr8], we investigate unknown connections between graph parameters to continue the work on the graph parameter hierarchy")
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
        .showed("rCDCaC", Pp(16), &dist_to_complete, &vertex_connectivity, Exclusion, "Proposition 3.8")
        .showed("AbzdZf", Pp(16), &clique_cover_num, &clique_width, Exclusion, "Proposition 3.9")
        .showed("W2GU1L", Pp(19), &clique_cover_num, &chordality, Exclusion, "Proposition 3.11")
        .showed("3Tunx6", Pp(19), &dist_to_perfect, &chordality, Exclusion, "Proposition 3.11")
        .showed("OKKYpY", Pp(20), &dist_to_co_cluster, &dist_to_chordal, Exclusion, "Proposition 3.12")
        .showed("A18O6S", Pp(20), &dist_to_bipartite, &dist_to_chordal, Exclusion, "Proposition 3.12")
        .showed("TiiRaX", Pp(20), &dist_to_co_cluster, &vertex_connectivity, Exclusion, "Proposition 3.12")
        .showed("uvXQGw", Pp(20), &dist_to_bipartite, &vertex_connectivity, Exclusion, "Proposition 3.12")
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
        .showed("KYpom9", Pp(31), &domatic_num, &vertex_connectivity, Exclusion, "Proposition 3.30")
        .showed("2nmdxu", Pp(33), &bisection_bandwidth, &chordality, Exclusion, "Proposition 3.31")
        .showed("bxep24", Pp(33), &bisection_bandwidth, &clique_width, Exclusion, "Proposition 3.32")
        .showed("bhJsnM", Pp(33), &bisection_bandwidth, &max_clique, Exclusion, "Proposition 3.33")
        .showed("SynFuK", Pp(33), &genus, &dist_to_planar, Exclusion, "Proposition 3.34")
        .showed("hoJGBX", Pp(35), &average_degree, &max_clique, Exclusion, "Proposition 3.35")
        .showed("JRqAlT", Pp(36), &average_degree, &chordality, Exclusion, "Proposition 3.36")
        .todo();
    // let reduced_edgeless = create.reduced("reduced edgeless", &edgeless, 0);
    let twin_width_1_2021 = create.source("nyaOye", "twinWidthI2021")
        .defined("s5Ktq7", Pp(2), &twin_width, "... we consider a sequence of graphs $G_n,G_{n-1},\\dots,G_2,G_1$, where $G_n$ is the original graph $G$, $G_1$ is the one-vertex graph, $G_i$ has $i$ vertices, and $G_{i-1}$ is obtained from $G_i$ by performing a single contraction of two (non-necessarily adjacent) vertices. For every vertex $u \\in V(G_i)$, let us denote by $u(G)$ the vertices of $G$ which have been contracted to $u$ along the sequence $G_n,\\dots,G_i$. A pair of disjoint sets of vertices is \\emph{homogeneous} if, between these sets, there are either all possible edges or no edge at all. The red edges ... consist of all pairs $uv$ of vertices of $G_i$ such that $u(G)$ and $v(G)$ are not homogeneous in $G$. If the red degree of every $G_i$ is at most $d$, then $G_n,G_{n-1},\\dots,G_2,G_1$ is called a \\emph{sequence of $d$-contractions}, or \\emph{$d$-sequence}. The twin-width of $G$ is the minimum $d$ for which there exists a sequence of $d$-contractions.")
        .showed("0RiLv2", Pp(15), &grid, &twin_width, UpperBound(Constant), "Theorem 4.3. For every positive integers $d$ and $n$, the $d$-dimensional $n$-grid has twin-width at most $3d$.")
        // .showed("7p2TWN", Unknown, &cograph, &reduced_edgeless, Equivalence, "") // todo
        .todo();
    // let reduced_star = &create.reduced(&star, 0);
    // let twin_width_beyond_2022 = create.source("3B7Kvt", "twinWidthBeyond2022")
        // .showed("AwGkfi", Pp(3), &all_graphs, &reduced_star, UpperBound(Constant), "Every graph has a reduction sequence in which every red graph is a star ...")
        // // .defined("M6H2kI", , &reduced_bandwidth, "")
        // .todo();
    let tran2022 = create.source("uXViPE", "Tran2022")
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
        // .showed("wFtd4d", Pp(24), &modular_width, &complement(modular_width), Equivalence, "Given any graph $G$, $\\mathrm{mw}(G) = \\mathrm{mw}(\\bar G)$.")
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
        // .showed("wFtd4d", Pp(36), &twin_width, &complement(twin_width), Equivalence, "")
        .showed("PK4H2R", Pp(36), &clique_width, &twin_width, StrictUpperBound(Exponential), "Proposition 6.2. Clique-width strictly upper bounds Twin-width.")
        .showed("2F5Zr8", Pp(37), &genus, &twin_width, StrictUpperBound(Linear), "Proposition 6.3. Genus strictly upper bounds Twin-width.")
        .showed("xsiECz", Pp(37), &dist_to_planar, &twin_width, StrictUpperBound(Exponential), "Theorem 6.4. Distance to Planar strictly upper bounds Twin-width.") // cite
        .showed("aa0bCE", Pp(38), &twin_width, &dist_to_interval, Incomparable, "Observation 6.5. Twin-width is incomparable to Distance to Interval.")
        .showed("YqjLQa", Pp(38), &twin_width, &dist_to_bipartite, Incomparable, "Proposition 6.6. Twin-width is incomparable to Distance to Bipartite.")
        .showed("n3WK3H", Pp(40), &twin_width, &clique_cover_num, Incomparable, "Proposition 6.7. Twin-width is incomparable to Clique Cover Number.")
        .showed("52S4T0", Pp(40), &twin_width, &max_degree, Incomparable, "Proposition 6.8. Twin-width is incomparable to Maximum Degree.") // cites
        .showed("1lfJWG", Pp(40), &twin_width, &bisection_bandwidth, Incomparable, "Observation 6.9. Twin-width is incomparable to Bisection Width.") // check with bisection width
        .showed("UN2Lbu", Pp(42), &degeneracy, &boxicity, Incomparable, "Proposition 7.1. Degeneracy is incomparable to Boxicity.")
        .done();
    let torunczyk2023 = create.source("KpkMZB", "Torunczyk2023")
        .defined("9VHraO", Unknown, &inf_flip_width, "See radius-r flip-width for $r=\\infty$.")
        .defined("gxeVOT", Unknown, &r_flip_width, "The radius-$r$ flip-width of a graph $G$, denoted $fwr (G)$, is the smallest number $k \\in \\mathbb{N}$ such that the cops have a winning strategy in the flipper game of radius $r$ and width $k$ on $G$")
        .showed("9DTyeJ", Unknown, &inf_flip_width, &rank_width, UpperBound(Linear), "For every graph $G$, we have $\\mathrm{rankwidth}(G) \\le 3 \\mathrm{fw}_\\infty(G)+1$ ...")
        .showed("zYQZyB", Unknown, &rank_width, &inf_flip_width, UpperBound(Exponential), "For every graph $G$, we have ... $3 \\mathrm{fw}_\\infty(G)+1 \\le O(2^{\\mathrm{rankwidth(G)}})$.")
        .showed("OdbuZP", Unknown, &twin_width, &r_flip_width, UpperBound(Exponential), "Theorem 7.1. Fix $r \\in \\mathbb N$. For every graph $G$ of twin-width $d$ we have: $\\mathrm{fw}_r(G) \\le 2^d \\cdot d^{O(r)}$.")
        .showed("gvSCeQ", Unknown, &inf_flip_width, &r_flip_width, UpperBound(Linear), by_definition)
        .todo();
    let twwsurfaces2024 = create.source("lgJ2j7", "twwsurfaces2024")
        .showed("3iR4qs", Pp(18), &genus, &twin_width, UpperBound(Linear), "The twin-width of every graph $G$ of Euler genus $g \\ge 1$ is at most ... $18 \\sqrt{47g}+O(1)$.") // todo sqrt
        .todo();

    create.build()
}

// t0TUmk
// YvvmJE
// YhbKPB
// F0p61H
// 4lmvZK
// LlWzhg
// 9JAQC7
// Lh05uc
// LUQLaI
// KaLXjx
// oBcMqr
// Pbg2ga
// G1Cwmc
// Ktfezk
// E7K0I5
// xwLQQ8
// sWR5Yw
// 5vq7po
// tiEYdy
// G5i5Bz
// H4YERL
// JA2nKw
// 1dQQ87
// QmlowC
// fccHmU
// ePpmZt
// 81rUKt
// ri9Seh
// yPnIog
// d7vRYU
// Q3Bz8d
// EV3FqL
// 47Xy7Z
// ugRZLU
// DDM0j3
// BqY0YK
// YFKpDU
// Iaf2d1
// LoQADQ
// Qme7wD
// fNR6QK
// aEGv5N
// nAjQi4
// ZtdvKW
// TgTiqK
// O1poSV
// tK4S1r
// OrH7et
// hnX8tG
// e26LM8
// szWO2M
// k4hlPW
// 1iQ54v
// mIBYMD
// f1nTaY
// rmHBsY
// GiDjOm
// 8io8sJ
// ZL7BOP
// RPTCxd
// kRR8zx
// AVc2K6
// kJZKgd
// Z10jME
// OdZQna
// MLJMRH
// 1yW82F
// hQZlLU
// 4INs10
// xNJnFb
// lPHVWU
// yk7XP0
// aXw3Co
// uDXX2i
// hbfWwE
// 2LDMQ6
// zUv8EU
