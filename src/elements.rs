//! Definitions for all non-deprecated HTML elements.
//!
//! <https://developer.mozilla.org/en-US/docs/Web/HTML/Element>

use crate::{Element, ElementKind};

macro_rules! element {
    ( $name:ident ) => {
        element!($name, ElementKind::Normal);
    };
    ( $name:ident, $kind:expr ) => {
        pub fn $name() -> Element {
            Element::new(stringify!($name), $kind)
        }
    };
}

// Main root
element!(html);

// Document metadata
element!(base, ElementKind::Void);
element!(head);
element!(link, ElementKind::Void);
element!(meta, ElementKind::Void);
element!(style, ElementKind::RawText);
element!(title, ElementKind::EscapableRawText);

// Sectioning root
element!(body);

// Content sectioning
element!(address);
element!(article);
element!(aside);
element!(footer);
element!(header);
element!(h1);
element!(h2);
element!(h3);
element!(h4);
element!(h5);
element!(h6);
element!(hgroup);
element!(main);
element!(nav);
element!(section);
element!(search);

// Text content
element!(blockquote);
element!(dd);
element!(div);
element!(dl);
element!(dt);
element!(figcaption);
element!(figure);
element!(hr, ElementKind::Void);
element!(li);
element!(menu);
element!(ol);
element!(p);
element!(pre);
element!(ul);

// Inline text semantics
element!(a);
element!(abbr);
element!(b);
element!(bdi);
element!(bdo);
element!(br, ElementKind::Void);
element!(cite);
element!(code);
element!(data);
element!(dfn);
element!(em);
element!(i);
element!(kbd);
element!(mark);
element!(q);
element!(rp);
element!(rt);
element!(ruby);
element!(s);
element!(samp);
element!(small);
element!(span);
element!(strong);
element!(sub);
element!(sup);
element!(time);
element!(u);
element!(var);
element!(wbr, ElementKind::Void);

// Image and multimedia
element!(area, ElementKind::Void);
element!(audio);
element!(img, ElementKind::Void);
element!(map);
element!(track, ElementKind::Void);
element!(video);

// Embedded content
element!(embed, ElementKind::Void);
element!(fencedframe);
element!(iframe);
element!(object);
element!(picture);
element!(portal);
element!(source, ElementKind::Void);

// SVG and MathML
// TODO Proper SVG and MathML support
element!(svg, ElementKind::Foreign);
element!(math, ElementKind::Foreign);

// Scripting
element!(canvas);
element!(noscript);
element!(script, ElementKind::RawText);

// Demarcating edits
element!(del);
element!(ins);

// Table content
element!(caption);
element!(col, ElementKind::Void);
element!(colgroup);
element!(table);
element!(tbody);
element!(td);
element!(tfoot);
element!(th);
element!(thead);
element!(tr);

// Forms
element!(button);
element!(datalist);
element!(fieldset);
element!(form);
element!(input, ElementKind::Void);
element!(label);
element!(legend);
element!(meter);
element!(optgroup);
element!(option);
element!(output);
element!(progress);
element!(select);
element!(textarea, ElementKind::EscapableRawText);

// Interactive elements
element!(details);
element!(dialog);
element!(summary);

// Web Components
element!(slot);
element!(template, ElementKind::Template);

// Obsolete and deprecated elements
// Intentionally excluded!
