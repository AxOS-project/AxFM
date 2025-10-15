mod files_panel;
mod headerbar;
mod sidebar;
mod style;

use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box as GtkBox, Orientation};
use gtk4::glib;

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

    let (sidebar_box, sidebar_selection) = sidebar::build_sidebar();
    let (files_scroll, files_list) = files_panel::build_files_panel();

    sidebar_selection.connect_selected_notify(glib::clone!(#[strong] files_list, move |sel| {
        let idx = sel.selected();
        if idx == gtk4::INVALID_LIST_POSITION { return; }

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
        }
    }));

    let content_box = GtkBox::new(Orientation::Horizontal, 6);
    content_box.append(&sidebar_box);
    content_box.append(&files_scroll);

    window.set_child(Some(&content_box));
    window.present();
}
