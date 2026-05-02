use crate::feature::Feature;
use crate::segment::Segment;
use crate::word::Word;

#[allow(unused)]
use crate::const_features::*;

fn setup() -> Segment {
    let f_vec = vec![
        Feature::BILABIAL(), 
        Feature::MIN_VOICE(), 
        Feature::MIN_DELAYED_RELEASE()];
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

    assert!(p.get_features().contains(&Feature::BILABIAL()));
    assert!(p.get_features().contains(&Feature::MIN_VOICE()));
    assert!(p.get_features().contains(&Feature::MIN_DELAYED_RELEASE()));
}

#[test]
fn get_feature_name() {
    let voiceless = Feature::MIN_VOICE();
    assert!(voiceless.get_name() == "voice");
}

#[test]
fn get_feature_assignment() {
    let voiceless = Feature::MIN_VOICE();
    assert!(voiceless.get_assignment() == &Some(false));   
}

#[test]
fn get_zero_assignment() {
    let zero_voice = Feature::new("voice", None);
    assert!(zero_voice.get_assignment() == &None);
}

#[test]
fn get_word_surface_form() {

    let p = Segment::new("p", Feature::to_feature_matrix("p").unwrap());
    let o = Segment::new("o", Feature::to_feature_matrix("o").unwrap());

    let pop_ur = vec![p.clone(), o, p.clone()];
    let pop = Word::new(pop_ur);

    let pop_sr = pop.get_surface_form();

    assert!(pop_sr == String::from("pop"));
}

#[test]
fn symbol_parsing() {
    let p = Feature::to_feature_matrix("p").expect("This symbol isn't int the parse_hash.");

    assert!(p.contains(&Feature::BILABIAL()));
    assert!(p.contains(&Feature::MIN_VOICE()));
    assert!(p.contains(&Feature::MIN_DELAYED_RELEASE()));

    let o = Feature::to_feature_matrix("o").expect("This symbol isn't int the parse_hash.");
    assert!(o.contains(&Feature::SYLLABIC()));
    assert!(!o.contains(&Feature::BILABIAL()));
}

#[test]
fn word_parsing() {
    let word_vec = vec!["p", "o", "p"];
    let word_struct = Word::from_vec(word_vec);
    let word_sr = word_struct.unwrap().get_surface_form();
    assert_eq!(&word_sr, "pop");
}