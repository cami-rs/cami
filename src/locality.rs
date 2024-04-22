#[cfg(test)]
mod l_tests;

#[cfg(test)]
mod l_tests_unreachable;

/// Used to indicate if a type implementing [CPartialEq]/[COrd] has custom logic in only one, or
/// both, of "local_*" & "non_local_*" methods.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Locality {
    PureLocal,
    PureNonLocal,
    Both,
}
impl Locality {
    #[inline]
    pub(crate) const fn has_local(&self) -> bool {
        match self {
            Locality::PureNonLocal => false,
            _ => true,
        }
    }

    #[inline]
    pub(crate) const fn has_non_local(&self) -> bool {
        match self {
            Locality::PureLocal => false,
            _ => true,
        }
    }

    /// NOT a part of public API. Only for use by macro-generated code (and internal code). Subject to change.
    #[doc(hidden)]
    #[inline]
    pub const fn debug_reachable_for_local(&self) {
        #[cfg(debug_assertions)]
        if !self.has_local() {
            panic!("Unreachable for 'local_*' functions because of its Locality.");
        }
    }

    /// NOT a part of public API. Only for use by macro-generated code (and internal code). Subject to change.
    #[doc(hidden)]
    #[inline]
    pub const fn debug_reachable_for_non_local(&self) {
        #[cfg(debug_assertions)]
        if !self.has_non_local() {
            panic!("Unreachable for 'non_local_*' functions because of its Locality.");
        }
    }
}

/// Panic, in debug only, with the same message as [Locality::debug_reachable_for_local] when called
/// with [Locality::PureNonLocal].
#[inline]
pub(crate) fn debug_fail_unreachable_for_local() {
    Locality::PureNonLocal.debug_reachable_for_local()
}

/// Panic, in debug only, with the same message as [Locality::debug_reachable_for_non_local] when
/// called with [Locality::PureLocal].
#[inline]
pub(crate) fn debug_fail_unreachable_for_non_local() {
    Locality::PureLocal.debug_reachable_for_non_local()
}
