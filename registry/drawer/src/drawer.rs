// SPDX-License-Identifier: MIT OR Apache-2.0
use proa_core::{ClassList, DataLoader, WebContext, WebRenderSync, WriteError};
use proa_macros::html_sync;

use crate::components::_proabono::icons;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DrawerSide {
    #[default]
    Right,
    Left,
    Bottom,
    Top,
}

impl DrawerSide {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Right => "right",
            Self::Left => "left",
            Self::Bottom => "bottom",
            Self::Top => "top",
        }
    }
}

/// Button that opens the drawer with `id = target`. Render it anywhere.
#[derive(Debug, Clone, Copy)]
pub struct DrawerTrigger {
    pub target: &'static str,
    pub label: &'static str,
    pub class: Option<&'static str>,
}

impl DrawerTrigger {
    pub const fn new(target: &'static str, label: &'static str) -> Self {
        Self {
            target,
            label,
            class: None,
        }
    }

    pub const fn class(mut self, class: &'static str) -> Self {
        self.class = Some(class);
        self
    }
}

impl<L: DataLoader> WebRenderSync<L> for DrawerTrigger {
    fn render(self, cx: &mut WebContext<L>) -> Result<(), WriteError> {
        let classes = ClassList::<2>::new().add("pb-trigger").add_opt(self.class);
        html_sync! {
            <button
                type="button"
                class={classes}
                commandfor={self.target}
                command="show-modal"
                data-pb-drawer-open
                aria-haspopup="dialog"
            >
                {self.label}
            </button>
        }
        .render(cx)
    }
}

/// Button that closes the surrounding drawer. Place it inside the body.
#[derive(Debug, Clone, Copy)]
pub struct DrawerClose {
    pub label: &'static str,
    pub value: Option<&'static str>,
    pub class: Option<&'static str>,
}

impl DrawerClose {
    pub const fn new(label: &'static str) -> Self {
        Self {
            label,
            value: None,
            class: None,
        }
    }

    /// Becomes the dialog's `returnValue`.
    pub const fn value(mut self, value: &'static str) -> Self {
        self.value = Some(value);
        self
    }

    pub const fn class(mut self, class: &'static str) -> Self {
        self.class = Some(class);
        self
    }
}

impl<L: DataLoader> WebRenderSync<L> for DrawerClose {
    fn render(self, cx: &mut WebContext<L>) -> Result<(), WriteError> {
        let classes = ClassList::<2>::new().add("pb-trigger").add_opt(self.class);
        html_sync! {
            <form method="dialog" style="display:contents">
                <button type="submit" class={classes} value={self.value}>{self.label}</button>
            </form>
        }
        .render(cx)
    }
}

/// Modal side panel. `body` is any component; put a [`DrawerClose`] or
/// your own `<form method="dialog">` inside it for footer actions.
#[derive(Debug, Clone, Copy)]
pub struct Drawer<Body> {
    pub id: &'static str,
    pub title: &'static str,
    pub description: Option<&'static str>,
    pub side: DrawerSide,
    pub body: Body,
    pub class: Option<&'static str>,
}

impl<Body> Drawer<Body> {
    pub const fn new(id: &'static str, title: &'static str, body: Body) -> Self {
        Self {
            id,
            title,
            description: None,
            side: DrawerSide::Right,
            body,
            class: None,
        }
    }

    pub const fn description(mut self, text: &'static str) -> Self {
        self.description = Some(text);
        self
    }

    pub const fn side(mut self, side: DrawerSide) -> Self {
        self.side = side;
        self
    }

    pub const fn class(mut self, class: &'static str) -> Self {
        self.class = Some(class);
        self
    }
}

struct Description {
    id: &'static str,
    text: Option<&'static str>,
}

impl<L: DataLoader> WebRenderSync<L> for Description {
    fn render(self, cx: &mut WebContext<L>) -> Result<(), WriteError> {
        match self.text {
            Some(text) => html_sync! {
                <p id={text!("{}-description", self.id)} class="pb-drawer-description">{text}</p>
            }
            .render(cx),
            None => Ok(()),
        }
    }
}

struct Handle(DrawerSide);

impl<L: DataLoader> WebRenderSync<L> for Handle {
    fn render(self, cx: &mut WebContext<L>) -> Result<(), WriteError> {
        match self.0 {
            DrawerSide::Bottom => html_sync! {
                <div aria-hidden="true" class="pb-drawer-handle"></div>
            }
            .render(cx),
            _ => Ok(()),
        }
    }
}

impl<L: DataLoader, Body: WebRenderSync<L>> WebRenderSync<L> for Drawer<Body> {
    fn render(self, cx: &mut WebContext<L>) -> Result<(), WriteError> {
        let classes = ClassList::<2>::new().add("pb-drawer").add_opt(self.class);
        let id = self.id;
        html_sync! {
            <dialog
                id={id}
                class={classes}
                data-pb-drawer
                data-side={self.side.as_str()}
                closedby="any"
                aria-labelledby={text!("{}-title", id)}
            >
                {Handle(self.side)}
                <header class="pb-drawer-header">
                    <h2 id={text!("{}-title", id)} class="pb-drawer-title">{self.title}</h2>
                    {Description { id, text: self.description }}
                </header>
                <form method="dialog" class="pb-drawer-close">
                    <button type="submit" class="pb-icon-button" aria-label="Close">{icons::X}</button>
                </form>
                <div class="pb-drawer-body">{self.body}</div>
            </dialog>
        }
        .render(cx)
    }
}
