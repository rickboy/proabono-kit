// SPDX-License-Identifier: MIT OR Apache-2.0
//! Inline SVG icons (Lucide-style geometry, drawn for this kit). Decorative:
//! every icon is `aria-hidden`, so pair it with visible or sr-only text.

use proa_core::StaticRaw;

macro_rules! icon {
    ($name:ident, $paths:literal) => {
        pub const $name: StaticRaw = StaticRaw(concat!(
            r#"<svg class="pb-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" focusable="false">"#,
            $paths,
            "</svg>"
        ));
    };
}

icon!(CHEVRON_DOWN, r#"<path d="m6 9 6 6 6-6"/>"#);
icon!(CHEVRONS_UP_DOWN, r#"<path d="m7 15 5 5 5-5"/><path d="m7 9 5-5 5 5"/>"#);
icon!(CHECK, r#"<path d="M20 6 9 17l-5-5"/>"#);
icon!(X, r#"<path d="M18 6 6 18M6 6l12 12"/>"#);
icon!(
    CIRCLE_ALERT,
    r#"<path d="M12 8v4m0 4h.01"/><circle cx="12" cy="12" r="10"/>"#
);
icon!(
    INFO,
    r#"<circle cx="12" cy="12" r="10"/><path d="M12 16v-4m0-4h.01"/>"#
);
icon!(
    CIRCLE_CHECK,
    r#"<circle cx="12" cy="12" r="10"/><path d="m9 12 2 2 4-4"/>"#
);
icon!(
    TRIANGLE_ALERT,
    r#"<path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3"/><path d="M12 9v4m0 4h.01"/>"#
);
icon!(
    CIRCLE_X,
    r#"<circle cx="12" cy="12" r="10"/><path d="m15 9-6 6m0-6 6 6"/>"#
);
