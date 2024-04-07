extern crate chrono;

use chrono::{Datelike, Utc};

#[cfg(target_os = "windows")]
compiler_error!("DarkLight does not support Windows");

fn get_os() -> u8 {
    #[cfg(target_os = "linux")]
    return 0;

    #[cfg(target_os = "macos")]
    return 1;
}

fn get_arch() -> u8 {
    #[cfg(target_arch = "x86_64")]
    return 0;

    #[cfg(target_arch = "x86")]
    return 1;

    #[cfg(target_arch = "aarch64")]
    return 2;

    #[cfg(target_arch = "arm")]
    return 3;
}

fn main() {
    let utc = Utc::now();

    println!("cargo:rustc-env=UC_DAY={}", utc.day());
    println!("cargo:rustc-env=UC_MONTH={}", utc.month());
    println!("cargo:rustc-env=UC_ARCH={}", get_arch());
    println!("cargo:rustc-env=UC_OS={}", get_os());
}
