#[allow(unused)]
#[derive(PartialEq, Clone)]
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