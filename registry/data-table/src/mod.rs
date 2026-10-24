// SPDX-License-Identifier: MIT OR Apache-2.0
//! Data table: typed columns over a slice of rows, rendered on the server.
//!
//! Without JavaScript it is a plain accessible `<table>` with a caption and
//! column headers. `proabono.js` adds click-to-sort headers (`aria-sort`), a
//! text filter, a live row count and client-side paging (`page_size`). For
//! large data sets, sort and page on the server and pair the table with the
//! `pagination` component instead.
//!
//! ```ignore
//! const COLUMNS: [Column<Call>; 3] = [
//!     Column::new("Call", |c: &Call| Cell::Text(c.id)),
//!     Column::new("District", |c: &Call| Cell::Text(c.district)).sortable(),
//!     Column::new("Payout", |c: &Call| Cell::Sorted { text: c.payout, key: c.payout_cr }).sortable().numeric(),
//! ];
//! DataTable::new("calls", "Night shift calls", &COLUMNS, &calls).filter().page_size(10)
//! ```
#![allow(dead_code, unused_imports)]

mod data_table;
mod md;

pub use data_table::{Cell, Column, DataTable, SortDirection};
