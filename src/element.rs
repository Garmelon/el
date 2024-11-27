use std::collections::{BTreeMap, HashMap};

/// <https://html.spec.whatwg.org/multipage/syntax.html#elements-2>
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ElementKind {
    Void,
    Template,
    RawText,
    EscapableRawText,
    Foreign,
    Normal,
}

#[derive(Clone)]
pub enum Content {
    Raw(String),
    Text(String),
    Comment(String),
    Element(Element),
}

impl Content {
    pub fn raw(str: impl ToString) -> Self {
        Self::Raw(str.to_string())
    }

    pub fn text(str: impl ToString) -> Self {
        Self::Text(str.to_string())
    }

    pub fn comment(str: impl ToString) -> Self {
        Self::Comment(str.to_string())
    }

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

#[derive(Clone)]
pub struct Element {
    pub name: String,
    pub kind: ElementKind,
    pub attributes: BTreeMap<String, String>,
    pub children: Vec<Content>,
}

impl Element {
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

    pub fn normal(name: impl ToString) -> Self {
        Self::new(name, ElementKind::Normal)
    }

    pub fn add(&mut self, component: impl ElementComponent) {
        component.add_to_element(self);
    }

    pub fn with(mut self, component: impl ElementComponent) -> Self {
        self.add(component);
        self
    }
}

pub trait ElementComponent {
    fn add_to_element(self, element: &mut Element);
}

// Attributes

pub struct Attr {
    name: String,
    value: String,
}

impl Attr {
    pub fn new(name: impl ToString, value: impl ToString) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_string(),
        }
    }

    pub fn yes(name: impl ToString) -> Self {
        Self::new(name, "")
    }

    pub fn id(id: impl ToString) -> Self {
        Self::new("id", id)
    }

    pub fn class(class: impl ToString) -> Self {
        Self::new("class", class)
    }

    pub fn data(name: impl ToString, value: impl ToString) -> Self {
        Self::new(format!("data-{}", name.to_string()), value)
    }
}

impl ElementComponent for Attr {
    fn add_to_element(mut self, element: &mut Element) {
        if element.kind != ElementKind::Foreign {
            self.name = self.name.to_ascii_lowercase();
        }
        element.attributes.insert(self.name, self.value);
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

/// An HTML document.
///
/// A `Document(el)` is basically the same as `[Content::doctype(), el.into()]`
/// for the purposes of the [`crate::Render`] trait.
#[derive(Clone)]
pub struct Document(pub Element);

impl From<Element> for Document {
    fn from(value: Element) -> Self {
        Self(value)
    }
}
