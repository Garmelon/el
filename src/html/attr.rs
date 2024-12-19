//! Definitions for common element attributes
//! (see [Attributes][0] and [Global attributes][1] on MDN).
//!
//! Deprecated or redundant attributes are not included.
//!
//! [0]: https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes
//! [1]: https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes

use std::fmt;

use crate::{Attr, Element, ElementComponent};

macro_rules! url {
    ( global, $name:expr ) => {
        concat!(
            "[MDN](https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/",
            $name,
            ")"
        )
    };
    ( normal, $name:expr ) => {
        concat!(
            "[MDN](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/",
            $name,
            ")"
        )
    };
    ( element $element:expr, $name:expr ) => {
        concat!(
            "[`<",
            $element,
            ">`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/",
            $element,
            "#",
            $name,
            ")"
        )
    };
}

macro_rules! attr_yes {
    (
        $name:ident as $article:ident $actual:expr;
        at $url:expr;
    ) => {
        #[doc = concat!("Create (or replace) ", stringify!($article), " `", $actual, "` attribute")]
        #[doc = concat!("(", $url, ").")]
        pub fn $name() -> Attr {
            Attr::yes($actual)
        }
    };
}

macro_rules! attr_set {
    (
        $name:ident as $article:ident $actual:expr;
        at $url:expr;
    ) => {
        #[doc = concat!("Create (or replace) ", stringify!($article), " `", $actual, "` attribute")]
        #[doc = concat!("(", $url, ").")]
        pub fn $name(value: impl ToString) -> Attr {
            Attr::set($actual, value)
        }
    };
}

macro_rules! attr_append {
    (
        $name:ident as $article:ident $actual:expr, separated by $separator:expr;
        at $url:expr;
    ) => {
        #[doc = concat!("Create (or append to) ", stringify!($article), " `", $actual, "` attribute")]
        #[doc = concat!("(", $url, ").")]
        pub fn $name(value: impl ToString) -> Attr {
            Attr::append($actual, value, $separator)
        }
    };
}

macro_rules! attr_enum {
    (
        $name:ident as $article:ident $actual:expr;
        at $url:expr;
        $( $valname:ident => $valstr:expr, )*
    ) => {
        #[doc = concat!("Create (or replace) ", stringify!($article), " `", $actual, "` attribute")]
        #[doc = concat!("(", $url, ").")]
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum $name {
            $(
                #[doc = concat!("The value `", stringify!($valstr), "`.")]
                $valname,
            )*
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    $( Self::$valname => $valstr.fmt(f), )*
                }
            }
        }

        impl ElementComponent for $name {
            fn add_to_element(self, element: &mut Element) {
                Attr::set($actual, self).add_to_element(element);
            }
        }
    };
    (
        $name:ident as $article:ident $actual:expr, separated by $separator:expr;
        at $url:expr;
        $( $valname:ident => $valstr:expr, )*
    ) => {
        #[doc = concat!("Create (or append to) ", stringify!($article), " `", $actual, "` attribute")]
        #[doc = concat!("(", $url, ").")]
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum $name {
            $(
                #[doc = concat!("The value `", stringify!($valstr), "`.")]
                $valname,
            )*
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    $( Self::$valname => $valstr.fmt(f), )*
                }
            }
        }

        impl ElementComponent for $name {
            fn add_to_element(self, element: &mut Element) {
                Attr::append($actual, self, $separator).add_to_element(element);
            }
        }
    };
}

////////////////
// Attributes //
////////////////

attr_append! {
    accept as an "accept", separated by ", ";
    at url!(normal, "accept");
}

attr_append! {
    accesskey as an "accesskey", separated by " ";
    at url!(global, "accesskey");
}

attr_set! {
    action as an "action";
    at url!(element "form", "action");
}

attr_append! {
    allow as an "allow", separated by "; ";
    at url!(element "iframe", "allow");
}

attr_set! {
    alt as an "alt";
    at concat!(
        url!(element "area", "alt"), ", ",
        url!(element "img", "alt"), ", ",
        url!(element "input", "alt")
    );
}

