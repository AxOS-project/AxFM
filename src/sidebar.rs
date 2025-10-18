use gtk4::prelude::*;
use gtk4::{
    Box as GtkBox, ListView, Orientation, ScrolledWindow, SignalListItemFactory, SingleSelection,
    StringList,
};
use gtk4::{gdk, gio, glib};
use std::path::{PathBuf, Path};
use xdg::BaseDirectories;
use crate::state::FmState;
use std::rc::Rc;
use std::cell::RefCell;

pub fn build_sidebar(fmstate: Rc<RefCell<FmState>>, files_list: &gtk4::StringList) -> (GtkBox, SingleSelection) {
    let sidebar_items = get_sidebar_items();
    let labels: Vec<&str> = sidebar_items.iter().map(|(name, _)| *name).collect();

    let sidebar_list = StringList::new(&labels);
    let sidebar_selection = SingleSelection::new(Some(sidebar_list.clone()));
    sidebar_selection.set_can_unselect(true);
    sidebar_selection.set_autoselect(false);

    let factory = SignalListItemFactory::new();
    factory.connect_setup(glib::clone!(#[strong] sidebar_items, #[weak] files_list, #[strong] fmstate, move |_, item| {
        let hbox = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);

        let icon = gtk4::Image::new();
        icon.set_pixel_size(24);

        let label = gtk4::Label::new(None);
        label.set_xalign(0.0);

        hbox.append(&icon);
        hbox.append(&label);

        hbox.set_margin_start(6);
        hbox.set_margin_end(6);
        hbox.set_margin_top(4);
        hbox.set_margin_bottom(4);

        // add drop target
        let drop_target = gtk4::DropTarget::new(String::static_type(), gdk::DragAction::COPY);
        drop_target.connect_drop(glib::clone!(
            #[strong]
            sidebar_items,
            #[weak_allow_none]
            label,
            #[weak_allow_none]
            files_list,
            #[strong]
            fmstate,
            move |_drop_target, value, _, _| {
                if let Some(label) = label.as_ref() {
                    let label_text = label.text();

                    if let Some((_, target_path)) =
                        sidebar_items.iter().find(|(n, _)| **n == label_text)
                    {
                        if let Ok(uri) = value.get::<glib::GString>() {
                            let src_file = gio::File::for_uri(&uri);
                            let src_filename =
                                src_file.basename().unwrap_or_else(|| "unknown".into());
                            let mut dest_path = PathBuf::from(target_path);
                            dest_path.push(src_filename);
                            let dest_file = gio::File::for_path(&dest_path);

                            match src_file.move_(
                                &dest_file,
                                gio::FileCopyFlags::OVERWRITE,
                                None::<&gio::Cancellable>,
                                None::<&mut dyn FnMut(i64, i64)>,
                            ) {
                                Ok(_) => {
                                    if let Some(files_list) = &files_list {
                                        crate::files_panel::populate_files_list(
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
        hbox.add_controller(drop_target);

        item.set_child(Some(&hbox));
    }));

    factory.connect_bind(glib::clone!(
        #[strong]
        sidebar_items,
        move |_, item| {
            let hbox = item.child().and_downcast::<gtk4::Box>().unwrap();
            let icon = hbox.first_child().and_downcast::<gtk4::Image>().unwrap();
            let label = hbox.last_child().and_downcast::<gtk4::Label>().unwrap();

            let obj = item.item().unwrap().downcast::<gtk4::StringObject>().unwrap();
            let label_text = obj.string();
            label.set_text(&label_text);

            if let Some((name, path)) = sidebar_items.iter().find(|(n, _)| *n == label_text) {
                label.set_tooltip_text(Some(&path.to_string_lossy()));

                // Choose an icon per item
                let icon_name = match *name {
                    "Home" => "user-home",
                    "Documents" => "folder-documents",
                    "Downloads" => "folder-download",
                    "Music" => "folder-music",
                    "Pictures" => "folder-pictures",
                    "Videos" => "folder-videos",
                    "Trash" => "user-trash",
                    _ => "folder",
                };
                icon.set_icon_name(Some(icon_name));
            } else {
                label.set_tooltip_text(None);
                icon.set_icon_name(Some("folder"));
            }
        }
    ));

    let list_view = ListView::new(Some(sidebar_selection.clone()), Some(factory));
    let scroll =
        ScrolledWindow::builder().child(&list_view).min_content_width(180).vexpand(true).build();

    // Building Sidebar
    let sidebar_box = GtkBox::new(Orientation::Vertical, 0);
    sidebar_box.set_hexpand(false);
    sidebar_box.set_width_request(180);

    let heading_box = GtkBox::new(Orientation::Horizontal, 0);
    let heading = gtk4::Label::new(Some("Places"));
    heading.add_css_class("sidebar-heading");
    heading.set_margin_top(6);
    heading.set_margin_bottom(6);
    heading.set_margin_start(12);
    heading.set_margin_end(12);
    heading.set_xalign(0.0);

    let headerbar = crate::headerbar::build_headerbar();

    // spacer to fill space
    let spacer = GtkBox::new(Orientation::Horizontal, 0);
    spacer.set_hexpand(true);

    heading_box.append(&heading);
    heading_box.append(&spacer);
    heading_box.append(&headerbar);

    sidebar_box.append(&heading_box);
    sidebar_box.append(&scroll);

    (sidebar_box, sidebar_selection)
}

fn get_sidebar_items() -> Vec<(&'static str, PathBuf)> {
    let xdg_dirs = BaseDirectories::with_prefix("user-dirs");
    let home = dirs::home_dir().unwrap_or(PathBuf::from("/"));

    vec![
        ("Home", home.clone()),
        ("Documents", xdg_dirs.find_data_file("Documents").unwrap_or(home.join("Documents"))),
        ("Downloads", xdg_dirs.find_data_file("Downloads").unwrap_or(home.join("Downloads"))),
        ("Music", xdg_dirs.find_data_file("Music").unwrap_or(home.join("Music"))),
        ("Pictures", xdg_dirs.find_data_file("Pictures").unwrap_or(home.join("Pictures"))),
        ("Videos", xdg_dirs.find_data_file("Videos").unwrap_or(home.join("Videos"))),
        ("Trash", home.join(".local/share/Trash/files")),
    ]
}
