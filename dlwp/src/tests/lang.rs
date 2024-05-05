use crate::langs::LATIN_UPPER;

#[test]
fn test_latin() {
    let text = "THISISUPPERCASELATINCHARACTERS";

    for c in text.chars() {
        assert_eq!(true, LATIN_UPPER.contains(&(c as u16)));
    }
}
