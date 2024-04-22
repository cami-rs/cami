use crate::locality::{debug_fail_unreachable_for_local, debug_fail_unreachable_for_non_local};

#[test]
#[should_panic(expected = "Unreachable for 'local_*' functions because of its Locality.")]
fn test_debug_fail_unreachable_for_local() {
    debug_fail_unreachable_for_local()
}

#[test]
#[should_panic(expected = "Unreachable for 'non_local_*' functions because of its Locality.")]
fn test_debug_fail_unreachable_for_non_local() {
    debug_fail_unreachable_for_non_local()
}
