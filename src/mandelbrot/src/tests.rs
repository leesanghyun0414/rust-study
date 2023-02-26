use crate::core::parse_pair;

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
}
