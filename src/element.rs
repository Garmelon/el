use std::collections::{btree_map::Entry, BTreeMap, HashMap};

/// The kind of an element.
///
/// Follows the [definitions from the HTML standard][spec].
///
/// [spec]: https://html.spec.whatwg.org/multipage/syntax.html#elements-2
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementKind {
    Void,
    Template,
    RawText,
    EscapableRawText,
    Foreign,
    Normal,
}

/// A single bit of [`Element`] content.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Content {
    /// A raw string to be rendered without any checks.
    ///
    /// Can also be constructed using [`Self::raw`].
    ///
    /// # Warning
    ///
    /// This is an escape hatch for including arbitrary text. Using it
    /// incorrectly may result in security vulnerabilities in the rendered HTML.
    Raw(String),
    /// Plain text.
    ///
    /// Can also be constructed using [`Self::text`].
    Text(String),
    /// An HTML comment (`<!-- ... -->`).
    ///
    /// Can also be constructed using [`Self::comment`].
    Comment(String),
    /// A child [`Element`].
    ///
    /// Can also be constructed using [`Self::element`].
    Element(Element),
}

impl Content {
    /// Construct [`Content::Raw`], a raw string to be rendered without any
    /// checks.
    ///
    /// # Warning
    ///
    /// This is an escape hatch for including arbitrary text. Using it
    /// incorrectly may result in security vulnerabilities in the rendered HTML.
    pub fn raw(str: impl ToString) -> Self {
        Self::Raw(str.to_string())
    }

    /// Construct [`Content::Text`], plain text.
    pub fn text(str: impl ToString) -> Self {
        Self::Text(str.to_string())
    }

    /// Construct [`Content::Comment`], an HTML comment (`<!-- ... -->`).
    pub fn comment(str: impl ToString) -> Self {
        Self::Comment(str.to_string())
    }

    /// Construct [`Content::Element`], a child [`Element`].
    ///
    /// Instead of calling `Content::element(foo)`, you can also use
    /// `foo.into()`.
    pub fn element(e: impl Into<Element>) -> Self {
        Self::Element(e.into())
    }

    /// Construct a doctype of the form `<!DOCTYPE html>`.
    ///
    /// # Example
    ///
    /// ```
    /// use el::Content;
    /// let doctype = Content::doctype();
    /// assert_eq!(doctype, Content::raw("<!DOCTYPE html>"));
    /// ```
    pub fn doctype() -> Self {
        Self::raw("<!DOCTYPE html>")
    }
}

impl From<String> for Content {
    fn from(value: String) -> Self {
        Self::text(value)
    }
}

impl From<&str> for Content {
    fn from(value: &str) -> Self {
        Self::text(value)
    }
}

impl From<Element> for Content {
    fn from(value: Element) -> Self {
        Self::Element(value)
    }
}

/// An HTML element.
///
/// SVG and MathML elements are also modelled using this type.
///
/// Errors (e.g. illegal characters or an element of [`ElementKind::Void`]
/// having children) are deferred until rendering and are not checked during
/// element construction. See also [`crate::Render`] and [`crate::Error`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Element {
    /// The tag name of the element.
    pub name: String,
    /// What kind of element this is.
    ///
    /// # Warning
    ///
    /// The element kind affects the correctness of the rendered output.
    /// Choosing an incorrect kind may result in security vulnerabilities in the
    /// rendered HTML. See [`ElementKind`] for more details.
    pub kind: ElementKind,
    /// The attributes (e.g. `id` or `class`) of the element.
    ///
    /// This map does not take into account case insensitivity of attributes.
    /// Any attributes contained in the map will appear in the rendered output.
    pub attributes: BTreeMap<String, String>,
    /// The children of the element.
    pub children: Vec<Content>,
}

impl Element {
    /// Create a new element of a specific [`ElementKind`].
    ///
    /// See also [`Self::normal`] to create elements of kind
    /// [`ElementKind::Normal`].
    ///
    /// # Warning
    ///
    /// The element kind affects the correctness of the rendered output.
    /// Choosing an incorrect kind may result in security vulnerabilities in the
    /// rendered HTML. See [`ElementKind`] for more details.
    ///
    /// # Example
    ///
    /// ```
    /// use el::{Element, ElementKind, html, svg};
    ///
    /// let p = Element::new("p", ElementKind::Normal);
    /// let link = Element::new("link", ElementKind::Void);
    /// let script = Element::new("script", ElementKind::RawText);
    /// let svg = Element::new("svg", ElementKind::Foreign);
    ///
    /// assert_eq!(p, html::p(()));
    /// assert_eq!(link, html::link(()));
    /// assert_eq!(script, html::script(()));
    /// assert_eq!(svg, svg::svg(()));
    /// ```
    pub fn new(name: impl ToString, kind: ElementKind) -> Self {
        let mut name = name.to_string();
        if kind != ElementKind::Foreign {
            name = name.to_ascii_lowercase()
        }

        Self {
            name,
            kind,
            attributes: BTreeMap::new(),
            children: vec![],
        }
    }

