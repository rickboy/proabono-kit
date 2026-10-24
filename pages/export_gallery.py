#!/usr/bin/env python3
"""Snapshot the running gallery site into static files for GitHub Pages.

    python3 pages/export_gallery.py http://127.0.0.1:3999 dist/gallery

Saves `/` as index.html and every `/?page=N` the pagination demo links to as
page-N.html, then rewrites root-absolute URLs so the snapshot works under the
Pages project prefix (/proabono-kit/gallery/). Static assets are copied by the
caller. Everything interactive is client-side, so the snapshot behaves like
the live gallery; only the server-side pagination demo is pre-rendered per page.
"""

import re
import sys
import urllib.request
from pathlib import Path

PAGES = 20  # total pages in the gallery's server-side pagination demo


def fetch(base: str, path: str) -> str:
    with urllib.request.urlopen(base + path, timeout=30) as response:
        return response.read().decode("utf-8")


def page_file(n: int) -> str:
    return "index.html" if n == 1 else f"page-{n}.html"


def rewrite(page: str) -> str:
    # Pagination links: /?page=N -> the matching static file.
    page = re.sub(r'href="/\?page=(\d+)"', lambda m: f'href="{page_file(int(m.group(1)))}"', page)
    # Assets and the favicon: /static/... -> static/...
    page = page.replace('"/static/', '"static/')
    # Demo form (GET to /) and any root link: stay on this page.
    page = page.replace('action="/"', 'action=""').replace('href="/"', 'href="index.html"')
    # The live gallery has no site chrome; give the snapshot a way back.
    page = page.replace(
        '<main class="mx-auto',
        '<nav aria-label="Site" style="max-width:56rem;margin:0 auto;padding:1rem 1rem 0;'
        'display:flex;gap:1.25rem;font-size:.875rem">'
        '<a href="../">&larr; proabono-kit</a>'
        '<a href="https://github.com/rickboy/proabono-kit">GitHub</a></nav>'
        '<main class="mx-auto',
        1,
    )
    leftovers = re.findall(r'(?:href|src|action)="/(?!/)[^"]*"', page)
    if leftovers:
        raise SystemExit(f"unrewritten root-absolute URLs: {sorted(set(leftovers))}")
    return page


def main() -> None:
    base, out = sys.argv[1].rstrip("/"), Path(sys.argv[2])
    out.mkdir(parents=True, exist_ok=True)
    for n in range(1, PAGES + 1):
        html = fetch(base, "/" if n == 1 else f"/?page={n}")
        (out / page_file(n)).write_text(rewrite(html), encoding="utf-8")
    print(f"exported {PAGES} gallery pages to {out}")


if __name__ == "__main__":
    main()
