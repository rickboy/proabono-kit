// SPDX-License-Identifier: MIT OR Apache-2.0
use proa_core::{ClassList, DataLoader, WebContext, WebRenderSync, WriteError};
use proa_macros::{html_sync, text};

use crate::components::_proabono::icons;

/// One runnable entry.
#[derive(Debug, Clone, Copy)]
pub struct Command {
    pub label: &'static str,
    /// Sent as `detail.value` in `proabono:command`; ignored for links.
    pub value: &'static str,
    /// Navigate instead of firing an event.
    pub href: Option<&'static str>,
    /// Extra search terms that are not shown, e.g. `"settings preferences"`.
    pub keywords: &'static str,
    /// Display-only hint such as `"Ctrl+S"`. Does not bind the key.
    pub shortcut: Option<&'static str>,
}

impl Command {
    pub const fn action(label: &'static str, value: &'static str) -> Self {
        Self {
            label,
            value,
            href: None,
            keywords: "",
            shortcut: None,
        }
    }

    pub const fn link(label: &'static str, href: &'static str) -> Self {
        Self {
            label,
            value: "",
            href: Some(href),
            keywords: "",
            shortcut: None,
        }
    }

    pub const fn keywords(mut self, words: &'static str) -> Self {
        self.keywords = words;
        self
    }

