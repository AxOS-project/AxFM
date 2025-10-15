use gtk4::prelude::*;
use gtk4::{
    ListView, ScrolledWindow, SignalListItemFactory, SingleSelection,
    StringList,
};
use std::path::PathBuf;
use xdg::BaseDirectories;

pub fn build_sidebar() -> (gtk4::Box, SingleSelection) {
    let sidebar_items = get_sidebar_items();
    let labels: Vec<&str> = sidebar_items.iter().map(|(name, _)| *name).collect();

    let sidebar_list = StringList::new(&labels);
    let sidebar_selection = SingleSelection::new(Some(sidebar_list.clone()));

    let factory = SignalListItemFactory::new();
    factory.connect_setup(|_, item| {
        let label = gtk4::Label::new(None);
        label.set_margin_top(6);
        label.set_margin_bottom(6);
        label.set_margin_start(12);
        label.set_margin_end(12);
        item.set_child(Some(&label));
    });
    factory.connect_bind(|_, item| {
        let label = item.child().and_downcast::<gtk4::Label>().unwrap();
        let obj = item
            .item()
            .unwrap()
            .downcast::<gtk4::StringObject>()
            .unwrap();
        label.set_text(&obj.string());
    });

    let list_view = ListView::new(Some(sidebar_selection.clone()), Some(factory));
    let scroll = ScrolledWindow::builder()
        .child(&list_view)
        .min_content_width(180)
        .vexpand(true)
        .build();

    // Sidebar box
    let sidebar_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);

    let heading_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    let heading = gtk4::Label::new(Some("Places"));
    heading.add_css_class("sidebar-heading");
    heading.set_margin_top(6);
    heading.set_margin_bottom(6);
    heading.set_margin_start(12);
    heading.set_margin_end(12);
    heading.set_xalign(0.0);

    let headerbar = crate::headerbar::build_headerbar();
    heading_box.append(&heading);
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
        (
            "Documents",
            xdg_dirs
                .find_data_file("Documents")
                .unwrap_or(home.join("Documents")),
        ),
        (
            "Downloads",
            xdg_dirs
                .find_data_file("Downloads")
                .unwrap_or(home.join("Downloads")),
        ),
        (
            "Music",
            xdg_dirs
                .find_data_file("Music")
                .unwrap_or(home.join("Music")),
        ),
        (
            "Pictures",
            xdg_dirs
                .find_data_file("Pictures")
                .unwrap_or(home.join("Pictures")),
        ),
        (
            "Videos",
            xdg_dirs
                .find_data_file("Videos")
                .unwrap_or(home.join("Videos")),
        ),
        ("Trash", home.join(".local/share/Trash")),
    ]
}
