use gtk4::gio::Menu;
use gtk4::prelude::*;
use gtk4::{Box as GtkBox, MenuButton};

pub fn build_headerbar() -> GtkBox {
    let headerbar = GtkBox::new(gtk4::Orientation::Horizontal, 6);

    // Main menu
    let menu = Menu::new();
    menu.append(Some("File"), Some("app.file"));
    menu.append(Some("Edit"), Some("app.edit"));

    // "View" submenu
    let view_submenu = Menu::new();
    view_submenu.append(Some("Show Hidden Files"), Some("win.show_hidden"));

    menu.append_submenu(Some("View"), &view_submenu);

    let menu_button = MenuButton::new();
    menu_button.set_menu_model(Some(&menu));

    headerbar.append(&menu_button);
    headerbar
}
