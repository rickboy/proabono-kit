// SPDX-License-Identifier: MIT OR Apache-2.0
use proa_core::shared::attr_value::AttrValue;
use proa_core::{ClassList, DataLoader, WebContext, WebRenderSync, WriteBuf, WriteError};
use proa_macros::html_sync;

use crate::components::_proabono::icons;

/// `href` attribute value `{prefix}{page}`, written straight into the
/// response without building a `String`. The URL-scheme guard checks the
/// prefix; a page number cannot introduce a scheme.
#[derive(Debug, Clone, Copy)]
pub struct PageHref<'a> {
    pub prefix: &'a str,
    pub page: u32,
}

impl AttrValue for PageHref<'_> {
    fn should_render(&self) -> bool {
        true
    }

    fn url_str(&self) -> Option<&str> {
        Some(self.prefix)
    }

    fn render_attr_value<B: WriteBuf, L: DataLoader>(
        &self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), WriteError> {
        self.prefix.render_attr_value(cx)?;
        let mut digits = [0u8; 10];
        let mut n = self.page;
        let mut i = digits.len();
        loop {
            i -= 1;
            digits[i] = b'0' + (n % 10) as u8;
            n /= 10;
            if n == 0 {
                break;
            }
        }
        cx.out.extend(&digits[i..])
    }
}

/// Numbered page links plus previous/next.
#[derive(Debug, Clone, Copy)]
pub struct Pagination<'a> {
    /// Current page, 1-based.
    pub page: u32,
    pub total_pages: u32,
    /// Link prefix; the page number is appended (`"/devlog?page="`).
    pub href: &'a str,
    /// Pages shown on each side of the current one.
    pub siblings: u32,
    /// Accessible name for the `<nav>` landmark.
    pub label: &'static str,
    pub class: Option<&'static str>,
}

impl<'a> Pagination<'a> {
    pub const fn new(page: u32, total_pages: u32, href: &'a str) -> Self {
        Self {
            page,
            total_pages,
            href,
            siblings: 1,
            label: "Pagination",
            class: None,
        }
    }

    pub const fn siblings(mut self, siblings: u32) -> Self {
        self.siblings = siblings;
        self
    }

    pub const fn label(mut self, label: &'static str) -> Self {
        self.label = label;
        self
    }

    pub const fn class(mut self, class: &'static str) -> Self {
        self.class = Some(class);
        self
    }

    /// Items to show, e.g. `1 … 4 5 6 … 20`. `0` marks a gap.
    fn window(&self) -> ([u32; MAX_ITEMS], usize) {
        let total = self.total_pages.max(1);
        let page = self.page.clamp(1, total);
        let siblings = self.siblings.min(MAX_SIBLINGS);
        let mut items = [0u32; MAX_ITEMS];
        let mut len = 0;
        let mut push = |value: u32| {
            items[len] = value;
            len += 1;
        };

        // first, last, current ± siblings, and a gap on each side at most.
        let shown = 2 * siblings + 5;
        if total <= shown {
            for p in 1..=total {
                push(p);
            }
            return (items, len);
        }
        let start = page.saturating_sub(siblings).max(2);
        let end = (page + siblings).min(total - 1);
        // Keep the strip a constant width near either end.
        let (start, end) = if start <= 3 {
            (2, 3 + 2 * siblings)
        } else if end >= total - 2 {
            (total - 2 - 2 * siblings, total - 1)
        } else {
            (start, end)
        };
        push(1);
        if start > 2 {
            push(0);
        }
        for p in start..=end {
            push(p);
        }
        if end < total - 1 {
            push(0);
        }
        push(total);
        (items, len)
    }
}

const MAX_SIBLINGS: u32 = 3;
const MAX_ITEMS: usize = (2 * MAX_SIBLINGS as usize) + 5;

struct PageItem<'a> {
    href: &'a str,
    page: u32,
    current: u32,
}

impl<L: DataLoader> WebRenderSync<L> for PageItem<'_> {
    fn render(self, cx: &mut WebContext<L>) -> Result<(), WriteError> {
        if self.page == 0 {
            return html_sync! {
                <li class="pb-page-gap" aria-hidden="true">"..."</li>
            }
            .render(cx);
        }
        let current = (self.page == self.current).then_some("page");
        html_sync! {
            <li>
                <a
                    class="pb-page-link"
                    href={PageHref { prefix: self.href, page: self.page }}
                    aria-current={current}
                >
                    {self.page}
                </a>
            </li>
        }
        .render(cx)
    }
}

/// Previous/next link, or an inert placeholder at either end.
struct Step<'a> {
    href: &'a str,
    target: Option<u32>,
    next: bool,
}

impl<L: DataLoader> WebRenderSync<L> for Step<'_> {
    fn render(self, cx: &mut WebContext<L>) -> Result<(), WriteError> {
        let (label, rel) = if self.next {
            ("Next", "next")
        } else {
            ("Previous", "prev")
        };
        let icon = if self.next {
            icons::CHEVRON_RIGHT
        } else {
            icons::CHEVRON_LEFT
        };
        match self.target {
            Some(page) => html_sync! {
                <li>
                    <a
                        class="pb-page-link pb-page-step"
                        href={PageHref { prefix: self.href, page }}
                        rel={rel}
                        data-step={rel}
                    >
                        {icon}
                        <span>{label}</span>
                    </a>
                </li>
            }
            .render(cx),
            None => html_sync! {
                <li>
                    <span class="pb-page-link pb-page-step" aria-disabled="true" data-step={rel}>
                        {icon}
                        <span>{label}</span>
                    </span>
                </li>
            }
            .render(cx),
        }
    }
}

impl<L: DataLoader> WebRenderSync<L> for Pagination<'_> {
    fn render(self, cx: &mut WebContext<L>) -> Result<(), WriteError> {
        let total = self.total_pages.max(1);
        let page = self.page.clamp(1, total);
        let (items, len) = self.window();
        let classes = ClassList::<2>::new().add("pb-pagination").add_opt(self.class);
        let href = self.href;
        html_sync! {
            <nav aria-label={self.label} class={classes} data-pb-pagination>
                <ul>
                    {Step { href, target: (page > 1).then(|| page - 1), next: false }}
                    {for p in items[..len].iter() {
                        html_sync! { {PageItem { href, page: *p, current: page }} }
                    }}
                    {Step { href, target: (page < total).then(|| page + 1), next: true }}
                </ul>
            </nav>
        }
        .render(cx)
    }
}
