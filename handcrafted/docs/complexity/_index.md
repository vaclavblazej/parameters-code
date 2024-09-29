---
layout: "single"
title: "Parameterized complexity"
---

# Complexity theory

This short section is meant to only introduce a few key notions used on this website.
The reader is encouraged to read up on topics further, possibly in one of the following sources.

* [Parameterized Algorithms](https://www.mimuw.edu.pl/~malcin/book/parameterized-algorithms.pdf) by Marek Cygan, Fedor V. Fomin, Łukasz Kowalik, Daniel Lokshtanov, Dániel Marx, Marcin Pilipczuk, Michał Pilipczuk, and Saket Saurabh
* [Computational Complexity](https://www.cambridge.org/core/books/computational-complexity/3453CAFDEB0B4820B186FE69A64E1086) by Sanjeev Arora and Boaz Barak

## Rough classical complexity theory intro

In decision problems we are given an input of size $n$ and should provide a *yes* or *no* answer.
Complexity class of the problem describes how hard is it to compute the answer.

The problems we are typically concerned with are in NP class, i.e., those which veryfying a *yes* answer can be done in polynomial time if we supply a certificate -- for example, this certificate can be the solution which witnesses the yes answer.
Historically, one of the most popular topics was to distinguish between problems that are solvable in $n^{\mathcal O(1)}$ (polynomial) time, and those that are probably not solvable in polynomial time.
We say *probably* because saying that a problem is definitely not solvable in polynomial time would resolve the [P vs NP problem](https://en.wikipedia.org/wiki/P_versus_NP_problem).

With a problem in hand, we focus on either coming up with a polynomial-time algorithm, or showing a polynomial-time reduction from a NP-hard problem to our problem.
NP-hard problems are the "hardest" in the NP class, meaning one can reduce *any* problem from NP to (any) NP-hard problem in polynomial time.
Such a reduction refutes that the problem could be solved in polynomial time because if we could solve it, then we could also take the NP-hard problem, reduce it to our problem, and solve it in polynomial time (giving yes or no answer).
Using the same logic, we could concatenate the reduction from any problem to the NP-hard problem with our reduction to solve any other problem in NP-class within polynomial time.
So solving any single NP-hard problem would show how to solve all problems in the NP class in polynomial time, also resolving the [P vs NP problem](https://en.wikipedia.org/wiki/P_versus_NP_problem).

## Parameterized complexity

This webpage is currently focused on structural parameters for undirected graphs.
Graph $G=(V,E)$ consists of vertices $V$ and edges $E$.
Edges are pairs $\{u,v\}$ of elements from $u,v \in V$.
There are many problems that can be modelled using graphs.

For the sake of an example, let us take a well-known problem called clique.
In the clique problem we have a graph and an integer $k$ on the input.
We are asked to decide whether the graph contains $k$ vertices that are all pairwise connected with edges.

The clique problem is known to be NP-hard but for some graph classes it is solvable in polynomial time.
For example, we know that [planar graphs](https://en.wikipedia.org/wiki/Planar_graph) cannot contain a clique on 5 vertices.
So if $k$ is 5 or more, we can answer immediately NO.
Otherwise, we can check all quadruplets of vertices, so we have an algorithm that runs in $n^4$ time.

It is clear that the above example can be generalized.
If we are guaranteed that the input does not contain a clique of size $h+1$, then we can solve the problem in roughly $\binom nh \le n^h$ by trying every $h$-tuplet of vertices.
The graphs that do not contain a clique of size $h+1$ form a class of graphs with maximum clique at most $h$ and we just showed that the problem can be solved in polynomial time for each fixed value of $h$.
This notion is easily expressed in terms of parameterized complexity.

Value of $h$ essentially restricts what input can be given to our algorithm.
Consider how one may look at the value of $h$.
If $h$ is constant, then the input is restricted to a subset of inputs that satisfy the assumption on size of their maximal clique.
If $h$ is not restricted, then we are solving the original problem without $h$.
In parameterized complexity, each input we get is the instance along with its value of $h$ -- guaranteeing that in the instance does have no clique of size $h+1$.

Formally, introducing a parameter is by defining parameterized problems as being over languages $L \subseteq \Sigma^* \times \mathbb N$ (instead of the classical $L \subseteq \Sigma^*$).
What we showed previously is that the clique problem parameterized by $h$ can be solved in $n^h$, we call this complexity XP (slice-wise polynomial).
Once we know a problem is in XP it makes sense to investigate its parameterized complexity more -- is $n^h$ the best we can do, or is there a better algorithm?

In somewhat similar manner to the polynomial vs NP-hard question, we may try to show whether a problem is FPT or W[1]-hard.
On one hand, we may show that the problem is fixed parameter tractable (FPT).
FPT algorithms run in time $f(h) \cdot n^{O(1)}$ for some (computable) function $f$, i.e., the polynom exponent does not depend on the parameter.
On the other hand, W[1]-hardness is shown by finding a parameterized reduction from a W[1]-hard problem to our problem.
Parameterized reduction means it must run in FPT time, while increasing size of the parameter by at most some computable function.
And again, if someone came up with an FPT algorithm to solve any W[1]-hard problem, then all problems in W[1] could be solved in FPT time.
So by showing a W[1]-hardness reduction, we claim there is no FPT algorithm, under the assumption that $W[1] \ne FPT$.

There are many other things one may show about a parameterized problem, which we do not introduce here, but to give at least some pointers, here is a list of terms to search for.

* [Kernelization](https://en.wikipedia.org/wiki/Kernelization) - A preprocessing method of reducing instance size, used for problems that are FPT.
* [Exponential Time Hypothesis (ETH)](https://en.wikipedia.org/wiki/Exponential_time_hypothesis) - $W[1]$-hard problem can be shown to be not solvable better than e.g. $n^h$ if we assume that 3-Satisfiability cannot be solved in $2^{o(n)}$ time.
* [$W$ hierarchy](https://en.wikipedia.org/wiki/Parameterized_complexity#W_hierarchy) - There is a hierarchy of $W[t]$ problems, not just $W[1]$.
* [Treewidth](https://en.wikipedia.org/wiki/Treewidth) - Perhaps the most prominent parameter.
* [Iterative compression](https://en.wikipedia.org/wiki/Iterative_compression) - A technique of turning an approximate to an exact solution in FPT time.
* [Matroids](https://en.wikipedia.org/wiki/Matroid) - Many problems can be expressed in the language of matroids, which have some general tractability results.
* and many others. As noted at the beginning, read Parameterized Algorithms book, which contains these (and other) topics.

