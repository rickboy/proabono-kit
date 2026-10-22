// SPDX-License-Identifier: MIT OR Apache-2.0
use proa_core::{ClassList, DataLoader, WebContext, WebRenderSync, WriteError};
use proa_macros::html_sync;

use crate::components::_proabono::icons;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ComboOption {
    pub value: &'static str,
    pub label: &'static str,
}

impl ComboOption {
    pub const fn new(value: &'static str, label: &'static str) -> Self {
        Self { value, label }
    }
}

/// Searchable single select.
///
/// `id` must be unique on the page; it prefixes the listbox
/// (`{id}-listbox`), options (`{id}-option-N`) and fallback datalist.
#[derive(Debug, Clone, Copy)]
pub struct Combobox {
    pub id: &'static str,
    pub name: &'static str,
    pub label: &'static str,
    pub options: &'static [ComboOption],
    /// Initially selected option value.
    pub value: Option<&'static str>,
    pub placeholder: Option<&'static str>,
    pub empty_text: &'static str,
    pub required: bool,
    /// Clear free text that matches no option when the field loses focus.
    pub strict: bool,
    pub class: Option<&'static str>,
}

impl Combobox {
    pub const fn new(
        id: &'static str,
        name: &'static str,
        label: &'static str,
        options: &'static [ComboOption],
    ) -> Self {
        Self {
            id,
            name,
            label,
            options,
            value: None,
            placeholder: None,
            empty_text: "No results.",
            required: false,
            strict: true,
            class: None,
        }
    }

    pub const fn value(mut self, value: &'static str) -> Self {
        self.value = Some(value);
        self
    }

    pub const fn placeholder(mut self, text: &'static str) -> Self {
        self.placeholder = Some(text);
        self
    }

    pub const fn empty_text(mut self, text: &'static str) -> Self {
        self.empty_text = text;
        self
    }

    pub const fn required(mut self) -> Self {
        self.required = true;
        self
    }

    pub const fn allow_free_text(mut self) -> Self {
        self.strict = false;
        self
    }

    pub const fn class(mut self, class: &'static str) -> Self {
        self.class = Some(class);
        self
    }

    /// Map a submitted value back to an option. Accepts the option value
    /// (JavaScript path) or its label, case-insensitively (no-JS path).
    pub fn resolve(options: &'static [ComboOption], submitted: &str) -> Option<&'static ComboOption> {
        let submitted = submitted.trim();
        options.iter().find(|o| o.value == submitted).or_else(|| {
            options
                .iter()
                .find(|o| o.label.eq_ignore_ascii_case(submitted))
        })
    }

    fn selected(&self) -> Option<&'static ComboOption> {
        let value = self.value?;
        self.options.iter().find(|o| o.value == value)
    }
}

impl<L: DataLoader> WebRenderSync<L> for Combobox {
    fn render(self, cx: &mut WebContext<L>) -> Result<(), WriteError> {
        let root = ClassList::<2>::new().add("pb-combobox").add_opt(self.class);
        let selected = self.selected();
        let shown = selected.map(|o| o.label).unwrap_or("");
        let value = selected.map(|o| o.value).unwrap_or("");
        let id = self.id;
        html_sync! {
            <div data-pb-combobox data-strict={self.strict.then_some("")} class={root}>
                <label for={id} class="pb-label">{self.label}</label>
                <div class="pb-combobox-control">
                    <input
                        id={id}
                        name={self.name}
                        type="text"
                        class="pb-input"
                        data-pb-combobox-input
                        list={text!("{}-datalist", id)}
                        aria-controls={text!("{}-listbox", id)}
                        autocomplete="off"
                        spellcheck="false"
                        placeholder={self.placeholder}
                        value={shown}
                        required={self.required.then_some("")}
                    />
                    <span class="pb-combobox-icon">{icons::CHEVRONS_UP_DOWN}</span>
                    <input type="hidden" data-pb-combobox-value value={value} />
                    <datalist id={text!("{}-datalist", id)}>
                        {for option in self.options.iter() {
                            html_sync! { <option value={option.label}></option> }
                        }}
                    </datalist>
                    <ul
                        id={text!("{}-listbox", id)}
                        role="listbox"
                        aria-label={self.label}
                        class="pb-listbox"
                        hidden
                    >
                        {for (i, option) in self.options.iter().enumerate() {
                            let is_selected = if Some(option.value) == self.value { "true" } else { "false" };
                            html_sync! {
                                <li
                                    id={text!("{}-option-{}", id, i)}
                                    role="option"
                                    class="pb-option"
                                    aria-selected={is_selected}
                                    data-value={option.value}
                                    data-label={option.label}
                                >
                                    {icons::CHECK}
                                    {option.label}
                                </li>
                            }
                        }}
                        <li role="presentation" class="pb-empty" data-pb-combobox-empty hidden>
                            {self.empty_text}
                        </li>
                    </ul>
                </div>
            </div>
        }
        .render(cx)
    }
}
