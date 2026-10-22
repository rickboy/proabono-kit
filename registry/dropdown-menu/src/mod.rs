// SPDX-License-Identifier: MIT OR Apache-2.0
//! Dropdown menu (WAI-ARIA menu button).
//!
//! Without JavaScript the menu opens and closes through the native `popover`
//! attribute (click, Escape, light dismiss). `proabono.js` adds arrow keys,
//! Home/End, typeahead, focus return, and a positioning fallback for browsers
//! without CSS anchor positioning. Action items dispatch a bubbling
//! `proabono:select` event with `{ value, label }`.
#![allow(dead_code, unused_imports)]

mod dropdown_menu;
mod md;

pub use dropdown_menu::{DropdownMenu, ItemTone, MenuAlign, MenuItem};
