use std::vec::Vec;


#[cfg(test)]
mod tests;

#[allow(unused)]
pub struct Segment {
    name: String,
    features: Vec<Feature>
}

#[allow(unused)]
impl Segment {
    pub fn new<T: Into<String>>(name: T, features: Vec<Feature>) -> Self {
        return Self{name: name.into(), features}
    }

    pub fn get_name(&self) -> &str {
        return &self.name;
    }

    pub fn get_features(&self) -> &Vec<Feature> {
        return &self.features;
    }
}

#[allow(unused)]
pub struct Feature {
    name: String,
    assignment: Option<bool>
}

#[allow(unused)]
impl Feature {
    pub fn new<T: Into<String>>(name: T, assignment: Option<bool>) -> Self {
        return Self {name: name.into(), assignment}
    }

    pub fn get_name(&self) -> &str {
        return &self.name;
    }

    pub fn get_assignment(&self) -> &Option<bool> {
        return &self.assignment;
    }
}