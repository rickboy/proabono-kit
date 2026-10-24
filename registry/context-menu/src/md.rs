// SPDX-License-Identifier: MIT OR Apache-2.0
use proa_core::{DataLoader, MdRenderSync, WebContext, WriteBuf, WriteError};
use proa_macros::{md_sync, omit};

use super::context_menu::Menu;
use super::ContextMenu;

impl<L: DataLoader, B: WriteBuf, Target: MdRenderSync<L, B>> MdRenderSync<L, B>
    for ContextMenu<Target>
{
    fn render_md(self, cx: &mut WebContext<L, B>) -> Result<(), WriteError> {
        omit!(Menu, "Pointer shortcut; the actions must also exist elsewhere on the page");
        md_sync! { r###"
            {self.target}
            "###
        }
        .render_md(cx)
    }
}
