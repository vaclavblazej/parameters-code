---
layout: "single"
title: "Collaboration"
---

# Collaboration

All forms of collaboration are appreciated, see the [list of contributors](#contributors) at the bottom of this page.
One may contribute in many ways; the next sections lists these in an increasing level of involvement.

### Feedback

The simplest way to contribute is to [write an e-mail](mailto:vaclavblazej@gmail.com) or [raise an issue](https://github.com/vaclavblazej/parameters-code/issues) (requires login to GitHub) regarding anything related to the website.
This concerns but is not limited to:

* missing parameters or definitions
* missing relations or exclusions
* missing citations
* incorrect information
* improvement suggestions

### Survey works

Works that create a comprehensive picture for relations among parameters are a great resource in general.
Though not directly working on HOPS, such a work is very likely to be linked from here as a reference for the proved or even cited relations.
For a student, such a work can make them familiar with the field and some parameters in particular -- allowing them to work on relevant research questions soonafter.

### Independent project

The data from this website is intended to be widely accessible.
Currently, there is no stable API because the structure is still in a non-trivial flux.
API is planned as a future feature.
In the meantime, it is possible to contact us and get an export of all the hierarchy information in a json format.
Another possibility is to run the backend code yourself and retrieve all the information directly.

### Frontend programming

The website is made as a static website where any interactivity comes from javascript.
Part of the content is generated and part is hand-made.
The final markdown is made by joining these two parts and then replacing commands (marked with double square brackets) by respective content.
Improvements to the presentation can be made almost independently on the backend.
There are several features one could work on:

* interactive viewer - Select parts of the hierarchy that you want to see and display just that. Possibly with draggable elements and export to TikZ.
* problem solution helper - Similar to the above but made mainly to see complexity of a problem, e.g., marking a fixed problem easy on some parameter should propagate this information to more restrictive parameters; similarly marking hardness propagates to less restrictive parameters.

### Backend programming

The backend is written in Rust which may be an obstacle for getting involved.
Be sure to get in touch if you plan to do some backend improvements or fixes.
This is mainly to prevent doing duplicate work.

## Contributors

This project is made and maintained by [Václav Blažej](https://vaclavblazej.github.io/about/).
Thanks to the memebrs of the community who provided feedback for the website -- in person, by e-mail, or on [github issues](https://github.com/vaclavblazej/parameters-code/issues?q=is%3Aissue).

People that contribute to this project will be listed below; let me know if you believe to be missing here.

