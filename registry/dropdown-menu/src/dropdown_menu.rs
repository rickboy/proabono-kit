// SPDX-License-Identifier: MIT OR Apache-2.0
use proa_core::{style_ok, ClassList, DataLoader, WebContext, WebRenderSync, WriteError};
use proa_macros::{html_sync, text};

use crate::components::_proabono::icons;

/// Horizontal alignment of the menu against its trigger.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MenuAlign {
    #[default]
    Start,
    Center,
    End,
}

impl MenuAlign {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Center => "center",
            Self::End => "end",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ItemTone {
    #[default]
    Default,
    Danger,
}

impl ItemTone {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Danger => "danger",
        }
    }
}

/// One entry in a [`DropdownMenu`].
#[derive(Debug, Clone, Copy)]
pub enum MenuItem {
    /// Navigates to `href`.
    Link {
        label: &'static str,
        href: &'static str,
        shortcut: Option<&'static str>,
        tone: ItemTone,
    },
    /// Fires `proabono:select` with `value`, then closes the menu.
    Action {
        label: &'static str,
        value: &'static str,
        shortcut: Option<&'static str>,
        tone: ItemTone,
        disabled: bool,
    },
    /// Non-interactive group heading.
    Label(&'static str),
    Separator,
}

impl MenuItem {
    pub const SEPARATOR: Self = Self::Separator;

    pub const fn link(label: &'static str, href: &'static str) -> Self {
        Self::Link {
            label,
            href,
            shortcut: None,
            tone: ItemTone::Default,
        }
    }

    pub const fn action(label: &'static str, value: &'static str) -> Self {
        Self::Action {
            label,
            value,
            shortcut: None,
            tone: ItemTone::Default,
            disabled: false,
        }
    }

    pub const fn label(text: &'static str) -> Self {
        Self::Label(text)
    }

    /// Display-only keyboard hint, e.g. `"⌘K"`. Does not bind the key.
    pub const fn shortcut(self, keys: &'static str) -> Self {
        match self {
            Self::Link {
                label, href, tone, ..
            } => Self::Link {
                label,
                href,
                shortcut: Some(keys),
                tone,
            },
            Self::Action {
                label,
                value,
                tone,
                disabled,
                ..
            } => Self::Action {
                label,
                value,
                shortcut: Some(keys),
                tone,
                disabled,
            },
            other => other,
        }
    }

    pub const fn danger(self) -> Self {
        match self {
            Self::Link {
                label,
                href,
                shortcut,
                ..
            } => Self::Link {
                label,
                href,
                shortcut,
                tone: ItemTone::Danger,
            },
            Self::Action {
                label,
                value,
                shortcut,
                disabled,
                ..
            } => Self::Action {
                label,
                value,
                shortcut,
                tone: ItemTone::Danger,
                disabled,
            },
            other => other,
        }
    }

    pub const fn disabled(self) -> Self {
        match self {
            Self::Action {
                label,
                value,
                shortcut,
                tone,
                ..
            } => Self::Action {
                label,
                value,
                shortcut,
                tone,
                disabled: true,
            },
            other => other,
        }
    }
}

/// Trigger button plus popover menu.
///
/// `id` must be unique on the page and a valid CSS identifier; it names the
/// popover, the trigger (`{id}-trigger`) and the CSS anchor (`--pb-{id}`).
#[derive(Debug, Clone, Copy)]
pub struct DropdownMenu {
    pub id: &'static str,
    pub label: &'static str,
    pub items: &'static [MenuItem],
    pub align: MenuAlign,
    pub class: Option<&'static str>,
    pub trigger_class: Option<&'static str>,
}

impl DropdownMenu {
    pub const fn new(id: &'static str, label: &'static str, items: &'static [MenuItem]) -> Self {
        Self {
            id,
            label,
            items,
            align: MenuAlign::Start,
            class: None,
            trigger_class: None,
        }
    }

    pub const fn align(mut self, align: MenuAlign) -> Self {
        self.align = align;
        self
    }

    pub const fn class(mut self, class: &'static str) -> Self {
        self.class = Some(class);
        self
    }

    pub const fn trigger_class(mut self, class: &'static str) -> Self {
        self.trigger_class = Some(class);
        self
    }
}

impl<L: DataLoader> WebRenderSync<L> for DropdownMenu {
    fn render(self, cx: &mut WebContext<L>) -> Result<(), WriteError> {
        let root = ClassList::<2>::new().add("pb-menu").add_opt(self.class);
        let trigger = ClassList::<2>::new()
            .add("pb-trigger")
            .add_opt(self.trigger_class);
        html_sync! {
            <div data-pb-menu class={root}>
                <button
                    type="button"
                    id={text!("{}-trigger", self.id)}
                    class={trigger}
                    data-pb-menu-trigger
                    popovertarget={self.id}
                    aria-haspopup="menu"
                    aria-controls={self.id}
                    aria-expanded="false"
                    style={style_ok(text!("anchor-name: --pb-{}", self.id))}
                >
                    {self.label}
                    <span class="pb-chevron">{icons::CHEVRON_DOWN}</span>
                </button>
                <div
                    id={self.id}
                    popover
                    role="menu"
                    aria-labelledby={text!("{}-trigger", self.id)}
                    data-pb-menu-content
                    data-align={self.align.as_str()}
                    class="pb-menu-content"
                    style={style_ok(text!("position-anchor: --pb-{}", self.id))}
                >
                    {for item in self.items.iter() {
                        html_sync! { {item} }
                    }}
                </div>
            </div>
        }
        .render(cx)
    }
}

struct Shortcut(Option<&'static str>);

impl<L: DataLoader> WebRenderSync<L> for Shortcut {
    fn render(self, cx: &mut WebContext<L>) -> Result<(), WriteError> {
        match self.0 {
            Some(keys) => html_sync! {
                <kbd class="pb-menu-shortcut">{keys}</kbd>
            }
            .render(cx),
            None => Ok(()),
        }
    }
}

impl<L: DataLoader> WebRenderSync<L> for &MenuItem {
    fn render(self, cx: &mut WebContext<L>) -> Result<(), WriteError> {
        match *self {
            MenuItem::Link {
                label,
                href,
                shortcut,
                tone,
            } => html_sync! {
                <a
                    role="menuitem"
                    tabindex="-1"
                    href={href}
                    class="pb-menu-item"
                    data-tone={tone.as_str()}
                >
                    {label}
                    {Shortcut(shortcut)}
                </a>
            }
            .render(cx),
            MenuItem::Action {
                label,
                value,
                shortcut,
                tone,
                disabled,
            } => html_sync! {
                <button
                    type="button"
                    role="menuitem"
                    tabindex="-1"
                    class="pb-menu-item"
                    data-value={value}
                    data-tone={tone.as_str()}
                    aria-disabled={disabled.then_some("true")}
                >
                    {label}
                    {Shortcut(shortcut)}
                </button>
            }
            .render(cx),
            MenuItem::Label(text) => html_sync! {
                <div role="presentation" class="pb-menu-label">{text}</div>
            }
            .render(cx),
            MenuItem::Separator => html_sync! {
                <div role="separator" class="pb-menu-separator"></div>
            }
            .render(cx),
        }
    }
}
