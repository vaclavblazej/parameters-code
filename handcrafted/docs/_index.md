---
layout: "single"
title: "HOPS"
---

# HOPS Tutorial & Documentation

This website uses several things that deserve to be explained in detail.
More elementary terms of the field are explained in *Complexity theory* section later on this page.

## Hierarchy images legend

Parameters are represented by boxes with relations between them depicted by edges as follows.

**Parameter box** identifies a parameter by its most well-known name.
Alternative names are listed on the page of the parameter.

**Directed edges** depict inclusions.
An edge from A to B implies that if A is bounded $k$ then B is bounded by $f(k)$ where $f$ is a computable function.

* type of edge further ties the bound
    * thick black - $f$ is a linear function
    * thin black - $f$ is at most a polynomial function
    * dotted - $f$ is at most an exponential function
    * gray - the source of the inclusion is not yet filled in HOPS
* head of an edge signified whether the inclusion is strict
    * normal arrow - unknown
    * subset arrow - the inclusion is proper, i.e., there is a graph class that has unbounded A but bounded B
* circle on edges link to the page of that relation
* the horizontal bar signify whether it is feasible that the relation could be improved
    * bar at head means that if we changed B endpoint of the edge to any (listed) parent of B then the known facts would imply false
    * bar at tail means that if we changed A endpoint of the edge to any child of B then the known facts would imply false

## Graph-parameter images legend

On parameter page there is an overview of several graph classes and vice-versa.
These images show whether the particular parameter is constant for the given graph class.

## Controls

**PDF Controls:**
Zoom with Ctrl+wheel, Click nodes or edge-circles to open relevant page.

## Complexity theory

This short section is meant to only introduce a few key notions used on this website.
The reader is encouraged to read up on topics further, possibly in one of the following sources.

* *Parameterized Algorithms* by Marek Cygan, Fedor V. Fomin, Łukasz Kowalik, Daniel Lokshtanov, Dániel Marx, Marcin Pilipczuk, Michał Pilipczuk, and Saket Saurabh
* *Computational Complexity* by Sanjeev Arora and Boaz Barak

### Rough intro

This page is currently focused on structural parameters for undirected graphs.
Graph $G=(V,E)$ consists of vertices $V$ and edges $E$.
Edges are pairs $\{u,v\}$ of elements from $u,v \in V$.

There are many problems that can be modelled on graphs.
In decision problems we are given an input of size $n$ and should provide a *yes* or *no* answer.
Complexity class of the problem describes how hard is it to compute the answer.
The most popular is to distinguish between problems that are solvable in $n^{\mathcal O(1)}$ (polynomial) time, and those problems that are probably not solvable in polynomial time.
We say *probably* because saying that they are definitely solvable in polynomial, or definitely not solvable in polynomial time would answer the [P vs NP problem](https://en.wikipedia.org/wiki/P_versus_NP_problem).

So with a problem in hand we focus on either coming up with a polynomial time algorithm, or showing a polynomial time reduction from a NP-hard problem to our problem.
The reduction opposes that the problem could be solved in polynomial time because if we could solve it, then we could also take the NP-hard problem, reduce it to our problem, solve it in polynomial time (giving yes or no answer).
NP-hard problems are the "hardest" in the NP class, meaning one can reduce *any* problem from NP to (any) NP-hard problem in polynomial time.
So solving one in polynomial time would show that we can solve all problems in NP in polynomial time.

### Parameterized complexity

For the sake of an example, let us take a well-known problem: clique.
In the clique problem we have a graph and an integer $k$ on the input.
We are asked to decide whether the graph contains $k$ vertices that are all pairwise connected with edges.

The clique problem is known to be NP-hard but for some graph classes it is tractable (i.e. solvable).
For example, we know that [planar graphs](https://en.wikipedia.org/wiki/Planar_graph) cannot contain a clique on 5 vertices.
So if $k$ is 5 or more, we can answer immediately NO.
Otherwise, we can check all quadruplets of vertices, so we have an algorithm that runs in $n^4$ time.

It is clear that the above example can be generalized.
If we are guaranteed that the input does not contain a clique of size $h+1$, then we can solve the problem in $n^h$ with approach that is very similar to the one presented for planar graphs.
The graphs that do not contain a fixed-sized clique of size $h+1$ form a class of graphs with maximum clique at most $h$ and we just showed that the problem can be solved in polynomial time for each fixed value of $h$.
This notion is better expressed in terms of parameterized complexity.

Let $h$ be our parameter, then what we just showed is called an XP (slice-wise polynomial) algorithm for the clique problem with respect to the maximum clique $h$.
There are further things that we may aim to show for a problem once we know it is XP.

* similar to P NP gap, under the assumption that ? we may be able to show only one of the following two options
    * find an algorithm that solves the problem in $f(k) \cdot n^{\mathcal O(1)}$ time -- FPT (fixed parameter tractable) algorithm
    * show a parameterized reduction from a W[1]-hard problem to our problem -- W[1]-hardness result
* if the problem is FPT then we can refine what we know about its kernel
    * polynomial kernel
    * unlikely polynomial kernel
* if the problem is W[1]-hard then we can try to go further in parameterized hardness hierarchy
    * W[k] membership for some $k$
    * W[k]-hard for some $k > 1$
    * W[k]-hard for all $k$
    * XNLP-hard
    * XNLP membership

### Refining results and how does HOPS help

It is known that clique problem is W[1]-hard with respect to maximum clique parameter.
Having that result we can try to refine it further.
One possible direction is to try to alter the problem.
Come up with a generalization or specification and show results for those variants.
The work may be little easier because a more general problem inherits hardness of the original problem and a more specific inherits problem's tractability.
Another axis of improving the result is to move along the parameter hierarchy -- this is where the HOPS comes into play.

From HOPS we observe that [maximum clique](../html/q7zHeT/) is upper bounded by [chromatic number](../html/w7MmyW/).
This means that graphs with chromatic number $h$ have maximum clique at most $f(h)$ (in this specific case $f$ is known to be a linear function).
Notice that when we substitute a parameter for a computable function of a parameter (substitute $h$ for $f(h)$) an algorithm that was FPT remains FPT, and an XP algorithm remains XP.
Hence, our tractability results automatically also work for any parameter that upper bounds maximum clique.
So to improve our results, we can now show that maximum clique problem remains W[1]-hard even when parameterized by chromatic number -- this result is indeed also known to be true.
A natural question now is: how far up can we push the hardness, and for which parameters does the problem become FPT?

There are many parameters, some more prominent than others.
Many are based on (or even defined by) some kind of decomposition that facilitates algorithm design.
From the other side, for many parameters we know problems that are W[1]-hard for them, making it feasible that a reduction from that problem could yield hardness for our problem as well (if the reduction preserves value of the parameter).
