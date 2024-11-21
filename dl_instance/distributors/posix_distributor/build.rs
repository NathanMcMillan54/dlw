use std::{
    fs::File,
    io::{BufRead, BufReader},
};

// For the distributor binary
fn bin() {
    // Makes keeping track of environement variables easier
    #[cfg(debug_assertions)]
    let vars_file = "test_envvars";

    #[cfg(not(debug_assertions))]
    let vars_file = "envvars";

    let file = File::options().read(true).open(vars_file).unwrap();
    let reader = BufReader::new(file);

    for line_ in reader.lines() {
        let line = line_.unwrap();
        if line.starts_with("#") {
            continue;
        }

        println!("cargo:rustc-env={}", line.replace("\n", ""));
    }
}

fn main() {
    bin()
}
