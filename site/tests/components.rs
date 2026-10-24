//! Render registry components directly and check their output.

#[path = "kit/mod.rs"]
mod components;

use components::data_table::{Cell, Column, DataTable};
use components::pagination::Pagination;
use proa_core::{MdRenderSync, WebContext, WebRenderSync};

fn html(component: impl WebRenderSync) -> String {
    let mut cx = WebContext::new();
    component.render(&mut cx).unwrap();
    cx.into_output().into_string()
}

fn markdown(component: impl MdRenderSync) -> String {
    let mut cx = WebContext::new();
    component.render_md(&mut cx).unwrap();
    cx.into_output().into_string()
}

struct Row {
    name: &'static str,
    score: i64,
}

const ROWS: [Row; 2] = [
    Row { name: "Juno", score: 7 },
    Row { name: "Pax", score: 12 },
];

const COLUMNS: [Column<Row>; 2] = [
    Column::new("Name", |r: &Row| Cell::Text(r.name)),
    Column::new("Score", |r: &Row| Cell::Int(r.score)).numeric(),
];

#[test]
fn data_table_markdown_is_a_table() {
    let md = markdown(DataTable::new("crew", "Crew", &COLUMNS, &ROWS));
    assert!(md.contains("| Name | Score |"), "{md}");
    assert!(md.contains("| --- | ---: |"), "{md}");
    assert!(md.contains("| Juno | 7 |"), "{md}");
    assert!(md.contains("| Pax | 12 |"), "{md}");
}

#[test]
fn data_table_html_has_sort_keys_and_page_size() {
    let out = html(DataTable::new("crew", "Crew", &COLUMNS, &ROWS).page_size(10));
    assert!(out.contains(r#"data-page-size="10""#), "{out}");
    assert!(out.contains(r#"data-sort="12""#), "{out}");
    // Text cells carry no sort key.
    assert!(!out.contains(r#"data-sort="Juno""#), "{out}");
}

fn page_labels(out: &str) -> Vec<String> {
    // Each page link ends `>N</a>`; gaps render as `...`.
    let mut labels = Vec::new();
    for part in out.split("<li") {
        if part.contains("pb-page-gap") {
            labels.push("...".into());
        } else if let Some(end) = part.find("</a>") {
            let text = &part[..end];
            let label = text.rsplit('>').next().unwrap_or("");
            if label.chars().all(|c| c.is_ascii_digit()) && !label.is_empty() {
                labels.push(label.into());
            }
        }
    }
    labels
}

#[test]
fn pagination_windows() {
    let cases: [(u32, u32, &[&str]); 5] = [
        (1, 20, &["1", "2", "3", "4", "5", "...", "20"]),
        (10, 20, &["1", "...", "9", "10", "11", "...", "20"]),
        (19, 20, &["1", "...", "16", "17", "18", "19", "20"]),
        (2, 4, &["1", "2", "3", "4"]),
        (99, 3, &["1", "2", "3"]),
    ];
    for (page, total, expected) in cases {
        let out = html(Pagination::new(page, total, "/list?page="));
        assert_eq!(page_labels(&out), expected, "page {page} of {total}: {out}");
    }
}

#[test]
fn pagination_marks_current_and_disables_ends() {
    let out = html(Pagination::new(1, 3, "/list?page="));
    assert!(out.contains(r#"href="/list?page=1" aria-current="page""#), "{out}");
    assert!(out.contains(r#"aria-disabled="true" data-step="prev""#), "{out}");
    assert!(out.contains(r#"href="/list?page=2" rel="next""#), "{out}");
}
