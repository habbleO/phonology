use super::*;
use crate::feature::Feature;
use crate::segment::Segment;
use crate::word::Word;

fn setup() -> Segment {
    let bilabial = Feature::new("bilabial", Some(true));
    let voiceless = Feature::new("voice", Some(false));
    let stop = Feature::new("delayed release", Some(false));
    let f_vec = vec![bilabial, voiceless, stop];
    let p = Segment::new("p", f_vec);
    return p;
}

#[test]
fn get_segment_name() {
    let p = setup();
    let p_name = p.get_name();
    assert!(p_name == "p");
}

#[test]
fn get_segment_features() {
    let p = setup();
    let bilabial = Feature::new("bilabial", Some(true));
    let voiceless = Feature::new("voice", Some(false));
    let stop = Feature::new("delayed release", Some(false));

    assert!(p.get_features().contains(&bilabial));
    assert!(p.get_features().contains(&voiceless));
    assert!(p.get_features().contains(&stop));
}

#[test]
fn get_feature_name() {
    let voiceless = Feature::new("voice", Some(false));
    assert!(voiceless.get_name() == "voice");
}

#[test]
fn get_feature_assignment() {
    let voiceless = Feature::new("voice", Some(false));
    assert!(voiceless.get_assignment() == &Some(false));   
}

#[test]
fn get_zero_assignment() {
    let zero_voice = Feature::new("voice", None);
    assert!(zero_voice.get_assignment() == &None);
}

#[test]
fn get_word_surface_form() {
    let bilabial = Feature::new("bilabial", Some(true));
    let voiceless = Feature::new("voice", Some(false));
    let stop = Feature::new("delayed release", Some(false));
    let syllabic = Feature::new("syllabic", Some(true));

    let p_vec = vec![bilabial, voiceless, stop];
    let p = Segment::new("p", p_vec);
    let p_2 = p.clone();

    let o_vec = vec![syllabic];
    let o = Segment::new("o", o_vec);

    let pop_ur = vec![p, o, p_2];
    let pop = Word::new(pop_ur);

    let pop_sr = pop.get_surface_form();
    assert!(pop_sr == String::from("pop"));
}