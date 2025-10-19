use crate::state::FmState;
use gtk4::gio::{Menu, SimpleAction};
use gtk4::prelude::*;
use gtk4::{Box as GtkBox, MenuButton, ApplicationWindow, Application};
use std::cell::RefCell;
use std::rc::Rc;
use gtk4::glib;

pub fn build_headerbar() -> GtkBox {
    let headerbar = GtkBox::new(gtk4::Orientation::Horizontal, 6);

    // Main menu model
    let menu = Menu::new();

    // "File" submenu
    let file_submenu = Menu::new();
    file_submenu.append(Some("New Window"), Some("win.open_new_window"));
    file_submenu.append(Some("Close Window"), Some("win.close_window"));
    menu.append_submenu(Some("File"), &file_submenu);

    // "Edit" submenu
    let edit_submenu = Menu::new();
    edit_submenu.append(Some("Undo"), Some("win.undo_history"));
    edit_submenu.append(Some("Redo"), Some("win.redo_history"));
    menu.append_submenu(Some("Edit"), &edit_submenu);

    // "View" submenu
    let view_submenu = Menu::new();
    view_submenu.append(Some("Show Hidden Files"), Some("win.show_hidden"));
    menu.append_submenu(Some("View"), &view_submenu);

    // Menu button
    let menu_button = MenuButton::new();
    menu_button.set_menu_model(Some(&menu));

    headerbar.append(&menu_button);
    headerbar
}

pub fn implement_actions(window: &ApplicationWindow, app: &Application, fmstate: Rc<RefCell<FmState>>) {
    // Show Hidden Files action
    let show_hidden_initial = fmstate.borrow().settings.show_hidden;
    let show_hidden_action =
        SimpleAction::new_stateful("show_hidden", None, &show_hidden_initial.into());

    show_hidden_action.connect_activate(glib::clone!(#[strong] fmstate, move |action, _| {
        let current: bool = action.state().unwrap().get().unwrap();
        action.set_state(&(!current).into());
        fmstate.borrow_mut().settings.show_hidden = !current;
    }));

    window.add_action(&show_hidden_action);

    let new_window_action = SimpleAction::new("open_new_window", None);
    new_window_action.connect_activate(glib::clone!(#[weak] app, move |_, _| {
        crate::build_fm(&app);
    }));
    window.add_action(&new_window_action);

    let close_window_action = SimpleAction::new("close_window", None);
    close_window_action.connect_activate(glib::clone!(#[weak] window, move |_, _| {
        window.close();
    }));
    window.add_action(&close_window_action);

    let undo_action = SimpleAction::new("undo_history", None);
    undo_action.connect_activate(move |_, _| {
        println!("Undo triggered");
    });
    window.add_action(&undo_action);

    let redo_action = SimpleAction::new("redo_history", None);
    redo_action.connect_activate(move |_, _| {
        println!("Redo triggered");
    });
    window.add_action(&redo_action);
}
