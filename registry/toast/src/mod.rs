// SPDX-License-Identifier: MIT OR Apache-2.0
//! Toasts: a Sonner-style stack in the corner of the screen.
//!
//! Render one [`Toaster`] per page (usually in the root layout) and pass any
//! flash messages from the request. From the browser, call
//! `window.proabono.toast({ title, description, tone, duration })` or add
//! `data-pb-toast="Title"` (plus optional `data-pb-toast-description`,
//! `data-pb-toast-tone`, `data-pb-toast-duration`) to any button.
//! Stacking, hover-to-expand and auto-dismiss need `proabono.js`.
#![allow(dead_code, unused_imports)]

mod md;
mod toast;

pub use toast::{Toast, ToastPosition, ToastTone, Toaster};
