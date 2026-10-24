// SPDX-License-Identifier: MIT OR Apache-2.0
use proa_core::{DataLoader, MdRenderSync, WebContext, WriteBuf, WriteError};
use proa_macros::md_sync;

use super::Pagination;

#[proa_macros::content(skip, reason = "Navigation chrome; Markdown readers get the page position only")]
impl<L: DataLoader, B: WriteBuf> MdRenderSync<L, B> for Pagination<'_> {
    fn render_md(self, cx: &mut WebContext<L, B>) -> Result<(), WriteError> {
        let total = self.total_pages.max(1);
        let page = self.page.clamp(1, total);
        md_sync! { r###"
            Page {page} of {total}
            "###
        }
        .render_md(cx)
    }
}
