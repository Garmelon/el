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

## But what about that small helper function?

Here it is in full, for posteriority:

```js
function el(name, attributes, ...children) {
  const element = document.createElement(name);
  for (const [name, value] of Object.entries(attributes))
    element.setAttribute(name, value);
  element.append(...children);
  return element;
}
```

Use it like so:

```js
const page = el("html", {},
  el("head", {},
    el("meta", { charset: "utf-8" }),
    el("meta", {
        name: "viewport",
        content: "width=device-width, initial-scale=1",
    }),
    el("title", {}, "Example page")
  ),
  el("body", {},
    el("h1", { id: "heading" }, "Example page"),
    el("p", {}, "This is an example for a ", el("em", {}, "simple"), " web page."),
  ),
);
```

## License

This entire project is dual-licensed under the [Apache 2.0] and [MIT] licenses.

[Apache 2.0]: LICENSE-APACHE
[MIT]: LICENSE-MIT
