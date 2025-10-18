use crate::state::FmState;
use crate::utils::WidgetDataExt;
use gtk4::gio;
use gtk4::gio::ThemedIcon;
use gtk4::prelude::*;
use gtk4::{
    DragIcon, DragSource, EventControllerMotion, ListView, ScrolledWindow, SignalListItemFactory,
    SingleSelection, StringList,
};
use gtk4::{gdk, glib};
use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;

pub fn build_files_panel(fmstate: Rc<RefCell<FmState>>) -> (ScrolledWindow, StringList, ListView) {
    let files_list = StringList::new(&[]);
    let files_selection = SingleSelection::new(Some(files_list.clone()));

    let factory = SignalListItemFactory::new();
    factory.connect_setup(glib::clone!(
        #[strong]
        fmstate,
        move |_, item| {
            let hbox = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
            let icon = gtk4::Image::new();
            icon.set_pixel_size(24);
            let label = gtk4::Label::new(None);
            hbox.append(&icon);
            hbox.append(&label);
            item.set_child(Some(&hbox));

            // setup hover detection
            let motion = EventControllerMotion::new();

            motion.connect_enter(glib::clone!(
                #[strong]
                fmstate,
                #[weak]
                item,
                move |_, _, _| {
                    if let Some(obj) = item.item() {
                        let file_path = obj.downcast_ref::<gtk4::StringObject>().unwrap().string();
                        fmstate.borrow_mut().hovered_file = Some(file_path);
                    }
                }
            ));
            motion.connect_leave(glib::clone!(
                #[strong]
                fmstate,
                move |_| {
                    fmstate.borrow_mut().hovered_file = None;
                }
            ));

            hbox.add_controller(motion);

            // setup drag
            let drag_source = DragSource::new();
            drag_source.set_actions(gdk::DragAction::COPY);

            // This provides the data when drag starts
            drag_source.connect_prepare(glib::clone!(
                #[strong]
                fmstate,
                move |_, _, _| {
                    if let Some(file) = &fmstate.borrow().hovered_file {
                        let uri = gtk4::gio::File::for_path(file).uri();
                        Some(gdk::ContentProvider::for_value(&uri.to_value()))
                    } else {
                        None
                    }
                }
            ));

            drag_source.connect_drag_begin(glib::clone!(
                #[weak]
                icon,
                move |_, drag| {
                    if let Some(gicon) = icon.gicon() {
                        let paintable = gtk4::IconTheme::default().lookup_by_gicon(
                            &gicon,
                            24,
                            1,
                            gtk4::TextDirection::None,
                            gtk4::IconLookupFlags::empty(),
                        );
                        DragIcon::set_from_paintable(drag, &paintable, 0, 0);
                    } else {
                        let icon_theme = gtk4::IconTheme::default();
                        let icon = icon_theme.lookup_by_gicon(
                            &ThemedIcon::new("text-x-generic"),
                            24,
                            1,
                            gtk4::TextDirection::None,
                            gtk4::IconLookupFlags::empty(),
                        );
                        DragIcon::set_from_paintable(drag, &icon, 0, 0);
                    }
                }
            ));

            hbox.add_controller(drag_source);
        }
    ));

    factory.connect_bind(glib::clone!(
        #[weak]
        files_list,
        #[strong]
        fmstate,
        move |_, item| {
            let hbox = item.child().and_downcast::<gtk4::Box>().unwrap();

            let icon = hbox.first_child().and_downcast::<gtk4::Image>().unwrap();
            let label = hbox.last_child().and_downcast::<gtk4::Label>().unwrap();
            let obj = item.item().unwrap().downcast::<gtk4::StringObject>().unwrap();

            let file = gtk4::gio::File::for_path(&obj.string());
            if let Ok(info) = file.query_info(
                "standard::icon,standard::display-name,standard::type",
                gtk4::gio::FileQueryInfoFlags::NONE,
                gtk4::gio::Cancellable::NONE,
            ) {
                if let Some(icon_gio) = info.icon() {
                    icon.set_from_gicon(&icon_gio);
                }
                label.set_text(info.display_name().as_str());
            } else {
                label.set_text(&obj.string());
            }

            // add drop target
            let drop_target = gtk4::DropTarget::new(String::static_type(), gdk::DragAction::COPY);

            drop_target.connect_drop(glib::clone!(
                #[weak_allow_none]
                files_list,
                #[strong]
                fmstate,
                move |drop_target, value, _, _| {
                    if let Some(target_widget) = drop_target.widget() {
                        if let Some(target_path) =
                            target_widget.get_typed_data::<glib::GString>("file-path")
                        {
                            if let Ok(uri) = value.get::<glib::GString>() {
                                let src_file = gio::File::for_uri(&uri);
                                let mut dest_path = PathBuf::from(&target_path);

                                // get the filename from the source file
                                let src_filename =
                                    src_file.basename().unwrap_or_else(|| "unknown".into());

                                // creating the absolute dest path
                                dest_path.push(src_filename);
                                let dest_file = gtk4::gio::File::for_path(dest_path);

                                match src_file.move_(
                                    &dest_file,
                                    gio::FileCopyFlags::OVERWRITE,
                                    None::<&gio::Cancellable>,
                                    None::<&mut dyn FnMut(i64, i64)>,
                                ) {
                                    Ok(_) => {
                                        if let Some(files_list) = &files_list {
                                            populate_files_list(
                                                files_list,
                                                Path::new(&fmstate.borrow().current_path),
                                            );
                                        }
                                    }
                                    Err(e) => eprintln!("Error while moving file: {}", e),
                                }
                            }
                        }
                    }

                    true
                }
            ));

            if let Some(obj) = item.item() {
                let file_path = obj.downcast_ref::<gtk4::StringObject>().unwrap().string();
                let is_dir = Path::new(&file_path).is_dir();

                hbox.set_typed_data("file-path", file_path);
                hbox.set_flag("is-dir", is_dir);
                hbox.track_widget_cleanup();

                if is_dir {
                    hbox.add_controller(drop_target);
                }
            }
        }
    ));

    let list_view = ListView::new(Some(files_selection.clone()), Some(factory));
    let scroll = ScrolledWindow::builder().child(&list_view).vexpand(true).hexpand(true).build();

    (scroll, files_list, list_view)
}

pub fn populate_files_list(files_list: &gtk4::StringList, path: &std::path::Path) {
    while files_list.n_items() > 0 {
        files_list.remove(0);
    }
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            files_list.append(&entry.path().to_string_lossy());
        }
    }
}
