use crate::output::color::Color;


pub fn dot_html(filename: String) -> String {
    format!(
        "<p><div id=\"{}\" class=\"svg-diagram\"></div></p>\
        <script>\
        Viz.instance().then(function(viz) {{\
            fetch('{}')\
                .then(response => response.text())\
                .then((data) => {{\
                    var svg = viz.renderSVGElement(data);\
                    svg.setAttribute(\"width\", \"100%\");\
                    svg.setAttribute(\"height\", \"300pt\");\
                    document.getElementById(\"{}\").appendChild(svg);\
                }})\
        }});\
        </script>\n\n",
        filename, filename, filename
    )
}

pub fn zoom_dot_html(filename: String) -> String {
    format!(
        "<p><div id=\"{}\" class=\"svg-diagram zoomable\"></div></p>\
            <script type=\"module\">\
            import {{ initializeSvgToolbelt }} from '{}';\
            Viz.instance().then(function(viz) {{\
            fetch('{}')\
            .then(response => response.text())\
            .then((data) => {{\
            var svg = viz.renderSVGElement(data);\
            document.getElementById(\"{}\").appendChild(svg);\
            initializeSvgToolbelt('.zoomable', {{\
            zoomStep: 0.3,\
            minScale: 1,\
            maxScale: 5,\
            }});\
            }})\
            }});\
            </script>\n\n",
        filename, "/parameters/svg-toolbelt.esm.js", filename, filename
    )
}

pub fn pdf_html(name: String, height: u32) -> String {
    format!(
        "\n<object data=\"{}\" type=\"application/pdf\" class=\"pdf-table-wrapper\" height=\"{}px\">\
            <embed src=\"{}\">\
            <p>This browser does not support PDFs. Please download the PDF to view it: <a href=\"{}\">Download PDF</a>.</p>\
            </embed>\
            </object>\n\n",
        name, height, name, name
    )
}

pub fn colorbox_html(color: Color) -> String {
    format!("<span style=\"color:{}\">■</span>", color.hex())
}

pub fn link_html(content: &str, address: String) -> String {
    format!("<a href=\"{}\">{}</a>", address, content)
}
