pub struct FMSettings {
    pub show_hidden: bool,
}

impl FMSettings {
    pub fn new() -> Self {
        Self { show_hidden: false }
    }
}
