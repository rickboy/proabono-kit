// SPDX-License-Identifier: MIT OR Apache-2.0
use proa_core::{DataLoader, MdRenderSync, WebContext, WriteBuf, WriteError};
use proa_macros::md_sync;

use super::Combobox;

impl<L: DataLoader, B: WriteBuf> MdRenderSync<L, B> for Combobox {
    fn render_md(self, cx: &mut WebContext<L, B>) -> Result<(), WriteError> {
        md_sync! { r###"
            **{self.label}**

            {for option in self.options.iter() {
            - {option.label}
            }}
            "###
        }
        .render_md(cx)
    }
}
