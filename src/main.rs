mod files_panel;
mod headerbar;
mod pathbar;
mod sidebar;
mod state;
mod style;

use gtk4::glib;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Orientation, Paned, Box as GtkBox};
use std::rc::Rc;
use std::cell::RefCell;

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

    let home_path = dirs::home_dir().unwrap_or_default();
    let fmstate = Rc::new(RefCell::new(state::FmState::new(home_path.clone())));

    let (sidebar_box, sidebar_selection) = sidebar::build_sidebar();
    let path_bar = pathbar::build_pathbar(&mut fmstate.borrow_mut());
    let (files_scroll, files_list) = files_panel::build_files_panel();

    populate_files_list(&files_list, &home_path);

    sidebar_selection.connect_selected_notify(glib::clone!(
        #[strong]
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
                dirs::home_dir().unwrap().join("Documents"),
                dirs::home_dir().unwrap().join("Downloads"),
                dirs::home_dir().unwrap().join("Music"),
                dirs::home_dir().unwrap().join("Pictures"),
                dirs::home_dir().unwrap().join("Videos"),
                dirs::home_dir().unwrap().join(".local/share/Trash"),
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

    // content area
    let content_area = GtkBox::new(Orientation::Vertical, 0);
    content_area.append(&path_bar);
    content_area.append(&files_scroll);

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
