use proa_core::{DataLoader, StaticRaw, WebContext, WebRenderSync, WriteError};
use proa_macros::html_sync;

use crate::components::combobox::{ComboOption, Combobox};
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

#[derive(Debug, Clone, Copy)]
pub struct HomePage;

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
                            "cargo install --git https://github.com/rickboy/proabono-kit proabono\ncd my-proa-site\nproabono add dropdown-menu combobox drawer toast"
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

use axum::response::IntoResponse;
use proa_framework_axum::RouteResponse;

use crate::{error::AppError, layouts::RootDocument};

pub async fn handler() -> Result<impl IntoResponse, AppError> {
    let html = RouteResponse::ssr_async(RootDocument {
        title: "proabono-kit · Free Proa components",
        content: proa_core::to_async(HomePage),
    });
    Ok(html)
}
