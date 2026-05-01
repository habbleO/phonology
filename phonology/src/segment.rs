use std::vec::Vec;
use super::*;

#[allow(unused)]
#[derive(PartialEq)]
pub struct Segment {
    name: String,
    features: Vec<feature::Feature>
}

#[allow(unused)]
impl Segment {
    pub fn new<T: Into<String>>(
        name: T, 
        features: Vec<feature::Feature>) -> Self {
        return Self{name: name.into(), features}
    }

    pub fn get_name(&self) -> &str {
        return &self.name;
    }

    pub fn get_features(&self) -> &Vec<feature::Feature> {
        return &self.features;
    }
}
