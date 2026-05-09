use crate::segment::Segment;
use crate::feature::Feature;
use crate::rule::{Rule, Environment};
use std::vec::Vec;
use std::fmt;

#[allow(unused)]
/// Struct that represents a word, with an underlying form. 
/// The underlying form is a sequence of segments.
pub struct Word {
    underlying_form: Vec<Segment>,
    surface_form: Vec<Segment>
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let mut to_write = String::new();

        for char in &self.underlying_form {
            let to_append = char.get_name();
            to_write.push_str(to_append);
        }

        write!(f, "{}", &to_write)
    }
}

#[allow(unused)]
impl Word {
    /// Returns a new underlying form given a vector of segments.
    pub fn new(underlying_form: Vec<Segment>) -> Self {
        return Self{underlying_form: underlying_form.clone(), 
                    surface_form: underlying_form.clone()};
    }

    pub fn get_underlying_form(&self) -> &Vec<Segment> {
        return &self.underlying_form;
    }

    pub fn get_surface_form(&self) -> &Vec<Segment> {
        return &self.surface_form;
    }

    /// Turns a vector of IPA characters into a word. For this to not error, 
    /// each element in the vector must be parseable as a feature matrix.
    /// If a character is not parseable, returns an error.
    pub fn from_vec(vector: Vec<&str>) -> Result<Self, String> {
        let mut result: Vec<Segment> = Vec::new();

        for item in vector{

            let f_matrix = Feature::to_feature_matrix(item)?;

            let segment = Segment::new(item, f_matrix);
            result.push(segment);
        }
        let new_word = Word::new(result);
        return Ok(new_word);
    }

    /// Tries to parse text as a sequence of IPA characters.
    fn parse_as_vec(text: &str) -> Result<Vec<String>, String> {
        let mut ipa_vec: Vec<String> = Vec::new();

        for symbol in text.chars() {
            let new_symbol = symbol.clone().to_string();
            ipa_vec.push(new_symbol);
        }

        return Ok(ipa_vec);
    }

    /// If possible, converts a &str into a word, where each character is 
    /// parsed as an IPA character. If this fails, returns Err.
    pub fn from_str(text: &str) -> Result<Self, String> {
        let ipa_vec = Self::parse_as_vec(text)?;

        let ipa_vec = ipa_vec.iter().map(|x| x.as_str()).collect();

        return Word::from_vec(ipa_vec);
    }

    /// Checks if a position
    fn in_range(&self, pos: i32) -> bool{
        if pos < 0 {
            return false;
        } else if pos > self.underlying_form.len() as i32 - 1 {
            return false;
        } else {
            return true;
        }
    }

    /// Given a position in the word, and a Rule object, checks if the
    /// Rule.left_env matches for the position of the word.
    ///
    /// Todo: Needs optimization!
    /// Todo: make this take an intermediate form, not just 
    /// the underlying form.
    pub fn left_env_match(&self, pos: i32, rule: &Rule) -> bool {
        let left_env = rule.get_left_env();
        let mut current_pos = pos - 1;

        match left_env {
            Some(x) => {
                for elem in x.into_iter().rev() {

                    if (current_pos == -1) && (*elem != Environment::Boundary) {
                        return false;
                    }

                    match elem {
                        Environment::Boundary => {
                            if current_pos != -1 {
                                return false;
                            }
                        },
                        Environment::FeatureMatrix(matrix) => {
                            if !self.in_range(current_pos) {
                                return false;
                            } else {
                                let seg = self.get_surface_form()[current_pos as usize].clone();
                                
                                for feat in matrix {
                                    if seg.is_feature(feat.get_name()) != Some(true) {
                                        return false;
                                    }
                                }
                            }
                        },
                        Environment::Segment(x) =>  {
                            if !self.in_range(current_pos) {
                                return false;
                            } else {
                                let seg = self.get_surface_form()[current_pos as usize].clone();
                                
                                if seg != *x {
                                    return false;
                                }
                            }
                        }
                    };

                    current_pos -= 1;
                }
                return true;
            },
            None => {return true;}
        };
    }
    
}