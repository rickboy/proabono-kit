// SPDX-License-Identifier: MIT OR Apache-2.0
use proa_core::{DataLoader, MdRenderSync, WebContext, WriteBuf, WriteError};
use proa_macros::md_sync;

use super::Drawer;

impl<L: DataLoader, B: WriteBuf, Body: MdRenderSync<L, B>> MdRenderSync<L, B> for Drawer<Body> {
    fn render_md(self, cx: &mut WebContext<L, B>) -> Result<(), WriteError> {
        md_sync! { r###"
            ## {self.title}

            {self.description}

            {self.body}
            "###
        }
        .render_md(cx)
    }
}
