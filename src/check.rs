/// <https://infra.spec.whatwg.org/#ascii-alpha>
pub fn is_ascii_alpha(c: char) -> bool {
    c.is_ascii_alphabetic()
}

/// <https://infra.spec.whatwg.org/#ascii-alphanumeric>
pub fn is_ascii_alphanumeric(c: char) -> bool {
    c.is_ascii_alphanumeric()
}

/// <https://html.spec.whatwg.org/multipage/syntax.html#syntax-tag-name>
///
/// The rules around what is a valid tag name are complicated. The standard
/// doesn't give an easy answer. Because of this, we're conservative in what we
/// allow. This way, the output we produce should parse correctly in a wide
/// range of circumstances while following the standard.
pub fn is_valid_tag_name(name: &str) -> bool {
    !name.is_empty()
        && name.chars().take(1).all(is_ascii_alpha)
        && name.chars().all(is_ascii_alphanumeric)
}

/// <https://html.spec.whatwg.org/multipage/syntax.html#syntax-attribute-name>
///
/// The rules around what is a valid attribute name are complicated. The
/// standard doesn't give an easy answer. Because of this, we're conservative in
/// what we allow. This way, the output we produce should parse correctly in a
/// wide range of circumstances while following the standard.
pub fn is_valid_attribute_name(name: &str) -> bool {
    !name.is_empty()
        && name.chars().take(1).all(is_ascii_alpha)
        && name
            .chars()
            .all(|c| is_ascii_alphanumeric(c) || c == '-' || c == '_')
}

/// https://html.spec.whatwg.org/multipage/syntax.html#cdata-rcdata-restrictions
///
/// The tag name must be ascii-only.
pub fn is_valid_raw_text(tag_name: &str, text: &str) -> bool {
    // In case we ever decide to relax tag name ascii requirements.
    assert!(tag_name.is_ascii());

    // "The text in raw text and escapable raw text elements must not contain
    // any occurrences of the string "</" (U+003C LESS-THAN SIGN, U+002F
    // SOLIDUS) [...]"
    for (i, _) in text.match_indices("</") {
        let start = i + "</".len();

        let potential_tag_name = text[start..]
            .chars()
            .take(tag_name.chars().count())
            .collect::<String>();

        // "[...] followed by characters that case-insensitively match the tag
        // name of the element [...]"
        //
        // Note: Since we know that tag names are ascii-only, we can convert
        // both to lowercase for a case-insensitive comparison without weird
        // unicode shenanigans.
        if potential_tag_name.to_ascii_lowercase() != tag_name.to_ascii_lowercase() {
            continue;
        }

        // "[...] followed by [...]"
        let Some(trailing) = text[start + potential_tag_name.len()..].chars().next() else {
            continue;
        };

        // "[...] one of U+0009 CHARACTER TABULATION (tab), U+000A LINE FEED
        // (LF), U+000C FORM FEED (FF), U+000D CARRIAGE RETURN (CR), U+0020
        // SPACE, U+003E GREATER-THAN SIGN (>), or U+002F SOLIDUS (/)."
        if matches!(trailing, '\t' | '\n' | '\x0C' | '\r' | ' ' | '>' | '/') {
            return false;
        }
    }
    true
}
