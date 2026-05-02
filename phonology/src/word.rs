use crate::segment::Segment;
use crate::feature::Feature;
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

    pub fn from_vec(vector: Vec<&str>) -> Result<Self, String> {
        let mut result: Vec<Segment> = Vec::new();

        for item in vector{
            match Feature::to_feature_matrix(item) {
                Some(matrix) => {
                    let segment = Segment::new(item, matrix);
                    result.push(segment);
                },
                None => {
                    let err_msg = format!("Could not parse {} as a feature matrix.", item);
                    return Err(err_msg);
                }
            };
        }
        let new_word = Word::new(result);
        return Ok(new_word);
    }
}