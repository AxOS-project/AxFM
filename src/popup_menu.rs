use crate::state::FmState;
use gtk4::prelude::{BoxExt, ButtonExt, FileExt, PopoverExt, WidgetExt};
use gtk4::{Box as GtkBox, Button, Orientation, Popover};
use gtk4::{gio, glib};
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

    new_folder.connect_clicked(glib::clone!(
        #[weak]
        popover,
        move |_| {
            println!("New Folder clicked");
            popover.popdown();
        }
    ));

    open_terminal.connect_clicked(glib::clone!(
        #[weak]
        popover,
        #[strong]
        fmstate,
        move |_| {
            let terminal_cmd = env::var("TERMINAL").unwrap_or_else(|_| "xterm".to_string());
            let file = &fmstate.borrow().current_path;

            if let Some(local_path) = file.path() {
                if let Err(err) = Command::new(&terminal_cmd).current_dir(local_path).spawn() {
                    eprintln!("Failed to open terminal '{}': {}", terminal_cmd, err);
                }
            } else {
                eprintln!(
                    "Cannot open terminal: current path is virtual or remote: {}",
                    file.uri()
                );
            }

            popover.popdown();
        }
    ));

    vbox.append(&new_folder);
    vbox.append(&open_terminal);

    popover.set_child(Some(&vbox));
    popover
}

pub fn get_file_right_click(
    content_area: &gtk4::Box,
    fmstate: Rc<RefCell<FmState>>,
    files_list: &gtk4::StringList,
) -> Popover {
    let popover = Popover::new();
    popover.set_parent(content_area);

    popover.connect_show(glib::clone!(
        #[strong]
        fmstate,
        #[weak]
        files_list,
        move |popover| {
            let vbox = GtkBox::new(Orientation::Vertical, 0);

            let open_file = Button::with_label("Open File");
            let open_in_terminal = Button::with_label("Open in Terminal");
            let move_to_trash = Button::with_label("Move to Trash");

            // Reconnect handlers if needed
            open_file.connect_clicked(|_| println!("Open File clicked"));

            open_in_terminal.connect_clicked(glib::clone!(
                #[strong]
                fmstate,
                #[weak]
                popover,
                move |_| {
                    let terminal_cmd = env::var("TERMINAL").unwrap_or_else(|_| "xterm".to_string());
                    if let Some(path) = &fmstate.borrow().popup_focused_file {
                        if let Err(err) = Command::new(&terminal_cmd).current_dir(path).spawn() {
                            eprintln!("Failed to open terminal '{}': {}", terminal_cmd, err);
                        }
                    };

                    popover.popdown();
                }
            ));

            move_to_trash.connect_clicked(glib::clone!(
                #[strong]
                fmstate,
                #[weak]
                popover,
                #[weak]
                files_list,
                move |_| {
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

                    popover.popdown();
                }
            ));

            if let Some(path) = &fmstate.borrow().popup_focused_file {
                if Path::new(path).is_dir() {
                    vbox.append(&open_in_terminal);
                } else {
                    vbox.append(&open_file);
                }
            }

            vbox.append(&move_to_trash);

            popover.set_child(Some(&vbox));
        }
    ));

    popover
}
