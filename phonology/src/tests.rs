use crate::feature::Feature;
use crate::segment::Segment;
use crate::word::Word;

use crate::const_features::*;

fn setup() -> Segment {
    let f_vec = vec![bilabial(), min_voice(), min_delayed_release()];
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

    assert!(p.get_features().contains(&bilabial()));
    assert!(p.get_features().contains(&min_voice()));
    assert!(p.get_features().contains(&min_delayed_release()));
}

#[test]
fn get_feature_name() {
    let voiceless = min_voice();
    assert!(voiceless.get_name() == "voice");
}

#[test]
fn get_feature_assignment() {
    let voiceless = min_voice();
    assert!(voiceless.get_assignment() == &Some(false));   
}

#[test]
fn get_zero_assignment() {
    let zero_voice = Feature::new("voice", None);
    assert!(zero_voice.get_assignment() == &None);
}

#[test]
fn get_word_surface_form() {

    let p_vec = vec![bilabial(), min_voice(), min_delayed_release()];
    let p = Segment::new("p", p_vec);
    let p_2 = p.clone();

    let o_vec = vec![syllabic()];
    let o = Segment::new("o", o_vec);

    let pop_ur = vec![p, o, p_2];
    let pop = Word::new(pop_ur);

    let pop_sr = pop.get_surface_form();
    assert!(pop_sr == String::from("pop"));
}