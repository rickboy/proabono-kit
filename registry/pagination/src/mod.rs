// SPDX-License-Identifier: MIT OR Apache-2.0
//! Pagination: links to numbered pages, rendered entirely on the server.
//!
//! Each link is `href` with the page number appended, so pass a prefix such
//! as `"/devlog?page="` or `"/devlog/page/"`. Pages are 1-based. Long ranges
//! collapse to `1 … 4 5 6 … 20` around the current page.
#![allow(dead_code, unused_imports)]

mod md;
mod pagination;

pub use pagination::{PageHref, Pagination};
