use std::{
    fs::File,
    io::{Read, Write},
};

pub(crate) fn generate_local_id() -> String {
    let mut key = env!("DLU_KEY");
    let mut id = String::new();

    for b in key.as_bytes() {
        if id.len() > 17 {
            break;
        }

        id.push_str(&(*b.to_string()))
    }

    return id;
}

pub(crate) fn write_local_id() {
    let mut id_file = File::options()
        .write(true)
        .create(true)
        .open("/etc/dlw/local_id")
        .expect("Failed to open \"/etc/dlw/local_id\"");
    id_file
        .write_fmt(format_args!("{}", generate_local_id()))
        .expect("Failed to write to \"/etc/dlw/local_id\"");
}

pub(crate) fn verify_id() -> bool {
    let mut id_file = File::options()
        .read(true)
        .open("/etc/dlw/local_id")
        .expect("Failed to open \"/etc/dlw/local_id\"");
    let mut id = String::new();

    id_file
        .read_to_string(&mut id)
        .expect("Failed to read from \"/etc/dlw/local_id\"");

    return if id == generate_local_id() {
        true
    } else {
        false
    };
}
