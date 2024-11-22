use crate::codes::*;
use crate::encryption::EncryptionInfo;
use crate::stream::*;
use std::thread;

fn test_stream_file_pending() {
    // One stream file should represent a DarkLight application and the other should be darklight_driver
    let mut stream_file1 = StreamFile::new(555, 5000, 0, [2, 1, 2, 0, 0, 0]);
    let mut stream_file2 = StreamFile::new(555, 5000, 0, [2, 1, 2, 0, 0, 0]);

    stream_file1.read_pending();
    stream_file1.read_recieved();
    assert_eq!(stream_file1.received.is_empty(), true);
    assert_eq!(stream_file1.pending.is_empty(), true);

    stream_file2.read_pending();
    stream_file2.read_recieved();
    assert_eq!(stream_file2.received.is_empty(), true);
    assert_eq!(stream_file2.pending.is_empty(), true);

    // Pending
    let mut done_pending = false;
    thread::spawn(move || {
        for i in 0..10 {
            stream_file1.wait_for_file("P");
            stream_file1.pending.push(format!("{}", i));
            stream_file1.write_pending();
            stream_file1.pending.remove(0);
        }
        done_pending = true;
    });

    for i in 0..10 {
        while stream_file2.pending.is_empty() {
            stream_file2.read_pending();
        }

        // Make sure that the first thread is not finished before this one
        assert_eq!(done_pending == false && i <= 9, true);

        // Check if the proper message was written
        assert_eq!(stream_file2.pending.last().unwrap().contains(&format!("{}", i)), true);
        stream_file2.pending.remove(0);
    }
}

fn test_stream_file_received() {
    // One stream file should represent a DarkLight application and the other should be darklight_driver
    let mut stream_file1 = StreamFile::new(555, 5000, 0, [2, 1, 2, 0, 0, 0]);
    let mut stream_file2 = StreamFile::new(555, 5000, 0, [2, 1, 2, 0, 0, 0]);

    let mut done_receiving = false;
    thread::spawn(move || {
        for i in 0..10 {
            stream_file1.wait_for_file("R");
            stream_file1.received.push(ReceivedMessage {
                recv_time: [1, 2, 3],
                message: format!("{}", i)
            });
            stream_file1.write_received();
            stream_file1.received.remove(0);
        }
        done_receiving = true;
    });

    for i in 0..10 {
        while stream_file2.received.is_empty() {
            stream_file2.read_recieved();
        }

        // Make sure that the first thread is not finished before this one
        assert_eq!(done_receiving == false && i <= 9, true);

        // Check if the proper message was written
        assert_eq!(stream_file2.received.last().unwrap().message.contains(&format!("{}", i)), true);
        stream_file2.received.remove(0);
    }
}

#[test]
fn test_stream_file() {
    test_stream_file_pending();
    test_stream_file_received();
}
