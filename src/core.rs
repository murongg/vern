use regex::Regex;

use crate::{
    constants::{LOOSE_RE, MAX_LENGTH},
    internal::identifiers::{compare_identifiers_with_int, CompareType},
    valid,
};

pub struct Vern {
    pub major: i32,
    pub minor: i32,
    pub patch: i32,
    pub prerelease: Vec<String>,
    pub build: Vec<String>,
    pub version: String,
}

impl Vern {
    /// Initialize Vern
    /// ```
    /// use vern::core::Vern;
    /// let vern = Vern::new("0.1.2-re.1.z266.1.2.3+build");
    /// assert_eq!(vern.major, 0);
    /// assert_eq!(vern.minor, 1);
    /// assert_eq!(vern.patch, 2);
    /// ```
    pub fn new(version: &str) -> Vern {
        let is_version = valid::valid(version);
        assert!(is_version);

        let mut vern = Vern {
            major: 0,
            minor: 0,
            patch: 0,
            prerelease: Vec::new(),
            build: Vec::new(),
            version: String::new(),
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

            if cap.len() >= 5 {
                if !cap.get(4).map_or("", |m| m.as_str()).is_empty() {
                    let prerelease_arr = cap[4].split(".");
                    for release in prerelease_arr {
                        if Regex::new(r"^[0-9]+$").unwrap().is_match(release) {
                            let num = release.parse::<isize>().unwrap();
                            if num >= 0 && num < isize::MAX {
                                vern.prerelease.push(release.to_owned());
                            }
                        } else {
                            vern.prerelease.push(release.to_owned());
                        }
                    }
                }
            }

            if cap.len() >= 6 {
                if !cap.get(5).map_or("", |m| m.as_str()).is_empty() {
                    for build in cap[5].split(".") {
                        vern.build.push(build.to_owned());
                    }
                }
            }
        }
        vern.format();
        vern
    }
    /// Format Vern version
    /// ```
    /// use vern::core::Vern;
    /// let mut vern = Vern::new("0.1.2-re.1.z266.1.2.3+build");
    /// assert_eq!(vern.format(), "0.1.2");
    /// ```
    pub fn format(&mut self) -> &str {
        self.version = format!("{}.{}.{}", self.major, self.minor, self.patch);
        &self.version
    }

    /// Vern version to string
    /// ```
    /// use vern::core::Vern;
    /// let vern = Vern::new("0.1.2-re.1.z266.1.2.3+build");
    /// assert_eq!(vern.to_string(), "0.1.2");
    /// ```
    pub fn to_string(&self) -> &str {
        &self.version
    }

    /// compare with other version
    /// ```
    /// use vern::{core::Vern, internal::identifiers::CompareType};
    /// let avern = Vern::new("0.1.2");
    /// let bvern = Vern::new("0.1.3");
    /// match avern.compare(bvern) {
    ///     CompareType::LARGER => assert!(false),
    ///     CompareType::SMALLER => assert!(true),
    ///     CompareType::EQUAL => assert!(false),
    /// }
    /// ```
    pub fn compare(&self, other: Vern) -> CompareType {
        if self.version == other.version {
            return CompareType::EQUAL;
        }

        self.compare_main(other)
    }

    /// compare with other version
    /// ```
    /// use vern::{core::Vern, internal::identifiers::CompareType};
    /// let avern = Vern::new("0.1.2");
    /// let bvern = Vern::new("0.1.3");
    /// match avern.compare_main(bvern) {
    ///     CompareType::LARGER => assert!(false),
    ///     CompareType::SMALLER => assert!(true),
    ///     CompareType::EQUAL => assert!(false),
    /// }
    /// ```
    pub fn compare_main(&self, other: Vern) -> CompareType {
        if compare_identifiers_with_int(self.major, other.major) == 1
            || compare_identifiers_with_int(self.minor, other.minor) == 1
            || compare_identifiers_with_int(self.patch, other.patch) == 1
        {
            CompareType::LARGER
        } else if compare_identifiers_with_int(self.major, other.major) == 0
            && compare_identifiers_with_int(self.minor, other.minor) == 0
            && compare_identifiers_with_int(self.patch, other.patch) == 0
        {
            CompareType::EQUAL
        } else {
            CompareType::SMALLER
        }
    }
}
