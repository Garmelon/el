//! # el
//!
//! Write, modify, and safely render HTML elements as simple data structures.
//!
//! This library is inspired by [hiccup] and named after a small helper function
//! I once wrote in JS.
//!
//! [hiccup]: https://github.com/weavejester/hiccup
//!
//! ## Library overview
//!
//! The basic data structure is the [`Element`], which can be rendered to a
//! [`String`] using the [`Render`] trait. Custom elements can be constructed
//! using [`Element::normal`] or [`Element::new`]. Once constructed, elements
//! can be modified by accessing their fields or using the [`Element::add`] or
//! [`Element::with`] methods, though this is usually not necessary.
//!
//! Constructor functions for all (non-deprecated) HTML tags can be found in the
//! [`html`] module, SVG tags in [`svg`] and MathML tags in [`mathml`]. These
//! three modules are designed to be wildcard-included for more concise code,
//! either on a per-function or per-file basis.
//!
//! Element construction uses the [`ElementComponent`] trait, which represents
//! not only element contents but also attributes. Tuples, arrays, [`Vec`],
//! [`Option`], and [`Result`] can be used to combine components. The order of
//! content components is preserved. To set attributes, include [`Attr`] values
//! as components.
//!
//! If you want to render an entire web page, wrap an [`html::html`] element in
//! a [`Document`]. When rendered, documents include the `<!DOCTYPE html>`
//! annotation required by the standard.
//!
//! ## Usage example
//!
//! ```
//! use el::{Attr, Render, html::*};
//!
//! let page: String = html((
//!     head((
//!         meta(Attr::new("charset", "utf-8")),
//!         meta((
//!             Attr::new("name", "viewport"),
//!             Attr::new("content", "width=device-width, initial-scale=1"),
//!         )),
//!         title("Example page"),
//!     )),
//!     body((
//!         h1((Attr::id("heading"), "Example page")),
//!         p(("This is an example for a ", em("simple"), " web page.")),
//!     )),
//! ))
//! .into_document()
//! .render_to_string()
//! .unwrap();
//! ```
//!
//! ## Axum support
//!
//! The [axum] crate is supported via the optional `axum` feature flag. When it
//! is enabled, [`Document`] implements axum's `IntoResponse` trait and can be
//! returned directly from handlers. In order to prevent accidentally returning
//! incomplete HTML documents, [`Element`] does not implement `IntoResponse`.
//!
//! ```toml
//! [dependencies]
//! el = { version = "...", features = ["axum"] }
//! ```
//!
//! [axum]: https://crates.io/crates/axum
//!
//! ## But what about that small helper function?
//!
//! See the readme for more details.

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
    use crate::{html::*, Attr, Element, Render};

    #[test]
    fn simple_website() {
        let page = html((
            head(title("Hello")),
            body((h1("Hello"), p(("Hello ", em("world"), "!")))),
        ))
        .into_document()
        .render_to_string()
        .unwrap();

        assert_eq!(
            page,
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
