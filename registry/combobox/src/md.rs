// SPDX-License-Identifier: MIT OR Apache-2.0
use proa_core::{DataLoader, MdRenderSync, WebContext, WriteBuf, WriteError};
use proa_macros::{md_sync, omit};

use super::Combobox;

impl<L: DataLoader, B: WriteBuf> MdRenderSync<L, B> for Combobox {
    fn render_md(self, cx: &mut WebContext<L, B>) -> Result<(), WriteError> {
        omit!(self.placeholder, "Input hint, not content");
        omit!(self.empty_text, "Shown only while filtering in the browser");
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
