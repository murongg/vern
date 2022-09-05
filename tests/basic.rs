use vern::core::Vern;

#[test]
fn test_basic_function() {
    let version = "0.1.2-re.1.z266.1.2.3+build";
    let vern = Vern::new(version);
    assert_eq!(vern.major, 0);
    assert_eq!(vern.minor, 1);
    assert_eq!(vern.patch, 2);
    assert_eq!(vern.prerelease.contains(&"re".to_string()), true);
    assert_eq!(vern.prerelease.contains(&"1".to_string()), true);
    assert_eq!(vern.prerelease.contains(&"z266".to_string()), true);
    assert_eq!(vern.prerelease.contains(&"266".to_string()), false);
    assert_eq!(vern.build.contains(&"build".to_string()), true);
}