attr_enum! {
    As as an "as";
    at url!(element "link", "as");
    Audio => "audio",
    Document => "document",
    Embed => "embed",
    Fetch => "fetch",
    Font => "font",
    Image => "image",
    Object => "object",
    Script => "script",
    Style => "style",
    Track => "track",
    Video => "video",
    Worker => "worker",
}

attr_yes! {
    r#async as an "async";
    at url!(element "script", "async");
}

attr_enum! {
    Autocapitalize as an "autocapitalize";
    at url!(global, "autocapitalize");
    None => "none",
    Sentences => "sentences",
    Words => "words",
    Characters => "characters",
}

attr_append! {
    autocomplete as an "autocomplete", separated by " ";
    at url!(normal, "autocomplete");
}

attr_yes! {
    autofocus as an "autofocus";
    at url!(global, "autofocus");
}

attr_yes! {
    autoplay as an "autoplay";
    at concat!(
        url!(element "audio", "autoplay"), ", ",
        url!(element "video", "autoplay")
    );
}

attr_enum! {
    Capture as a "capture";
    at url!(normal, "capture");
    User => "user",
    Environment => "environment",
}

attr_yes! {
    checked as a "checked";
    at url!(element "input", "checked");
}

attr_set! {
    cite as a "cite";
    at concat!(
        url!(element "blockquote", "cite"), ", ",
        url!(element "del", "cite"), ", ",
        url!(element "ins", "cite"), ", ",
        url!(element "q", "cite")
    );
}

attr_append! {
    class as a "class", separated by " ";
    at url!(global, "class");
}

attr_set! {
    cols as a "cols";
    at url!(element "textarea", "cols");
}

attr_set! {
    colspan as a "colspan";
    at concat!(
        url!(element "td", "colspan"), ", ",
        url!(element "th", "colspan")
    );
}

attr_set! {
    content as a "content";
    at url!(element "meta", "content");
}

attr_enum! {
    Contenteditable as a "contenteditable";
    at url!(global, "contenteditable");
    True => "",
    False => "false",
    PlaintextOnly => "plaintext-only",
}

attr_yes! {
    controls as a "controls";
    at concat!(
        url!(element "audio", "controls"), ", ",
        url!(element "video", "controls")
    );
}

attr_set! {
    coords as a "coords";
    at url!(element "area", "coords");
}

attr_enum! {
    Crossorigin as a "crossorigin";
    at url!(normal, "crossorigin");
    Anonymous => "anonymous",
    UseCredentials => "use-credentials",
}

attr_set! {
    data as a "data";
    at url!(element "object", "data");
}

/// Create (or replace) a `data-*` attribute
/// ([MDN](https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/data-*)).
pub fn data_x(name: impl ToString, value: impl ToString) -> Attr {
    Attr::set(format!("data-{}", name.to_string()), value)
}

attr_set! {
    datetime as a "datetime";
    at concat!(
        url!(element "del", "datetime"), ", ",
        url!(element "ins", "datetime"), ", ",
        url!(element "time", "datetime")
    );
}

attr_enum! {
    Decoding as a "decoding";
    at url!(element "img", "decoding");
    Sync => "sync",
    Async => "async",
    Auto => "auto",
}

attr_yes! {
    default as a "default";
    at url!(element "track", "default");
}

attr_yes! {
    defer as a "defer";
    at url!(element "script", "defer");
}

attr_enum! {
    Dir as a "dir";
    at url!(global, "dir");
    Ltr => "ltr",
    Rtl => "rtl",
    Auto => "auto",
}

attr_set! {
    dirname as a "dirname";
    at url!(normal, "dirname");
}

attr_yes! {
    disabled as a "disabled";
    at url!(normal, "disabled");
}

attr_set! {
    download as a "download";
    at concat!(
        url!(element "a", "download"), ", ",
        url!(element "area", "download")
    );
}

attr_enum! {
    Draggable as a "draggable";
    at url!(global, "draggable");
    True => "true",
    False => "false",
}

attr_enum! {
    Enctype as an "enctype";
    at url!(element "form", "enctype");
    Form => "application/x-www-form-urlencoded",
    Multipart => "multipart/form-data",
    Plain => "text/plain",
}