    /// Create a new element of the kind [`ElementKind::Normal`].
    ///
    /// `Element::normal(foo)` is equivalent to calling `Element::new(foo,
    /// ElementKind::Normal)`.
    ///
    /// # Warning
    ///
    /// The element kind affects the correctness of the rendered output.
    /// Choosing an incorrect kind may result in security vulnerabilities in the
    /// rendered HTML. See [`ElementKind`] for more details.
    ///
    /// # Example
    ///
    /// ```
    /// use el::{Element, ElementKind};
    /// let element = Element::normal("custom");
    /// assert_eq!(element.kind, ElementKind::Normal);
    /// ```
    pub fn normal(name: impl ToString) -> Self {
        Self::new(name, ElementKind::Normal)
    }

    /// Add components to the element in-place.
    ///
    /// To add multiple components, either call this function repeatedly or use
    /// a type like tuples, arrays, [`Vec`], [`Option`], [`Result`] to combine
    /// multiple components. See [`ElementComponent`] for more info.
    ///
    /// # Example
    ///
    /// ```
    /// use el::{Attr, html::*};
    /// let mut element = p(());
    ///
    /// // Adding single components
    /// element.add("some text");
    /// element.add(Attr::class("foo"));
    ///
    /// // Adding multiple components
    /// element.add((Attr::id("bar"), " ", em("and"), " some more text"));
    /// ```
    pub fn add(&mut self, c: impl ElementComponent) {
        c.add_to_element(self);
    }

    /// A more builder-pattern-like version of [`Self::add`].
    ///
    /// Instead of a mutable reference, this function takes ownership of the
    /// element before returning it again. This can be more ergonomic in some
    /// cases.
    ///
    /// # Example
    ///
    /// ```
    /// use el::{Attr, html::*};
    ///
    /// let element = p(())
    ///     // Adding single components
    ///     .with("some text")
    ///     .with(Attr::class("foo"))
    ///     // Adding multiple components
    ///     .with((Attr::id("bar"), " ", em("and"), " some more text"));
    /// ```
    pub fn with(mut self, c: impl ElementComponent) -> Self {
        self.add(c);
        self
    }
}

/// A component can add itself to an [`Element`] by modifying it.
///
/// A component usually represents either a bit of content or an attribute for
/// the element it is being added to. Some components (e.g. tuples, arrays,
/// [`Vec`], [`Option`], [`Result`]) consist of further components. This creates
/// a flexible API for building [`Element`]s:
///
/// ```
/// use el::{Attr, Render, html::*};
/// let p = p((
///     Attr::id("foo"),
///     Attr::class("bar"),
///     Attr::class("baz"),
///     "Hello ", em("world"), "!",
/// ));
/// assert_eq!(
///     p.render_to_string().unwrap(),
///     r#"<p class="bar baz" id="foo">Hello <em>world</em>!</p>"#,
/// );
/// ```
pub trait ElementComponent {
    /// Add a component to an element, consuming the component in the process.
    fn add_to_element(self, element: &mut Element);
}

/// An element attribute, used during [`Element`] construction.
///
/// # Example
///
/// ```
/// use el::{Attr, html::*};
/// let p = p(Attr::class("foo"));
/// assert_eq!(p.attributes["class"], "foo");
/// ```
pub struct Attr {
    name: String,
    value: String,
    append_by: Option<String>,
}

