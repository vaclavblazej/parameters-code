---
layout: "single"
title: "Controls"
---

# List of controls for website's elements

## Diagrams

Many diagrams on the website are embedded client-side *graphviz* generated SVG images.
These diagrams have embedded links so their elements are clickable and they redirect you to the relevant website.

* zoom - mouse wheel
* move - left mouse click and drag
* open link - left click on the element

These diagrams have a known bug that dragging while mouse is over a clickable element results in the link being open.

## PDFs

The website contains various PDF diagrams.

Particular display of a PDF depends on the browser you are using.
Even then, there are some common controls:

* move - mouse wheel or shift+mouse wheel
* zoom - Ctrl+mouse wheel
* open link - click on a PDF element to open a relevant page

## Website

The project right now is made to be a simple static website with pre-generated content -- there is no backend server.
It is also designed to work well without javascript, but contains a few javascript-based quality of life features (like interactable diagrams, table sorting).

* Use Ctrl+f on the landing page to invoke browser's search bar. There is some effort put into having all relevant terms on the landing page to make such a search useful.
* Top bar contains breadcrumbs which go back to the ancestor pages.

