use dlcns::get::CNSGet;

fn main() {
    let mut cnsget = CNSGet::new();

    let id = cnsget.get_owner_name(String::from("visu.test_web.prs"));

    if id.is_some() {
        println!("{:?}", id.unwrap());
    } else {
        println!("no");
    }
}
