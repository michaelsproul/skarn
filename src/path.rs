use std::path::Path;

pub trait StringComponents {
    fn string_components(&self) -> Vec<String>;
}

impl StringComponents for Path {
    fn string_components(&self) -> Vec<String> {
        self.components().map(|c| c.as_os_str().to_string_lossy().into_owned()).collect()
    }
}
