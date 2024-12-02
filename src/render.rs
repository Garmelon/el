use std::{error, fmt};

use crate::{
    check,
    element::{Content, Element, ElementKind},
    Document,
};

#[derive(Debug)]
pub enum ErrorCause {
    Format(fmt::Error),
    InvalidTagName(String),
    InvalidAttrName(String),
    InvalidChild,
    InvalidRawText(String),
}

#[derive(Debug)]
pub struct Error {
    reverse_path: Vec<String>,
    cause: ErrorCause,
}

impl Error {
    pub fn new(cause: ErrorCause) -> Self {
        Self {
            reverse_path: vec![],
            cause,
        }
    }

    pub fn at(mut self, index: usize, child: &Content) -> Self {
        self.reverse_path.push(match child {
            Content::Element(el) => format!("{index}[{}]", el.name),
            _ => index.to_string(),
        });
        self
    }

    /// A human-readable path from the topmost element to the element that
    /// caused the error.
    ///
    /// The path consists of elements of the form `index(tagname)` or `index`,
    /// depending on whether the [`Content`] at that position is a
    /// [`Content::Element`] or not.
    ///
    /// # Example
    ///
    /// ```
    /// use el::{Render, html::*};
    /// let result = form(("greeting: ", input("hello"))).render_to_string();
    /// assert!(result.is_err()); // <input> is a void element
    /// assert_eq!(result.unwrap_err().path(), "/1(input)/0");
    /// ```
    pub fn path(&self) -> String {
        if self.reverse_path.is_empty() {
            return "/".to_string();
        }

        self.reverse_path
            .iter()
            .rev()
            .map(|(index, name)| match name {
                Some(name) => format!("/{index}({name})"),
                None => format!("/{index}"),
            })
            .collect::<String>()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Render error at {}: ", self.path())?;

        match &self.cause {
            ErrorCause::Format(error) => write!(f, "{error}")?,
            ErrorCause::InvalidTagName(name) => write!(f, "Invalid tag name {name:?}")?,
            ErrorCause::InvalidAttrName(name) => write!(f, "Invalid attribute name {name:?}")?,
            ErrorCause::InvalidChild => write!(f, "Invalid child")?,
            ErrorCause::InvalidRawText(text) => write!(f, "Invalid raw text {text:?}")?,
        }

        Ok(())
    }
}

impl error::Error for Error {}

impl From<fmt::Error> for Error {
    fn from(value: fmt::Error) -> Self {
        Self::new(ErrorCause::Format(value))
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait Render {
    fn render<W: fmt::Write>(&self, w: &mut W) -> Result<()>;

    fn render_to_string(&self) -> Result<String> {
        let mut result = String::new();
        self.render(&mut result)?;
        Ok(result)
    }
}

impl Render for Document {
    fn render<W: fmt::Write>(&self, w: &mut W) -> Result<()> {
        Content::doctype().render(w)?;
        self.0.render(w)?;
        Ok(())
    }
}

impl Render for [Content] {
    fn render<W: fmt::Write>(&self, w: &mut W) -> Result<()> {
        for content in self {
            content.render(w)?;
        }
        Ok(())
    }
}

impl Render for Content {
    fn render<W: fmt::Write>(&self, w: &mut W) -> Result<()> {
        match self {
            Self::Raw(text) => write!(w, "{text}")?,
            Self::Text(text) => render_text(w, text)?,
            Self::Comment(text) => render_comment(w, text)?,
            Self::Element(element) => element.render(w)?,
        }
        Ok(())
    }
}

impl Render for Element {
    fn render<W: fmt::Write>(&self, w: &mut W) -> Result<()> {
        // Checks
        if !check::is_valid_tag_name(&self.name) {
            return Err(Error::new(ErrorCause::InvalidTagName(self.name.clone())));
        }
        for name in self.attributes.keys() {
            if !check::is_valid_attribute_name(name) {
                return Err(Error::new(ErrorCause::InvalidAttrName(name.clone())));
            }
        }

        // Opening tag
        write!(w, "<{}", self.name)?;
        for (name, value) in &self.attributes {
            write!(w, " {name}")?;
            if !value.is_empty() {
                write!(w, "=")?;
                render_attribute_value(w, value)?;
            }
        }
        if self.children.is_empty() {
            // Closing early
            match self.kind {
                ElementKind::Void => write!(w, ">")?,
                ElementKind::Foreign => write!(w, " />")?,
                _ => write!(w, "></{}>", self.name)?,
            }
            return Ok(());
        }
        write!(w, ">")?;

        // Children
        for (i, child) in self.children.iter().enumerate() {
            match self.kind {
                ElementKind::Void => Err(Error::new(ErrorCause::InvalidChild)),
                ElementKind::RawText => match child {
                    c @ Content::Raw(_) => c.render(w),
                    Content::Text(text) if check::is_valid_raw_text(&self.name, text) => {
                        write!(w, "{text}").map_err(|e| e.into())
                    }
                    Content::Text(text) => {
                        Err(Error::new(ErrorCause::InvalidRawText(text.clone())))
                    }
                    _ => Err(Error::new(ErrorCause::InvalidChild)),
                },
                ElementKind::EscapableRawText => match child {
                    c @ (Content::Raw(_) | Content::Text(_)) => c.render(w),
                    _ => Err(Error::new(ErrorCause::InvalidChild)),
                },
                _ => child.render(w),
            }
            .map_err(|e| e.at(i, child))?;
        }

        // Closing tag
        if self.kind != ElementKind::Void {
            write!(w, "</{}>", self.name)?;
        }

        Ok(())
    }
}

fn render_text<W: fmt::Write>(w: &mut W, text: &str) -> Result<()> {
    // As far as I can tell, it should be sufficient to escape `&` and `<`.
    // `>` is escaped too for symmetry, not for any real reason.
    //
    // Reasoning: Whenever we're inside tags, we're in one of these states,
    // https://html.spec.whatwg.org/multipage/parsing.html#data-state
    // https://html.spec.whatwg.org/multipage/parsing.html#rawtext-state
    // https://html.spec.whatwg.org/multipage/parsing.html#rcdata-state

    for c in text.chars() {
        match c {
            '&' => write!(w, "&amp;")?,
            '<' => write!(w, "&lt;")?,
            '>' => write!(w, "&gt;")?,
            c => write!(w, "{c}")?,
        }
    }

    Ok(())
}

fn render_comment<W: fmt::Write>(w: &mut W, text: &str) -> Result<()> {
    // A comment...
    // - must not start with the string ">"
    // - must not start with the string "->"
    // - must not contain the strings "<!--", "-->", or "--!>"
    // - must not end with the string "<!-"
    //
    // https://html.spec.whatwg.org/multipage/syntax.html#comments

    let text = text
        .replace("<!--", "<!==")
        .replace("-->", "==>")
        .replace("--!>", "==!>");

    if text.starts_with(">") || text.starts_with("->") {
        write!(w, " ")?;
    }

    write!(w, "{text}")?;

    if text.ends_with("<!-") {
        write!(w, " ")?;
    }

    Ok(())
}

fn render_attribute_value<W: fmt::Write>(w: &mut W, text: &str) -> Result<()> {
    // Quoted attribute values are escaped like text, but the set of characters
    // to escape is different.
    //
    // https://html.spec.whatwg.org/multipage/syntax.html#attributes-2

    write!(w, "\"")?;

    for c in text.chars() {
        match c {
            '"' => write!(w, "&quot;")?,
            c => write!(w, "{c}")?,
        }
    }

    write!(w, "\"")?;

    Ok(())
}
