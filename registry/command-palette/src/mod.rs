// SPDX-License-Identifier: MIT OR Apache-2.0
//! Command palette: a modal `<dialog>` with a search box over grouped
//! commands, opened by Ctrl/Cmd+K (configurable) or a [`CommandTrigger`].
//!
//! Commands with an `href` are links and navigate; the rest fire a bubbling
//! `proabono:command` event from the dialog with `{ value, label }`. Without
//! JavaScript the trigger still opens the dialog (invoker commands) and link
//! commands still work; filtering and the keyboard shortcut need
//! `proabono.js`.
#![allow(dead_code, unused_imports)]

mod command_palette;
mod md;

pub use command_palette::{Command, CommandGroup, CommandPalette, CommandTrigger};
