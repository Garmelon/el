# el

`el` is a no-dependencies Rust library for writing, modifying, and safely
rendering HTML elements as simple data structures. It is inspired by [hiccup]
and named after a small helper function I once wrote in JS.

[hiccup]: https://github.com/weavejester/hiccup

## Show me a simple example

```rs
use el::{Render, html::*};

let page: String = html((
    head((
        meta((
            attr::name("viewport"),
            attr::content("width=device-width, initial-scale=1"),
        )),
        title("Example page"),
    )),
    body((
        h1((attr::id("heading"), "Example page")),
        p(("This is an example for a ", em("simple"), " web page.")),
    )),
))
.into_document()
.render_to_string()
.unwrap();
```

## What now?

See the top-level crate documentation for more info.

## But what about that small helper function?

Here it is in full, for posterity:

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
    el("meta", {
      name: "viewport",
      content: "width=device-width, initial-scale=1",
    }),
    el("title", {}, "Example page"),
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
