use vern::{core::Vern, internal::identifiers::CompareType};

#[test]
fn test_basic_function() {
    let version = "0.1.2-re.1.z266.1.2.3+build";
    let mut vern = Vern::new(version);
    assert_eq!(vern.major, 0);
    assert_eq!(vern.minor, 1);
    assert_eq!(vern.patch, 2);
    assert_eq!(vern.prerelease.contains(&"re".to_string()), true);
    assert_eq!(vern.prerelease.contains(&"1".to_string()), true);
    assert_eq!(vern.prerelease.contains(&"z266".to_string()), true);
    assert_eq!(vern.prerelease.contains(&"266".to_string()), false);
    assert_eq!(vern.build.contains(&"build".to_string()), true);
    assert_eq!(vern.version, "0.1.2");
    assert_eq!(vern.format(), "0.1.2");
    assert_eq!(vern.to_string(), vern.version);
}

#[test]
fn test_compare() {
    let avern = Vern::new("0.1.2");
    let bvern = Vern::new("0.1.3");
    match avern.compare(bvern) {
        CompareType::LARGER => assert!(false),
        CompareType::SMALLER => assert!(true),
        CompareType::EQUAL => assert!(false),
    }
    let cvern = Vern::new("0.1.1");
    match avern.compare(cvern) {
        CompareType::LARGER => assert!(true),
        CompareType::SMALLER => assert!(false),
        CompareType::EQUAL => assert!(false),
    }

    let dvern = Vern::new("0.1.2");
    match avern.compare(dvern) {
        CompareType::LARGER => assert!(false),
        CompareType::SMALLER => assert!(false),
        CompareType::EQUAL => assert!(true),
    }
}
