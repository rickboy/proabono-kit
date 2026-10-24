// SPDX-License-Identifier: MIT OR Apache-2.0
use proa_core::{style_ok, ClassList, DataLoader, WebContext, WebRenderSync, WriteError};
use proa_macros::{html_sync, text};

use crate::components::dropdown_menu::MenuItem;

/// One top-level menu. `id` must be unique on the page and a valid CSS
/// identifier (it also names the trigger `{id}-trigger` and anchor `--pb-{id}`).
#[derive(Debug, Clone, Copy)]
pub struct MenubarMenu {
    pub id: &'static str,
    pub label: &'static str,
    pub items: &'static [MenuItem],
}

impl MenubarMenu {
    pub const fn new(id: &'static str, label: &'static str, items: &'static [MenuItem]) -> Self {
        Self { id, label, items }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Menubar {
    /// Accessible name, e.g. `"Editor"`.
    pub label: &'static str,
    pub menus: &'static [MenubarMenu],
    pub class: Option<&'static str>,
}

impl Menubar {
    pub const fn new(label: &'static str, menus: &'static [MenubarMenu]) -> Self {
        Self {
            label,
            menus,
            class: None,
        }
    }

    pub const fn class(mut self, class: &'static str) -> Self {
        self.class = Some(class);
        self
    }
}

impl<L: DataLoader> WebRenderSync<L> for &MenubarMenu {
    fn render(self, cx: &mut WebContext<L>) -> Result<(), WriteError> {
        html_sync! {
            <button
                type="button"
                role="menuitem"
                id={text!("{}-trigger", self.id)}
                class="pb-menubar-trigger"
                data-pb-menu-trigger
                popovertarget={self.id}
                aria-haspopup="menu"
                aria-controls={self.id}
                aria-expanded="false"
                style={style_ok(text!("anchor-name: --pb-{}", self.id))}
            >
                {self.label}
            </button>
            <div
                id={self.id}
                popover
                role="menu"
                aria-labelledby={text!("{}-trigger", self.id)}
                data-pb-menu-content
                data-align="start"
                class="pb-menu-content"
                style={style_ok(text!("position-anchor: --pb-{}", self.id))}
            >
                {for item in self.items.iter() {
                    html_sync! { {item} }
                }}
            </div>
        }
        .render(cx)
    }
}

impl<L: DataLoader> WebRenderSync<L> for Menubar {
    fn render(self, cx: &mut WebContext<L>) -> Result<(), WriteError> {
        let classes = ClassList::<2>::new().add("pb-menubar").add_opt(self.class);
        html_sync! {
            <div role="menubar" aria-label={self.label} class={classes} data-pb-menubar>
                {for menu in self.menus.iter() {
                    html_sync! { {menu} }
                }}
            </div>
        }
        .render(cx)
    }
}
