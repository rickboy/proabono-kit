// SPDX-License-Identifier: MIT OR Apache-2.0
use proa_core::{ClassList, DataLoader, WebContext, WebRenderSync, WriteError};
use proa_macros::{html_sync, text};

use crate::components::_proabono::attr::Maybe;
use crate::components::_proabono::icons;

/// What one table cell shows.
#[derive(Debug, Clone, Copy)]
pub enum Cell<'a> {
    Text(&'a str),
    Int(i64),
    /// Preformatted text that sorts by `key` (money, dates, durations).
    Sorted { text: &'a str, key: i64 },
    Link { text: &'a str, href: &'a str },
    Empty,
}

impl Cell<'_> {
    /// Numeric sort key exposed as `data-sort`; text cells sort by content.
    pub(super) fn sort_key(&self) -> Option<i64> {
        match *self {
            Cell::Int(n) => Some(n),
            Cell::Sorted { key, .. } => Some(key),
            _ => None,
        }
    }
}

impl<L: DataLoader> WebRenderSync<L> for Cell<'_> {
    fn render(self, cx: &mut WebContext<L>) -> Result<(), WriteError> {
        match self {
            Cell::Text(text) | Cell::Sorted { text, .. } => html_sync! {
                {text}
            }.render(cx),
            Cell::Int(n) => html_sync! {
                {n}
            }.render(cx),
            Cell::Link { text, href } => html_sync! {
                <a href={href}>{text}</a>
            }.render(cx),
            Cell::Empty => Ok(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortDirection {
    Ascending,
    Descending,
}

impl SortDirection {
    pub const fn aria(self) -> &'static str {
        match self {
            Self::Ascending => "ascending",
            Self::Descending => "descending",
        }
    }
}

/// One column: a header and a function that reads its cell from a row.
pub struct Column<R> {
    pub header: &'static str,
    pub cell: for<'r> fn(&'r R) -> Cell<'r>,
    pub sortable: bool,
    /// Right-aligned, tabular figures, numeric sort.
    pub numeric: bool,
}

impl<R> Column<R> {
    pub const fn new(header: &'static str, cell: for<'r> fn(&'r R) -> Cell<'r>) -> Self {
        Self {
            header,
            cell,
            sortable: false,
            numeric: false,
        }
    }

    pub const fn sortable(mut self) -> Self {
        self.sortable = true;
        self
    }

    pub const fn numeric(mut self) -> Self {
        self.numeric = true;
        self
    }
}

pub struct DataTable<'a, R> {
    /// Unique on the page; prefixes the caption id.
    pub id: &'static str,
    pub caption: &'static str,
    /// Show the caption visibly above the table (otherwise screen-reader only).
    pub show_caption: bool,
    pub columns: &'a [Column<R>],
    pub rows: &'a [R],
    /// Render the filter box (needs `proabono.js`).
    pub filter: bool,
    /// Rows per page in the browser; `0` shows every row.
    pub page_size: u32,
    /// Column the rows arrive sorted by, for `aria-sort`.
    pub sorted_by: Option<(usize, SortDirection)>,
    pub empty_text: &'static str,
    pub class: Option<&'static str>,
}

impl<'a, R> DataTable<'a, R> {
    pub const fn new(
        id: &'static str,
        caption: &'static str,
        columns: &'a [Column<R>],
        rows: &'a [R],
    ) -> Self {
        Self {
            id,
            caption,
            show_caption: false,
            columns,
            rows,
            filter: false,
            page_size: 0,
            sorted_by: None,
            empty_text: "No results.",
            class: None,
        }
    }

    pub const fn show_caption(mut self) -> Self {
        self.show_caption = true;
        self
    }

    pub const fn filter(mut self) -> Self {
        self.filter = true;
        self
    }

    pub const fn page_size(mut self, rows: u32) -> Self {
        self.page_size = rows;
        self
    }

    pub const fn sorted_by(mut self, column: usize, direction: SortDirection) -> Self {
        self.sorted_by = Some((column, direction));
        self
    }

    pub const fn empty_text(mut self, text: &'static str) -> Self {
        self.empty_text = text;
        self
    }

    pub const fn class(mut self, class: &'static str) -> Self {
        self.class = Some(class);
        self
    }
}

/// Filter box and live count. Browser-only, so the Markdown view omits it.
pub(super) struct Toolbar {
    caption: &'static str,
}

impl<L: DataLoader> WebRenderSync<L> for Toolbar {
    fn render(self, cx: &mut WebContext<L>) -> Result<(), WriteError> {
        html_sync! {
            <div class="pb-table-toolbar">
                <label class="pb-table-filter">
                    {icons::SEARCH}
                    <span class="pb-sr-only">{text!("Filter {}", self.caption)}</span>
                    <input
                        type="search"
                        class="pb-input"
                        data-pb-table-filter
                        placeholder="Filter..."
                        autocomplete="off"
                    />
                </label>
                <p class="pb-table-count" data-pb-table-count aria-live="polite"></p>
            </div>
        }
        .render(cx)
    }
}

struct HeaderCell {
    header: &'static str,
    index: usize,
    sortable: bool,
    numeric: bool,
    sort: Option<&'static str>,
}

impl<L: DataLoader> WebRenderSync<L> for HeaderCell {
    fn render(self, cx: &mut WebContext<L>) -> Result<(), WriteError> {
        let numeric = self.numeric.then_some("");
        if self.sortable {
            html_sync! {
                <th scope="col" aria-sort={self.sort} data-numeric={numeric}>
                    <button type="button" class="pb-table-sort" data-pb-sort={self.index}>
                        {self.header}
                        {icons::ARROW_UP_DOWN}
                    </button>
                </th>
            }
            .render(cx)
        } else {
            html_sync! {
                <th scope="col" data-numeric={numeric}>{self.header}</th>
            }
            .render(cx)
        }
    }
}

struct Row<'a, R> {
    columns: &'a [Column<R>],
    row: &'a R,
}

