use crate::feature::{Feature};
use crate::segment::Segment;
use crate::word::Word;
use crate::rule::{Rule, RuleIO, Environment};

#[test]
fn get_underlying_form() {
    let take = Word::from_str("take").unwrap();
    let take_ur = take.get_underlying_form();

    assert_eq!(take_ur[0], Segment::from_symbol("t").unwrap());
    assert_eq!(take_ur[1], Segment::from_symbol("a").unwrap());
    assert_eq!(take_ur[2], Segment::from_symbol("k").unwrap());
    assert_eq!(take_ur[3], Segment::from_symbol("e").unwrap());
}

#[test]
fn get_surface_form() {
    let take = Word::from_str("take").unwrap();
    let take_sr = take.get_underlying_form();

    assert_eq!(take_sr[0], Segment::from_symbol("t").unwrap());
    assert_eq!(take_sr[1], Segment::from_symbol("a").unwrap());
    assert_eq!(take_sr[2], Segment::from_symbol("k").unwrap());
    assert_eq!(take_sr[3], Segment::from_symbol("e").unwrap());
}

#[test]
fn display() {
    let pot = Word::from_str("pot").unwrap();

    let pot_sr = format!("{}", pot);

    assert!(pot_sr == String::from("pot"));
}

#[test]
fn from_vec() {
    // Tests Word::from_vec().
    // Tests parsing a vector of IPA characters as a Word.
    let word_vec = vec!["p", "o", "p"];
    let word_struct = Word::from_vec(word_vec);
    let word_sr = format!("{}", word_struct.unwrap());
    assert_eq!(&word_sr, "pop");
}

#[test]
fn from_vec_fail() {
    // Tests a failure state for Word::from_vec().
    let failure = Word::from_vec(vec!["0", "1", "2"]);
    assert!(failure.is_err());
}

#[test]
fn from_str() {
    // Tests Word::from_str().
    let new_word = Word::from_str("pot").unwrap();
    let new_word_sr = format!("{}", new_word);

    assert_eq!(new_word_sr, "pot");
}

#[test]
fn from_str_fail() {
    // Tests a failure state for Word::from_str().
    let failure = Word::from_str("000");
    assert!(failure.is_err());
}

#[test]
fn left_env_match_boundary() {
    let potato = Word::from_str("potato").unwrap();
    let onset_devoicing: Rule;

    let voice = Feature::new("voice", Some(true));
    let min_voice = Feature::new("voice", Some(false));

    // Onset Devoicing
    {
        let input = RuleIO::FeatureMatrix(vec![voice.clone()]);
        let output = RuleIO::FeatureMatrix(vec![min_voice.clone()]);

        let env = Some(vec![Environment::Boundary]);

        onset_devoicing = Rule::new("Onset Devoicing",
                                    &input, &output, &env, &None);
    }

    let fails = potato.left_env_match(1, &onset_devoicing);
    assert!(fails == false);

    let success = potato.left_env_match(0, &onset_devoicing);
    assert!(success == true);
}

#[test]
fn left_env_match_features() {
    let potato = Word::from_str("potato").unwrap();
    let intervocalic_voicing: Rule;

    let voice = Feature::new("voice", Some(true));
    let min_voice = Feature::new("voice", Some(false));
    let syllabic = Feature::new("syllabic", Some(true));

    // Intervocalic Voicing
    {
        let input = RuleIO::FeatureMatrix(vec![min_voice.clone()]);
        let output = RuleIO::FeatureMatrix(vec![voice.clone()]);

        let env = Some(vec![
            Environment::FeatureMatrix(vec![syllabic.clone()])]);

        intervocalic_voicing = Rule::new("Intervocalic Voicing", 
                                         &input, &output, &env, &env);
    }

    let fails = potato.left_env_match(0, &intervocalic_voicing);
    assert!(fails == false);

    let success = potato.left_env_match(2, &intervocalic_voicing);
    assert!(success == true);
}

#[test]
fn left_env_match_segment() {
    let post_s_raising: Rule;
    let sell = Word::from_str("set").unwrap();

    {
        let e = Segment::from_symbol("e").unwrap();
        let i = Segment::from_symbol("i").unwrap();
        let s = Segment::from_symbol("s").unwrap();

        let input = RuleIO::Segment(e);
        let output = RuleIO::Segment(i);
        let left_env = Some(vec![Environment::Segment(s)]);

        post_s_raising = Rule::new("Post-S Rasing",
                                   &input, &output, &left_env, &None);
    }

    let fails = sell.left_env_match(0, &post_s_raising);
    assert!(fails == false);

    let success = sell.left_env_match(1, &post_s_raising);
    assert!(success == true);
}

#[test]
fn left_env_match_multiple() {
    let palatalization: Rule;
    let pis = Word::from_str("pise").unwrap();

    {
        let p = Segment::from_symbol("p").unwrap();
        let syllabic = Feature::new("syllabic", Some(true));

        let s = RuleIO::Segment(Segment::from_symbol("s").unwrap());
        let esh = RuleIO::Segment(Segment::from_symbol("ʃ").unwrap());
        
        let env_0 = Environment::Boundary;
        let env_1 = Environment::Segment(p);
        let env_2 = Environment::FeatureMatrix(vec![syllabic]);

        let left_env = Some(vec![env_0, env_1, env_2]);

        palatalization = Rule::new("Palatalization",
                                   &s, &esh, &left_env, &None);
    }

    let fails = pis.left_env_match(0, &palatalization);
    assert!(fails == false);

    let fails = pis.left_env_match(1, &palatalization);
    assert!(fails == false);

    let success = pis.left_env_match(2, &palatalization);
    assert!(success == true);

    let fails = pis.left_env_match(3, &palatalization);
    assert!(fails == false);
}