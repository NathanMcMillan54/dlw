use std::str::FromStr;

use cerpton::{libcerpton_decode, libcerpton_encode};

use crate::{
    encryption::EncryptionInfo,
    message::{
        contents_to_string, string_to_contents, valid_message_string, Message, ReceiveInfo,
        TransmitInfo, CONTENTS_LENGTH,
    },
};

#[test]
fn test_string_contents_conversions() {
    // Mix of 8 and 16 bit characters
    let short_string = String::from("this is short, це багато байтів, 這也是");

    let short_string_bytes = string_to_contents(short_string);

    assert_eq!(CONTENTS_LENGTH, short_string_bytes.len());
    assert_eq!(
        String::from("this is short, це багато байтів, 這也是"),
        contents_to_string(short_string_bytes).replace("\0", "")
    );

    let mut long_string = String::from_utf8(vec![1; 4096]).unwrap();
    long_string.push_str("A");
    let long_string_bytes = string_to_contents(long_string);

    assert_eq!(false, long_string_bytes.last().unwrap() == &65);
    assert_eq!(true, long_string_bytes.last().unwrap() == &1);
}

#[test]
fn test_string_message_conversions() {
    let empty_message = Message::empty();
    let empty_message_string = empty_message.as_string();

    // The message is empty, this will always fail
    assert_eq!(false, valid_message_string(&empty_message_string, true));
    assert_eq!(false, valid_message_string(&empty_message_string, false));

    let message = Message {
        ri: ReceiveInfo {
            rid: 1,
            rdid: 3,
            instance_id: 0,
            port: 0,
        },
        ti: TransmitInfo {
            tid: 2,
            tdid: 3,
            code: 0,
        },
        day: 0,
        week: 0,
        month: 0,
        contents: string_to_contents(String::from("this is a test!")),
    };
    let message_string = message.as_string();

    assert_eq!(true, valid_message_string(&message_string, true));
    assert_eq!(true, valid_message_string(&message_string, false));

    let message_from_string = Message::from_string(&message_string);
    assert_eq!(false, message_from_string == Message::empty());
}

#[test]
fn get_ri_from_message() {
    let message_str = String::from_str("123345568 3 0 5000 \\|\\ ZZ5ZZZZZcZZHZ5ZЩь Z С Н Z Н \\|\\ 人T,66Н НZ74ь").unwrap();
    let ri = ReceiveInfo {
        rid: 123345568,
        rdid: 3,
        instance_id: 0,
        port: 5000
    };

    assert_eq!(ri, ReceiveInfo::get_from_message_string(message_str));
}
