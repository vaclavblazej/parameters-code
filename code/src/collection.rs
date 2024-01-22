//! Collects the information we have about parameterized complexity classes, their inclusions,
//! and related graph classes, topics, bibliographic sources, theorems, proofs, and so on.

use crate::data::Page::{Unknown, Pp};
use crate::{complexity::CpxTime::*, data::Cpx, build::Builder};

pub fn build_collection() -> crate::data::RawData {
    let mut create = Builder::new();

    let bipartite = create.graph_class("cLHJkW", "bipartite");
    let block = create.graph_class("QrxQsH", "block");
    let chordal = create.graph_class("Cv1PaJ", "chordal");
    let cluster = create.graph_class("WAU7vf", "cluster");
    let co_cluster = create.graph_class("7HR4uV", "co-cluster");
    let cograph = create.graph_class("9Qd0Mx", "cograph");
    let bounded_degree = create.graph_class("yeKPCw", "bounded degree");
    let parameter_degree = create.graph_class("PUSZhY", "parameter degree");
    let complete = create.connected(&cluster, "complete");
    let const_components = create.graph_class("FJ8gmU", "constant components");
    let forest = create.graph_class("JngPPm", "forest");
    let tree = create.connected(&forest, "tree");
    let interval = create.graph_class("p5skoj", "interval");
    let isolated = create.graph_class("LsiBbX", "isolated vertices");
    let linear_forest = create.graph_class("skQuFN", "linear forest");
    let outerplanar = create.graph_class("0oCyaG", "outerplanar");
    let perfect = create.graph_class("RmssrZ", "perfect");
    let planar = create.graph_class("loZ5LD", "planar");
    let stars = create.graph_class("10JR3F", "stars");
    let cycles = create.graph_class("2iJr52", "cycles");
    let cycle = create.connected(&cycles, "cycle");
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
        .isgci(&isolated, 1247)
        .isgci(&outerplanar, 110)
        .isgci(&perfect, 56)
        .isgci(&planar, 43)
        .isgci(&stars, 1297)
        .isgci(&grid, 464)
        ;

    create.source("", "unknown")
        .defined("2kG0kY", Unknown, &block, "Every block (maximal 2-connected subgraph) is a clique.")
        .defined("roSFzV", Unknown, &cluster, "Disjoint union of complete graphs.")
        .defined("FDbIDy", Unknown, &co_cluster, "Complete multipartite graph.")
        .defined("ivxOm1", Unknown, &parameter_degree, "Degree is upper bounded by the parameter")
        .defined("FvgORV", Unknown, &const_components, "Disjoint union of components of constant size.")
        .defined("51KDFn", Unknown, &stars, "Disjoint union of stars.")
        .defined("sAZHF4", Unknown, &cycles, "Every component is a cycle.")
        .defined("cBDurK", Unknown, &disjoint_cycles, "All cycles in the graph are disjoint. Can contain arbitrary trees attached to and between the cycles.")
        .defined("sp6LGE", Unknown, &grid, "Cartesian product of two paths.")
        .showed("piRTZw", Unknown, &chordal, &perfect, Cpx::Todo, "")
        .showed("stwHRi", Unknown, &cograph, &perfect, Cpx::Todo, "")
        .showed("ogyvLp", Unknown, &bipartite, &perfect, Cpx::Todo, "")
        .showed("FM1wVJ", Unknown, &cluster, &interval, Cpx::Todo, "")
        .showed("rHotfs", Unknown, &linear_forest, &interval, Cpx::Todo, "")
        // .showed("OT3dig", Unknown, &stars, &interval, Cpx::Todo, "")
        .showed("fKpyMg", Unknown, &interval, &chordal, Cpx::Todo, "")
        .showed("cZy5xs", Unknown, &co_cluster, &cograph, Cpx::Todo, "")
        .showed("AbAK8n", Unknown, &forest, &bipartite, Cpx::Todo, "")
        .showed("ZiCzGe", Unknown, &outerplanar, &planar, Cpx::Todo, "")
        .showed("6TFVVG", Unknown, &complete, &co_cluster, Cpx::Todo, "")
        .showed("2Jde0p", Unknown, &block, &chordal, Cpx::Todo, "")
        .showed("DxYTTn", Unknown, &cluster, &block, Cpx::Todo, "")
        .showed("lmKGuy", Unknown, &linear_forest, &forest, Cpx::Todo, "")
        .showed("hIuPAJ", Unknown, &disjoint_cycles, &outerplanar, Cpx::Todo, "")
        .showed("eruyce", Unknown, &forest, &disjoint_cycles, Cpx::Todo, "")
        .showed("WJHhf0", Unknown, &forest, &block, Cpx::Todo, "")
        .showed("VsrnoK", Unknown, &isolated, &linear_forest, Cpx::Todo, "")
        .showed("E8B2Gj", Unknown, &stars, &forest, Cpx::Todo, "")
        .showed("BWJDZs", Unknown, &isolated, &stars, Cpx::Todo, "")
        .showed("yWSq1V", Unknown, &isolated, &const_components, Cpx::Todo, "")
        .showed("HtdoRP", Unknown, &isolated, &co_cluster, Cpx::Todo, "")
        .showed("1PLbSg", Unknown, &grid, &planar, Cpx::Todo, "")
        .showed("RQcVkC", Unknown, &grid, &bipartite, Cpx::Todo, "")
        .showed("KxMj5k", Unknown, &grid, &bounded_degree, Cpx::Todo, "")
        .showed("VnTIL0", Unknown, &bounded_degree, &parameter_degree, Cpx::Todo, "")
        .showed("rSYFkG", Unknown, &const_components, &bounded_degree, Cpx::Todo, "")
        .showed("ZEEhCr", Unknown, &linear_forest, &bounded_degree, Cpx::Todo, "")
        .showed("CJ76wg", Unknown, &cycles, &disjoint_cycles, Cpx::Todo, "")
        .showed("a3JKzR", Unknown, &cycles, &bounded_degree, Cpx::Todo, "")
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
    let twin_cover_num = create.parameter("MUnHA0", "twin cover number");
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
    let linear_forest = create.parameter("eqXlUB", "linear forest");
    let forest = create.parameter("3QzEjG", "forest");
    let feedback_vertex_set = create.parameter("GNOiyB", "feedback vertex set");
    let cycle = create.parameter("jMSJmT", "cycle");
    let cycles = create.parameter("qHgnkF", "cycles");
    let disjoint_cycles = create.parameter("VuXKqR", "disjoint cycles");
    let grid = create.parameter("L2BQoF", "grid");
    let outerplanar = create.parameter("NTuzgB", "outerplanar");
    let planar = create.parameter("fgOnvT", "planar");
    let interval = create.parameter("dSvNpX", "interval");
    let shrub_depth = create.parameter("NTgNzT", "shrub-depth");
    let linear_clique_width = create.parameter("fQj3wU", "linear clique-width");
    let pathwidth = create.parameter("VHClqR", "pathwidth");
    let treewidth = create.parameter("5Q7fuR", "treewidth");
    let branch_width = create.parameter("lIcmuR", "branch width");
    let clique_width = create.parameter("wg5HuV", "clique width");
    let rank_width = create.parameter("fojquT", "rank width");
    let boolean_width = create.parameter("A2jPWT", "boolean width");
    let inf_flip_width = create.parameter("nYXiuT", "inf-flip-width");
    let twin_width = create.parameter("MNTjuW", "twin width");
    let r_flip_width = create.parameter("4DIiH0", "radius-r flip-width");
    let book_thickness = create.parameter("doijTS", "book thickness"); // top_drawing
    let hindex = create.parameter("GNTwUS", "h-index");
    let acyclic_chromatic_number = create.parameter("QGZuUW", "acyclic chromatic number");
    let odd_cycle_transversal = create.parameter("Ve5ruW", "odd cycle transversal");
    let bipartite = create.parameter("Gp9XyW", "bipartite");
    let degeneracy = create.parameter("VowkuW", "degeneracy");
    let chromatic_num = create.parameter("w7MmyW", "chromatic number");
    let average_degree = create.parameter("z0y4TW", "average degree");
    let min_degree = create.parameter("GPmOeT", "minimum degree");
    let max_clique = create.parameter("q7zHeT", "maximum clique");
    let edge_connectivity = create.parameter("JbqZoT", "edge connectivity");
    let boxicity = create.parameter("a7MpiT", "boxicity");
    let block = create.parameter("g7K1jT", "block");
    let chordality = create.parameter("fTqo40", "chordality");
    let max_induced_matching = create.parameter("GzMYlT", "maximum induced matching");
    let diameter = create.parameter("a7MpiT", "diameter");
    let average_distance = create.parameter("zH8PpT", "average distance");
    let girth = create.parameter("BCwUeT", "girth");

    let dist_to_complete = create.distance_to(&complete);
    let dist_to_co_cluster = create.distance_to(&co_cluster);
    let dist_to_cograph = create.distance_to(&cograph);
    let dist_to_cluster = create.distance_to(&cluster);
    let dist_to_linear_forest = create.distance_to(&linear_forest);
    let dist_to_outerplanar = create.distance_to(&outerplanar);
    let dist_to_block = create.distance_to(&block);
    let dist_to_edgeless = create.distance_to(&isolated);
    let dist_to_forest = create.distance_to(&forest);
    let dist_to_bipartite = create.distance_to(&bipartite);

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

    create.source("", "unknown")
        .defined("XK5Xxy", Unknown, &linear_forest, "Disjoint union of paths.")
        .showed("H1gQ6m", Unknown, &feedback_vertex_set, &dist_to_forest, Cpx::Equivalence, "")
        .showed("hDNUsi", Unknown, &vertex_cover, &dist_to_edgeless, Cpx::Equivalence, "")
        .showed("8Mm5qJ", Unknown, &vertex_cover, &max_matching, Cpx::Equivalence, "Kőnig's theorem")
        // Cite(id="gBA7dc", url="https://en.wikipedia.org/wiki/K%C5%91nig%27s_theorem_(graph_theory)", text="Kőnig's theorem"),
        .showed("U14yX4", Unknown, &odd_cycle_transversal, &dist_to_bipartite, Cpx::Equivalence, "Bipartite graphs is the graph class without any odd cycles.")
        // Note(id="lqOY3G", text="Bipartite graphs is the graph class without any odd cycles."),
        ;

    // create.source("", "hops")
        // .defined("aQQnbF", NotApplicable, &vertex_integrity, "Minimum $k$ such that there exists $k$ vertices whose removal results in connected components of sizes at most $k$.")
        // .defined("nTIDMU", NotApplicable, &twin_cover_num, "Distance to cluster where all vertices of each clique are siblings.")
        // .defined("8tk4SI", NotApplicable, &max_degree, "maximum degree of graph's vertices")
        // .defined("81zlqB", NotApplicable, &feedback_vertex_set, "can be thought of as a *distance to forest*")
        // .defined("CKNuj2", NotApplicable, &min_degree, "minimum degree of graph's vertices")
        // .showed("q3qJkr", NotApplicable, &treedepth, &pathwidth, Cpx::UpperBound(Exists), "Saving the set of open vertices in a DFS over the tree treedepth after every step yields bags of a nice path decomposition.")
        // .showed("7tFsi6", NotApplicable, &treedepth, &diameter, Cpx::UpperBound(Exists), "An unbounded diameter implies the class contains paths as subgraphs. Minimum treedepth to embed a path of length $n$ in a treedepth tree is $\\log n$.")
        // ;

    create.source("ez07Er", "https://en.wikipedia.org/wiki/Vertex_cover")
        .defined("", Unknown, &vertex_cover, "... set of vertices that includes at least one endpoint of every edge of the graph.");
    create.source("f3q99d", "https://www.graphclasses.org/classes/par_13.html")
        .defined("", Unknown, &max_matching, "A matching in a graph is a subset of pairwise disjoint edges (any two edges that do not share an endpoint). The parameter maximum matching of a graph $G$ is the largest size of a matching in $G$.");
    create.source("QHJ1Kl", "https://en.wikipedia.org/wiki/Tree-depth")
        .defined("", Unknown, &treedepth, "The tree-depth of a graph $G$ may be defined as the minimum height of a forest $F$ with the property that every edge of $G$ connects a pair of nodes that have an ancestor-descendant relationship to each other in $F$.");
    create.source("jh0OIZ", "https://en.wikipedia.org/wiki/Clique_cover")
        .defined("", Unknown, &clique_cover_num, "... is a partition of the vertices into cliques ... A minimum clique cover is a clique cover that uses as few cliques as possible. The minimum $k$ for which a clique cover exists is called the clique cover number of the given graph.");
    create.source("0cYayY", "https://en.wikipedia.org/wiki/Maximal_independent_set")
        .defined("", Unknown, &max_independent_set, "For a graph $G=(V,E)$, an independent set $S$ is a maximal independent set if for $v \\in V$, one of the following is true: 1) $v \\in S$ 2), $N(v) \\cap S \\ne \\emptyset$ where $N(v)$ denotes the neighbors of $v$. ... the largest maximum independent set of a graph is called a maximum independent set.");
    create.source("82RsGb", "https://mathworld.wolfram.com/DominationNumber.html")
        .defined("", Unknown, &domination_num, "The domination number $\\gamma(G)$ of a graph $G$ is the minimum size of a dominating set of vertices in $G$ ...");
    create.source("L2KX25", "https://link.springer.com/article/10.1007/s00453-011-9554-x")
        .defined("", Unknown, &neighborhood_diversity, "We will say that two vertices $v, v'$ of a graph $G(V, E)$ have the same *type* iff they have the same colors and $N(v) \\setminus \\{v\\}=N(v') \\setminus \\{v\\}$, where $N(v)$ denotes the set of neighbors of $v$. ... A colored graph $G(V, E)$ has neighborhood diversity at most $w$, if there exists a partition of $V$ into at most $w$ sets, such that all the vertices in each set have the same type.");
    create.source("Q3HJs5", "https://mathworld.wolfram.com/MaximumLeafNumber.html")
        .defined("", Unknown, &max_leaf_num, "... the largest number of tree leaves in any of its spanning trees.");
    create.source("WP7pFA", "https://stackoverflow.com/questions/10791689/how-to-find-feedback-edge-set-in-undirected-graph")
        .defined("", Unknown, &feedback_edge_set, "Let $G=(V,E)$ be an undirected graph. A set $F \\subseteq E$ of edges is called a feedback-edge set if every cycle of $G$ has at least one edge in $F$.");
    create.source("8ryhNq", "https://en.wikipedia.org/wiki/Genus_(mathematics)#Graph_theory")
        .defined("", Unknown, &genus, "The genus of a graph is the minimal integer $n$ such that the graph can be drawn without crossing itself on a sphere with $n$ handles.");
    create.source("bnOBjM", "https://link.springer.com/article/10.1007/bf01215352")
        .defined("", Unknown, &carving_width, "Let $V$ be a finite set with $|V| \\ge 2$. Two subsets $A,B\\subseteq V$ \\emph{cross} if $A\\cap B$, $A-B$, $B-A$, $V-(A\\cup B)$ are all non-empty. A \\emph{carving} in $V$ is a set $\\mathscr{C}$ of subsets of $V$ such that 1) $\\emptyset, V \\notin \\mathscr{C}$ 2) no two members of $\\mathscr{C}$ cross, and 3) $\\mathscr{C}$ is maximal subject to (1) and (2). ... For $A \\subseteq V(G)$, we denote by $\\delta(A)$ ... the set of all edges with an end in $A$ and an end in $V(G)-A$. For each $e \\in E(G)$, let $p(e) \\ge 0$ be an integer. For $X \\subseteq E(G)$ we denote $\\sum_{e \\in X}p(e)$ by $p(X)$, and if $|V(G)| \\ge 2$ we define the \\emph{$p$-carving-width} of $G$ to be the minimum, over all carvings $\\mathscr{C}$ in $V(G)$, of the maximum, over all $A \\in \\mathscr{C}$, of $p(\\delta(A))$. ... The \\emph{carving-width} of $G$ is the $p$-carving-width of $G$ where $p(e)=1$ for every edge $e$.");
    create.source("s11UF7", "https://en.wikipedia.org/wiki/Carving_width")
        .defined("", Unknown, &carving_width, "A carving can be described as an unrooted binary tree whose leaves are labeled with the vertices of the given graph. Removing any edge from this tree partitions the tree into two subtrees, and correspondingly partitions the vertices of the tree into two clusters. ... The width of a carving, defined in this way, is the maximum number of edges that connect two complementary clusters. The carving width of the graph is the minimum width of any hierarchical clustering.");
    create.source("s7OvjQ", "https://en.wikipedia.org/wiki/Graph_bandwidth")
        .defined("", Unknown, &bandwidth, "(paraphrased) Label graph vertices with distinct integers. Bandwidth of this labelling is the maximum over label differences over all edges. Bandwidth of a graph is the minimum over all labellings.");
    create.source("iWUynL", "https://en.wikipedia.org/wiki/Bisection_bandwidth")
        .defined("", Unknown, &bisection_bandwidth, "... bisected into two equal-sized partitions, the bisection bandwidth of a network topology is the bandwidth available between the two partitions.");
    create.source("AeRM2B", "http://parallelcomp.github.io/Lecture3.pdf")
        .defined("", Unknown, &bisection_bandwidth, "(number of) links across smallest cut that divides nodes in two (nearly) equal parts");
    create.source("BJhqpe", "https://en.wikipedia.org/wiki/Feedback_vertex_set")
        .defined("", Unknown, &feedback_vertex_set, "... a feedback vertex set (FVS) of a graph is a set of vertices whose removal leaves a graph without cycles... . The feedback vertex set number of a graph is the size of a smallest feedback vertex set.");
    create.source("4Dua5N", "https://www.fi.muni.cz/~hlineny/papers/shrubdepth-warw18-slipp.pdf")
        .defined("", Unknown, &shrub_depth, "Tree-model of $m$ colors and depth $d$: a rooted tree $T$ of height $d$, leaves are the vertices of $G$, each leaf has one of $m$ colors, an associated signature determining the edge set of $G$ as follows: for $i=1,2,\\dots,d$, let $u$ and $v$ be leaves with the least common ancestor at height $i$ in $T$, then $uv \\in E(G)$ iff the color pair of $u,v$ is in the signature at height $i$.");
    create.source("dxaIhi", "https://mathworld.wolfram.com/Pathwidth.html")
        .defined("", Unknown, &pathwidth, "The pathwidth of a graph $G$, also called the interval thickness, vertex separation number, and node searching number, is one less than the size of the largest set in a path decomposition G.");
    create.source("W4j934", "https://en.wikipedia.org/wiki/Treewidth")
        .defined("", Unknown, &treewidth, "..., the treewidth of an undirected graph is an integer number which specifies, informally, how far the graph is from being a tree.");
    create.source("xnhT1P", "https://www.mimuw.edu.pl/~malcin/book/parameterized-algorithms.pdf")
        .defined("", Unknown, &treewidth, "Very roughly, treewidth captures how similar a graph is to a tree. There are many ways to define ``tree-likeness'' of a graph; ... it appears that the approach most useful from algorithmic and graph theoretical perspectives, is to view tree-likeness of a graph $G$ as the existence of a structural decomposition of $G$ into pieces of bounded size that are connected in a tree-like fashion. This intuitive concept is formalized via the notions of a *tree decomposition* and the *treewidth* of a graph; the latter is a quantitative measure of how good a tree decomposition we can possibly obtain.");
    create.source("ZhBkjd", "https://en.wikipedia.org/wiki/Branch-decomposition")
        .defined("", Unknown, &branch_width, "... branch-decomposition of an undirected graph $G$ is a hierarchical clustering of the edges of $G$, represented by an unrooted binary tree $T$ with the edges of $G$ as its leaves. Removing any edge from $T$ partitions the edges of $G$ into two subgraphs, and the width of the decomposition is the maximum number of shared vertices of any pair of subgraphs formed in this way. The branchwidth of $G$ is the minimum width of any branch-decomposition of $G$.");
    create.source("9Ckusi", "https://en.wikipedia.org/wiki/Clique-width")
        .defined("", Unknown, &clique_width, "... the minimum number of labels needed to construct G by means of the following 4 operations: 1. Creation of a new vertex... 2. Disjoint union of two labeled graphs... 3. Joining by an edge every vertex labeled $i$ to every vertex labeled $j$, where $i \\ne j$ 4. Renaming label $i$ to label $j$");
    create.source("pjVGlR", "https://www.sciencedirect.com/science/article/pii/S0095895605001528")
        .defined("", Unknown, &rank_width, "see Section 6");
    create.source("nyaOye", "https://dl.acm.org/doi/10.1145/3486655")
        .defined("", Unknown, &twin_width, "... we consider a sequence of graph $G_n,G_{n-1},\\dots,G_2,G_1$, where $G_n$ is the original graph $G$, $G_1$ is the one-vertex graph, $G_i$ has $i$ vertices, and $G_{i-1}$ is obtained from $G_i$ by performing a single contraction of two (non-necessarily adjacent) vertices. For every vertex $u \\in V(G_i)$, let us denote by $u(G)$ the vertices of $G$ which have been contracted to $u$ along the sequence $G_n,\\dots,G_i$. A pair of disjoint sets of vertices is homogeneous if, between these sets, there are either all possible edges or no edge at all. The red edges mentioned previously consist of all pairs $uv$ of vertices of $G_i$ such that $u(G)$ and $v(G)$ are not homogeneous in $G$. If the red degree of every $G_i$ is at most $d$, then $G_n,G_{n-1},\\dots,G_2,G_1$ is called a sequence of $d$-contractions, or $d$-sequence. The twin-width of $G$ is the minimum $d$ for which there exists a sequence of $d$-contractions.");
    create.source("YGmwCG", "https://en.wikipedia.org/wiki/Book_embedding")
        .defined("", Unknown, &book_thickness, "... a book embedding is a generalization of planar embedding of a graph to embeddings into a book, a collection of half-planes all having the same line as their boundary. Usually, the vertices of the graph are required to lie on this boundary line, called the spine, and the edges are required to stay within a single half-plane. The book thickness of a graph is the smallest possible number of half-planes for any book embedding of the graph.");
    create.source("WY6oNX", "https://link.springer.com/chapter/10.1007/978-3-642-03367-4_25")
        .defined("", Unknown, &hindex, "... $h$ is the $h$-index of the graph, the maximum number such that the graph contains $h$ vertices of degree at least $h$.");
    create.source("cNSdgE", "https://www.graphclasses.org/classes/par_31.html")
        .defined("", Unknown, &acyclic_chromatic_number, "The acyclic chromatic number of a graph $G$ is the smallest size of a vertex partition $V_1,\\dots,V_\\ell$ such that each $V_i$ is an independent set and for all $i,j$ that graph $G[V_i \\cup V_j]$ does not contain a cycle.");
    create.source("rj2m4h", "https://en.wikipedia.org/wiki/Acyclic_coloring")
        .defined("", Unknown, &acyclic_chromatic_number, "... an acyclic coloring is a (proper) vertex coloring in which every 2-chromatic subgraph is acyclic.");
    create.source("6LCwBu", "https://en.wikipedia.org/wiki/Degeneracy_(graph_theory)")
        .defined("", Unknown, &degeneracy, "... the least $k$ for which there exists an ordering of the vertices of $G$ in which each vertex has fewer than $k$ neighbors that are earlier in the ordering.");
    create.source("VqwUmp", "https://mathworld.wolfram.com/ChromaticNumber.html")
        .defined("", Unknown, &chromatic_num, "The chromatic number of a graph G is the smallest number of colors needed to color the vertices of G so that no two adjacent vertices share the same color (Skiena 1990, p. 210), ...");
    create.source("o6tFCJ", "https://bookdown.org/omarlizardo/_main/2-7-average-degree.html")
        .defined("", Unknown, &average_degree, "Average degree is simply the average number of edges per node in the graph. ... Total Edges/Total Nodes=Average Degree");
    create.source("PVi4lL", "https://mathworld.wolfram.com/MaximumClique.html")
        .defined("", Unknown, &max_clique, "A maximum clique of a graph $G$ is a clique (i.e., complete subgraph) of maximum possible size for $G$.");
    create.source("ZunX1e", "https://mathworld.wolfram.com/EdgeConnectivity.html")
        .defined("", Unknown, &edge_connectivity, "The edge connectivity, also called the line connectivity, of a graph is the minimum number of edges $\\lambda(G)$ whose deletion from a graph $G$ disconnects $G$.");
    create.source("XWbXPm", "https://en.wikipedia.org/wiki/Boxicity")
        .defined("", Unknown, &boxicity, "The boxicity of a graph is the minimum dimension in which a given graph can be represented as an intersection graph of axis-parallel boxes.");
    create.source("8eXjAy", "https://mathworld.wolfram.com/DomaticNumber.html")
        .defined("", Unknown, &domination_num, "The maximum number of disjoint dominating sets in a domatic partition of a graph $G$ is called its domatic number $d(G)$. ");
    create.source("zYzUJ5", "https://onlinelibrary.wiley.com/doi/abs/10.1002/jgt.3190170210")
        .defined("", Unknown, &chordality, "The \\emph{chordality} of a graph $G=(V,E)$ is defined as the minimum $k$ such that we can write $E=E_1,\\cap\\dots\\cap E_k$ with each $(V,E_i)$ a chordal graph.");
    create.source("TKnuNP", "https://www.sciencedirect.com/science/article/pii/0166218X9290275F?via%3Dihub")
        .defined("", Unknown, &max_induced_matching, "An induced matching in a graph G is a set of edges, no two of which meet a common node or are joined by an edge of G;");
    create.source("055mG5", "https://en.wikipedia.org/wiki/Distance_(graph_theory)#Related_concepts")
        .defined("", Unknown, &diameter, "... [diameter] is the greatest distance between any pair of vertices ...");
    create.source("GfSsR4", "https://onlinelibrary.wiley.com/doi/abs/10.1002/jgt.3190120309")
        .defined("", Unknown, &average_degree, "The average distance in a graph is defined as the average length of a shortest path between two vertices, taken over all pairs of vertices.");
    create.source("u13WN1", "https://en.wikipedia.org/wiki/Girth_(graph_theory)")
        .defined("", Unknown, &girth, "In graph theory, the girth of an undirected graph is the length of a shortest cycle contained in the graph.");


    // #  Chung1985=Source("DkY1Aq", "Chung1985", shows that for trees bandwidth and cutwidth linearly bound each other
    let bodlaender1993 = create.source("a3yKzk", "Bodlaender1993")
        .done();
    let bodlaender1998 = create.source("BOFCWg", "Bodlaender1998")
        .defined("", Unknown, &cutwidth, "Let $G=(V,E)$ be a graph, and let $f\\colon V\\to \\{1,2,\\dots,n\\}$ be a linear ordering of $G$. ... 2. The \\emph{cutwidth} of $f$ is $\\max_{1\\le i\\le n} |\\{(u,v)\\in E \\mid f(u) \\le i < f(v) \\}|$. ... [cutwidth] of a graph $G$ is the minimum [cutwidth] ... over all possible linear orderings of $G$.")
        .defined("hajrD0", Unknown, &bandwidth, "Let $G=(V,E)$ be a graph, and let $f\\colon V\\to \\{1,2,\\dots,n\\}$ be a linear ordering of $G$. 1. The \\emph{bandwidth} of $f$ is $\\max\\{|f(v)-f(w)| \\mid (v,w) \\in E\\}$. ... The bandwidth ... is the minimum bandwidth ... over all possible linear orderings of $G$.")
        .defined("H3lAh2", Unknown, &topological_bandwidth, "The \\emph{topological bandwidth} of a graph $G$ is the minimum [bandwidth](../aP5a38) over all graphs $G'$ which are obtained by addition of an arbitrary number of vertices along edges of $G$.")
        .showed("uHJAUo", Unknown, &pathwidth, &treewidth, Cpx::UpperBound(Exists), "Lemma 3. (a) For all graphs $G$, $pathwidth(G) \\ge treewidth(G)$. ...")
        .done();
    let corneil2005 = create.source("HCGunF", "Corneil2005")
        .showed("sGBrPC", Unknown, &treewidth, &clique_width, Cpx::UpperBound(Exponential), "... the clique-width of $G$ is at most $3 \\cdot 2k - 1$ and, more importantly, that there is an exponential lower bound on this relationship. In particular, for any $k$, there is a graph $G$ with treewidth equal to $k$, where the clique-width of $G$ is at least $2\\lfloor k/2\\rfloor-1$.")
        .done();
    let oum2006 = create.source("1ZTWBd", "Oum2006")
        .done();
    let sasak2010 = create.source("XlBXyo", "Sasak2010")
        .done();
    let bui_xuan2011 = create.source("cNjhWx", "BuiXuan2011")
        .defined("L7aY6D", Unknown, &boolean_width, "\\textbf{Definition 1.} A decomposition tree of a graph $G$ is a pair $(T,\\delta)$ where $T$ is a tree having internal nodes of degree three and $\\delta$ a bijection between the leaf set of $T$ and the vertex set of $G$. Removing an edge from $T$ results in two subtrees, and in a cut $\\{A,\\comp{A}\\}$ of $G$ given by the two subsets of $V(G)$ in bijection $\\delta$ with the leaves of the two subtrees. Let $f\\colon w^V \\to \\mathbb{R}$ be a symmetric function that is also called a cut function: $f(A)=f(\\comp{A})$ for all $A\\subseteq V(G)$. The $f$-width of $(T,\\delta)$ is the maximum value of $f(A)$ over all cuts $\\{A,\\comp{A}\\}$ of $G$ given by the removal of an edge of $T$. ... \\textbf{Definition 2.} Let $G$ be a graph and $A \\subseteq V(G)$. Define the set of unions of neighborhoods of $A$ across the cut $\\{A,\\comp{A}\\}$ as $U(A) = \\{Y \\subseteq \\comp{A} \\mid \\exists X \\subseteq A \\land Y=N(X)\\cap \\comp{A}\\}. The \\emph{bool-dim}$\\colon 2^{V(G)} \\to \\mathbb{R}$ function of a graph $G$ is defined as $\\mathrm{bool-dim}(A)=\\log_2 |U(A)|$. Using Definition 1 with $f=\\mathrm{bool-dim}$ we define the boolean-width of a decomposition tree, denoted by $boolw(T,\\delta)$, and the boolean-width of a graph, denoted by $boolw(G)$.")
        .done();
    let belmonte2013 = create.source("sJ476m", "Belmonte2013")
        .done();
    let jansen2013 = create.source("FLOjic", "Jansen2013")
        .defined("PV6tGG", Unknown, &topological_bandwidth, "The \\emph{topological bandwidth} of a graph $G$ is the minimum [bandwidth](../aP5a38) over all subdivisions of $G$")
        .done();
    let parameterizedAlgorithms2015 = create.source("", "ParameterizedAlgorithms2015")
        .done();
    let diestel2017 = create.source("r2Lwky", "Diestel2017")
        .defined("hxpfbI", Pp(3), &complete, "If all the vertices of $G$ are pairwise adjacent, then $G$ is \\emph{complete}.")
        .defined("T8RJcC", Pp(5), &isolated, "A vertex of degree $0$ is \\emph{isolated}.")
        .defined("8XlBpy", Pp(13), &forest, "An acyclic graph, one not containing any cycles, is called a \\emph{forest}.")
        .defined("P1ExcE", Pp(17), &bipartite, "Instead of `2-partite' one usually says bipartite.")
        .defined("eMZCoY", Pp(89), &planar, "When we draw a graph on a piece of paper, we naturally try to do this as transparently as possible. One obvious way to limit the mess created by all the lines is to avoid intersections. ... Graphs drawn in this way are called \\emph{plane graphs}; abstract graphs that can be drawn in this way are called \\emph{planar}.")
        .defined("6Q0kuL", Pp(115), &outerplanar, "A graph is called outerplanar if it has a drawing in which every vertex lies on the boundary of the outer face.")
        .defined("wkrz7h", Pp(135), &chordal, "... a graph is chordal (or triangulated) if each of its cycles of length at least $4$ has a chord, i.e. if it contains no induced cycles other than triangles.")
        .defined("54XChb", Pp(135), &perfect, "A graph is called perfect if every induced subgraph $H \\subseteq G$ has chromatic number $\\chi(H)=\\omega(H)$, i.e. if the trivial lower bound of $\\omega(H)$ colours always suffices to colour the vertices of $H$.")
        .defined("pMo8dB", Pp(145), &interval, "A graph $G$ is called an \\emph{interval graph} if there exists a set $\\{ I_v \\mid v \\in V(G) \\}$ of real intervals such that $I_u \\cap I_v \\ne \\emptyset$ if and only if $uv \\in E(G)$.")
        .done();
    let froemmrich2018 = create.source("45xW87", "Froemmrich2018")
        .done();
    let ganian2019 = create.source("Scw7zm", "Ganian2019")
        .done();
    let sorge2019 = create.source("VnTIL0", "Sorge2019")
        .done();
    let schroder_thesis = create.source("DYGiYb", "SchroderThesis")
        .cited("pJxHVS", Unknown, sorge2019, "Based on the work by [Sa19] as well as [Fr8], we investigate unknown connections between graph parameters to continue the work on the graph parameter hierarchy")
        .cited("", Unknown, froemmrich2018, "Based on the work by [Sa19] as well as [Fr8], we investigate unknown connections between graph parameters to continue the work on the graph parameter hierarchy")
        .showed("R9eI61", Unknown, &treedepth, &diameter, Cpx::UpperBound(Linear), "Proposition 3.1")
        .done();
    let tran2022 = create.source("uXViPE", "Tran2022")
        .defined("J1sHj8", Unknown, &twin_cover_num, "An edge $\\{v,w\\}$ is a twin edge if vertices $v$ and $w$ have the same neighborhood excluding each other. The twin cover number $tcn(G)$ of a graph $G$ is the size of a smallest set $V' \\subseteq V(G)$ of vertices such that every edge in $E(G)$ is either a twin edge or incident to a vertex in $V'$")
        .defined("MlTT7n", Unknown, &edge_clique_cover, "The edge clique cover number $eccn(G); of a graph $G$ is the minimum number of complete subgraphs required such that each edge is contained in at least one of them.")
        .defined("iAkCJ3", Unknown, &neighborhood_diversity, "The neighborhood diversity $nd(G); of a graph $G$ is the smallest number $k$ such that there is a $k$-partition $(V_1,\\dots,V_k)$ of $G$, where each subset $V_i$, $i \\in [k]$ is a module and is either a complete set or an independent set.")
        .defined("i3su9L", Unknown, &modular_width, "The modular-width $mw(G)$ of a graph $G$ is the smallest number $h$ such that a $k$-partition $(V_1,\\dots,V_k)$ of $G$ exists, where $k \\le h$ and each subset $V_i$, $i \\in [k]$ is a module and either contains a single vertex or for which the modular-subgraph $G[V_i]$ has a modular-width of $h$.")
        .defined("eGC0vH", Unknown, &boxicity, "The boxicity of a graph $G$ is the minimum amount of interval graphs required, such that their intersecten results in $G$")
        .defined("gZtk6B", Unknown, &chordality, "The chordality of a graph $G$ is the minimum amount of chordal graphs required, such that their intersecten results in $G$")
        .done();
    let torunczyk2023 = create.source("KpkMZB", "Torunczyk2023")
        .defined("9VHraO", Unknown, &inf_flip_width, "See radius-r flip-width for $r=\\infty$.")
        .defined("gxeVOT", Unknown, &r_flip_width, "The radius-$r$ flip-width of a graph $G$, denoted $fwr (G)$, is the smallest number $k \\in \\mathbb{N}$ such that the cops have a winning strategy in the flipper game of radius $r$ and width $k$ on $G$")
        .done();

    // tran2022.defined("i3su9L", modular_width, "The modular-width $mw(G)$ of a graph $G$ is the smallest number $h$ such that a $k$-partition $(V_1,\\dots,V_k)$ of $G$ exists, where $k \\le h$ and each subset $V_i$, $i \\in [k]$ is a module and either contains a single vertex or for which the modular-subgraph $G[V_i]$ has a modular-width of $h$.");
    return create.done();
}

