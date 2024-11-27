//! Create HTML by manipulating elements as structured data. Inspired by the
//! clojure library [hiccup][hiccup].
//!
//! [hiccup]: https://github.com/weavejester/hiccup

#[cfg(feature = "axum")]
mod axum;
mod check;
mod element;
pub mod html;
pub mod mathml;
mod render;
pub mod svg;

pub use self::{element::*, render::*};

#[cfg(test)]
mod tests {
    use crate::{html::*, Attr, Content, Element, Render};

    #[test]
    fn simple_website() {
        let els = [
            Content::doctype(),
            html((
                head(title("Hello")),
                body((h1("Hello"), p(("Hello ", em("world"), "!")))),
            ))
            .into(),
        ];

        assert_eq!(
            els.render_to_string().unwrap(),
            concat!(
                "<!DOCTYPE html><html>",
                "<head><title>Hello</title></head>",
                "<body><h1>Hello</h1><p>Hello <em>world</em>!</p></body>",
                "</html>",
            ),
        );
    }

    #[test]
    fn void_elements() {
        // Difference between void and non-void
        assert_eq!(head(()).render_to_string().unwrap(), "<head></head>");
        assert_eq!(input(()).render_to_string().unwrap(), "<input>");

        // Void elements must not contain any children
        assert!(input(p(())).render_to_string().is_err());
    }

    #[test]
    fn raw_text_elements() {
        assert_eq!(
            script("foo <script> & </style> bar")
                .render_to_string()
                .unwrap(),
            "<script>foo <script> & </style> bar</script>",
        );

        println!("{:?}", script("hello </script> world").render_to_string(),);

        assert!(script("hello </script> world").render_to_string().is_err());

        assert!(script("hello </ScRiPt ... world")
            .render_to_string()
            .is_err());
    }

    #[test]
    fn escaped_text_elements() {
        assert_eq!(
            textarea("foo <p> & bar").render_to_string().unwrap(),
            "<textarea>foo &lt;p&gt; &amp; bar</textarea>",
        );

        assert!(textarea(p(())).render_to_string().is_err());
    }

    #[test]
    fn attributes() {
        assert_eq!(
            input((
                Attr::new("name", "tentacles"),
                Attr::new("type", "number"),
                Attr::new("min", 10),
                Attr::new("max", 100),
            ))
            .render_to_string()
            .unwrap(),
            r#"<input max="100" min="10" name="tentacles" type="number">"#,
        );

        assert_eq!(
            input((Attr::new("name", "horns"), Attr::yes("checked")))
                .render_to_string()
                .unwrap(),
            r#"<input checked name="horns">"#,
        );
    }

    #[test]
    fn always_lowercase() {
        assert_eq!(
            Element::normal("HTML")
                .with(Attr::new("LANG", "EN"))
                .render_to_string()
                .unwrap(),
            r#"<html lang="EN"></html>"#,
        );
    }
}
