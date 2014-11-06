pub trait StringComponents {
    fn string_components(&self) -> Vec<String>;
}

impl StringComponents for Path {
    fn string_components(&self) -> Vec<String> {
        self.str_components().map(|s| s.unwrap().to_string()).collect()
    }
}
