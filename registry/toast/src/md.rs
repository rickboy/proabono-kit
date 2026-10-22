// SPDX-License-Identifier: MIT OR Apache-2.0
use proa_core::{DataLoader, MdRenderSync, WebContext, WriteBuf, WriteError};
use proa_macros::md_sync;

use super::Toaster;

impl<L: DataLoader, B: WriteBuf> MdRenderSync<L, B> for Toaster<'_> {
    fn render_md(self, cx: &mut WebContext<L, B>) -> Result<(), WriteError> {
        md_sync! { r###"
            {for toast in self.toasts.iter() {
            > **{toast.title}** {toast.description}

            }}
            "###
        }
        .render_md(cx)
    }
}
