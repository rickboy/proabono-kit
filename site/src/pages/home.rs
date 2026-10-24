use proa_core::{DataLoader, StaticRaw, WebContext, WebRenderSync, WriteError};
use proa_macros::html_sync;

use crate::components::combobox::{ComboOption, Combobox};
use crate::components::data_table::{Cell, Column, DataTable, SortDirection};
use crate::components::pagination::Pagination;
use crate::components::drawer::{Drawer, DrawerClose, DrawerSide, DrawerTrigger};
use crate::components::dropdown_menu::{DropdownMenu, MenuAlign, MenuItem};
use crate::components::toast::{Toast, ToastTone, Toaster};

const ACCOUNT_MENU: [MenuItem; 7] = [
    MenuItem::label("My account"),
    MenuItem::link("Profile", "#profile").shortcut("⇧P"),
    MenuItem::link("Billing", "#billing").shortcut("⌘B"),
    MenuItem::action("Keyboard shortcuts", "shortcuts").shortcut("?"),
    MenuItem::action("Invite users", "invite").disabled(),
    MenuItem::SEPARATOR,
    MenuItem::action("Log out", "logout").danger(),
];

const THEME_MENU: [MenuItem; 3] = [
    MenuItem::action("System", "system"),
    MenuItem::action("Light", "light"),
    MenuItem::action("Dark", "dark"),
];

const ROLE_MENU: [MenuItem; 4] = [
    MenuItem::action("Pilot", "pilot").shortcut("1"),
    MenuItem::action("Medic", "medic").shortcut("2"),
    MenuItem::action("Gunner", "gunner").shortcut("3"),
    MenuItem::action("Ship", "ship").shortcut("4"),
];

const FRAMEWORKS: [ComboOption; 8] = [
    ComboOption::new("proa", "Proa"),
    ComboOption::new("axum", "Axum"),
    ComboOption::new("leptos", "Leptos"),
    ComboOption::new("dioxus", "Dioxus"),
    ComboOption::new("yew", "Yew"),
    ComboOption::new("rocket", "Rocket"),
    ComboOption::new("actix", "Actix Web"),
    ComboOption::new("loco", "Loco"),
];

const DISTRICTS: [ComboOption; 3] = [
    ComboOption::new("spire", "Spire"),
    ComboOption::new("midstack", "Midstack"),
    ComboOption::new("undercity", "Undercity"),
];

const WELCOME: [Toast<'static>; 1] = [Toast::new("Gallery loaded")
    .description("This toast was rendered on the server.")
    .tone(ToastTone::Success)];

/// Theme switcher wiring: listens to the menu's `proabono:select` event.
const THEME_SCRIPT: StaticRaw = StaticRaw(
    r#"<script>document.addEventListener("proabono:select",e=>{if(e.target.id!=="theme-switch")return;const v=e.detail.value;if(v==="system")document.documentElement.removeAttribute("data-pb-theme");else document.documentElement.dataset.pbTheme=v;document.documentElement.style.colorScheme=v==="system"?"":v;window.proabono.toast({title:"Theme: "+e.detail.label});});</script>"#,
);

const SECTION: &str = "flex flex-col gap-6 border-t border-[var(--pb-border)] py-12";
const H2: &str = "text-xl font-semibold tracking-tight";
const LEDE: &str = "max-w-2xl text-sm leading-6 text-[var(--pb-muted)]";
const CODE: &str = "overflow-x-auto rounded-lg border border-[var(--pb-border)] bg-[var(--pb-surface-hover)] p-4 font-mono text-xs leading-5";

struct DrawerBody;

impl<L: DataLoader> WebRenderSync<L> for DrawerBody {
    fn render(self, cx: &mut WebContext<L>) -> Result<(), WriteError> {
        html_sync! {
            <div class="flex h-full flex-col gap-4 text-sm leading-6">
                <p class="text-[var(--pb-muted)]">
                    "Native <dialog>: focus is trapped, Escape closes it, and clicking the backdrop dismisses it. Tab through to check."
                </p>
                {Combobox::new("drawer-district", "district", "Home district", &DISTRICTS).placeholder("Pick a district")}
                <label class="grid gap-1.5">
                    <span class="font-medium">"Call sign"</span>
                    <input class="pb-input" value="LAZARUS 1" />
                </label>
                <div class="mt-auto flex justify-end gap-2 pt-4">
                    {DrawerClose::new("Cancel").value("cancel")}
                    {DrawerClose::new("Save").value("save").class("!border-[var(--pb-accent)] !bg-[var(--pb-accent)] !text-[var(--pb-accent-fg)]")}
                </div>
            </div>
        }
        .render(cx)
    }
}

