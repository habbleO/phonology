use crate::feature::Feature;

pub fn bilabial() -> Feature {
    return Feature::new("bilabial", Some(true));
}

pub fn min_voice() -> Feature {
    return Feature::new("voice", Some(false));
}

pub fn min_delayed_release() -> Feature {
    return Feature::new("delayed release", Some(false));
}

pub fn syllabic() -> Feature {
    return Feature::new("syllabic", Some(true));
}