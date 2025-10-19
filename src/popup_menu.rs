use crate::state::FmState;
use gtk4::prelude::*;
use gtk4::{
    Box as GtkBox, Label, ListView, Popover, ScrolledWindow, SignalListItemFactory, SingleSelection,
    StringList,
};
use gtk4::{gio, glib};
use std::cell::RefCell;
use std::env;
use std::process::Command;
use std::rc::Rc;
use std::path::Path;

struct MenuItem<'a> {
    label: &'a str,
    icon_name: &'a str,
    show_if_file: bool,
    show_if_dir: bool,
}

pub fn get_empty_right_click(content_area: &GtkBox, fmstate: Rc<RefCell<FmState>>) -> Popover {
    let popover = Popover::new();
    popover.set_parent(content_area);

    let menu_items: Vec<Rc<MenuItem>> = vec![
        Rc::new(MenuItem {
            label: "New Folder",
            icon_name: "folder-new-symbolic",
            show_if_file: true,
            show_if_dir: true,
        }),
        Rc::new(MenuItem {
            label: "Open Terminal Here",
            icon_name: "utilities-terminal-symbolic",
            show_if_file: true,
            show_if_dir: true,
        }),
    ];

    let items_to_show: Vec<Rc<MenuItem>> = menu_items.clone();

    let string_list: StringList = StringList::new(
        &items_to_show.iter().map(|item| item.label).collect::<Vec<_>>()
    );

    let selection_model = SingleSelection::new(Some(string_list.clone()));
    selection_model.set_can_unselect(true);
    selection_model.set_autoselect(false);
    selection_model.unselect_all();

    let factory = SignalListItemFactory::new();
    factory.connect_setup(|_, list_item| {
        let row = GtkBox::new(gtk4::Orientation::Horizontal, 6);
        let image = gtk4::Image::new();
        row.append(&image);
        let label = Label::new(None);
        label.set_xalign(0.0);
        row.append(&label);

        list_item.set_child(Some(&row));
    });

    factory.connect_bind(glib::clone!(#[strong] items_to_show, move |_, list_item| {
        let row = list_item.child().unwrap().downcast::<GtkBox>().unwrap();
        let image = row.first_child().unwrap().downcast::<gtk4::Image>().unwrap();
        let label = row.last_child().unwrap().downcast::<Label>().unwrap();

        let obj = list_item.item().unwrap().downcast::<gtk4::StringObject>().unwrap();
        let text = obj.string();
        label.set_text(&text);

        if let Some(menu_item) = items_to_show.iter().find(|i| i.label == text) {
            image.set_icon_name(Some(menu_item.icon_name));
        }
    }));

    selection_model.connect_selected_notify(glib::clone!(#[weak] popover, #[strong] fmstate, move |sel| {
        if let Some(item) = sel.selected_item() {
            let obj = item.downcast_ref::<gtk4::StringObject>().unwrap();
            let text = obj.string();

            match text.as_str() {
                "New Folder" => println!("New Folder clicked"),
                "Open Terminal Here" => {
                    let terminal_cmd = env::var("TERMINAL").unwrap_or_else(|_| "xterm".to_string());
                    let file = &fmstate.borrow().current_path;

                    if let Some(local_path) = file.path() {
                        if let Err(err) =
                            Command::new(&terminal_cmd).current_dir(local_path).spawn()
                        {
                            eprintln!("Failed to open terminal '{}': {}", terminal_cmd, err);
                        }
                    } else {
                        eprintln!(
                            "Cannot open terminal: current path is virtual or remote: {}",
                            file.uri()
                        );
                    }
                }
                _ => {}
            }

            sel.unselect_all();
            popover.popdown();
        }
    }));

    let list_view = ListView::new(Some(selection_model), Some(factory));
    let scroll = ScrolledWindow::new();
    scroll.set_child(Some(&list_view));
    scroll.set_vexpand(true);
    scroll.set_min_content_width(180);

    popover.set_child(Some(&scroll));
    popover
}

