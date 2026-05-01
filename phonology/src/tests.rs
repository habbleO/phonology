use super::*;

#[test]
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

