use crate::default_feature::DefaultFeature;

#[allow(unused)]
pub const IPA_SYMBOLS: [&'static str; 14] = [
    "p", "b", "t", "d", "k", "g",
    "m", "n", "ŋ",
    "a", "e", "i", "o", "u"
];

#[allow(unused)]
#[derive(PartialEq, Clone, Debug)]
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

    pub fn get_default_features() -> Vec<DefaultFeature> {
        // Features are from Hayes, 2011, Introductory Phonology
        // Currently used symbols and features
        // p, b, t, d, k, g, a, e, i, o, u
        // m, n, ŋ
        // labial, coronal, dorsal, voice, syllabic,
        // high, low, front, back
        // consonantal, sonorant, nasal

        let labial = DefaultFeature::new(
            "labial", 
            vec!["p", "b", "m"], 
            vec!["t", "d", "k", "g", 
                 "a", "e", "i", "o", "u", "n", "ŋ"]);
        
        let coronal = DefaultFeature::new(
            "coronal",
            vec!["t", "d", "n"],
            vec!["p", "b", "k", "g", 
                 "a", "e", "i", "o", "u", "m", "ŋ"]);
        
        let dorsal = DefaultFeature::new(
            "dorsal",
            vec!["k", "g", "a", "e", "i", "o", "u", "ŋ"],
            vec!["p", "b", "t", "d", "m", "n"]);

        let voice = DefaultFeature::new(
            "voice",
            vec!["b", "d", "g", 
                 "a", "e", "i", "o", "u", "m", "n", "ŋ"], 
            vec!["p", "t", "k"]);

        let consonantal = DefaultFeature::new(
            "consonantal",
            vec!["p", "b", "t", "d", "k", "g", "m", "n", "ŋ"],
            vec!["a", "e", "i", "o", "u"]
        );

        let sonorant = DefaultFeature::new(
            "sonorant",
            vec!["m", "n", "ŋ", "a", "e", "i", "o", "u"],
            vec!["p", "b", "t", "d", "k", "g"]
        );

        let nasal = DefaultFeature::new(
            "nasal",
            vec!["m", "n", "ŋ"],
            vec!["p", "b", "t", "d", "k", "g",
                 "a", "e", "i", "o", "u"]
        );

        let syllabic = DefaultFeature::new(
            "syllabic",
            vec!["a", "e", "i", "o", "u"],
            vec!["p", "b", "t", "d", "k", "g", "m", "n", "ŋ"]);

        let high = DefaultFeature::new(
            "high",
            vec!["i", "u", "k", "g", "ŋ"],
            vec!["a", "e", "o"]
        );

        let low = DefaultFeature::new(
            "low",
            vec!["a"],
            vec!["i", "e", "u", "o", "k", "g", "ŋ"]
        );

        let front = DefaultFeature::new(
            "front",
            vec!["i", "e"],
            vec!["a", "u", "o"]
        );

        let back = DefaultFeature::new(
            "back",
            vec!["u", "o"],
            vec!["a", "i", "e"]
        );

        let all_defaults = vec![
            labial, coronal,
            dorsal, voice,
            consonantal, sonorant,
            nasal, syllabic, 
            high, low, 
            front, back
        ];

        return all_defaults;
    }

    pub fn to_feature_matrix(symbol: &str) -> Result<Vec<Feature>, String>{
        /// TODO: Add Doc Comment.

        let all_defaults = Feature::get_default_features();

        let mut result: Vec<Feature> = Vec::new();
        let mut feat_assignment: Option<bool>;

        for feat in all_defaults {

            if feat.get_plus().contains(&symbol) {
                feat_assignment = Some(true);
            } else if feat.get_minus().contains(&symbol) {
                feat_assignment = Some(false);
            } else {
                feat_assignment = None;
            }

            
            let feat_name = feat.get_name();
            
            let new_feature = Feature::new(feat_name, feat_assignment);
            result.push(new_feature);
        }

        if result.iter().all(|x| *x.get_assignment() == None) {
            let err_msg = format!("Could not parse {} as a feature matrix.", symbol);
            return Err(err_msg);
        }

        return Ok(result);
    }
    

}