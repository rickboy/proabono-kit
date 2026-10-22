// SPDX-License-Identifier: MIT OR Apache-2.0
use proa_core::{DataLoader, MdRenderSync, WebContext, WriteBuf, WriteError};
use proa_macros::{md_sync, omit};

use super::toast::{Description, DismissButton};
use super::{Toast, Toaster};

impl<L: DataLoader, B: WriteBuf> MdRenderSync<L, B> for &Toast<'_> {
    fn render_md(self, cx: &mut WebContext<L, B>) -> Result<(), WriteError> {
        omit!(DismissButton, "Browser-only control");
        omit!(Description<'_>, "Rendered inline after the title");
        md_sync! { r###"
            > **{self.title}** {self.description}

            "###
        }
        .render_md(cx)
    }
}

impl<L: DataLoader, B: WriteBuf> MdRenderSync<L, B> for Toaster<'_> {
    fn render_md(self, cx: &mut WebContext<L, B>) -> Result<(), WriteError> {
        md_sync! { r###"
            {for toast in self.toasts.iter() {
            {toast}
            }}
            "###
        }
        .render_md(cx)
    }
}