attr_enum! {
    Enterkeyhint as an "enterkeyhint";
    at url!(global, "enterkeyhint");
    Enter => "enter",
    Done => "done",
    Go => "go",
    Next => "next",
    Previous => "previous",
    Search => "search",
    Send => "send",
}

attr_append! {
    exportparts as an "exportparts", separated by ", ";
    at url!(global, "exportparts");
}

attr_set! {
    r#for as a "for";
    at url!(normal, "for");
}

attr_set! {
    form as a "form";
    at concat!(
        url!(element "button", "form"), ", ",
        url!(element "fieldset", "form"), ", ",
        url!(element "input", "form"), ", ",
        url!(element "label", "form"), ", ",
        url!(element "meter", "form"), ", ",
        url!(element "object", "form"), ", ",
        url!(element "output", "form"), ", ",
        url!(element "progress", "form"), ", ",
        url!(element "select", "form"), ", ",
        url!(element "textarea", "form")
    );
}

attr_set! {
    formaction as a "formaction";
    at concat!(
        url!(element "button", "formaction"), ", ",
        url!(element "input", "formaction")
    );
}

attr_enum! {
    Formenctype as a "formenctype";
    at concat!(
        url!(element "button", "formenctype"), ", ",
        url!(element "input", "formenctype")
    );
    Form => "application/x-www-form-urlencoded",
    Multipart => "multipart/form-data",
    Plain => "text/plain",
}

attr_enum! {
    Formmethod as a "formmethod";
    at concat!(
        url!(element "button", "formmethod"), ", ",
        url!(element "input", "formmethod")
    );
    Post => "post",
    Get => "get",
    Dialog => "dialog",
}

attr_yes! {
    formnovalidate as a "formnovalidate";
    at concat!(
        url!(element "button", "formnovalidate"), ", ",
        url!(element "input", "formnovalidate")
    );
}

attr_enum! {
    Formtarget as a "formtarget";
    at concat!(
        url!(element "button", "formtarget"), ", ",
        url!(element "input", "formtarget")
    );
    Self_ => "_self",
    Blank => "_blank",
    Parent => "_parent",
    Top => "_top",
}

attr_append! {
    headers as a "headers", separated by " ";
    at concat!(
        url!(element "td", "headers"), ", ",
        url!(element "th", "headers")
    );
}

attr_set! {
    height as a "height";
    at concat!(
        url!(element "canvas", "height"), ", ",
        url!(element "embed", "height"), ", ",
        url!(element "iframe", "height"), ", ",
        url!(element "img", "height"), ", ",
        url!(element "input", "height"), ", ",
        url!(element "object", "height"), ", ",
        url!(element "video", "height")
    );
}

attr_enum! {
    Hidden as a "hidden";
    at url!(global, "hidden");
    Yes => "",
    UntilFound => "until-found",
}

attr_set! {
    high as a "high";
    at url!(element "meter", "high");
}

attr_set! {
    href as an "href";
    at concat!(
        url!(element "a", "href"), ", ",
        url!(element "area", "href"), ", ",
        url!(element "base", "href"), ", ",
        url!(element "link", "href")
    );
}

attr_set! {
    hreflang as an "hreflang";
    at concat!(
        url!(element "a", "hreflang"), ", ",
        url!(element "link", "hreflang")
    );
}

attr_enum! {
    HttpEquiv as an "http-equiv";
    at url!(element "meta", "http-equiv");
    ContentSecurityPolicy => "content-security-policy",
    ContentType => "content-type",
    DefaultStyle => "default-style",
    XUaCompatible => "x-ua-compatible",
    Refresh => "refresh",
}

attr_set! {
    id as an "id";
    at url!(global, "id");
}

attr_yes! {
    inert as an "inert";
    at url!(global, "inert");
}

attr_set! {
    integrity as an "integrity";
    at concat!(
        url!(element "link", "integrity"), ", ",
        url!(element "script", "integrity")
    );
}

attr_enum! {
    Inputmode as an "inputmode";
    at url!(global, "inputmode");
    None => "none",
    Text => "text",
    Decimal => "decimal",
    Numeric => "numeric",
    Tel => "tel",
    Search => "search",
    Email => "email",
    Url => "url",
}

