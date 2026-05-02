use std::collections::HashMap;

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

    pub fn get_parse_hash() -> HashMap<String, Vec<Feature>> {
        let parse_hash: HashMap<String, Vec<Feature>> = HashMap::from([
            ("p".to_string(), vec![Feature::BILABIAL(), Feature::MIN_VOICE(), Feature::MIN_DELAYED_RELEASE()]),
            ("o".to_string(), vec![Feature::SYLLABIC()])
        ]);

        return parse_hash;
    }

    pub fn to_feature_matrix(symbol: &str) -> Option<Vec<Feature>>{
        
        let parse_hash = Feature::get_parse_hash();
        return parse_hash.get(symbol).cloned();

    }
    

}