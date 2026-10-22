// SPDX-License-Identifier: MIT OR Apache-2.0
//! Combobox (WAI-ARIA combobox with list autocomplete).
//!
//! With `proabono.js` the field filters a listbox, supports arrow keys,
//! Enter and Escape, and submits the chosen option's `value` under `name`.
//! Without JavaScript it is a text input with a native `<datalist>` and
//! submits the typed label; use [`Combobox::resolve`] on the server to accept
//! either form.
#![allow(dead_code, unused_imports)]

mod combobox;
mod md;

pub use combobox::{ComboOption, Combobox};
