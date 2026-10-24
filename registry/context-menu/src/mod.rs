// SPDX-License-Identifier: MIT OR Apache-2.0
//! Context menu: a [`MenuItem`](crate::components::dropdown_menu::MenuItem)
//! menu opened by right click, long press, or Shift+F10 / the Menu key on the
//! focused target.
//!
//! A context menu is a shortcut, never the only way to reach an action:
//! without JavaScript the browser's own menu appears instead, so keep the same
//! actions available elsewhere on the page. Action items fire the same
//! `proabono:select` event as the dropdown menu, from the menu element.
#![allow(dead_code, unused_imports)]

mod context_menu;
mod md;

pub use context_menu::ContextMenu;
