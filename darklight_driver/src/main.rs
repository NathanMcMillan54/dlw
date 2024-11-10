use std::{
    alloc::dealloc,
    env,
    fs::{create_dir, read_to_string, remove_dir_all, remove_file, File},
    io::{Read, Write},
    net::TcpStream,
    path::Path,
    process::exit,
    thread::{self, sleep},
    time::Duration,
};

use dlwp::{config::DLConfig, serialport::posix::TTYPort};
use driver::DarkLightDriver;
use streams::StreamsHandler;

pub(crate) mod cmd;
pub(crate) mod cns;
pub(crate) mod driver;
pub(crate) mod ids;
pub(crate) mod streams;

fn files_setup() -> bool {
    if Path::new("/tmp/darklight/").exists() {
        println!("Removing \"/tmp/darklight/\"...");
        remove_dir_all("/tmp/darklight/").unwrap();
    }

    if !Path::new("/etc/dlw/").exists() {
        println!("\"/etc/dlw/\" does not exist, creating directory...");
        create_dir("/etc/dlw/").unwrap();
    }

    if !Path::new("/etc/dlw/local_id").exists() {
        println!("Generating local id...");
        ids::write_local_id();
    } else {
        println!("Verifying local Id....");
        if ids::verify_id() == false {
            println!("Local Id could not be verified, deleting \"/etc/dlw/local_id\"");
            remove_file("/etc/dlw/local_id").unwrap();
            println!("Restart darklight_driver");
            return false;
        }
        println!("Local Id verified");
    }

    if !Path::new("/etc/dlw/local_did").exists() {
        println!("Creating distributor Id file...");
        File::options()
            .create(true)
            .write(true)
            .open("/etc/dlw/local_did")
            .unwrap()
            .write_fmt(format_args!("{}", 1))
            .unwrap();
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
    File::options()
        .read(true)
        .write(true)
        .create(true)
        .open("/tmp/darklight/cmd_input")
        .unwrap();

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
    let config: DLConfig = dlwp::serde_json::from_str(Box::leak(config_contents.into_boxed_str()))
        .expect("Failed to parse configuration file");

    if config.serial == true && config.tcp == true {
        panic!("\"serial\" and \"tcp\" are both set to true when only one should be")
    } else if config.serial == true && config.closed == true
        || config.tcp == true && config.closed == true
    {
        panic!("\"closed\" and \"serial\" or \"tcp\" are set to true when only one should be");
    }

    let mut darklight_driver = DarkLightDriver::new(StreamsHandler::new(), config);

    if config.serial == true {
        let settings = dlwp::serialport::SerialPortSettings {
            baud_rate: 9600,
            data_bits: dlwp::serialport::DataBits::Eight,
            flow_control: dlwp::serialport::FlowControl::None,
            parity: dlwp::serialport::Parity::None,
            stop_bits: dlwp::serialport::StopBits::One,
            timeout: Duration::from_millis(4096),
        };
        let serial_port = TTYPort::open(Path::new(config.serial_path), &settings)
            .expect("Failed to setup serial port");
        darklight_driver.serial_port = Some(serial_port);
    } else if config.tcp == true {
        let tcp_stream = TcpStream::connect(config.ip_address).expect("Failed to connect");

        darklight_driver.tcp_stream = Some(tcp_stream);
    }

    darklight_driver.connect_to_distributor();

    loop {
        cmd::check_cmd_input(&mut darklight_driver);
        darklight_driver.streams_handler.read_local_streams();
        darklight_driver.send_to_distributor();
        darklight_driver.read_from_distributor();

        sleep(Duration::from_millis(10));
    }
}
