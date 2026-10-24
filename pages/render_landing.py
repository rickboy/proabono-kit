#!/usr/bin/env python3
"""Render the GitHub Pages landing page from the built registry index.

    python3 pages/render_landing.py dist/r/index.json > dist/index.html

The page is static HTML: the component list is baked in at build time from
index.json, so it needs no JavaScript and always matches the published
registry.
"""

import html
import json
import sys
from pathlib import Path

REPO = "https://github.com/rickboy/proabono-kit"

# Short labels for the cards; the long description comes from index.json.
TITLES = {
    "core": "Core",
    "dropdown-menu": "Dropdown menu",
    "combobox": "Combobox",
    "drawer": "Drawer",
    "toast": "Toast",
    "data-table": "Data table",
    "pagination": "Pagination",
    "menubar": "Menubar",
    "context-menu": "Context menu",
    "command-palette": "Command palette",
}

# Gallery section each item is demoed in.
ANCHORS = {
    "dropdown-menu": "dropdown-menu",
    "combobox": "combobox",
    "drawer": "drawer",
    "toast": "toast",
    "data-table": "data-table",
    "pagination": "pagination",
    "menubar": "menus",
    "context-menu": "menus",
    "command-palette": "menus",
}


def card(item: dict) -> str:
    name = item["name"]
    e = html.escape
    deps = [d for d in item.get("dependencies", []) if d != "core"]
    dep_html = (
        '<p class="deps">Also installs ' + ", ".join(f"<code>{e(d)}</code>" for d in deps) + "</p>"
        if deps
        else ""
    )
    demo = (
        f'<a class="demo" href="gallery/#{e(ANCHORS[name])}">Live demo</a>' if name in ANCHORS else ""
    )
    command = f"proabono add {name}"
    return f"""
      <article class="card" id="{e(name)}">
        <header>
          <h3>{e(TITLES.get(name, name))}</h3>
          <code class="version" title="{e(item['version'])}">{e(item['version'][7:15])}</code>
        </header>
        <p>{e(item['description'])}</p>
        {dep_html}
        <div class="cmd"><code>{e(command)}</code><button type="button" data-copy="{e(command)}">Copy</button></div>
        {demo}
      </article>"""


def main() -> None:
    index = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
    items = index["items"]
    # Components first in a sensible reading order, core last (it is automatic).
    order = list(TITLES)
    items.sort(key=lambda i: (i["name"] == "core", order.index(i["name"]) if i["name"] in order else 99))
    components = [i for i in items if i["name"] != "core"]
    template = (Path(__file__).parent / "index.template.html").read_text(encoding="utf-8")
    page = (
        template.replace("{{CARDS}}", "".join(card(i) for i in items))
        .replace("{{COUNT}}", str(len(components)))
        .replace("{{ALL}}", html.escape(" ".join(i["name"] for i in components)))
        .replace("{{REPO}}", REPO)
    )
    sys.stdout.write(page)


if __name__ == "__main__":
    main()
