// SPDX-License-Identifier: MIT OR Apache-2.0
use proa_core::{ClassList, DataLoader, WebContext, WebRenderSync, WriteError};
use proa_macros::html_sync;

use crate::components::dropdown_menu::MenuItem;

/// Wraps `target` and attaches a menu to it.
///
/// `id` names the menu popover and must be unique on the page.
#[derive(Debug, Clone, Copy)]
pub struct ContextMenu<Target> {
    pub id: &'static str,
    /// Accessible name for the menu, e.g. `"File actions"`.
    pub label: &'static str,
    pub items: &'static [MenuItem],
    pub target: Target,
    /// Make the wrapper focusable so Shift+F10 works on it directly. Turn
    /// off when `target` already contains the focusable element.
    pub focusable: bool,
    pub class: Option<&'static str>,
}

impl<Target> ContextMenu<Target> {
    pub const fn new(
        id: &'static str,
        label: &'static str,
        items: &'static [MenuItem],
        target: Target,
    ) -> Self {
        Self {
            id,
            label,
            items,
            target,
            focusable: true,
            class: None,
        }
    }

    pub const fn not_focusable(mut self) -> Self {
        self.focusable = false;
        self
    }

    pub const fn class(mut self, class: &'static str) -> Self {
        self.class = Some(class);
        self
    }
}

/// The popover menu itself. Browser-only, so the Markdown view omits it.
pub(super) struct Menu {
    pub(super) id: &'static str,
    pub(super) label: &'static str,
    pub(super) items: &'static [MenuItem],
}

impl<L: DataLoader> WebRenderSync<L> for Menu {
    fn render(self, cx: &mut WebContext<L>) -> Result<(), WriteError> {
        html_sync! {
            <div
                id={self.id}
                popover
                role="menu"
                aria-label={self.label}
                data-pb-menu-content
                data-pb-context
                class="pb-menu-content"
            >
                {for item in self.items.iter() {
                    html_sync! { {item} }
                }}
            </div>
        }
        .render(cx)
    }
}

impl<L: DataLoader, Target: WebRenderSync<L>> WebRenderSync<L> for ContextMenu<Target> {
    fn render(self, cx: &mut WebContext<L>) -> Result<(), WriteError> {
        let classes = ClassList::<2>::new()
            .add("pb-context-target")
            .add_opt(self.class);
        let tabindex = self.focusable.then_some("0");
        html_sync! {
            <div class={classes} data-pb-context-target={self.id} tabindex={tabindex}>
                {self.target}
            </div>
            {Menu { id: self.id, label: self.label, items: self.items }}
        }
        .render(cx)
    }
}
