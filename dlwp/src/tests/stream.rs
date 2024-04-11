use crate::codes::*;
use crate::encryption::EncryptionInfo;
use crate::stream::*;

#[test]
fn test_stream_setup() {
    let encryption_info = EncryptionInfo {
        function: cerpton::libcerpton_encode,
        info: [1, 2, 3, 0, 0, 0],
    };

    let mut stream = Stream::new(
        StreamType::Client {
            rid: 2,
            rdid: 3,
            port: 1,
        },
        false,
    );

    stream.add_encryption_info(encryption_info);

    assert_ne!(EMPTY_ENCRYPTIONIFNO.info, stream.encryption.info);
    assert_eq!(STATUS_OK, stream.start());
    assert_eq!(STATUS_OK, stream.stop());
}