pub fn get_file_right_click(
    content_area: &GtkBox,
    fmstate: Rc<RefCell<FmState>>,
    files_list: &gtk4::StringList,
) -> Popover {
    let popover = Popover::new();
    popover.set_parent(content_area);

    popover.connect_show(glib::clone!(
        #[strong] fmstate,
        #[weak] files_list,
        move |popover| {
            let fmstate_ref = fmstate.borrow();
            if let Some(path) = &fmstate_ref.popup_focused_file {
                let menu_items: Vec<Rc<MenuItem>> = vec![
                    Rc::new(MenuItem {
                        label: "Open File",
                        icon_name: "document-open-symbolic",
                        show_if_file: true,
                        show_if_dir: false,
                    }),
                    Rc::new(MenuItem {
                        label: "Move To Trash",
                        icon_name: "user-trash-symbolic",
                        show_if_file: true,
                        show_if_dir: true,
                    }),
                    Rc::new(MenuItem {
                        label: "Open in Terminal",
                        icon_name: "utilities-terminal-symbolic",
                        show_if_file: false,
                        show_if_dir: true,
                    }),
                ];

                let path = Path::new(path);
                let is_file = path.is_file();
                let is_dir = path.is_dir();

                let items_to_show: Vec<Rc<MenuItem>> = menu_items
                    .into_iter()
                    .filter(|item| (item.show_if_file && is_file) || (item.show_if_dir && is_dir))
                    .collect();
                let string_list: StringList = StringList::new(
                    &items_to_show.iter().map(|item| item.label).collect::<Vec<_>>()
                );
                let selection_model = SingleSelection::new(Some(string_list.clone()));
                selection_model.set_can_unselect(true);
                selection_model.set_autoselect(false);
                selection_model.unselect_all();

                let factory = SignalListItemFactory::new();
                factory.connect_setup(|_, list_item| {
                    let row = GtkBox::new(gtk4::Orientation::Horizontal, 6);
                    let image = gtk4::Image::new();
                    row.append(&image);
                    let label = Label::new(None);
                    label.set_xalign(0.0);
                    row.append(&label);

                    list_item.set_child(Some(&row));
                });

                factory.connect_bind(glib::clone!(#[strong] items_to_show, move |_, list_item| {
                    let row = list_item.child().unwrap().downcast::<GtkBox>().unwrap();
                    let image = row.first_child().unwrap().downcast::<gtk4::Image>().unwrap();
                    let label = row.last_child().unwrap().downcast::<Label>().unwrap();

                    let obj = list_item.item().unwrap().downcast::<gtk4::StringObject>().unwrap();
                    let text = obj.string();
                    label.set_text(&text);

                    if let Some(menu_item) = items_to_show.iter().find(|i| i.label == text) {
                        image.set_icon_name(Some(menu_item.icon_name));
                    }
                }));

                selection_model.connect_selected_notify(glib::clone!(#[strong] fmstate, #[weak] files_list, #[weak] popover, move |sel| {
                    if let Some(item) = sel.selected_item() {
                        let obj = item.downcast_ref::<gtk4::StringObject>().unwrap();
                        let text = obj.string();

                        match text.as_str() {
                            "Open File" => println!("Open File clicked"),
                            "Open in Terminal" => {
                                let terminal_cmd =
                                    env::var("TERMINAL").unwrap_or_else(|_| "xterm".to_string());
                                if let Some(path) = &fmstate.borrow().popup_focused_file {
                                    if let Err(err) =
                                        Command::new(&terminal_cmd).current_dir(path).spawn()
                                    {
                                        eprintln!("Failed to open terminal '{}': {}", terminal_cmd, err);
                                    }
                                }
                            }
                            "Move to Trash" => {
                                if let Some(path) = &fmstate.borrow().popup_focused_file {
                                    let file = gio::File::for_path(path);
                                    match file.trash(None::<&gio::Cancellable>) {
                                        Ok(_) => {
                                            let fmstate_ref = fmstate.borrow();
                                            crate::files_panel::populate_files_list(
                                                &files_list,
                                                &fmstate_ref.current_path,
                                                &fmstate_ref.settings.show_hidden,
                                            );
                                        }
                                        Err(e) => eprintln!("Error while moving to trash: {}", e),
                                    }
                                }
                            }
                            _ => {}
                        }

                        sel.unselect_all();
                        popover.popdown();
                    }
                }));

                let list_view = ListView::new(Some(selection_model), Some(factory));
                let scroll = ScrolledWindow::new();
                scroll.set_child(Some(&list_view));
                scroll.set_vexpand(true);
                scroll.set_min_content_width(180);

                popover.set_child(Some(&scroll));
            }
        }
    ));

    popover
}
