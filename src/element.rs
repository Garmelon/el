use std::collections::BTreeMap;

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
        if kind == ElementKind::Foreign {
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

    pub fn attr(mut self, name: impl ToString, value: impl ToString) -> Self {
        self.attributes
            .insert(name.to_string().to_ascii_lowercase(), value.to_string());
        self
    }

    pub fn attr_true(self, name: impl ToString) -> Self {
        self.attr(name, "")
    }

    pub fn data(self, name: impl ToString, value: impl ToString) -> Self {
        self.attr(format!("data-{}", name.to_string()), value)
    }

    pub fn child(mut self, child: impl Into<Content>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn children(mut self, children: impl AddChildren) -> Self {
        children.add_children(&mut self.children);
        self
    }
}

pub trait AddChildren {
    fn add_children(self, children: &mut Vec<Content>);
}

impl<T: Into<Content>> AddChildren for T {
    fn add_children(self, children: &mut Vec<Content>) {
        children.push(self.into());
    }
}

impl AddChildren for Vec<Content> {
    fn add_children(self, children: &mut Vec<Content>) {
        children.extend(self);
    }
}

impl<const L: usize> AddChildren for [Content; L] {
    fn add_children(self, children: &mut Vec<Content>) {
        children.extend(self);
    }
}

macro_rules! add_children_tuple {
    ( $( $t:ident ),* ) => {
        impl <$( $t: AddChildren ),*> AddChildren for ($( $t ),*) {
            fn add_children(self, children: &mut Vec<Content>) {
                #[allow(non_snake_case)]
                let ($( $t ),*) = self;
                $( $t.add_children(children); )*
            }
        }
    };
}

add_children_tuple!(C1, C2);
add_children_tuple!(C1, C2, C3);
add_children_tuple!(C1, C2, C3, C4);
add_children_tuple!(C1, C2, C3, C4, C5);
add_children_tuple!(C1, C2, C3, C4, C5, C6);
add_children_tuple!(C1, C2, C3, C4, C5, C6, C7);
add_children_tuple!(C1, C2, C3, C4, C5, C6, C7, C8);
add_children_tuple!(C1, C2, C3, C4, C5, C6, C7, C8, C9);

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
