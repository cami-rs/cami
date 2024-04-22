use crate::Locality;

#[test]
fn methods() {
    assert_eq!(Locality::PureNonLocal.has_local(), false);
    assert_eq!(Locality::PureNonLocal.has_non_local(), true);

    assert_eq!(Locality::PureLocal.has_local(), true);
    assert_eq!(Locality::PureLocal.has_non_local(), false);

    assert_eq!(Locality::Both.has_local(), true);
    assert_eq!(Locality::Both.has_non_local(), true);
}
