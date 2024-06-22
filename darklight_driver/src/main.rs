use std::{
    env,
    fs::{create_dir, read_to_string, remove_dir_all, remove_file, File},
    io::Write,
    path::Path,
    process::exit,
    thread,
    time::Duration,
};

use dlwp::config::DLConfig;

pub(crate) mod cmd;
pub(crate) mod cns;
pub(crate) mod ids;
pub(crate) mod streams;

fn files_setup() -> bool {
    if Path::new("/tmp/darklight/").exists() {
        println!("Removing \"/tmp/darklight/\"...");

        remove_dir_all("/tmp/darklight/").unwrap();
        println!("Restart darklight_driver");
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
            println!("Local Id could not be verified, deleting \"/etc/dlw/local_id\"");
            remove_file("/etc/dlw/local_id").unwrap();
            println!("Restart darklight_driver");
            return false;
        }
        println!("Local Id verified");
    }

    if !Path::new("/etc/dlw/first_key").exists() {
        File::options()
            .create(true)
            .write(true)
            .open("/etc/dlw/first_key")
            .expect("Failed to create \"/etc/dlw/first_key\"")
            .write_fmt(format_args!("{}", env!("DLU_KEY")))
            .expect("Failed to write key");
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
    } else {
        println!("File setup finished");
    }

    let config_contents = read_to_string(&args[1]).expect("Failed to read configuration file");
    // There should probably be a better way of doing this
    let config_json: DLConfig =
        dlwp::serde_json::from_str(Box::leak(config_contents.into_boxed_str()))
            .expect("Failed to parse configuration file");
    unsafe {
        streams::STREAMS_HANDLER.config = config_json;
    }

    thread::spawn(|| {
        cmd::cmd_input_thread();
    });

    thread::spawn(|| {
        streams::handle_streams();
    });

    loop {
        thread::sleep(Duration::from_millis(1500));
    }
}
