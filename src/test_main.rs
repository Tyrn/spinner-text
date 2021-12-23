// use super::*;

#[test]
fn test_format() {
    assert_eq!(format!("{:01$}", 1, 4), "0001");
    assert_eq!(format!("{:+}", 4), "+4");
    assert_eq!(format!("{:+}", -4), "-4");
}
