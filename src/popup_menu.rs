use crate::state::FmState;
use gtk4::glib;
use gtk4::prelude::{BoxExt, ButtonExt, PopoverExt, WidgetExt};
use gtk4::{Box as GtkBox, Button, Orientation, Popover};
use std::cell::RefCell;
use std::env;
use std::path::Path;
use std::process::Command;
use std::rc::Rc;

pub fn get_empty_right_click(content_area: &gtk4::Box, fmstate: Rc<RefCell<FmState>>) -> Popover {
    let popover = Popover::new();
    popover.set_parent(content_area);
    let vbox = GtkBox::new(Orientation::Vertical, 0);

    let new_folder = Button::with_label("New Folder");
    let open_terminal = Button::with_label("Open Terminal Here");

    new_folder.connect_clicked(|_| {
        println!("New Folder clicked");
    });

    open_terminal.connect_clicked(move |_| {
        let terminal_cmd = env::var("TERMINAL").unwrap_or_else(|_| "xterm".to_string());
        let path = &fmstate.borrow().current_path;

        if let Err(err) = Command::new(&terminal_cmd).current_dir(path).spawn() {
            eprintln!("Failed to open terminal '{}': {}", terminal_cmd, err);
        }
    });

    vbox.append(&new_folder);
    vbox.append(&open_terminal);

    popover.set_child(Some(&vbox));

    popover
}

pub fn get_file_right_click(content_area: &gtk4::Box, fmstate: Rc<RefCell<FmState>>) -> Popover {
    let popover = Popover::new();
    popover.set_parent(content_area);
    let vbox = GtkBox::new(Orientation::Vertical, 0);

    let open_file = Button::with_label("Open File");
    let open_in_terminal = Button::with_label("Open in Terminal");
    let move_to_trash = Button::with_label("Move to Trash");

    open_file.connect_clicked(|_| {
        println!("Open File clicked");
    });

    open_in_terminal.connect_clicked(glib::clone!(
        #[strong]
        fmstate,
        move |_| {
            let terminal_cmd = env::var("TERMINAL").unwrap_or_else(|_| "xterm".to_string());
            if let Some(path) = &fmstate.borrow().popup_focused_file {
                if let Err(err) = Command::new(&terminal_cmd).current_dir(path).spawn() {
                    eprintln!("Failed to open terminal '{}': {}", terminal_cmd, err);
                }
            };
        }
    ));

    move_to_trash.connect_clicked(|_| {
        println!("Move to Trash clicked");
    });

    if let Some(path) = &fmstate.borrow().popup_focused_file {
        // if dir, show open in terminal
        // if not dir (i.e a file), then show open file
        if Path::new(path).is_dir() {
            vbox.append(&open_in_terminal);
        } else {
            vbox.append(&open_file);
        }
    }

    vbox.append(&move_to_trash);

    popover.set_child(Some(&vbox));

    popover
}
