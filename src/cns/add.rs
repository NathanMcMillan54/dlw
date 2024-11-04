use crate::NAMES_LIST;
use dlcns::name::{Name, Owner};
use dlcns::OWNERS_LIST;
use dlwp::cerpton::*;
use dlwp::chrono::{Datelike, Utc};
use dlwp::codes::*;
use dlwp::langs::*;
use dlwp::message::TransmitInfo;

pub struct RecentlyClients;

unsafe fn add0(split: Vec<&str>) -> (String, Code) {
    let s1 = split[1].parse::<i32>();
    let s2 = split[2].parse::<i32>();
    let s3 = split[3].parse::<i32>();

    if s1.is_ok() && s2.is_ok() && s3.is_ok() {
        let mut encoder = Encoder::new(s1.clone().unwrap(), s2.clone().unwrap());
        encoder.set_alphabet();
        if encoder.setting_good() {
            for i in 0..NAMES_LIST.list.len() {
                let mut dates = vec![];

                if NAMES_LIST.list[i].current_dlu_key
                    == libcerpton_decode(
                        [
                            s1.clone().unwrap(),
                            s2.clone().unwrap(),
                            s3.clone().unwrap(),
                            0,
                            0,
                            0,
                        ],
                        split[4].to_string(),
                    )
                    && NAMES_LIST.list[i].og_dlu_key
                        == libcerpton_decode(
                            [
                                s1.clone().unwrap(),
                                s2.clone().unwrap(),
                                s3.clone().unwrap(),
                                0,
                                0,
                                0,
                            ],
                            split[5].to_string(),
                        )
                    || NAMES_LIST.list[i].og_dlu_key
                        == libcerpton_decode(
                            [
                                s1.clone().unwrap(),
                                s2.clone().unwrap(),
                                s3.clone().unwrap(),
                                0,
                                0,
                                0,
                            ],
                            split[4].to_string(),
                        )
                        && NAMES_LIST.list[i].current_dlu_key
                            == libcerpton_decode(
                                [
                                    s1.clone().unwrap(),
                                    s2.clone().unwrap(),
                                    s3.clone().unwrap(),
                                    0,
                                    0,
                                    0,
                                ],
                                split[4].to_string(),
                            )
                {
                    dates.push(NAMES_LIST.list[i].date.to_vec());
                } /*else {
                  println!("already used date");
                          return (String::from("INVALID"), REGULAR_RESPONSE);
                      }*/

                return match dates.len() {
                    0 => (String::from("ALLOW_ADD0 0"), REGULAR_RESPONSE),
                    _ => {
                        let utc = Utc::now();

                        if dates.last().unwrap()[1] as u32 != utc.month()
                            || dates.last().unwrap()[1] as u32 != utc.month()
                                && dates.last().unwrap()[2] != utc.year()
                        {
                            (format!("ALLOW_ADD0 {}", dates.len()), REGULAR_RESPONSE)
                        } else {
                            (String::from("Recently registered a name"), REGULAR_RESPONSE)
                        }
                    }
                };
            }

            return (String::from("ALLOW_ADD0 0"), REGULAR_RESPONSE);
        } else {
            return (String::from("INVALID"), INVALID_RR);
        }
    } else {
        return (String::from("INVALID"), INVALID_RR);
    }

    //return (String::from("INVALID"), INVALID_RR);
}

unsafe fn add1(split: Vec<&str>, ti: TransmitInfo) -> (String, Code) {
    let s1 = split[1].parse::<i32>();
    let s2 = split[2].parse::<i32>();
    let s3 = split[3].parse::<i32>();
    let port = split[7].parse::<u16>();

    if s1.is_err() || s2.is_err() || s3.is_err() || port.is_err() {
        println!(
            "Error parsing nums {:?} {:?} {:?} {:?}",
            split[1], split[2], split[3], port
        );
        return (String::from("INVALID"), INVALID_RR);
    }

    let mut encoder = Encoder::new(s1.clone().unwrap(), s2.clone().unwrap());
    encoder.set_alphabet();

    if encoder.setting_good() {
        let current_key = libcerpton_decode(
            [
                s1.clone().unwrap(),
                s2.clone().unwrap(),
                s3.clone().unwrap(),
                0,
                0,
                0,
            ],
            split[4].to_string(),
        );
        let og_key = libcerpton_decode(
            [
                s1.clone().unwrap(),
                s2.clone().unwrap(),
                s3.clone().unwrap(),
                0,
                0,
                0,
            ],
            split[5].to_string(),
        );
        let name = libcerpton_decode(
            [
                s1.clone().unwrap(),
                s2.clone().unwrap(),
                s3.clone().unwrap(),
                0,
                0,
                0,
            ],
            split[6].to_string(),
        );

        let utc = Utc::now();

        for i in 0..NAMES_LIST.list.len() {
            if NAMES_LIST.list[i].owner.name == name {
                return (String::from("Name in use"), REGULAR_RESPONSE);
            }
        }

        if !name.starts_with("info.")
            && !name.starts_with("visu.")
            && !name.ends_with(".org")
            && !name.ends_with(".com")
            && !name.ends_with(".prs")
        {
            return (String::from("Name is invalid"), REGULAR_RESPONSE);
        }

        NAMES_LIST.list.push(Name {
            owner: Owner {
                id: ti.tid,
                did: ti.tdid,
                port: port.clone().unwrap(),
                name: name,
                name_type: 0,
            },
            requests: 0,
            date: [utc.day() as i32, utc.month() as i32, utc.year()],
            current_dlu_key: current_key,
            og_dlu_key: og_key,
        });
        NAMES_LIST.write_to_file(OWNERS_LIST.to_string());
        return (String::from("Added name"), REGULAR_RESPONSE);
    } else {
        return (String::from("INVALID"), INVALID_RR);
    }
}

pub unsafe fn check_allowadd(contents: String, ti: TransmitInfo) -> (String, Code) {
    let split = contents.split(" ").collect::<Vec<&str>>();

    if split[0].contains("REQUEST_ADD0") {
        return add0(split);
    } else if split[0].contains("REQUEST_ADD1") {
        return add1(split, ti);
    } else {
        return (String::from("INVALID"), INVALID_RR);
    }
}
