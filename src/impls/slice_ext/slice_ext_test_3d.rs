//! Not proving much here, but if you're curious...

use crate::SliceExt;

const ITEM_0_0: &[u8] = &[9];
const ITEM_0_1: &[u8] = &[0, 5];
const ITEM_0: &[&[u8]] = &[ITEM_0_0, ITEM_0_1];

const ITEM_1_0: &[u8] = &[7];
const ITEM_1_1: &[u8] = &[1, 7];
const ITEM_1_2: &[u8] = &[4, 1];
const ITEM_1: &[&[u8]] = &[ITEM_1_0, ITEM_1_1, ITEM_1_2];

const ITEM_2_0: &[u8] = &[5];
const ITEM_2_1: &[u8] = &[0, 7];
const ITEM_2_2: &[u8] = &[4, 3];
const ITEM_2_3: &[u8] = &[0, 1, 0];
const ITEM_2: &[&[u8]] = &[ITEM_2_0, ITEM_2_1, ITEM_2_2, ITEM_2_3];

const ITEM_3_0: &[u8] = &[5];
const ITEM_3_1: &[u8] = &[6];
const ITEM_3_2: &[u8] = &[0, 9, 9];
const ITEM_3_3: &[u8] = &[1, 3, 2];
const ITEM_3_4: &[u8] = &[1, 3, 4];
const ITEM_3: &[&[u8]] = &[ITEM_3_0, ITEM_3_1, ITEM_3_2, ITEM_3_3, ITEM_3_4];

const ALL: &[&[&[u8]]] = &[ITEM_0, ITEM_1, ITEM_2, ITEM_3];

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
