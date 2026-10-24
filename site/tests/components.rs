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

// ---------------------------------------------------------------------------
// Batch 2: menubar, context menu, command palette
// ---------------------------------------------------------------------------

use components::command_palette::{Command, CommandGroup, CommandPalette};
use components::context_menu::ContextMenu;
use components::dropdown_menu::MenuItem;
use components::menubar::{Menubar, MenubarMenu};

const FILE: [MenuItem; 2] = [
    MenuItem::action("New", "new"),
    MenuItem::link("Docs", "/docs"),
];
const EDIT: [MenuItem; 1] = [MenuItem::action("Undo", "undo")];
const MENUS: [MenubarMenu; 2] = [
    MenubarMenu::new("m-file", "File", &FILE),
    MenubarMenu::new("m-edit", "Edit", &EDIT),
];

#[test]
fn menubar_wires_each_trigger_to_its_menu() {
    let out = html(Menubar::new("App", &MENUS));
    assert!(out.contains(r#"role="menubar" aria-label="App""#), "{out}");
    for id in ["m-file", "m-edit"] {
        assert!(out.contains(&format!(r#"id="{id}-trigger""#)), "{out}");
        assert!(out.contains(&format!(r#"popovertarget="{id}""#)), "{out}");
        assert!(out.contains(&format!(r#"aria-labelledby="{id}-trigger""#)), "{out}");
    }
    // Every trigger stays tabbable until the runtime installs roving focus.
    assert!(!out.contains("tabindex=\"-1\" class=\"pb-menubar-trigger\""), "{out}");
    let md = markdown(Menubar::new("App", &MENUS));
    assert!(md.contains("**File**") && md.contains("[Docs](/docs)") && md.contains("Undo"), "{md}");
}

struct Card;

impl<L: proa_core::DataLoader> WebRenderSync<L> for Card {
    fn render(self, cx: &mut WebContext<L>) -> Result<(), proa_core::WriteError> {
        proa_macros::html_sync! {
            <p>"Juno"</p>
        }.render(cx)
    }
}

impl<L: proa_core::DataLoader, B: proa_core::WriteBuf> MdRenderSync<L, B> for Card {
    fn render_md(self, cx: &mut WebContext<L, B>) -> Result<(), proa_core::WriteError> {
        proa_macros::md_sync! { "Juno" }.render_md(cx)
    }
}

#[test]
fn context_menu_targets_its_menu_and_markdown_keeps_only_content() {
    let out = html(ContextMenu::new("ctx", "Crew actions", &FILE, Card));
    assert!(out.contains(r#"data-pb-context-target="ctx" tabindex="0""#), "{out}");
    assert!(out.contains(r#"id="ctx" popover role="menu" aria-label="Crew actions""#), "{out}");
    let quiet = html(ContextMenu::new("ctx", "Crew actions", &FILE, Card).not_focusable());
    assert!(!quiet.contains("tabindex=\"0\""), "{quiet}");
    let md = markdown(ContextMenu::new("ctx", "Crew actions", &FILE, Card));
    assert!(md.contains("Juno") && !md.contains("New"), "{md}");
}

const NAV: [Command; 2] = [
    Command::link("Home", "/"),
    Command::link("Devlog", "/devlog").keywords("blog news"),
];
const ACTIONS: [Command; 1] = [Command::action("Toggle theme", "theme").shortcut("Ctrl+T")];
const GROUPS: [CommandGroup; 2] = [
    CommandGroup::new("Go to", &NAV),
    CommandGroup::new("Actions", &ACTIONS),
];

#[test]
fn command_palette_options_are_unique_and_typed() {
    let out = html(CommandPalette::new("cmd", &GROUPS));
    for id in ["cmd-0-0", "cmd-0-1", "cmd-1-0"] {
        assert_eq!(out.matches(&format!(r#"id="{id}""#)).count(), 1, "{id} in {out}");
    }
    assert!(out.contains(r#"data-hotkey="k""#), "{out}");
    assert!(out.contains(r#"href="/devlog""#) && out.contains(r#"data-keywords="blog news""#), "{out}");
    assert!(out.contains(r#"data-value="theme""#), "{out}");
    assert_eq!(out.matches(r#"role="option""#).count(), 3, "{out}");
    assert_eq!(out.matches(r#"aria-selected="false""#).count(), 3, "{out}");
    let no_hotkey = html(CommandPalette::new("cmd", &GROUPS).hotkey(None));
    assert!(!no_hotkey.contains("data-hotkey"), "{no_hotkey}");
    let md = markdown(CommandPalette::new("cmd", &GROUPS));
    assert!(md.contains("**Go to**") && md.contains("- [Devlog](/devlog)") && md.contains("- Toggle theme"), "{md}");
}
