use dlcns::get::CNSGet;

fn main() {
    let mut cnsget = CNSGet::new();

    let id = cnsget.get_name(11011111611410197, 3, 4321);

    if id.is_some() {
        println!("{:?}", id.unwrap());
    } else {
        println!("Could not find name")
    }
}
