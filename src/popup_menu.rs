use gtk4::{Popover, Box as GtkBox, Orientation, Button};
use gtk4::prelude::{WidgetExt, ButtonExt, BoxExt, PopoverExt};

pub fn get_empty_right_click(content_area: &gtk4::Box) -> Popover {
    let popover = Popover::new();
    popover.set_parent(content_area);
    let vbox = GtkBox::new(Orientation::Vertical, 0);

    let new_folder = Button::with_label("New Folder");

    new_folder.connect_clicked(|_| {
        println!("New Folder clicked");
    });

    vbox.append(&new_folder);
    vbox.append(&refresh);

    popover.set_child(Some(&vbox));

    popover
}
