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