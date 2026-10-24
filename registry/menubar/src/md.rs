// SPDX-License-Identifier: MIT OR Apache-2.0
use proa_core::{DataLoader, MdRenderSync, WebContext, WriteBuf, WriteError};
use proa_macros::{md_sync, omit};

use super::{Menubar, MenubarMenu};

impl<L: DataLoader, B: WriteBuf> MdRenderSync<L, B> for &MenubarMenu {
    fn render_md(self, cx: &mut WebContext<L, B>) -> Result<(), WriteError> {
        md_sync! { r###"
            **{self.label}**

            {for item in self.items.iter() {
            {item}
            }}
            "###
        }
        .render_md(cx)
    }
}

impl<L: DataLoader, B: WriteBuf> MdRenderSync<L, B> for Menubar {
    fn render_md(self, cx: &mut WebContext<L, B>) -> Result<(), WriteError> {
        omit!(self.label, "Accessible name of the menubar landmark");
        md_sync! { r###"
            {for menu in self.menus.iter() {
            {menu}

            }}
            "###
        }
        .render_md(cx)
    }
}
