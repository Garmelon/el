//! Definitions for all non-deprecated MathML elements.
//!
//! <https://developer.mozilla.org/en-US/docs/Web/MathML/Element>

use crate::{Element, ElementComponent, ElementKind};

macro_rules! element {
    ( $name:ident ) => {
        element!($name, stringify!($name));
    };
    ( $name:ident, $tag:expr ) => {
        pub fn $name(c: impl ElementComponent) -> Element {
            Element::new($tag, ElementKind::Foreign).with(c)
        }
    };
}

// MathML elements A to Z

// Deprecated and non-standard elements intentionally omitted.

element!(annotation);
element!(annotation_xml, "annotation-xml");
element!(math);
element!(merror);
element!(mfrac);
element!(mi);
element!(mmultiscripts);
element!(mn);
element!(mo);
element!(mover);
element!(mpadded);
element!(mphantom);
element!(mprescripts);
element!(mroot);
element!(mrow);
element!(ms);
element!(mspace);
element!(msqrt);
element!(mstyle);
element!(msub);
element!(msubsup);
element!(msup);
element!(mtable);
element!(mtd);
element!(mtext);
element!(mtr);
element!(munder);
element!(munderover);
element!(semantics);
