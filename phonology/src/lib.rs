use std::vec::Vec;

#[allow(unused)]
pub struct Segment {
    name: String,
    features: Vec<Feature>
}

#[allow(unused)]
impl Segment {
    fn new<T: Into<String>>(name: T, features: Vec<Feature>) -> Self {
        return Self{name: name.into(), features}
    }
}

#[allow(unused)]
pub struct Feature {
    name: String,
    assignment: Option<bool>
}

#[allow(unused)]
impl Feature {
    fn new<T: Into<String>>(name: T, assignment: Option<bool>) -> Self {
        return Self {name: name.into(), assignment}
    }
}

#[allow(unused)]
fn setup() -> Segment {
    let bilabial = Feature::new("bilabial", Some(true));
    let voiceless = Feature::new("voice", Some(false));
    let stop = Feature::new("delayed release", Some(false));
    let f_vec = vec![bilabial, voiceless, stop];
    let p = Segment::new("p", f_vec);
    return p;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        let p = setup();
        assert!(p.name == String::from("p"));
    }
}
