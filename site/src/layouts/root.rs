// proa:document-root
use std::future::Future;

use proa_core::{raw_text_ok, DataLoader, RenderOutcome, WebContext, WebRender, WriteError};
use proa_macros::html;

pub struct RootDocument<C> {
    pub title: &'static str,
    pub content: C,
}

impl<L, C> WebRender<L> for RootDocument<C>
where
    L: DataLoader,
    C: WebRender<L> + Send + 'static,
{
    fn render(
        self,
        cx: &mut WebContext<L>,
    ) -> RenderOutcome<impl Future<Output = Result<(), WriteError>>> {
        RenderOutcome::pending(async move {
            let RootDocument { title, content } = self;
            html! {
                <!DOCTYPE html>
                <html lang="en">
                    <head>
                        <meta charset="utf-8" />
                        <meta name="viewport" content="width=device-width, initial-scale=1" />
                        <meta name="color-scheme" content="light dark" />
                        <meta
                            name="description"
                            content="Free, source-owned Proa components: dropdown menu, combobox, drawer and toast."
                        />
                        <title>{raw_text_ok(title)}</title>
                        <link rel="icon" href="/static/favicon.svg" />
                        <link rel="stylesheet" href="/static/styles.css" />
                        <link rel="stylesheet" href="/static/proa/proabono/proabono.css" />
                        <script src="/static/proa/proabono/proabono.js" defer></script>
                    </head>
                    <body class="bg-[var(--pb-surface)] text-[var(--pb-fg)] antialiased">
                        {content}
                    </body>
                </html>
            }
            .render(cx)
            .resolve()
            .await
        })
    }
}