impl Attr {
    /// Create or replace an attribute.
    ///
    /// When this attribute is added to an [`Element`] through
    /// [`ElementComponent::add_to_element`] and an attribute of the same name
    /// already exists, it replaces that attribute's value.
    pub fn new(name: impl ToString, value: impl ToString) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_string(),
            append_by: None,
        }
    }

    /// Create or append to an attribute.
    ///
    /// When this attribute is added to an [`Element`] through
    /// [`ElementComponent::add_to_element`] and an attribute of the same name
    /// already exists, it appends the separator and then its own value to that
    /// attribute's value.
    pub fn append(name: impl ToString, value: impl ToString, separator: impl ToString) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_string(),
            append_by: Some(separator.to_string()),
        }
    }

    /// Create (or replace) a new empty attribute.
    ///
    /// `Attr::yes(name)` is equivalent to `Attr::new(name, "").`
    ///
    /// When rendering an empty attribute as HTML, the value can be omitted:
    /// `name=""` is equivalent to just `name`.
    pub fn yes(name: impl ToString) -> Self {
        Self::new(name, "")
    }

    /// Create (or replace) an `id` attribute.
    ///
    /// `Attr::id(id)` is equivalent to `Attr::new("id", id)`.
    pub fn id(id: impl ToString) -> Self {
        Self::new("id", id)
    }

    /// Create (or append) to a `class` attribute.
    ///
    /// `Attr::class(class)` is equivalent to
    /// `Attr::append("class", class, " ")`.
    pub fn class(class: impl ToString) -> Self {
        Self::append("class", class, " ")
    }

    /// Create (or append) to a `style` attribute.
    ///
    /// `Attr::style(style)` is equivalent to
    /// `Attr::append("style", style, ";")`.
    pub fn style(style: impl ToString) -> Self {
        Self::append("style", style, ";")
    }

    /// Create (or replace) a new [`data-*` attribute][mdn].
    ///
    /// `Attr::data(name, value)` is equivalent to
    /// `Attr::new(format!("data-{name}"), value)`.
    ///
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/data-*
    pub fn data(name: impl ToString, value: impl ToString) -> Self {
        Self::new(format!("data-{}", name.to_string()), value)
    }
}

impl ElementComponent for Attr {
    fn add_to_element(mut self, element: &mut Element) {
        if element.kind != ElementKind::Foreign {
            self.name = self.name.to_ascii_lowercase();
        }
        match element.attributes.entry(self.name) {
            Entry::Vacant(entry) => {
                entry.insert(self.value);
            }
            Entry::Occupied(mut entry) => match self.append_by {
                None => {
                    entry.insert(self.value);
                }
                Some(sep) => {
                    let value = entry.get_mut();
                    value.push_str(&sep);
                    value.push_str(&self.value);
                }
            },
        }
    }
}

impl ElementComponent for HashMap<String, String> {
    fn add_to_element(self, element: &mut Element) {
        for (name, value) in self {
            Attr::new(name, value).add_to_element(element);
        }
    }
}

impl ElementComponent for BTreeMap<String, String> {
    fn add_to_element(self, element: &mut Element) {
        for (name, value) in self {
            Attr::new(name, value).add_to_element(element);
        }
    }
}

// Children

impl<T: Into<Content>> ElementComponent for T {
    fn add_to_element(self, element: &mut Element) {
        element.children.push(self.into());
    }
}

// Combining components

impl<T: ElementComponent> ElementComponent for Option<T> {
    fn add_to_element(self, element: &mut Element) {
        if let Some(component) = self {
            component.add_to_element(element)
        }
    }
}

impl<T: ElementComponent, E: ElementComponent> ElementComponent for Result<T, E> {
    fn add_to_element(self, element: &mut Element) {
        match self {
            Ok(component) => component.add_to_element(element),
            Err(component) => component.add_to_element(element),
        }
    }
}

impl<T: ElementComponent> ElementComponent for Vec<T> {
    fn add_to_element(self, element: &mut Element) {
        for component in self {
            component.add_to_element(element);
        }
    }
}

impl<const L: usize, T: ElementComponent> ElementComponent for [T; L] {
    fn add_to_element(self, element: &mut Element) {
        for component in self {
            component.add_to_element(element);
        }
    }
}

// Varargs emulation with tuples

impl ElementComponent for () {
    fn add_to_element(self, _element: &mut Element) {}
}

impl<C1: ElementComponent> ElementComponent for (C1,) {
    fn add_to_element(self, element: &mut Element) {
        let (c1,) = self;
        c1.add_to_element(element);
    }
}

macro_rules! element_component_tuple {
    ( $( $t:ident ),* ) => {
        impl <$( $t: ElementComponent ),*> ElementComponent for ($( $t ),*) {
            fn add_to_element(self, element: &mut Element) {
                #[allow(non_snake_case)]
                let ($( $t ),*) = self;
                $( $t.add_to_element(element); )*
            }
        }
    };
}

element_component_tuple!(C1, C2);
element_component_tuple!(C1, C2, C3);
element_component_tuple!(C1, C2, C3, C4);
element_component_tuple!(C1, C2, C3, C4, C5);
element_component_tuple!(C1, C2, C3, C4, C5, C6);
element_component_tuple!(C1, C2, C3, C4, C5, C6, C7);
element_component_tuple!(C1, C2, C3, C4, C5, C6, C7, C8);
element_component_tuple!(C1, C2, C3, C4, C5, C6, C7, C8, C9);

/// A full HTML document including doctype.
///
/// A `Document(el)` is basically the same as `[Content::doctype(), el.into()]`
/// for the purposes of the [`Render`][crate::Render] trait.
#[derive(Debug, Clone)]
pub struct Document(pub Element);

impl From<Element> for Document {
    fn from(value: Element) -> Self {
        Self(value)
    }
}
