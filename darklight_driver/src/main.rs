use std::{
    env,
    fs::{create_dir, remove_dir},
    path::Path,
};

pub(crate) mod ids;

fn files_setup() -> bool {
    if Path::new("/tmp/darklight/").exists() {
        println!("Removing \"/tmp/darklight/\"");
        remove_dir("/tmp/darklight/").unwrap();
        return false;
    }

    if !Path::new("/etc/dlw/").exists() {
        println!("\"/etc/dlw/\" does not exist, creating dir, restart darklight_driver...");
        create_dir("/etc/dlw/").unwrap();
        return false;
    } else if !Path::new("/etc/dlw/local_id").exists() {
        println!("Generating local id...");
        ids::write_local_id();
    } else if Path::new("/etc/dlw/local_id").exists() {
        println!("Verifying local Id....");
        if ids::verify_id() == false {
            println!("Local Id could not be verified, delete \"/etc/dlw/local_id\"");
            return false;
        }
        println!("Local Id verified");
    }

    create_dir("/tmp/darklight/").unwrap();
    create_dir("/tmp/darklight/connections/").unwrap();

    return true;
}

fn main() {
    let start_info = format!(
        "| {}:{} By: {} |",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS")
    );
    let mut start_msg = format!("| Check for updates on DarkLight's Github page");
    start_msg.push_str(" ".repeat(start_info.len() - start_msg.len() - 1).as_str());
    start_msg.push('|');
    let banner_outline = "-".repeat(start_info.len());
    print!(
        "{}\n{}\n{}\n{}\n",
        banner_outline, start_info, start_msg, banner_outline
    );

    drop(start_info);
    drop(start_msg);
    drop(banner_outline);

    let args: Vec<String> = env::args().collect();

    if files_setup() == false {
        exit(-1);
    }
}
