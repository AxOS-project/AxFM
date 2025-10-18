use crate::state::FmState;
use gtk4::glib::Type;
use gtk4::prelude::EditableExt;
use gtk4::prelude::EntryExt;
use gtk4::prelude::ToValue;
use gtk4::prelude::WidgetExt;
use gtk4::{Entry, EntryCompletion, ListStore};
use std::fs;

pub fn build_pathbar(fmstate: &mut FmState) -> Entry {
    let pathbar = Entry::new();

    let completion = EntryCompletion::new();
    completion.set_inline_completion(true);
    completion.set_inline_selection(true);

    let model = ListStore::new(&[Type::STRING]);

    let current_path = fmstate.current_path.clone();

    if let Ok(entries) = fs::read_dir(&current_path) {
        for entry in entries.flatten() {
            let full_path = entry.path();
            let full_path_str = full_path.to_string_lossy().to_string();
            let iter = model.append();
            model.set(&iter, &[(0, &full_path_str.to_value())]);
        }
    }

    completion.set_model(Some(&model));
    completion.set_text_column(0);

    pathbar.set_completion(Some(&completion));

    pathbar.set_text(fmstate.current_path.to_str().expect("Failed to convert PathBuf to &str"));

    fmstate.connect_path_changed({
        let pathbar = pathbar.clone();
        move |new_path| {
            if let Some(s) = new_path.to_str() {
                pathbar.set_text(s);

                model.clear();

                if let Ok(entries) = fs::read_dir(&s) {
                    for entry in entries.flatten() {
                        let full_path = entry.path();
                        let full_path_str = full_path.to_string_lossy().to_string();
                        let iter = model.append();
                        model.set(&iter, &[(0, &full_path_str.to_value())]);
                    }
                }
            }
        }
    });

    pathbar.add_css_class("pathbar");

    pathbar
}
