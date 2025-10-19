pub enum FileView {
    IconView,
    ListView,
}

pub struct FMSettings {
    pub show_hidden: bool,
    pub file_view: FileView,
}

impl FMSettings {
    pub fn new() -> Self {
        Self { show_hidden: false, file_view: FileView::IconView }
    }
}