attr_set! {
    is as an "is";
    at url!(global, "is");
}

attr_yes! {
    ismap as an "ismap";
    at url!(element "img", "ismap");
}

attr_set! {
    itemid as an "itemid";
    at url!(global, "itemid");
}

attr_set! {
    itemprop as an "itemprop";
    at url!(global, "itemprop");
}

attr_set! {
    itemref as an "itemref";
    at url!(global, "itemref");
}

attr_yes! {
    itemscope as an "itemscope";
    at url!(global, "itemscope");
}

attr_set! {
    itemtype as an "itemtype";
    at url!(global, "itemtype");
}

attr_enum! {
    Kind as a "kind";
    at url!(element "track", "kind");
    Subtitles => "subtitles",
    Captions => "captions",
    Chapters => "chapters",
    Metadata => "metadata",
}

attr_set! {
    lang as a "lang";
    at url!(global, "lang");
}

attr_enum! {
    Loading as a "loading";
    at concat!(
        url!(element "img", "loading"), ", ",
        url!(element "iframe", "loading")
    );
    Eager => "eager",
    Lazy => "lazy",
}

attr_set! {
    list as a "list";
    at url!(element "input", "list");
}

attr_yes! {
    r#loop as a "loop";
    at concat!(
        url!(element "audio", "loop"), ", ",
        url!(element "video", "loop")
    );
}

attr_set! {
    low as a "low";
    at url!(element "meter", "low");
}

attr_set! {
    max as a "max";
    at url!(normal, "max");
}

attr_set! {
    maxlength as a "maxlength";
    at url!(normal, "maxlength");
}

attr_set! {
    minlength as a "minlength";
    at url!(normal, "minlength");
}

attr_enum! {
    Method as a "method";
    at url!(element "form", "method");
    Post => "post",
    Get => "get",
    Dialog => "dialog",
}

attr_set! {
    min as a "min";
    at url!(normal, "min");
}

attr_yes! {
    multiple as a "multiple";
    at url!(normal, "multiple");
}

attr_yes! {
    muted as a "muted";
    at concat!(
        url!(element "audio", "muted"), ", ",
        url!(element "video", "muted")
    );
}

attr_set! {
    name as a "name";
    at concat!(
        url!(element "button", "name"), ", ",
        url!(element "form", "name"), ", ",
        url!(element "fieldset", "name"), ", ",
        url!(element "iframe", "name"), ", ",
        url!(element "input", "name"), ", ",
        url!(element "object", "name"), ", ",
        url!(element "output", "name"), ", ",
        url!(element "select", "name"), ", ",
        url!(element "textarea", "name"), ", ",
        url!(element "map", "name"), ", ",
        url!(element "meta", "name")
    );
}

attr_set! {
    nonce as a "nonce";
    at url!(global, "nonce");
}

attr_yes! {
    novalidate as a "novalidate";
    at url!(element "form", "novalidate");
}

attr_yes! {
    open as an "open";
    at concat!(
        url!(element "details", "open"), ", ",
        url!(element "dialog", "open")
    );
}

attr_set! {
    optimum as an "optimum";
    at url!(element "meter", "optimum");
}

attr_append! {
    part as a "part", separated by " ";
    at url!(global, "part");
}

attr_set! {
    pattern as a "pattern";
    at url!(normal, "pattern");
}

attr_append! {
    ping as a "ping", separated by " ";
    at concat!(
        url!(element "a", "ping"), ", ",
        url!(element "area", "ping")
    );
}

attr_set! {
    placeholder as a "placeholder";
    at url!(normal, "placeholder");
}

attr_yes! {
    playsinline as a "playsinline";
    at url!(element "video", "playsinline");
}

attr_enum! {
    Popover as a "popover";
    at url!(global, "popover");
    Auto => "",
    Manual => "manual",
}

attr_set! {
    poster as a "poster";
    at url!(element "video", "poster");
}

attr_enum! {
    Preload as a "preload";
    at concat!(
        url!(element "audio", "preload"), ", ",
        url!(element "video", "preload")
    );
    None => "none",
    Metadata => "metadata",
    Auto => "auto",
}