struct Call {
    id: &'static str,
    district: &'static str,
    kind: &'static str,
    precedence: &'static str,
    minutes: i64,
    payout: &'static str,
    payout_cr: i64,
}

const fn call(
    id: &'static str,
    district: &'static str,
    kind: &'static str,
    precedence: &'static str,
    minutes: i64,
    payout: &'static str,
    payout_cr: i64,
) -> Call {
    Call {
        id,
        district,
        kind,
        precedence,
        minutes,
        payout,
        payout_cr,
    }
}

const CALLS: [Call; 24] = [
    call("C-0412", "Midstack", "Gang crossfire", "Urgent", 38, "4,200 cr", 4200),
    call("C-0413", "Spire", "Club overdose", "Priority", 22, "9,800 cr", 9800),
    call("C-0414", "Undercity", "Structure collapse", "Urgent", 51, "1,150 cr", 1150),
    call("C-0415", "Midstack", "Transit crash", "Urgent-Surgical", 44, "3,600 cr", 3600),
    call("C-0416", "Undercity", "Chrome rejection", "Priority", 29, "640 cr", 640),
    call("C-0417", "Spire", "Executive extraction", "Urgent", 17, "14,500 cr", 14500),
    call("C-0418", "Midstack", "Workplace accident", "Routine", 33, "1,900 cr", 1900),
    call("C-0419", "Undercity", "Mass casualty", "Urgent", 58, "2,300 cr", 2300),
    call("C-0420", "Spire", "Corp sabotage", "Urgent-Surgical", 26, "11,200 cr", 11200),
    call("C-0421", "Midstack", "Downed courier", "Urgent", 47, "5,050 cr", 5050),
    call("C-0422", "Undercity", "Scav ambush", "Priority", 40, "0 cr", 0),
    call("C-0423", "Midstack", "Rooftop fall", "Priority", 19, "2,750 cr", 2750),
    call("C-0424", "Spire", "Insurance dispute", "Routine", 12, "6,400 cr", 6400),
    call("C-0425", "Undercity", "Smog inhalation", "Routine", 35, "380 cr", 380),
    call("C-0426", "Midstack", "Market stabbing", "Urgent", 24, "3,100 cr", 3100),
    call("C-0427", "Spire", "Drone strike", "Urgent-Surgical", 31, "12,900 cr", 12900),
    call("C-0428", "Undercity", "Hoist rescue", "Urgent", 55, "1,700 cr", 1700),
    call("C-0429", "Midstack", "Gang crossfire", "Urgent", 42, "4,450 cr", 4450),
    call("C-0430", "Spire", "Gala poisoning", "Priority", 15, "8,300 cr", 8300),
    call("C-0431", "Undercity", "Flooded tunnel", "Urgent", 49, "990 cr", 990),
    call("C-0432", "Midstack", "Signage collapse", "Priority", 27, "2,050 cr", 2050),
    call("C-0433", "Spire", "VIP cardiac arrest", "Urgent", 9, "16,000 cr", 16000),
    call("C-0434", "Undercity", "Feral swarm", "Urgent", 53, "1,400 cr", 1400),
    call("C-0435", "Midstack", "Rival crew clash", "Priority", 36, "2,600 cr", 2600),
];

const CALL_COLUMNS: [Column<Call>; 6] = [
    Column::new("Call", |c: &Call| Cell::Text(c.id)),
    Column::new("District", |c: &Call| Cell::Text(c.district)).sortable(),
    Column::new("Type", |c: &Call| Cell::Text(c.kind)).sortable(),
    Column::new("Precedence", |c: &Call| Cell::Text(c.precedence)).sortable(),
    Column::new("Minutes", |c: &Call| Cell::Int(c.minutes))
        .sortable()
        .numeric(),
    Column::new("Payout", |c: &Call| Cell::Sorted {
        text: c.payout,
        key: c.payout_cr,
    })
    .sortable()
    .numeric(),
];

#[derive(Debug, Clone, Copy)]
pub struct HomePage {
    /// Current page for the server-side pagination demo (`?page=`).
    pub page: u32,
}

