// SPDX-License-Identifier: MIT OR Apache-2.0
//! Attribute helpers.

use proa_core::shared::attr_value::AttrValue;
use proa_core::{DataLoader, WebContext, WriteBuf, WriteError};

/// Optional attribute for any value type: omitted when `None`.
/// (`proa_core` covers `Option<&str>`; this extends it to numbers.)
#[derive(Debug, Clone, Copy)]
pub struct Maybe<T>(pub Option<T>);

impl<T: AttrValue> AttrValue for Maybe<T> {
    fn should_render(&self) -> bool {
        self.0.as_ref().is_some_and(AttrValue::should_render)
    }

    fn url_str(&self) -> Option<&str> {
        self.0.as_ref().and_then(AttrValue::url_str)
    }

    fn render_attr_value<B: WriteBuf, L: DataLoader>(
        &self,
        cx: &mut WebContext<L, B>,
    ) -> Result<(), WriteError> {
        match &self.0 {
            Some(value) => value.render_attr_value(cx),
            None => Ok(()),
        }
    }
}
