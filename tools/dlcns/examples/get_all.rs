use dlcns::get::CNSGet;

fn main() {
    let mut cnsget = CNSGet::new();

    let try_all = cnsget.get_all_names();

    if try_all.is_some() {
        let all = try_all.unwrap();
        println!("Got all names:");
        for i in 0..all.len() {
            println!("{} - {} {} {}", all[i].name, all[i].id, all[i].did, all[i].port);
        }
    } else {
        println!("Could not get all names");
    }
}
