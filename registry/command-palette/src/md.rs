// SPDX-License-Identifier: MIT OR Apache-2.0
use proa_core::{DataLoader, MdRenderSync, WebContext, WriteBuf, WriteError};
use proa_macros::{md_sync, omit};

use super::command_palette::SearchBox;
use super::{Command, CommandPalette};

impl<L: DataLoader, B: WriteBuf> MdRenderSync<L, B> for &Command {
    fn render_md(self, cx: &mut WebContext<L, B>) -> Result<(), WriteError> {
        match self.href {
            Some(href) => md_sync! { "- [" { self.label } "](" { href } ")\n" }.render_md(cx),
            None => md_sync! { "- " { self.label } "\n" }.render_md(cx),
        }
    }
}

/// Lists the commands, grouped, so Markdown readers see what is available.
impl<L: DataLoader, B: WriteBuf> MdRenderSync<L, B> for CommandPalette {
    fn render_md(self, cx: &mut WebContext<L, B>) -> Result<(), WriteError> {
        omit!(SearchBox, "Browser-only search input");
        omit!(self.label, "Accessible name of the dialog");
        omit!(self.empty_text, "Shown only while filtering in the browser");
        for group in self.groups {
            md_sync! { "**" { group.heading } "**\n\n" }.render_md(cx)?;
            for command in group.commands {
                command.render_md(cx)?;
            }
            md_sync! { "\n" }.render_md(cx)?;
        }
        Ok(())
    }
}