attr_yes! {
    readonly as a "readonly";
    at url!(normal, "readonly");
}

attr_enum! {
    Referrerpolicy as a "referrerpolicy";
    at concat!(
        url!(element "a", "referrerpolicy"), ", ",
        url!(element "area", "referrerpolicy"), ", ",
        url!(element "iframe", "referrerpolicy"), ", ",
        url!(element "img", "referrerpolicy"), ", ",
        url!(element "link", "referrerpolicy"), ", ",
        url!(element "script", "referrerpolicy"), ", ",
    );
    NoReferrer => "no-referrer",
    NoReferrerWhenDowngrade => "no-referrer-when-downgrade",
    Origin => "origin",
    OriginWhenCrossOrigin => "origin-when-cross-origin",
    SameOrigin => "same-origin",
    StrictOrigin => "strict-origin",
    StrictOriginWhenCrossOrigin => "strict-origin-when-cross-origin",
    UnsafeUrl => "unsafe-url",
}

attr_enum! {
    Rel as a "rel", separated by " ";
    at url!(normal, "rel");
    Alternate => "alternate",
    Author => "author",
    Bookmark => "bookmark",
    Canonical => "canonical",
    DnsPrefetch => "dns-prefetch",
    External => "external",
    Expect => "expect",
    Help => "help",
    Icon => "icon",
    License => "license",
    Manifest => "manifest",
    Me => "me",
    Modulepreload => "modulepreload",
    Next => "next",
    Nofollow => "nofollow",
    Noopener => "noopener",
    Noreferrer => "noreferrer",
    Opener => "opener",
    Pingback => "pingback",
    Preconnect => "preconnect",
    Prefetch => "prefetch",
    Preload => "preload",
    Prerender => "prerender",
    Prev => "prev",
    PrivacyPolicy => "privacy-policy",
    Search => "search",
    Stylesheet => "stylesheet",
    Tag => "tag",
    TermsOfService => "terms-of-service",
}

attr_append! {
    rel as a "rel", separated by " ";
    at url!(normal, "rel");
}

attr_yes! {
    required as a "required";
    at url!(normal, "required");
}

attr_yes! {
    reversed as a "reversed";
    at url!(element "ol", "reversed");
}

attr_set! {
    rows as a "rows";
    at url!(element "textarea", "rows");
}

attr_set! {
    rowspan as a "rowspan";
    at concat!(
        url!(element "td", "rowspan"), ", ",
        url!(element "th", "rowspan")
    );
}

attr_append! {
    sandbox as a "sandbox", separated by " ";
    at url!(element "iframe", "sandbox");
}

attr_enum! {
    Scope as a "scope";
    at url!(element "th", "scope");
    Row => "row",
    Col => "col",
    Rowgroup => "rowgroup",
    Colgroup => "colgroup",
}

attr_yes! {
    selected as a "selected";
    at url!(element "option", "selected");
}

attr_enum! {
    Shape as a "shape";
    at url!(element "area", "shape");
    Rect => "rect",
    Circle => "circle",
    Poly => "poly",
    Default => "default",
}

attr_set! {
    size as a "size";
    at url!(normal, "size");
}

// The "sizes" attribute for <link> is whitespace-separated while the "sizes"
// attribute for <img> and <source> is comma-separated. The naming here assumes
// that you usually want to set "sizes" on an <img> and not a <link>.

attr_append! {
    sizes as a "sizes", separated by ", ";
    at concat!(
        url!(element "img", "sizes"), ", ",
        url!(element "source", "sizes")
    );
}

attr_append! {
    sizes_link as a "sizes", separated by " ";
    at url!(element "link", "sizes");
}

attr_set! {
    slot as a "slot";
    at url!(global, "slot");
}

attr_set! {
    span as a "span";
    at concat!(
        url!(element "col", "span"), ", ",
        url!(element "colgroup", "span")
    );
}

attr_enum! {
    Spellcheck as a "spellcheck";
    at url!(global, "spellcheck");
    True => "",
    False => "false",
}

