use super::*;

#[test]
fn to_bytes_without_0x() {
    let str = "B8 4C 00";
    let bytes = str.to_bytes();
    assert_eq!(bytes, vec![0xB8, 0x4C, 0x00]);
}
