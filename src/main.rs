mod files_panel;
mod headerbar;
mod pathbar;
mod popup_menu;
mod sidebar;
mod state;
mod style;

use gtk4::glib;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box as GtkBox, GestureClick, Orientation, Paned};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

const APP_ID: &str = "org.filemanager.axfm";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_fm);
    app.run();
}

fn build_fm(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Ax File Manager")
        .default_width(800)
        .default_height(500)
        .build();

    style::load_css();

    // where files will be shown
    let content_area = GtkBox::new(Orientation::Vertical, 0);

    let home_path = dirs::home_dir().unwrap_or(PathBuf::from("/")).join("");
    let fmstate = Rc::new(RefCell::new(state::FmState::new(home_path.clone())));

    let (sidebar_box, sidebar_selection) = sidebar::build_sidebar();
    let path_bar = pathbar::build_pathbar(&mut fmstate.borrow_mut());
    let (files_scroll, files_list, list_view) = files_panel::build_files_panel(fmstate.clone());

    // right click menus
    let empty_area_menu = popup_menu::get_empty_right_click(&content_area, fmstate.clone());
    let file_area_menu = popup_menu::get_file_right_click(&content_area, fmstate.clone());

    populate_files_list(&files_list, &home_path);

    sidebar_selection.connect_selected_notify(glib::clone!(
        #[weak]
        files_list,
        #[strong]
        fmstate,
        move |sel| {
            let idx = sel.selected();
            if idx == gtk4::INVALID_LIST_POSITION {
                return;
            }

            let paths = [
                dirs::home_dir().unwrap().join(""),
                dirs::home_dir().unwrap().join("Documents/"),
                dirs::home_dir().unwrap().join("Downloads/"),
                dirs::home_dir().unwrap().join("Music/"),
                dirs::home_dir().unwrap().join("Pictures/"),
                dirs::home_dir().unwrap().join("Videos/"),
                dirs::home_dir().unwrap().join(".local/share/Trash/files/"),
            ];

            if let Some(path) = paths.get(idx as usize) {
                while files_list.n_items() > 0 {
                    files_list.remove(0);
                }
                if let Ok(entries) = std::fs::read_dir(path) {
                    for entry in entries.flatten() {
                        files_list.append(&entry.path().to_string_lossy());
                    }
                }
                fmstate.borrow_mut().set_path(path.clone());
            }
        }
    ));

    path_bar.connect_activate(glib::clone!(
        #[strong]
        files_list,
        move |widget| {
            let path = PathBuf::from(widget.text());
            if let Ok(entries) = std::fs::read_dir(path) {
                while files_list.n_items() > 0 {
                    files_list.remove(0);
                }

                for entry in entries.flatten() {
                    files_list.append(&entry.path().to_string_lossy());
                }
            }
        }
    ));

    list_view.connect_activate(glib::clone!(
        #[strong]
        fmstate,
        #[weak]
        files_list,
        move |lv, position| {
            if let Some(obj) = lv.model().and_then(|m| m.item(position)) {
                let path = std::path::PathBuf::from(
                    obj.downcast::<gtk4::StringObject>().unwrap().string(),
                );
                if path.is_dir() {
                    populate_files_list(&files_list, &path);
                    fmstate.borrow_mut().set_path(path.join(""));
                }
            }
        }
    ));

    // controllers
    let right_click = GestureClick::new();
    right_click.set_button(3);

    right_click.connect_released(glib::clone!(
        #[strong]
        fmstate,
        #[weak]
        empty_area_menu,
        #[weak]
        file_area_menu,
        move |_, _, x, y| {
            let click_rect = gtk4::gdk::Rectangle::new(x as i32, y as i32, 1, 1);

            let mut fmstate_mut = fmstate.borrow_mut();
            let hovered_file_opt = fmstate_mut.hovered_file.clone();

            if let Some(file_name) = hovered_file_opt {
                fmstate_mut.popup_focused_file = Some(file_name);

                // immedeatly drop to prevent issues
                // trust me, it will panic without this.
                drop(fmstate_mut);

                file_area_menu.set_pointing_to(Some(&click_rect));
                file_area_menu.popup();

                file_area_menu.connect_closed(glib::clone!(#[strong] fmstate, move |_| {
                    fmstate.borrow_mut().popup_focused_file = None;
                }));
            } else {
                empty_area_menu.set_pointing_to(Some(&click_rect));
                empty_area_menu.popup();
            }
        }
    ));

    // content area
    content_area.append(&path_bar);
    content_area.append(&files_scroll);

    // setup controllers
    content_area.add_controller(right_click);

    let paned = Paned::new(Orientation::Horizontal);
    paned.set_start_child(Some(&sidebar_box));
    paned.set_end_child(Some(&content_area));
    paned.set_position(200);
    paned.set_wide_handle(true);
    paned.set_resize_start_child(false);
    paned.set_shrink_start_child(false);

    window.set_child(Some(&paned));
    window.present();
}

fn populate_files_list(files_list: &gtk4::StringList, path: &std::path::Path) {
    while files_list.n_items() > 0 {
        files_list.remove(0);
    }
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            files_list.append(&entry.path().to_string_lossy());
        }
    }
}
