use regex::Regex;

use crate::{
    constants::{self, LOOSE_RE, MAX_LENGTH},
    valid,
};

pub struct Vern {
    pub major: i32,
    pub minor: i32,
    pub patch: i32,
    pub prerelease: Vec<String>,
}

impl Vern {
    pub fn new(version: &str) -> Vern {
        let is_version = valid::valid(version);
        assert!(is_version);

        let mut vern = Vern {
            major: 0,
            minor: 0,
            patch: 0,
            prerelease: Vec::new(),
        };

        for cap in LOOSE_RE.captures_iter(version) {
            let major = cap[1].parse::<i32>().unwrap();
            assert!(major <= MAX_LENGTH);
            vern.major = major;
            let minor = cap[2].parse::<i32>().unwrap();
            assert!(minor <= MAX_LENGTH);
            vern.minor = minor;
            let patch = cap[3].parse::<i32>().unwrap();
            assert!(patch <= MAX_LENGTH);
            vern.patch = patch;

            if cap.len() > 5 {
                if !cap.get(4).map_or("", |m| m.as_str()).is_empty() {
                    let prerelease_arr = cap[4].split(".");
                    for release in prerelease_arr {
                        if Regex::new(r"^[0-9]+$").unwrap().is_match(release) {
                            let num = release.parse::<i32>().unwrap();
                            if num >= 0 && num < constants::MAX_LENGTH {
                                vern.prerelease.push(release.to_owned());
                            }
                        } else {
                            vern.prerelease.push(release.to_owned());
                        }
                    }
                }
            }
        }
        vern
    }
}
