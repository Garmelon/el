//! Definitions for all non-deprecated SVG elements
//! ([MDN](https://developer.mozilla.org/en-US/docs/Web/SVG/Element)).

use crate::{Element, ElementComponent, ElementKind};

macro_rules! element {
    ( $name:ident ) => {
        element!($name, stringify!($name));
    };
    ( $name:ident, $tag:expr ) => {
        #[doc = concat!("The `<", $tag, ">` tag")]
        #[doc = concat!("([MDN](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/", $tag, ")).")]
        pub fn $name(c: impl ElementComponent) -> Element {
            Element::new($tag, ElementKind::Foreign).with(c)
        }
    };
}

// SVG elements A to Z

// Deprecated elements intentionally omitted.

element!(a);
element!(animate);
element!(animate_motion, "animateMotion");
element!(animate_transform, "animateTransform");
element!(circle);
element!(clip_path, "clipPath");
element!(defs);
element!(desc);
element!(ellipse);
element!(fe_blend, "feBlend");
element!(fe_color_matrix, "feColorMatrix");
element!(fe_component_transfer, "feComponentTransfer");
element!(fe_composite, "feComposite");
element!(fe_convolve_matrix, "feConvolveMatrix");
element!(fe_diffuse_lighting, "feDiffuseLighting");
element!(fe_displacement_map, "feDisplacementMap");
element!(fe_distant_light, "feDistantLight");
element!(fe_drop_shadow, "feDropShadow");
element!(fe_flood, "feFlood");
element!(fe_func_a, "feFuncA");
element!(fe_func_b, "feFuncB");
element!(fe_func_g, "feFuncG");
element!(fe_func_r, "feFuncR");
element!(fe_gaussian_blue, "feGaussianBlur");
element!(fe_image, "feImage");
element!(fe_merge, "feMerge");
element!(fe_merge_node, "feMergeNode");
element!(fe_morphology, "feMorphology");
element!(fe_offset, "feOffset");
element!(fe_point_light, "fePointLight");
element!(fe_specular_lighting, "feSpecularLighting");
element!(fe_spot_light, "feSpotLight");
element!(fe_tile, "feTile");
element!(fe_turbulence, "feTurbulence");
element!(filter);
element!(foreign_object, "foreignObject");
element!(g);
element!(image);
element!(line);
element!(linear_gradient, "linearGradient");
element!(marker);
element!(mask);
element!(metadata);
element!(mpath);
element!(path);
element!(pattern);
element!(polygon);
element!(polyline);
element!(radial_gradient, "radialGradient");
element!(rect);
element!(script);
element!(set);
element!(stop);
element!(style);
element!(svg);
element!(switch);
element!(symbol);
element!(text);
element!(text_path, "textPath");
element!(title);
element!(tspan);
element!(r#use, "use");
element!(view);
