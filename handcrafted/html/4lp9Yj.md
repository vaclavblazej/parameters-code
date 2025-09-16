**Vertex cover** is perhaps the simplest but useful parameter.
Although the graph class of bounded vertex cover is small, it is often the first parameter for which we aim to design an FPT algorithm.
Vertex cover can be easily 2-approximated in polynomial time.

From the definition, we see the graph of vertex cover $k$ can be partitioned into a *modulator* of size $k$ and an independent set.
The edges here can be only between the vertices of the moduator or from the modulator to the independent set.

FPT algorithms often exploit the fact that vertices of the independent set can be partitioned into $2^k$ groups based on their neighborhood in the modulator.
Now, one can either enumerate all the solutions or notice that whenever a part contains too many vertices we can ignore the vast majority of them as they do not influence the solution.

