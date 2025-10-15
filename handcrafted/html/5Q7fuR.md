**Treewidth** is arguably the most important graph parameter.
Intuitively, it describes how close a graph is to a tree.

On this page we cover only an overview of treewidth.
For a more comprehensive introduction, we direct the reader to [[Xlsyce]].

## Definition

A *tree decomposition* is a tree $T$ together with a function $\chi$.
To distinguish $T$ from $G$ we call $V(G)$ *vertices* and $V(T)$ *nodes*.
The function $\chi$ maps each node to a set of vertices in $G$.
For each node, the set of vertices assigned to it are called a *bag*.
The tree decomposition $(T,\chi)$ has to follow two rules:

1. Each vertex of the graph is in bags that form a (nonempty) connected subtree of $T$.
2. Each edge of the graph has both of its endpoints in some common bag.

A graph has treewidth $k$ if there exists a tree decomposition such that each bag has size at most $k+1$.

### Other definitions and examples

Alternative equivalent definitions of a graph with treewidth $\le k$ are e.g.

* subgraph of an intersection graph of subtrees of a tree with ply at most $k+1$,
* there exists a completion to a chordal graph with maximal cliques of size at most $k+1$,
* partial $k$-tree.

Notably, $k=1$ are forests and $k=2$ are series-parallel graphs.
On the other hand, cliques $K_n$ have treewidth $k=n-1$ and complete bipartite graphs $K_{a,b}$ have $k=\min\{a,b\}$.

## Properties

* property of having treewidth $\le k$ is minor closed
* *grid minor theorem* shows that if a graph has treewidth $\ge k$ then it contains a grid minor of size $f(k)$ for some unbounded function $f$, this is closely related to solving graph problems through *bidimensionality*
* a graph has bounded treewidth if and only if it has *balanced separators*
* *Courcelle's theorem* shows that any problem expressible in monadic second-order logic on graphs can be solved in FPT time when parameterized by size of the formula and treewidth

## Computing treewidth

Deciding whether a graph has treewidth $k$ is NP-complete, [even on cubic graphs](https://www.combinatorics.org/ojs/index.php/eljc/article/view/v32i3p36?utm_source=chatgpt.com).
Computing treewidth is FPT with respect to $k$; approximating is FPT but linear in $n$.

## Dynamic programming

Structure of the decomposition implies several important properties.
The main property is that each bag constitutes a separator.
This allows us to design a bottom-up dynamic programming (DP) algorithms over the tree decomposition for many problems.
For the DP examples below we assume the reader is comfortable in designing DP bottom-up algorithms on trees for problems like vertex cover, dominating set, weighted independent set, etc.

Though it is possible to design DP over tree decomposition directly we usually simplify the decomposition so that at every step only one simple 'thing' is happening.
*Nice tree decomposition* can be computed from a tree decomposition in $O(n)$ time and has the following additional properties:

1. Every node has one of types -- leaf, introduce vertex, forget vertex, join.
2. The decomposition is rooted in some leaf node.
3. Leaf nodes are empty and childess, except the root which has one child.
4. Introduce vertex node has the same bag as its only child, and adds a new vertex.
5. Forget vertex node has the same bag as its only child, and removes one of the vertices.
6. Join node has exactly the same bag as its two children.

If introduce edge nodes are not present, then an edge is introduced when its second incident vertex is introduced.

---