impl<L: DataLoader, R> WebRenderSync<L> for Row<'_, R> {
    fn render(self, cx: &mut WebContext<L>) -> Result<(), WriteError> {
        let row = self.row;
        html_sync! {
            <tr>
                {for column in self.columns.iter() {
                    let cell = (column.cell)(row);
                    let numeric = column.numeric.then_some("");
                    html_sync! {
                        <td data-sort={Maybe(cell.sort_key())} data-numeric={numeric}>{cell}</td>
                    }
                }}
            </tr>
        }
        .render(cx)
    }
}

impl<L: DataLoader, R> WebRenderSync<L> for DataTable<'_, R> {
    fn render(self, cx: &mut WebContext<L>) -> Result<(), WriteError> {
        let root = ClassList::<2>::new().add("pb-table").add_opt(self.class);
        let caption_class = if self.show_caption {
            "pb-table-caption"
        } else {
            "pb-sr-only"
        };
        let page_size = (self.page_size > 0).then_some(self.page_size);
        let empty_hidden = (!self.rows.is_empty()).then_some("");
        let sorted_by = self.sorted_by;
        html_sync! {
            <div id={self.id} class={root} data-pb-table data-page-size={Maybe(page_size)}>
                {match self.filter {
                    true => html_sync! { {Toolbar { caption: self.caption }} },
                    false => html_sync! { <></> },
                }}
                <div class="pb-table-scroll">
                    <table>
                        <caption class={caption_class}>{self.caption}</caption>
                        <thead>
                            <tr>
                                {for (index, column) in self.columns.iter().enumerate() {
                                    let sort = match sorted_by {
                                        Some((i, direction)) if i == index => Some(direction.aria()),
                                        _ => None,
                                    };
                                    html_sync! {
                                        {HeaderCell {
                                            header: column.header,
                                            index,
                                            sortable: column.sortable,
                                            numeric: column.numeric,
                                            sort,
                                        }}
                                    }
                                }}
                            </tr>
                        </thead>
                        <tbody>
                            {for row in self.rows.iter() {
                                html_sync! { {Row { columns: self.columns, row }} }
                            }}
                        </tbody>
                    </table>
                </div>
                <p class="pb-table-empty" data-pb-table-empty hidden={empty_hidden}>
                    {self.empty_text}
                </p>
                <nav
                    class="pb-pagination"
                    data-pb-table-pager
                    aria-label={text!("{} pages", self.caption)}
                    hidden
                ></nav>
            </div>
        }
        .render(cx)
    }
}
