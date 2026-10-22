# proabono-kit

Free, source-owned components for [Proa](https://proa.so) sites, built from
the WAI-ARIA Authoring Practices patterns. No Proa UI Pro licence needed.

| Item            | What it is                                                                 |
|-----------------|----------------------------------------------------------------------------|
| `dropdown-menu` | Menu button on the native `popover` attribute; arrow keys, typeahead, links and actions |
| `combobox`      | Searchable select; submits the option value, degrades to an input + `<datalist>` |
| `drawer`        | Slide-in panel from any edge on native `<dialog>`                           |
| `toast`         | Sonner-style stacked notifications, server-rendered or from JS              |
| `core`          | Icons, `proabono.css` (theme tokens + styles), `proabono.js` (runtime). Pulled in automatically |

Everything renders as plain HTML on the server and works without JavaScript
where the platform allows (popover, dialog, datalist). `proabono.js` adds
keyboard support, positioning fallbacks and toasts.

These are original implementations. No Proa UI Pro source was used.

## Layout

```
registry/<item>/item.json   name, Rust module, description, dependencies
registry/<item>/src/        Rust sources  -> @components/<module>/
registry/core/assets/       CSS + JS      -> @assets/proabono/
cli/                        `proabono` tool: build / list / add
site/                       Gallery: a Proa site that compiles registry/ in place
```

## Use it in a Proa site

The repo is private for now, so install from a local checkout:

```bash
cargo install --path ~/proabono-kit/cli
cd ~/proabono-kit && proabono build            # writes dist/r/
cd ~/my-proa-site
proabono add dropdown-menu combobox --registry ~/proabono-kit/dist/r
```

`add` resolves dependencies and verifies each file's SHA-256 before writing
anything. It copies sources into `src/components/` and assets into
`public/proa/proabono/`, appends `pub mod …;` to `src/components/mod.rs`, and
records versions in `proabono.lock.json`. If you've edited an installed file
locally, it refuses to overwrite it unless you pass `--force`. `--dry-run`
verifies without writing.

Then link the assets once in `src/layouts/root.rs`:

```html
<link rel="stylesheet" href="/static/proa/proabono/proabono.css" />
<script src="/static/proa/proabono/proabono.js" defer></script>
```

### Why not `proa ui add`?

The registry output uses Proa UI's `ui-component-manifest-v1` format. Proa
CLI 0.1.3 accepts only the `@proa` and `@proa-pro` registries, though. If Proa
opens up custom registries later, the same `dist/r` should work by adding
`"@proabono": "<url>/{name}.json"` to `proa.config.json`.

## Examples

```rust
use crate::components::dropdown_menu::{DropdownMenu, MenuAlign, MenuItem};

const ACCOUNT: [MenuItem; 3] = [
    MenuItem::link("Profile", "/profile").shortcut("Shift+P"),
    MenuItem::SEPARATOR,
    MenuItem::action("Log out", "logout").danger(),
];

html_sync! { {DropdownMenu::new("account", "Account", &ACCOUNT).align(MenuAlign::End)} }
```

Action items fire `proabono:select` on the menu element (`event.target.id` is
the menu id, `event.detail` is `{ value, label }`).

```rust
use crate::components::combobox::{ComboOption, Combobox};
const FRAMEWORKS: [ComboOption; 2] = [ComboOption::new("proa", "Proa"), ComboOption::new("axum", "Axum")];
html_sync! { {Combobox::new("framework", "framework", "Framework", &FRAMEWORKS).value("proa")} }
// Server side, accept either the value (JS) or the typed label (no JS):
// Combobox::resolve(&FRAMEWORKS, &form.framework)
```

```rust
use crate::components::drawer::{Drawer, DrawerClose, DrawerSide, DrawerTrigger};
html_sync! {
    {DrawerTrigger::new("settings", "Settings")}
    {Drawer::new("settings", "Settings", SettingsBody).side(DrawerSide::Right)}
}
```

```rust
use crate::components::toast::{Toast, ToastTone, Toaster};
let flash = [Toast::new("Saved").tone(ToastTone::Success)];
html_sync! { {Toaster::new(&flash)} }
```

```js
proabono.toast({ title: "Saved", description: "All changes stored", tone: "success" });
```

```html
<button data-pb-toast="Copied" data-pb-toast-tone="info">Copy</button>
```

## Theming

Override the `--pb-*` custom properties on `:root` or on any wrapper. For example:

```css
:root {
  --pb-surface: #0b0d14; --pb-surface-hover: #1b1f2d; --pb-fg: #ece6d6;
  --pb-muted: #a8a3b3; --pb-border: rgb(236 230 214 / 0.16);
  --pb-accent: #ffb02e; --pb-accent-fg: #1a1206; --pb-radius: 0;
}
```

Light and dark defaults follow `prefers-color-scheme`. To force one, set
`data-pb-theme="light|dark"` on `<html>`. Styles live in the `components`
cascade layer, so Tailwind utilities passed through `class` win.

## Develop

Proa only builds on Linux, so on Windows use WSL.

```bash
cd site && proa dev                    # gallery at http://localhost:3001, reloads on registry/ edits
cd cli && cargo build --release
./cli/target/release/proabono build    # registry -> dist/r
```

## Licence

MIT OR Apache-2.0, at your option.
