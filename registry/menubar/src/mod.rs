// SPDX-License-Identifier: MIT OR Apache-2.0
//! Menubar (WAI-ARIA menubar): a horizontal row of menu buttons.
//!
//! Each menu is a native popover listing
//! [`MenuItem`](crate::components::dropdown_menu::MenuItem)s, so every trigger
//! works without JavaScript. With `proabono.js` the bar becomes a single tab
//! stop with Left/Right/Home/End between triggers, Left/Right inside an open
//! menu moves to the neighbouring menu, and hovering another trigger while a
//! menu is open switches to it.
#![allow(dead_code, unused_imports)]

mod md;
mod menubar;

pub use menubar::{Menubar, MenubarMenu};
