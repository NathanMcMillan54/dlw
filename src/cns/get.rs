use crate::NAMES_LIST;

pub fn get_cns_info(inputs: Vec<&str>) -> String {
    let location = if inputs[0] == "GET_NAME" {
        let did = inputs[1].parse::<u32>();
        let id = inputs[2].parse::<u64>();
        let port = inputs[3].parse::<u16>();

        if did.is_err() || id.is_err() || port.is_err() {
            return format!("Parsing error");
        }

        vec![did.unwrap() as u64, id.unwrap(), port.unwrap() as u64]
    } else {
        vec![]
    };

    unsafe {
        for i in 0..NAMES_LIST.list.len() {
            match inputs[0] {
                "GET_ID" => {
                    if &NAMES_LIST.list[i].owner.name == inputs[1] {
                        return format!(
                            "{} {} {} {} {}",
                            NAMES_LIST.list[i].owner.id,
                            NAMES_LIST.list[i].owner.did,
                            NAMES_LIST.list[i].owner.port,
                            NAMES_LIST.list[i].owner.name,
                            NAMES_LIST.list[i].owner.name_type
                        );
                    }
                }
                "GET_ALL_NAMES" => {
                    // TODO: Implement large file reading
                }
                "GET_NAME" => {
                    if NAMES_LIST.list[i].owner.did == location[0] as u32
                        && NAMES_LIST.list[i].owner.id == location[1] as u64
                        && NAMES_LIST.list[i].owner.port == location[2] as u16
                    {
                        return format!(
                            "{} {}",
                            NAMES_LIST.list[i].owner.name, NAMES_LIST.list[i].owner.name_type
                        );
                    }
                }
                _ => {}
            }
        }
    }

    String::new()
}
