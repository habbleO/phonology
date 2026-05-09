use crate::segment::Segment;
use crate::word::Word;

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