use dlwp::id::Port;

#[cfg(feature = "visu_dl")]
const PORT: Port = 5000;

#[cfg(feature = "info_dl")]
const PORT: Port = 5001;

fn main() {
    println!("Darklight web server");
}