    pub const fn shortcut(mut self, keys: &'static str) -> Self {
        self.shortcut = Some(keys);
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CommandGroup {
    pub heading: &'static str,
    pub commands: &'static [Command],
}

impl CommandGroup {
    pub const fn new(heading: &'static str, commands: &'static [Command]) -> Self {
        Self { heading, commands }
    }
}

/// Modal palette. `id` must be unique on the page.
#[derive(Debug, Clone, Copy)]
pub struct CommandPalette {
    pub id: &'static str,
    /// Accessible name for the dialog and its list.
    pub label: &'static str,
    pub groups: &'static [CommandGroup],
    pub placeholder: &'static str,
    pub empty_text: &'static str,
    /// Letter that opens the palette with Ctrl/Cmd; `None` disables the shortcut.
    pub hotkey: Option<&'static str>,
    pub class: Option<&'static str>,
}

impl CommandPalette {
    pub const fn new(id: &'static str, groups: &'static [CommandGroup]) -> Self {
        Self {
            id,
            label: "Command palette",
            groups,
            placeholder: "Type a command or search...",
            empty_text: "No results found.",
            hotkey: Some("k"),
            class: None,
        }
    }

    pub const fn label(mut self, label: &'static str) -> Self {
        self.label = label;
        self
    }

    pub const fn placeholder(mut self, text: &'static str) -> Self {
        self.placeholder = text;
        self
    }

    pub const fn empty_text(mut self, text: &'static str) -> Self {
        self.empty_text = text;
        self
    }

    pub const fn hotkey(mut self, key: Option<&'static str>) -> Self {
        self.hotkey = key;
        self
    }

    pub const fn class(mut self, class: &'static str) -> Self {
        self.class = Some(class);
        self
    }
}

/// Button that opens the palette with `id = target`.
#[derive(Debug, Clone, Copy)]
pub struct CommandTrigger {
    pub target: &'static str,
    pub label: &'static str,
    /// Display-only hint shown in the button, e.g. `"Ctrl K"`.
    pub hint: Option<&'static str>,
    pub class: Option<&'static str>,
}

impl CommandTrigger {
    pub const fn new(target: &'static str, label: &'static str) -> Self {
        Self {
            target,
            label,
            hint: Some("Ctrl K"),
            class: None,
        }
    }

    pub const fn hint(mut self, hint: Option<&'static str>) -> Self {
        self.hint = hint;
        self
    }

    pub const fn class(mut self, class: &'static str) -> Self {
        self.class = Some(class);
        self
    }
}

struct Kbd(Option<&'static str>);

impl<L: DataLoader> WebRenderSync<L> for Kbd {
    fn render(self, cx: &mut WebContext<L>) -> Result<(), WriteError> {
        match self.0 {
            Some(keys) => html_sync! {
                <kbd class="pb-kbd">{keys}</kbd>
            }.render(cx),
            None => Ok(()),
        }
    }
}

impl<L: DataLoader> WebRenderSync<L> for CommandTrigger {
    fn render(self, cx: &mut WebContext<L>) -> Result<(), WriteError> {
        let classes = ClassList::<2>::new()
            .add("pb-trigger pb-command-trigger")
            .add_opt(self.class);
        html_sync! {
            <button
                type="button"
                class={classes}
                commandfor={self.target}
                command="show-modal"
                data-pb-dialog-open
                aria-haspopup="dialog"
            >
                {icons::SEARCH}
                <span>{self.label}</span>
                {Kbd(self.hint)}
            </button>
        }
        .render(cx)
    }
}

/// One option. Ids are `{palette}-{group}-{index}` so they stay unique.
struct Option_<'a> {
    palette: &'static str,
    group: usize,
    index: usize,
    command: &'a Command,
}

impl<L: DataLoader> WebRenderSync<L> for Option_<'_> {
    fn render(self, cx: &mut WebContext<L>) -> Result<(), WriteError> {
        let c = self.command;
        let id = text!("{}-{}-{}", self.palette, self.group, self.index);
        match c.href {
            Some(href) => html_sync! {
                <a
                    id={id}
                    role="option"
                    aria-selected="false"
                    href={href}
                    class="pb-command-item"
                    data-label={c.label}
                    data-keywords={c.keywords}
                >
                    <span>{c.label}</span>
                    {Kbd(c.shortcut)}
                </a>
            }
            .render(cx),
            None => html_sync! {
                <div
                    id={id}
                    role="option"
                    aria-selected="false"
                    class="pb-command-item"
                    data-value={c.value}
                    data-label={c.label}
                    data-keywords={c.keywords}
                >
                    <span>{c.label}</span>
                    {Kbd(c.shortcut)}
                </div>
            }
            .render(cx),
        }
    }
}

/// Search box. Browser-only, so the Markdown view omits it.
pub(super) struct SearchBox {
    id: &'static str,
    placeholder: &'static str,
}

impl<L: DataLoader> WebRenderSync<L> for SearchBox {
    fn render(self, cx: &mut WebContext<L>) -> Result<(), WriteError> {
        html_sync! {
            <div class="pb-command-search">
                {icons::SEARCH}
                <input
                    type="text"
                    class="pb-command-input"
                    data-pb-command-input
                    role="combobox"
                    aria-expanded="true"
                    aria-autocomplete="list"
                    aria-controls={text!("{}-list", self.id)}
                    aria-label="Search commands"
                    placeholder={self.placeholder}
                    autocomplete="off"
                    spellcheck="false"
                    autofocus
                />
                <kbd class="pb-kbd">"Esc"</kbd>
            </div>
        }
        .render(cx)
    }
}

impl<L: DataLoader> WebRenderSync<L> for CommandPalette {
    fn render(self, cx: &mut WebContext<L>) -> Result<(), WriteError> {
        let classes = ClassList::<2>::new().add("pb-command").add_opt(self.class);
        let id = self.id;
        html_sync! {
            <dialog
                id={id}
                class={classes}
                data-pb-command
                data-hotkey={self.hotkey}
                closedby="any"
                aria-label={self.label}
            >
                {SearchBox { id, placeholder: self.placeholder }}
                <div
                    id={text!("{}-list", id)}
                    role="listbox"
                    aria-label={self.label}
                    class="pb-command-list"
                >
                    {for (g, group) in self.groups.iter().enumerate() {
                        html_sync! {
                            <div
                                role="group"
                                aria-labelledby={text!("{}-g{}", id, g)}
                                class="pb-command-group"
                                data-pb-command-group
                            >
                                <div id={text!("{}-g{}", id, g)} class="pb-command-heading" role="presentation">
                                    {group.heading}
                                </div>
                                {for (i, command) in group.commands.iter().enumerate() {
                                    html_sync! { {Option_ { palette: id, group: g, index: i, command }} }
                                }}
                            </div>
                        }
                    }}
                    <p class="pb-command-empty" data-pb-command-empty hidden>{self.empty_text}</p>
                </div>
            </dialog>
        }
        .render(cx)
    }
}
