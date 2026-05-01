use crate::segment::Segment;
use std::vec::Vec;

#[allow(unused)]
pub struct Word {
    underlying_form: Vec<Segment>
}

#[allow(unused)]
impl Word {
    pub fn new(underlying_form: Vec<Segment>) -> Self {
        return Self{underlying_form};
    }

    pub fn get_surface_form(&self) -> String {
        let mut result = String::new();

        for char in &self.underlying_form {
            let to_append = char.get_name();
            result.push_str(to_append);
        }

        return result;
    }
}