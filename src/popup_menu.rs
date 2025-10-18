use gtk4::prelude::{BoxExt, ButtonExt, PopoverExt, WidgetExt};
use gtk4::{Box as GtkBox, Button, Orientation, Popover};

pub fn get_empty_right_click(content_area: &gtk4::Box) -> Popover {
    let popover = Popover::new();
    popover.set_parent(content_area);
    let vbox = GtkBox::new(Orientation::Vertical, 0);

    let new_folder = Button::with_label("New Folder");

    new_folder.connect_clicked(|_| {
        println!("New Folder clicked");
    });

    vbox.append(&new_folder);

    popover.set_child(Some(&vbox));

    popover
}

pub fn get_file_right_click(content_area: &gtk4::Box) -> Popover {
    let popover = Popover::new();
    popover.set_parent(content_area);
    let vbox = GtkBox::new(Orientation::Vertical, 0);

    let open_file = Button::with_label("Open File");

    open_file.connect_clicked(|_| {
        println!("Open File clicked");
    });

    vbox.append(&open_file);

    popover.set_child(Some(&vbox));

    popover
}
