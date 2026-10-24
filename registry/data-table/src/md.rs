// SPDX-License-Identifier: MIT OR Apache-2.0
use proa_core::{DataLoader, MdRenderSync, WebContext, WriteBuf, WriteError};
use proa_macros::{md_sync, omit};

use super::data_table::Toolbar;
use super::{Cell, DataTable};

impl<L: DataLoader, B: WriteBuf> MdRenderSync<L, B> for Cell<'_> {
    fn render_md(self, cx: &mut WebContext<L, B>) -> Result<(), WriteError> {
        match self {
            Cell::Text(text) | Cell::Sorted { text, .. } => md_sync! { { text } }.render_md(cx),
            Cell::Int(n) => md_sync! { { n } }.render_md(cx),
            Cell::Link { text, href } => md_sync! { "[" { text } "](" { href } ")" }.render_md(cx),
            Cell::Empty => Ok(()),
        }
    }
}

/// Renders the rows as a GitHub-flavoured Markdown table.
impl<L: DataLoader, B: WriteBuf, R> MdRenderSync<L, B> for DataTable<'_, R> {
    fn render_md(self, cx: &mut WebContext<L, B>) -> Result<(), WriteError> {
        omit!(Toolbar, "Browser-only filter controls");
        omit!(self.empty_text, "Shown only when filtering hides every row");
        md_sync! { r###"
            **{self.caption}**

            "###
        }
        .render_md(cx)?;
        for column in self.columns {
            md_sync! { "| " { column.header } " " }.render_md(cx)?;
        }
        md_sync! { "|\n" }.render_md(cx)?;
        for column in self.columns {
            if column.numeric {
                md_sync! { "| ---: " }.render_md(cx)?;
            } else {
                md_sync! { "| --- " }.render_md(cx)?;
            }
        }
        md_sync! { "|\n" }.render_md(cx)?;
        for row in self.rows {
            for column in self.columns {
                md_sync! { "| " { (column.cell)(row) } " " }.render_md(cx)?;
            }
            md_sync! { "|\n" }.render_md(cx)?;
        }
        md_sync! { "\n" }.render_md(cx)
    }
}
