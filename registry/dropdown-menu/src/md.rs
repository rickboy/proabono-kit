// SPDX-License-Identifier: MIT OR Apache-2.0
use proa_core::{DataLoader, MdRenderSync, WebContext, WriteBuf, WriteError};
use proa_macros::md_sync;

use super::{DropdownMenu, MenuItem};

impl<L: DataLoader, B: WriteBuf> MdRenderSync<L, B> for DropdownMenu {
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

impl<L: DataLoader, B: WriteBuf> MdRenderSync<L, B> for &MenuItem {
    fn render_md(self, cx: &mut WebContext<L, B>) -> Result<(), WriteError> {
        match *self {
            MenuItem::Link { label, href, .. } => md_sync! { r###"
                - [{label}]({href})
                "###
            }
            .render_md(cx),
            MenuItem::Action { label, .. } => md_sync! { r###"
                - {label}
                "###
            }
            .render_md(cx),
            MenuItem::Label(text) => md_sync! { r###"
                - *{text}*
                "###
            }
            .render_md(cx),
            MenuItem::Separator => Ok(()),
        }
    }
}
