pub fn load_css() {
    let provider = gtk4::CssProvider::new();

    provider.load_from_data(
        "
	    .sidebar-heading {
	        font-size: 18px;
	        font-weight: bold;
	    }
        .pathbar {
            margin: 5px;
        }
        .remove_styles {
            all: unset;
        }
	",
    );

    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default().expect("No display found"),
        &provider,
        900,
    );
}
