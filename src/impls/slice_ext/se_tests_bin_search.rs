use crate::SliceExt;

const ITEM_0: u8 = 0;
const ITEM_1: u8 = 2;
const ITEM_2: u8 = 6;
const ITEM_3: u8 = 40;
const ITEM_4: u8 = 41;
const ITEM_5: u8 = 80;
const ITEM_6: u8 = 81;
const ALL: &[u8] = &[ITEM_0, ITEM_1, ITEM_2, ITEM_3, ITEM_4, ITEM_5, ITEM_6];

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
