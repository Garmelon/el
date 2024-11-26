//! Create HTML by manipulating elements as structured data. Inspired by the
//! clojure library [hiccup][hiccup].
//!
//! [hiccup]: https://github.com/weavejester/hiccup

mod check;
mod element;
pub mod elements;
mod render;

pub use self::{element::*, elements::*, render::*};

#[cfg(test)]
mod tests {
    use crate::{elements::*, render::Render, Content, Element};

    #[test]
    fn simple_website() {
        let els = [
            Content::doctype(),
            html()
                .child(head().child(title().child("Hello")))
                .child(
                    body()
                        .child(h1().child("Hello"))
                        .child(p().child("Hello ").child(em().child("world")).child("!")),
                )
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
        assert_eq!(head().render_to_string().unwrap(), "<head></head>");
        assert_eq!(input().render_to_string().unwrap(), "<input>");

        // Void elements must not contain any children
        assert!(input().child(p()).render_to_string().is_err());
    }

    #[test]
    fn raw_text_elements() {
        assert_eq!(
            script()
                .child("foo <script> & </style> bar")
                .render_to_string()
                .unwrap(),
            "<script>foo <script> & </style> bar</script>",
        );

        println!(
            "{:?}",
            script().child("hello </script> world").render_to_string(),
        );

        assert!(script()
            .child("hello </script> world")
            .render_to_string()
            .is_err());

        assert!(script()
            .child("hello </ScRiPt ... world")
            .render_to_string()
            .is_err());
    }

    #[test]
    fn escaped_text_elements() {
        assert_eq!(
            textarea()
                .child("foo <p> & bar")
                .render_to_string()
                .unwrap(),
            "<textarea>foo &lt;p&gt; &amp; bar</textarea>",
        );

        assert!(textarea().child(p()).render_to_string().is_err());
    }

    #[test]
    fn attributes() {
        assert_eq!(
            input()
                .attr("name", "tentacles")
                .attr("type", "number")
                .attr("min", 10)
                .attr("max", 100)
                .render_to_string()
                .unwrap(),
            r#"<input max="100" min="10" name="tentacles" type="number">"#,
        );

        assert_eq!(
            input()
                .attr("name", "horns")
                .attr_true("checked")
                .render_to_string()
                .unwrap(),
            r#"<input checked name="horns">"#,
        );
    }

    #[test]
    fn always_lowercase() {
        assert_eq!(
            Element::normal("HTML")
                .attr("LANG", "EN")
                .render_to_string()
                .unwrap(),
            r#"<html lang="EN"></html>"#,
        );
    }
}
