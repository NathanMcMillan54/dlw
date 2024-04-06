use crate::codes::{Code, CONNECTION_DENIED, DISTRIBUTOR_NOT_FOUND, REQUEST_FILE, TEAPOT};

#[test]
fn test_codes() {
    let test_request = 3;
    let test_response = 201;
    let test_error = 302;
    let test_status = 418;

    assert_eq!(REQUEST_FILE, Code::new(test_request));
    assert_eq!(CONNECTION_DENIED, Code::new(test_response));
    assert_eq!(DISTRIBUTOR_NOT_FOUND, Code::new(test_error));
    assert_eq!(TEAPOT, Code::new(test_status));
}
