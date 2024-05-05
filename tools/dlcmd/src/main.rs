use std::env::args;
use std::fs::File;
use std::io::Write;

fn main() {
    let args = args().collect::<Vec<String>>();

    println!("{:?}", args);
    let mut file = File::options()
        .write(true)
        .open("/tmp/darklight/cmd_input")
        .expect("Failed to open input file");

    for i in 1..args.len() {
        println!("{}", args[i]);
        file.write_fmt(format_args!("{}{}", args[i], " ")).unwrap();
    }
}
