/// Use a custom encryption method by implementing it in a function that follows the ``function`` field, any
/// information used for encoding/decoding text can be put in the ``info`` field
#[derive(Clone, Copy, PartialEq)]
pub struct EncryptionInfo {
    #[allow(improper_ctypes_definitions)]
    pub function: extern "C" fn([i32; 6], &'static str) -> &'static str,
    pub info: [i32; 6],
}
