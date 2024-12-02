# el

`el` is a Rust library for writing, modifying, and safely rendering HTML
elements as simple data structures. It is inspired by [hiccup] and named after a
small helper function I once wrote in JS.

[hiccup]: https://github.com/weavejester/hiccup

## Usage example

```rs
use el::{Attr, Render, html::*};

let page: String = html((
    head((
        meta(Attr::new("charset", "utf-8")),
        meta((
            Attr::new("name", "viewport"),
            Attr::new("content", "width=device-width, initial-scale=1"),
        )),
        title("Example page"),
    )),
    body((
        h1((Attr::id("heading"), "Example page")),
        p(("This is an example for a ", em("simple"), " web page.")),
    )),
))
.render_to_string()
.unwrap();
```

## License

This entire project is dual-licensed under the [Apache 2.0] and [MIT] licenses.

[Apache 2.0]: LICENSE-APACHE
[MIT]: LICENSE-MIT
