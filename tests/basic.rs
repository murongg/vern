use vern::core::Vern;

#[test]
fn test_basic_function() {
    let version = "0.1.2";
    let vern = Vern::new(version);
    assert_eq!(vern.major, 0);
    assert_eq!(vern.minor, 1);
    assert_eq!(vern.patch, 2);
}