impl<L: DataLoader> WebRenderSync<L> for HomePage {
    fn render(self, cx: &mut WebContext<L>) -> Result<(), WriteError> {
        html_sync! {
            <main class="mx-auto flex max-w-4xl flex-col px-4 py-12 sm:px-6">
                <header class="flex flex-col gap-4 pb-12 sm:flex-row sm:items-end sm:justify-between">
                    <div class="flex flex-col gap-3">
                        <p class="font-mono text-xs uppercase tracking-[0.2em] text-[var(--pb-muted)]">
                            "proabono-kit · v0.1"
                        </p>
                        <h1 class="text-4xl font-semibold tracking-tight">
                            "Free Proa components"
                        </h1>
                        <p class={LEDE}>
                            "Source-owned components for Proa sites, written from the WAI-ARIA patterns. Server-rendered HTML first; a small vanilla-JS runtime adds keyboard support. MIT OR Apache-2.0."
                        </p>
                    </div>
                    {DropdownMenu::new("theme-switch", "Theme", &THEME_MENU).align(MenuAlign::End)}
                </header>
                <section aria-labelledby="dropdown-menu" class={SECTION}>
                    <h2 id="dropdown-menu" class={H2}>"Dropdown menu"</h2>
                    <p class={LEDE}>
                        "Native popover, so it opens without JavaScript. With the runtime: arrow keys, Home/End, typeahead, focus return. Action items fire a proabono:select event."
                    </p>
                    <div class="flex flex-wrap items-start gap-4">
                        {DropdownMenu::new("account-menu", "Account", &ACCOUNT_MENU)}
                        {DropdownMenu::new("role-menu", "Swap to…", &ROLE_MENU).align(MenuAlign::Center)}
                    </div>
                    <pre class={CODE}>
                        <code>
                            "DropdownMenu::new(\"account-menu\", \"Account\", &[\n    MenuItem::link(\"Profile\", \"/profile\").shortcut(\"Shift+P\"),\n    MenuItem::SEPARATOR,\n    MenuItem::action(\"Log out\", \"logout\").danger(),\n])"
                        </code>
                    </pre>
                </section>
                <section aria-labelledby="combobox" class={SECTION}>
                    <h2 id="combobox" class={H2}>"Combobox"</h2>
                    <p class={LEDE}>
                        "Type to filter. Submits the option value under its name. Without JavaScript it falls back to a text input with a native datalist; Combobox::resolve accepts either."
                    </p>
                    <form method="get" action="/" class="flex flex-wrap items-end gap-3">
                        {Combobox::new("framework", "framework", "Framework", &FRAMEWORKS).placeholder("Search frameworks…").value("proa")}
                        <button type="submit" class="pb-trigger">"Submit"</button>
                    </form>
                </section>
                <section aria-labelledby="drawer" class={SECTION}>
                    <h2 id="drawer" class={H2}>"Drawer"</h2>
                    <p class={LEDE}>
                        "Slides in from any edge. Built on native <dialog> with invoker commands."
                    </p>
                    <div class="flex flex-wrap gap-3">
                        {DrawerTrigger::new("drawer-right", "Open right")}
                        {DrawerTrigger::new("drawer-bottom", "Open bottom")}
                        {DrawerTrigger::new("drawer-left", "Open left")}
                    </div>
                    {Drawer::new("drawer-right", "Edit crew", DrawerBody).description("Changes apply to the next shift.")}
                    {Drawer::new("drawer-left", "Navigation", DrawerBody).side(DrawerSide::Left)}
                    {Drawer::new("drawer-bottom", "Dispatch", DrawerBody).description("Mobile-style sheet from the bottom edge.").side(DrawerSide::Bottom)}
                </section>
                <section aria-labelledby="toast" class={SECTION}>
                    <h2 id="toast" class={H2}>"Toast"</h2>
                    <p class={LEDE}>
                        "Sonner-style stack: hover to expand, timers pause while you read. Server-rendered flash toasts, window.proabono.toast(), or a data-pb-toast attribute."
                    </p>
                    <div class="flex flex-wrap gap-3">
                        <button
                            type="button"
                            class="pb-trigger"
                            data-pb-toast="Event created"
                            data-pb-toast-description="Friday, 20:00 · Midstack L42"
                        >
                            "Default"
                        </button>
                        <button
                            type="button"
                            class="pb-trigger"
                            data-pb-toast="Casualty delivered"
                            data-pb-toast-tone="success"
                            data-pb-toast-description="Payout 4,200 cr"
                        >
                            "Success"
                        </button>
                        <button
                            type="button"
                            class="pb-trigger"
                            data-pb-toast="Rival crew inbound"
                            data-pb-toast-tone="warning"
                        >
                            "Warning"
                        </button>
                        <button
                            type="button"
                            class="pb-trigger"
                            data-pb-toast="Hull breach"
                            data-pb-toast-tone="error"
                            data-pb-toast-description="Return to hangar."
                        >
                            "Error"
                        </button>
                        <button
                            type="button"
                            class="pb-trigger"
                            data-pb-toast="Sticky notice"
                            data-pb-toast-tone="info"
                            data-pb-toast-duration="0"
                        >
                            "No auto-dismiss"
                        </button>
                    </div>
                </section>
                <section aria-labelledby="data-table" class={SECTION}>
                    <h2 id="data-table" class={H2}>"Data table"</h2>
                    <p class={LEDE}>
                        "Typed columns over a slice of rows, rendered on the server. The runtime adds sortable headers, a filter, a live count and client-side paging. Without JavaScript it is a plain accessible table."
                    </p>
                    {DataTable::new("calls", "Night shift calls", &CALL_COLUMNS, &CALLS)
                        .filter()
                        .page_size(8)
                        .sorted_by(0, SortDirection::Ascending)}
                </section>
                <section aria-labelledby="pagination" class={SECTION}>
                    <h2 id="pagination" class={H2}>"Pagination"</h2>
                    <p class={LEDE}>
                        "Server-rendered page links: no JavaScript. Pass a prefix and the page number is appended. These links set ?page= on this gallery."
                    </p>
                    {Pagination::new(self.page, 20, "/?page=")}
                    {Pagination::new(self.page.min(4), 4, "/?page=").label("Short pagination")}
                </section>
                <section aria-labelledby="theming" class={SECTION}>
                    <h2 id="theming" class={H2}>"Theming"</h2>
                    <p class={LEDE}>
                        "Override the --pb-* custom properties anywhere. This panel uses a Golden Hour palette."
                    </p>
                    <div
                        class="flex flex-wrap items-start gap-4 rounded-xl p-6"
                        style="--pb-fg:#ece6d6;--pb-muted:#a8a3b3;--pb-surface:#0b0d14;--pb-surface-hover:#1b1f2d;--pb-border:rgb(236 230 214/.16);--pb-accent:#ffb02e;--pb-accent-fg:#1a1206;--pb-radius:0;background:#06070b;color:#ece6d6"
                    >
                        {DropdownMenu::new("gh-role-menu", "Swap crew", &ROLE_MENU)}
                        {Combobox::new("gh-district", "district", "District", &DISTRICTS).placeholder("Search districts")}
                    </div>
                </section>
                <section aria-labelledby="install" class={SECTION}>
                    <h2 id="install" class={H2}>"Install"</h2>
                    <pre class={CODE}>
                        <code>
                            "cargo install --git https://github.com/rickboy/proabono-kit proabono\ncd my-proa-site\nproabono add dropdown-menu combobox drawer toast data-table pagination"
                        </code>
                    </pre>
                    <p class={LEDE}>
                        "Then link the stylesheet and runtime once in src/layouts/root.rs:"
                    </p>
                    <pre class={CODE}>
                        <code>
                            "<link rel=\"stylesheet\" href=\"/static/proa/proabono/proabono.css\" />\n<script src=\"/static/proa/proabono/proabono.js\" defer></script>"
                        </code>
                    </pre>
                </section>
                {Toaster::new(&WELCOME)}
                {THEME_SCRIPT}
            </main>
        }
        .render(cx)
    }
}

use axum::{extract::Query, response::IntoResponse};
use proa_framework_axum::RouteResponse;
use serde::Deserialize;

use crate::{error::AppError, layouts::RootDocument};

#[derive(Deserialize)]
pub struct HomeQuery {
    page: Option<u32>,
}

pub async fn handler(Query(query): Query<HomeQuery>) -> Result<impl IntoResponse, AppError> {
    let page = query.page.unwrap_or(1).clamp(1, 20);
    let html = RouteResponse::ssr_async(RootDocument {
        title: "proabono-kit · Free Proa components",
        content: proa_core::to_async(HomePage { page }),
    });
    Ok(html)
}
