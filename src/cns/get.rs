use crate::NAMES_LIST;

pub fn get_cns_info(inputs: Vec<&str>) -> String {
    unsafe {
        for i in 0..NAMES_LIST.list.len() {
            match inputs[0] {
                "GET_ID" => {
                    if &NAMES_LIST.list[i].owner.name == inputs[1] {
                        return format!("{} {} {} {} {}", NAMES_LIST.list[i].owner.id, NAMES_LIST.list[i].owner.did, NAMES_LIST.list[i].owner.port, NAMES_LIST.list[i].owner.name, NAMES_LIST.list[i].owner.name_type)
                    }
                }
                _ => {},
            }
        }
    }

    String::new()
}
