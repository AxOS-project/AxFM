mod files_panel;
mod headerbar;
mod pathbar;
mod popup_menu;
mod sidebar;
mod state;
mod style;
mod utils;

use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box as GtkBox, GestureClick, Orientation, Paned};
use gtk4::{gio, glib};
use std::cell::RefCell;
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

    let home_path = gio::File::for_path(glib::home_dir());
    let fmstate = Rc::new(RefCell::new(state::FmState::new(home_path.clone())));

    // implement all actions for the headerbar
    headerbar::implement_actions(&window, &app, fmstate.clone());

    let (files_scroll, files_list, list_view) = files_panel::build_files_panel(fmstate.clone());
    let (sidebar_box, sidebar_selection) = sidebar::build_sidebar(fmstate.clone(), &files_list);
    let path_bar = pathbar::build_pathbar(&mut fmstate.borrow_mut());

    // right click menus
    let empty_area_menu = popup_menu::get_empty_right_click(&content_area, fmstate.clone());
    let file_area_menu =
        popup_menu::get_file_right_click(&content_area, fmstate.clone(), &files_list);

    files_panel::populate_files_list(
        &files_list,
        &home_path,
        &fmstate.borrow().settings.show_hidden,
    );

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

            let sidebar_items = sidebar::get_sidebar_items();

            if let Some((_, file)) = sidebar_items.get(idx as usize) {
                let mut fmstate_mut = fmstate.borrow_mut();

                files_panel::populate_files_list(
                    &files_list,
                    &file,
                    &fmstate_mut.settings.show_hidden,
                );
                fmstate_mut.set_path(file.clone());
            }
        }
    ));

    path_bar.connect_activate(glib::clone!(
        #[weak]
        files_list,
        #[weak]
        sidebar_selection,
        #[strong]
        fmstate,
        move |widget| {
            let text = widget.text();

            let file = if std::path::Path::new(&text).exists() {
                gio::File::for_path(&text)
            } else {
                gio::File::for_uri(&text)
            };

            files_panel::populate_files_list(
                &files_list,
                &file,
                &fmstate.borrow().settings.show_hidden,
            );

            fmstate.borrow_mut().set_path(file);
            sidebar_selection.unselect_all();
        }
    ));

    list_view.connect_activate(glib::clone!(
        #[strong]
        fmstate,
        #[weak]
        files_list,
        #[weak]
        sidebar_selection,
        move |lv, position| {
            if let Some(obj) = lv.model().and_then(|m| m.item(position)) {
                let string_obj = obj.downcast::<gtk4::StringObject>().unwrap();
                let file_str = string_obj.string();

                // Try local path first, otherwise fallback to URI
                let file = if std::path::Path::new(&file_str).exists() {
                    gio::File::for_path(&file_str)
                } else {
                    gio::File::for_uri(&file_str)
                };

                let file_type = file
                    .query_file_type(gio::FileQueryInfoFlags::NONE, None::<&gio::Cancellable>);

                if file_type == gio::FileType::Directory {
                    files_panel::populate_files_list(
                        &files_list,
                        &file,
                        &fmstate.borrow().settings.show_hidden,
                    );
                    fmstate.borrow_mut().set_path(file.clone());
                    sidebar_selection.unselect_all();
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

                file_area_menu.connect_closed(glib::clone!(
                    #[strong]
                    fmstate,
                    move |_| {
                        fmstate.borrow_mut().popup_focused_file = None;
                    }
                ));
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
