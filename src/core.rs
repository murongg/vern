use crate::{
    constants::{LOOSE_RE, MAX_LENGTH},
    valid,
};

pub struct Vern {
    pub major: i32,
    pub minor: i32,
    pub patch: i32,
}

impl Vern {
    pub fn new(version: &str) -> Vern {
        let is_version = valid::valid(version);
        assert!(is_version);

        let mut vern = Vern {
            major: 0,
            minor: 0,
            patch: 0,
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
        }
        vern
    }
}
