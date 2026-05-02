use std::collections::HashMap;

#[allow(unused)]
#[derive(PartialEq, Clone)]
pub struct Feature {
    /// A phonological Feature object. It has a name and an assignment, which can be +, -, or 0 / underspecified / undefined.
    /// A [+X] feature is represented by Some(true).
    /// A [-X] feature is represented by Some(false).
    /// A [0X] feature is represented by None.
    name: String,
    assignment: Option<bool>
}

#[allow(unused)]
impl Feature {

    pub fn new<T: Into<String>>(name: T, assignment: Option<bool>) -> Self {
        /// Creates a new feature with the given name and assignment.
        return Self {name: name.into(), assignment}
    }

    pub fn get_name(&self) -> &str {
        /// Returns a reference to the Feature's name.
        return &self.name;
    }

    pub fn get_assignment(&self) -> &Option<bool> {
        /// Returns a reference to the Feature's assignment.
        return &self.assignment;
    }

    pub fn get_parse_hash() -> HashMap<&'static str, Vec<Feature>> {
        /// The phonology crate supports parsing IPA characters as Feature matrices. This is done using a HashMap containing all character-feature correspondences. get_parse_hash() returns that HashMap.
        let parse_hash: HashMap<&'static str, Vec<Feature>> = HashMap::from([
            ("p", vec![Feature::BILABIAL(), Feature::MIN_VOICE(), Feature::MIN_DELAYED_RELEASE()]),
            ("o", vec![Feature::SYLLABIC()])
        ]);

        return parse_hash;
    }

    pub fn to_feature_matrix(symbol: &str) -> Option<Vec<Feature>>{
        /// If symbol can be parsed as a feature matrix using the HashMap in get_parse_hash, return Some(feature_matrix). If it cannot be parsed as a feature matrix, returns None.
        let parse_hash = Feature::get_parse_hash();
        return parse_hash.get(symbol).cloned();

    }
    

}