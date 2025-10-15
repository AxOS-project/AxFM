use gtk4::gio::Menu;
use gtk4::prelude::*;
use gtk4::{Box as GtkBox, MenuButton};

pub fn build_headerbar() -> GtkBox {
    let headerbar = GtkBox::new(gtk4::Orientation::Horizontal, 6);
    let menu = Menu::new();
    menu.append(Some("File"), Some("app.file"));
    menu.append(Some("Edit"), Some("app.edit"));
    menu.append(Some("View"), Some("app.view"));

    let menu_button = MenuButton::new();
    menu_button.set_menu_model(Some(&menu));

    headerbar.append(&menu_button);
    headerbar
}
