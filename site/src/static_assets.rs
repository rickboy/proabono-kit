use tower_http::services::ServeDir;

pub fn service() -> ServeDir {
    ServeDir::new("public").append_index_html_on_directories(false)
}

/// Serves the kit's CSS and runtime straight from `registry/core/assets`, at
/// the same URL an installed app uses (`/static/proa/proabono/...`).
pub fn kit_assets() -> ServeDir {
    ServeDir::new("../registry/core/assets").append_index_html_on_directories(false)
}
