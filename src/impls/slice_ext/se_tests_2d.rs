use crate::SliceExt;

const ITEM_0: &[u8] = &[9];
const ITEM_1: &[u8] = &[5, 4];
const ITEM_2: &[u8] = &[7, 1];
const ITEM_3: &[u8] = &[3, 5, 1];
const ITEM_4: &[u8] = &[3, 5, 3];
const ITEM_5: &[u8] = &[1, 0, 9, 9];
const ITEM_6: &[u8] = &[1, 7, 0, 0];
const ITEM_7: &[u8] = &[0, 0, 1, 9, 9];
const ITEM_8: &[u8] = &[0, 2, 0, 5, 5];
const ALL: &[&[u8]] = &[
    ITEM_0, ITEM_1, ITEM_2, ITEM_3, ITEM_4, ITEM_5, ITEM_6, ITEM_7, ITEM_8,
];

#[test]
fn item_0() {
    assert_eq!(ALL.binary_search_ca(&ITEM_0), Ok(0));
}
#[test]
fn item_1() {
    assert_eq!(ALL.binary_search_ca(&ITEM_1), Ok(1));
}
#[test]
fn item_2() {
    assert_eq!(ALL.binary_search_ca(&ITEM_2), Ok(2));
}
#[test]
fn item_3() {
    assert_eq!(ALL.binary_search_ca(&ITEM_3), Ok(3));
}
#[test]
fn item_4() {
    assert_eq!(ALL.binary_search_ca(&ITEM_4), Ok(4));
}
#[test]
fn item_5() {
    assert_eq!(ALL.binary_search_ca(&ITEM_5), Ok(5));
}
#[test]
fn item_6() {
    assert_eq!(ALL.binary_search_ca(&ITEM_6), Ok(6));
}
#[test]
fn item_7() {
    assert_eq!(ALL.binary_search_ca(&ITEM_7), Ok(7));
}
#[test]
fn item_8() {
    assert_eq!(ALL.binary_search_ca(&ITEM_8), Ok(8));
}
