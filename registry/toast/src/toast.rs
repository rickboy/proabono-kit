// SPDX-License-Identifier: MIT OR Apache-2.0
use proa_core::{DataLoader, StaticRaw, WebContext, WebRenderSync, WriteError};
use proa_macros::html_sync;

use crate::components::_proabono::icons;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToastTone {
    #[default]
    Default,
    Info,
    Success,
    Warning,
    Error,
}

impl ToastTone {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Info => "info",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }

    const fn icon(self) -> StaticRaw {
        match self {
            Self::Default => icons::CIRCLE_ALERT,
            Self::Info => icons::INFO,
            Self::Success => icons::CIRCLE_CHECK,
            Self::Warning => icons::TRIANGLE_ALERT,
            Self::Error => icons::CIRCLE_X,
        }
    }

    /// Errors interrupt (`alert`); everything else waits politely (`status`).
    const fn role(self) -> &'static str {
        match self {
            Self::Error => "alert",
            _ => "status",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToastPosition {
    #[default]
    BottomRight,
    BottomLeft,
    BottomCenter,
    TopRight,
    TopLeft,
    TopCenter,
}

impl ToastPosition {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BottomRight => "bottom-right",
            Self::BottomLeft => "bottom-left",
            Self::BottomCenter => "bottom-center",
            Self::TopRight => "top-right",
            Self::TopLeft => "top-left",
            Self::TopCenter => "top-center",
        }
    }
}

/// One notification. Borrowed strings, so request data (flash messages)
/// works without leaking.
#[derive(Debug, Clone, Copy)]
pub struct Toast<'a> {
    pub title: &'a str,
    pub description: Option<&'a str>,
    pub tone: ToastTone,
    /// Auto-dismiss delay in milliseconds; `0` keeps it until closed.
    pub duration_ms: u32,
}

impl<'a> Toast<'a> {
    pub const fn new(title: &'a str) -> Self {
        Self {
            title,
            description: None,
            tone: ToastTone::Default,
            duration_ms: 4000,
        }
    }

    pub const fn description(mut self, text: &'a str) -> Self {
        self.description = Some(text);
        self
    }

    pub const fn tone(mut self, tone: ToastTone) -> Self {
        self.tone = tone;
        self
    }

    pub const fn duration_ms(mut self, ms: u32) -> Self {
        self.duration_ms = ms;
        self
    }
}

/// Fixed-position region that holds the toast stack.
#[derive(Debug, Clone, Copy, Default)]
pub struct Toaster<'a> {
    pub position: ToastPosition,
    pub toasts: &'a [Toast<'a>],
}

impl<'a> Toaster<'a> {
    pub const fn new(toasts: &'a [Toast<'a>]) -> Self {
        Self {
            position: ToastPosition::BottomRight,
            toasts,
        }
    }

    pub const fn position(mut self, position: ToastPosition) -> Self {
        self.position = position;
        self
    }
}

struct Description<'a>(Option<&'a str>);

impl<L: DataLoader> WebRenderSync<L> for Description<'_> {
    fn render(self, cx: &mut WebContext<L>) -> Result<(), WriteError> {
        match self.0 {
            Some(text) => html_sync! {
                <p class="pb-toast-description">{text}</p>
            }
            .render(cx),
            None => Ok(()),
        }
    }
}

impl<L: DataLoader> WebRenderSync<L> for &Toast<'_> {
    fn render(self, cx: &mut WebContext<L>) -> Result<(), WriteError> {
        html_sync! {
            <li
                class="pb-toast"
                role={self.tone.role()}
                data-tone={self.tone.as_str()}
                data-duration={self.duration_ms}
            >
                <span class="pb-toast-icon">{self.tone.icon()}</span>
                <div>
                    <p class="pb-toast-title">{self.title}</p>
                    {Description(self.description)}
                </div>
                <button type="button" class="pb-icon-button" data-pb-toast-close aria-label="Dismiss notification">
                    {icons::X}
                </button>
            </li>
        }
        .render(cx)
    }
}

impl<L: DataLoader> WebRenderSync<L> for Toaster<'_> {
    fn render(self, cx: &mut WebContext<L>) -> Result<(), WriteError> {
        let count = self.toasts.len();
        html_sync! {
            <section
                class="pb-toaster"
                data-pb-toaster
                data-position={self.position.as_str()}
                aria-label={text!("Notifications ({})", count)}
            >
                <ol class="pb-toast-list" aria-live="polite">
                    {for toast in self.toasts.iter() {
                        html_sync! { {toast} }
                    }}
                </ol>
            </section>
        }
        .render(cx)
    }
}
