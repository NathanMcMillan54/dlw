use std::fs::File;
use std::io::{Seek, SeekFrom, Write};

pub(crate) const SEND: &str = "SEND";
pub(crate) const CONNECT: &str = "CONNECT";
pub(crate) const DISCONNECT: &str = "DISCONNECT";

pub(crate) fn send_dlcmd(command: &str, arguments: Vec<&str>) {
    let mut cmd_file = File::options()
        .write(true)
        .append(true)
        .open("/tmp/darklight/cmd_input")
        .unwrap();
    cmd_file.seek(SeekFrom::End(0)).unwrap();

    let mut write = String::new();
    write.push_str(command);

    for i in 0..arguments.len() {
        write.push_str(" ");
        write.push_str(arguments[i]);
    }

    cmd_file
        .write_fmt(format_args!("{} {}", write, "\n"))
        .unwrap();
}
