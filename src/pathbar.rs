use crate::state::FmState;
use gtk4::Entry;
use gtk4::prelude::EditableExt;
use gtk4::prelude::WidgetExt;

pub fn build_pathbar(fmstate: &mut FmState) -> Entry {
    let pathbar = Entry::new();
    pathbar.set_text(
        fmstate
            .current_path
            .to_str()
            .expect("Failed to convert PathBuf to &str"),
    );

    fmstate.connect_path_changed({
	    let pathbar = pathbar.clone();
	    move |new_path| {
	        if let Some(s) = new_path.to_str() {
	            pathbar.set_text(s);
	        }
	    }
	});

    pathbar.add_css_class("pathbar");

    pathbar
}
