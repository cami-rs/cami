use crate::SliceExt;

const ALL: &[&str] = &["a", "f", "g", "z", "dd", "ccc", "bbbb", "aaaaa"];

#[test]
fn item_0() {
    assert_eq!(ALL.binary_search_ca(&"a"), Ok(0));
}
#[test]
fn item_1() {
    assert_eq!(ALL.binary_search_ca(&"f"), Ok(1));
}
#[test]
fn item_2() {
    assert_eq!(ALL.binary_search_ca(&"g"), Ok(2));
}
#[test]
fn item_3() {
    assert_eq!(ALL.binary_search_ca(&"z"), Ok(3));
}
#[test]
fn item_4() {
    assert_eq!(ALL.binary_search_ca(&"dd"), Ok(4));
}
#[test]
fn item_5() {
    assert_eq!(ALL.binary_search_ca(&"ccc"), Ok(5));
}
#[test]
fn item_6() {
    assert_eq!(ALL.binary_search_ca(&"bbbb"), Ok(6));
}
#[test]
fn item_7() {
    assert_eq!(ALL.binary_search_ca(&"aaaaa"), Ok(7));
}
