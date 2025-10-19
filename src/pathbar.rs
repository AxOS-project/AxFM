use crate::state::FmState;
use gtk4::gio;
use gtk4::glib::Type;
use gtk4::prelude::*;
use gtk4::{Entry, EntryCompletion, ListStore};

pub fn build_pathbar(fmstate: &mut FmState) -> Entry {
    let pathbar = Entry::new();

    let completion = EntryCompletion::new();
    completion.set_inline_completion(true);
    completion.set_inline_selection(true);

    let model = ListStore::new(&[Type::STRING]);

    let current_file = fmstate.current_path.clone();

    // Populate entries
    if let Ok(enumerator) = current_file.enumerate_children(
        "*",
        gio::FileQueryInfoFlags::NONE,
        None::<&gio::Cancellable>,
    ) {
        while let Some(info) = enumerator.next_file(None::<&gio::Cancellable>).unwrap_or(None) {
            let name = info.display_name();
            let child_file = current_file.child(&name);
            let full_path_str = child_file
                .path()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|| child_file.uri().to_string());

            let iter = model.append();
            model.set(&iter, &[(0, &full_path_str.to_value())]);
        }
    }

    completion.set_model(Some(&model));
    completion.set_text_column(0);
    pathbar.set_completion(Some(&completion));

    // Set the pathbar text
    let current_text = current_file
        .path()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|| current_file.uri().to_string());
    pathbar.set_text(&current_text);

    fmstate.connect_path_changed({
        let pathbar = pathbar.clone();
        move |new_file: &gio::File| {
            let text = new_file
                .path()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|| new_file.uri().to_string());
            pathbar.set_text(&text);

            model.clear();

            if let Ok(enumerator) = new_file.enumerate_children(
                "*",
                gio::FileQueryInfoFlags::NONE,
                None::<&gio::Cancellable>,
            ) {
                while let Some(info) =
                    enumerator.next_file(None::<&gio::Cancellable>).unwrap_or(None)
                {
                    let name = info.display_name();
                    let child_file = new_file.child(&name);
                    let full_path_str = child_file
                        .path()
                        .map(|p| p.display().to_string())
                        .unwrap_or_else(|| child_file.uri().to_string());

                    let iter = model.append();
                    model.set(&iter, &[(0, &full_path_str.to_value())]);
                }
            }
        }
    });

    pathbar.add_css_class("pathbar");

    pathbar
}
