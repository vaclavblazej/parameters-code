---
layout: "single"
title: "Collaboration"
---

# Collaboration

All forms of collaboration are appreciated.

* [Related projects](#related-projects)
* [How to contribute to HOPS](#contribute-to-hops)
* [List of contributors](#contributors)

## Related projects

* [Information System on Graph Classes and their Inclusions (ISGCI) - H. N. de Ridder and others](https://www.graphclasses.org/)
* [The House of Graphs - K. Coolsaet, S. D'hondt and J. Goedgebeur](https://houseofgraphs.org/)
* [Graph Parameter Hierarchy Visualizer - Yuto Okada (岡田優斗)](https://yutookada.com/tools/graph-parameters/)
* [Graph Parameter Hierarchy - Yeonsu Chang (장연수)](https://www.graphparameterhierarchy.com/)

## Contribute to HOPS

One may contribute in many ways; the next sections lists some options in an increasing level of involvement.

### Feedback

If you find missing information, mistakes, or have suggestions [get in touch](./contact).

Perhaps the most straight-forward help is to submit sources of relations that are listed under the [unknown source](../html/myit4D/).

### Survey works

Works that create a comprehensive picture for relations among parameters are a great resource in general.
Though not directly working on HOPS, such a work is very likely to be linked from here as a reference for the proved or even cited relations.
For a student, such a work can make them familiar with the field and some parameters in particular -- allowing them to work on relevant research questions soonafter.

### Independent project

The data from this website is intended to be widely accessible.
Currently, there is only a simple [fragment of the api](../api/simple.json) that can be used for basic visualizations.
More full-featured API is planned in future.
Before it is ready, it is possible to contact us and get an export of all the hierarchy information in a json format.
Another possibility is to run the backend code yourself and retrieve all the information directly.

### Frontend programming

This website is made as a static website where any interactivity comes from javascript.
Part of the content is generated and part is hand-made.
The final markdown is made by joining these two parts and then replacing commands (marked with double square brackets) by respective content.
Improvements to the presentation can be made almost independently on the backend.
There are several features one could work on:

* interactive viewer - Select parts of the hierarchy that you want to see and display just that. Possibly with draggable elements and export to TikZ -- draggable elements are in the viwer of Yeonsu Chang (see related projects above).
* problem solution helper - Similar to the above but made mainly to see complexity of a problem, e.g., marking a fixed problem easy on some parameter should propagate this information to more restrictive parameters; similarly marking hardness propagates to less restrictive parameters -- this was graphically implemented in the project of Yuto Okada (see related projects above).

### Backend programming

Code of the backend is [open source, hosted on GitHub](https://github.com/vaclavblazej/parameters-code), and open for pull requests.
The backend is written in Rust which may be an obstacle for getting involved.
Be sure to get in touch if you plan to do backend improvements or fixes.
This is mainly to prevent doing duplicate work.

## Contributors

This project is made and maintained by [Václav Blažej](https://vaclavblazej.github.io/about/).
Thanks to the memebrs of the community who provided [feedback](./contact) for the website.

People that contribute to this project will be listed below; let me know if you believe to be missing here.

