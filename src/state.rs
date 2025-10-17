use std::path::PathBuf;

pub struct FmState {
    pub current_path: PathBuf,
    pub on_path_changed: Vec<Box<dyn Fn(&PathBuf)>>,
}

impl FmState {
    pub fn new(current_path: PathBuf) -> Self {
        Self {
            current_path,
            on_path_changed: Vec::new(),
        }
    }

    pub fn set_path(&mut self, new_path: PathBuf) {
        self.current_path = new_path.clone();
        for cb in self.on_path_changed.iter() {
            cb(&new_path);
        }
    }

    pub fn connect_path_changed<F: Fn(&PathBuf) + 'static>(&mut self, f: F) {
        self.on_path_changed.push(Box::new(f));
    }
}
