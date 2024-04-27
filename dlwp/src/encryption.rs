/// Use a custom encryption method by implementing it in a function that follows the ``encode_function`` and
/// ``decode_function`` fields, any information used for encoding/decoding text can be put in the ``info`` field
#[derive(Clone, Copy, PartialEq)]
pub struct EncryptionInfo {
    #[allow(improper_ctypes_definitions)]
    pub encode_function: fn([i32; 6], String) -> String,
    #[allow(improper_ctypes_definitions)]
    pub decode_function: fn([i32; 6], String) -> String,
    pub info: [i32; 6],
}
