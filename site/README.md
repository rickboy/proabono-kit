# proabono-kit

A production-shaped Proa `minimal` site.

```console
cp .env.example .env
proa dev --open
```

Proa dependencies resolve from crates.io. No GitHub credentials or Cargo registry token is required to build the generated project.

The first dependency-resolving command creates the real `Cargo.lock`; commit it before deployment. Use `proa build` for a linked release build and `proa start` to run that exported release.

The HTML page is available at `/`. Prefer keyed loading through `cx` when several components may request the same resource, the work should deduplicate, or sections can render independently. Keep a one-owner query in the handler when rendering cannot begin without it.

Package: `site`.
