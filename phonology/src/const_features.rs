use crate::feature::Feature;

#[allow(unused, non_snake_case)]
impl Feature {
    pub fn BILABIAL() -> Feature {
        /// Returns a +bilabial Feature.
        return Feature::new("bilabial", Some(true));
    }

    pub fn MIN_VOICE() -> Feature {
        /// Returns a -voice Feature.
        return Feature::new("voice", Some(false));
    }

    pub fn MIN_DELAYED_RELEASE() -> Feature {
        /// Returns a -delayed release Feature.
        return Feature::new("delayed release", Some(false));
    }

    pub fn SYLLABIC() -> Feature {
        /// Returns a +syllabic Feature.
        return Feature::new("syllabic", Some(true));
    }
}

pub struct DefaultFeature {
    name: String,
    plus: Vec<&'static str>,
    minus: Vec<&'static str>
}

impl DefaultFeature{
    pub fn new<T: Into<String>>(
        name: T, plus: Vec<&'static str>, minus: Vec<&'static str>) -> Self {

        let name: String = name.into();
        return Self{name, plus, minus};
    }

    pub fn get_name(&self) -> &str {
        return &self.name;
    }

    pub fn get_plus(&self) -> &Vec<&'static str> {
        return &self.plus;
    }

    pub fn get_minus(&self) -> &Vec<&'static str> {
        return &self.minus;
    }
}

