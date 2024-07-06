use crate::langs::{is_human_readable, is_human_readable_including};

#[test]
fn test_lan() {
    let regular_text = "this is some text 12345932";
    let not_really_regular = ".,!?-";

    assert_eq!(
        true,
        is_human_readable_including(regular_text.to_string(), vec![' '])
    );
    assert_eq!(
        false,
        is_human_readable_including(not_really_regular.to_string(), vec![' '])
    );

    assert_eq!(false, is_human_readable(regular_text.to_string()));
    assert_eq!(false, is_human_readable(not_really_regular.to_string()));
}
