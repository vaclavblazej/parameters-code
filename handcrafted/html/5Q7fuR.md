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

### Basic DP example

... work in progress

### A few brief examples

* vertex cover -- state: part of the solution in the bag; value: size of the solution in the subgraph; introduce vertex node tries both options and verifies all present edges are present; forget vertex node does nothing interesting; join node sums the solution minus size of the solution in the bag which was counted twice
* ... work in progress

---

