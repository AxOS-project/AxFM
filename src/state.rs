use crate::utils::FMSettings;
use gtk4::gio;
use gtk4::glib::GString;

pub struct FmState {
    pub current_path: gio::File,
    pub on_path_changed: Vec<Box<dyn Fn(&gio::File)>>,
    pub settings: FMSettings,
    pub hovered_file: Option<GString>,
    pub popup_focused_file: Option<GString>,
}

impl FmState {
    pub fn new(current_path: gio::File) -> Self {
        Self {
            current_path,
            on_path_changed: Vec::new(),
            settings: FMSettings::new(),
            hovered_file: None,
            popup_focused_file: None,
        }
    }

    pub fn set_path(&mut self, new_path: gio::File) {
        self.current_path = new_path.clone();
        for cb in self.on_path_changed.iter() {
            cb(&new_path);
        }
    }

    pub fn connect_path_changed<F: Fn(&gio::File) + 'static>(&mut self, f: F) {
        self.on_path_changed.push(Box::new(f));
    }
}