attr_set! {
    src as a "src";
    at concat!(
        url!(element "audio", "src"), ", ",
        url!(element "embed", "src"), ", ",
        url!(element "iframe", "src"), ", ",
        url!(element "img", "src"), ", ",
        url!(element "input", "src"), ", ",
        url!(element "script", "src"), ", ",
        url!(element "source", "src"), ", ",
        url!(element "track", "src"), ", ",
        url!(element "video", "src")
    );
}

attr_set! {
    srcdoc as a "srcdoc";
    at url!(element "iframe", "srcdoc");
}

attr_set! {
    srclang as a "srclang";
    at url!(element "track", "srclang");
}

attr_append! {
    srcset as a "srcset", separated by ", ";
    at concat!(
        url!(element "img", "srcset"), ", ",
        url!(element "source", "srcset")
    );
}

attr_set! {
    start as a "start";
    at url!(element "ol", "start");
}

attr_set! {
    step as a "step";
    at url!(normal, "step");
}

attr_append! {
    style as a "style", separated by "; ";
    at url!(global, "style");
}

attr_set! {
    tabindex as a "tabindex";
    at url!(global, "tabindex");
}

attr_enum! {
    Target as a "target";
    at concat!(
        url!(element "a", "target"), ", ",
        url!(element "area", "target"), ", ",
        url!(element "base", "target"), ", ",
        url!(element "form", "target")
    );
    Self_ => "_self",
    Blank => "_blank",
    Parent => "_parent",
    Top => "_top",
    UnfencedTop => "_unfencedTop",
}

attr_set! {
    title as a "title";
    at url!(global, "title");
}

attr_enum! {
    Translate as a "translate";
    at url!(global, "translate");
    Yes => "",
    No => "no",
}

attr_set! {
    r#type as a "type";
    at concat!(
        url!(element "embed", "type"), ", ",
        url!(element "object", "type"), ", ",
        url!(element "source", "type"), ", ",
        url!(element "link", "type")
    );
}

attr_enum! {
    TypeButton as a "type";
    at url!(element "button", "type");
    Submit => "submit",
    Reset => "reset",
    Button => "button",
}

attr_enum! {
    TypeInput as a "type";
    at url!(element "input", "type");
    Button => "button",
    Checkbox => "checkbox",
    Color => "color",
    Date => "date",
    DatetimeLocal => "datetime-local",
    Email => "email",
    File => "file",
    Hidden => "hidden",
    Image => "image",
    Month => "month",
    Number => "number",
    Password => "password",
    Radio => "radio",
    Range => "range",
    Reset => "reset",
    Search => "search",
    Submit => "submit",
    Tel => "tel",
    Text => "text",
    Time => "time",
    Url => "url",
    Week => "week",
}

attr_enum! {
    TypeOl as a "type";
    at url!(element "ol", "type");
    LowercaseAlphabetic => "a",
    UppercaseAlphabetic => "A",
    LowercaseRoman => "i",
    UppercaseRoman => "I",
    Numbers => "1",
}

attr_enum! {
    TypeScript as a "type";
    at url!(element "script", "type");
    Classic => "",
    Importmap => "importmap",
    Module => "module",
}

attr_set! {
    usemap as a "usemap";
    at url!(element "img", "usemap");
}

attr_set! {
    value as a "value";
    at concat!(
        url!(element "button", "value"), ", ",
        url!(element "data", "value"), ", ",
        url!(element "input", "value"), ", ",
        url!(element "li", "value"), ", ",
        url!(element "meter", "value"), ", ",
        url!(element "option", "value"), ", ",
        url!(element "progress", "value")
    );
}

attr_set! {
    width as a "width";
    at concat!(
        url!(element "canvas", "width"), ", ",
        url!(element "embed", "width"), ", ",
        url!(element "iframe", "width"), ", ",
        url!(element "img", "width"), ", ",
        url!(element "input", "width"), ", ",
        url!(element "object", "width"), ", ",
        url!(element "video", "width")
    );
}

attr_enum! {
    Wrap as a "wrap";
    at url!(element "textarea", "wrap");
    Hard => "hard",
    Soft => "soft",
}

attr_enum! {
    WritingSuggestions as a "writingsuggestions";
    at url!(global, "writingsuggestions");
    True => "",
    False => "false",
}
