// SPDX-License-Identifier: MIT OR Apache-2.0
//! Drawer: a modal `<dialog>` that slides in from an edge.
//!
//! The trigger uses invoker commands (`commandfor` / `command="show-modal"`)
//! and the dialog uses `closedby="any"`, so open, Escape and backdrop dismiss
//! need no JavaScript where supported; `proabono.js` fills in elsewhere.
//! Close buttons are `<form method="dialog">` and always work.
#![allow(dead_code, unused_imports)]

mod drawer;
mod md;

pub use drawer::{Drawer, DrawerClose, DrawerSide, DrawerTrigger};
