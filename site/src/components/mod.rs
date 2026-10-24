//! The gallery compiles the registry sources in place, so editing a file under
//! `registry/` is what you see here. Installed apps get copies via `proabono add`.

#[path = "../../../registry/core/src/mod.rs"]
pub mod _proabono;
#[path = "../../../registry/combobox/src/mod.rs"]
pub mod combobox;
#[path = "../../../registry/data-table/src/mod.rs"]
pub mod data_table;
#[path = "../../../registry/drawer/src/mod.rs"]
pub mod drawer;
#[path = "../../../registry/dropdown-menu/src/mod.rs"]
pub mod dropdown_menu;
#[path = "../../../registry/pagination/src/mod.rs"]
pub mod pagination;
#[path = "../../../registry/toast/src/mod.rs"]
pub mod toast;
