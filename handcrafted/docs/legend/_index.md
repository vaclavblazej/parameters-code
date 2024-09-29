---
layout: "single"
title: "Legend"
---

# Legend for website's graphical elements

## Hasse diagrams

The Hasse diagrams show parameter inclusions.

Arrows that can be implied by other arrows are hidden for clarity.

Parameters are represented by boxes with relations between them are depicted by directed edges.

**Parameter box** identifies a parameter by its most prominent name.
Alternative names should be listed on its page.

**Directed edges** depict inclusions.
An edge from A to B implies that if A is upper bounded by $k$ then B is upper bounded by $f(k)$ where $f$ is a computable function.
Style of an arc represents the known upper bound on $f$:

* very thick black - $f$ is a constant, i.e., $A$ can have unbounded values while $B$ one is always constant $O(1)$
* thick black - $f$ is a linear or sublinear function $O(k)$
* thin black - $f$ is a polynomial function $k^{O(1)}$
* dotted - $f$ is an exponential $2^{O(k)}$ or tower function
* gray - the value of $f$ inclusion is not yet filled in HOPS

Graphs are depicted in Hasse diagrams as well, for that purpose $k$ is assumed to be arbitrary.

## Local Hasse diagrams

Are the same as the Hasse diagrams above but display a smaller number of parameters.

## Graph diagram for a fixed parameter and parameter diagram for a fixed graph

Red-green colored diagrams show inclusion for a particular relation.

For a fixed parameter we display graph Hasse diagram with the following colors.

* green graph - has constant parameter
* red graph - has unbounded parameter
* gray graph - unknown to HOPS

For a fixed graph class we display a parameter Hasse diagram with the folowing colors.

* green parameter - is constant for the graph class
* red parameter - is unbounded for the graph class
* gray parameter - unknown to HOPS

## Pairwise relation tables

A 2D table allows a simple depiction of all pairwise relations at once.
Each cell at row $A$ and column $B$ represents relation from parameter $A$ to $B$.
Assume $A$ is bounded by $k$, then color of the cell represents the following.

* green cell - $B$ is bounded by $f(k)$ for some computable function of $k$
* red cell - $B$ is unbounded
* blue cell - it is unknown to HOPS whether $B$ is bounded or unbounded

