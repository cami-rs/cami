//#![no_std]
#![feature(hint_assert_unchecked)]

use core::cmp::Ordering::{self, *};
use core::{hint, mem};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Locality {
    PureNonLocal,
    PureLocal,
    SomeLocal,
}
impl Locality {
    const fn has_local(&self) -> bool {
        match self {
            Locality::PureNonLocal => false,
            _ => true,
        }
    }
    const fn has_non_local(&self) -> bool {
        match self {
            Locality::PureLocal => false,
            _ => true,
        }
    }
}

pub trait CfPartialEq {
    const LOCALITY: Locality;

    // If unsure, then it's `false`.
    const COMPATIBLE_WITH_PARTIAL_EQ: bool;

    fn eq_local(&self, other: &Self) -> bool;

    fn eq_non_local(&self, other: &Self) -> bool;

    fn eq_full(&self, other: &Self) -> bool {
        if Self::LOCALITY.has_local() {
            let local = self.eq_local(other);
            if local {
                Self::LOCALITY.has_non_local() || self.eq_non_local(other)
            } else {
                false
            }
        } else {
            self.eq_non_local(other)
        }
    }
}

/** Cache-friendly ordering.
 *
 *  NOT extending [Ord], because they MAY be INCOMPATIBLE.
 */
pub trait CfOrd: CfPartialEq {
    // If unsure, then it's `false`.
    const COMPATIBLE_WITH_ORD: bool;

    /// Comparison based on local (non-referenced) field(s) only (if any).
    ///
    /// Result must be [Ordering::Equal] or the same as the result of [cmp_full].
    ///
    /// Any implementation must NOT call [cmp_full] (whether directly or indirectly).
    fn cmp_local(&self, other: &Self) -> Ordering;

    /// Comparison based on non-local (referenced) field(s) only (if any).
    ///
    /// Any implementation must NOT call [cmp_full] (whether directly or indirectly).
    fn cmp_non_local(&self, other: &Self) -> Ordering;

    /// Full comparison.
    ///
    /// Any implementation must be equivalent to the default one. The default implementation
    /// respects [CfOrd::LOCALITY] and calls [CfOrd::cmp_local] and/or [CfOrd::cmp_non_local] only
    /// when they're applicable and when they're needed.
    fn cmp_full(&self, other: &Self) -> Ordering {
        // @TODO apply https://rust.godbolt.org/z/698eYffTx
        if Self::LOCALITY.has_local() {
            let local = self.cmp_local(other);
            if local == Equal {
                if Self::LOCALITY.has_non_local() {
                    self.cmp_non_local(other)
                } else {
                    Equal
                }
            } else {
                local
            }
        } else {
            self.cmp_non_local(other)
        }
    }
}

/// A (zero-cost) wrapper & bridge that implements [Ord] forwarding to [CfOrd::cmp_full] of `T`,
/// used FOR BENCHMARKING. That allows us to run slice's `binary_search`:
/// `<[T]>::binary_search(self, given)` using the full comparison, and benchmark any benefits of
/// this crate.
///
/// [PartialEq] is implemented NOT by forwarding to [PartialEq::eq] and [PartialEq::ne] of `T`, but
/// by forwarding to[CfOrd::cmp_local] and [CfOrd::cmp_non_local] of `T` instead. (Hence `T` doesn't
/// need to be [PartialEq].)
#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct OrdWrap<T> {
    t: T,
}

impl<T: CfOrd> PartialEq for OrdWrap<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        // @TODO Self::LOCALITY
        self.t
            .cmp_local(&other.t)
            .then_with(|| self.t.cmp_non_local(&other.t))
            == Equal
    }

    #[inline]
    fn ne(&self, other: &Self) -> bool {
        // @TODO Self::LOCALITY
        self.t.cmp_local(&other.t) != Equal || self.t.cmp_non_local(&other.t) != Equal
    }
}
impl<T: CfOrd> Eq for OrdWrap<T> {}
impl<T: CfOrd> PartialOrd for OrdWrap<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.t.cmp_full(&other.t))
    }

    #[inline]
    fn lt(&self, other: &Self) -> bool {
        self.t.cmp_full(&other.t) == Less
    }
    #[inline]
    fn le(&self, other: &Self) -> bool {
        self.t.cmp_full(&other.t) != Greater
    }
    #[inline]
    fn gt(&self, other: &Self) -> bool {
        self.t.cmp_full(&other.t) == Greater
    }
    #[inline]
    fn ge(&self, other: &Self) -> bool {
        self.t.cmp_full(&other.t) != Less
    }
}
impl<T: CfOrd> Ord for OrdWrap<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.t.cmp_full(&other.t)
    }
    /*
    // Provided methods
    fn max(self, other: Self) -> Self
       where Self: Sized {}
    fn min(self, other: Self) -> Self
       where Self: Sized {}
    fn clamp(self, min: Self, max: Self) -> Self
       where Self: Sized + PartialOrd {}
    */
}

/// A (zero-cost) wrapper & bridge that implements [CfOrd], [PartialOrd] and [Ord] forwarding to
/// [Ord] methods of `T`. NO cache benefit - use for compatibility only!
#[derive(Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct CfOrdWrap<T> {
    t: T,
}

impl<T: Ord> CfPartialEq for CfOrdWrap<T> {
    const LOCALITY: Locality = Locality::PureNonLocal;
    const COMPATIBLE_WITH_PARTIAL_EQ: bool = true;

    fn eq_local(&self, other: &Self) -> bool {
        debug_assert!(false, "unreachable");
        self.t == other.t
    }

    fn eq_non_local(&self, other: &Self) -> bool {
        self.t == other.t
    }
}

impl<T: Ord> CfOrd for CfOrdWrap<T> {
    const COMPATIBLE_WITH_ORD: bool = true;

    fn cmp_local(&self, other: &Self) -> Ordering {
        unreachable!()
    }

    fn cmp_non_local(&self, other: &Self) -> Ordering {
        unreachable!()
    }

    fn cmp_full(&self, other: &Self) -> Ordering {
        unreachable!()
    }
}

impl<T: PartialOrd> PartialOrd for CfOrdWrap<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.t.partial_cmp(&other.t)
    }

    fn lt(&self, other: &Self) -> bool {
        self.t.lt(&other.t)
    }
    fn le(&self, other: &Self) -> bool {
        self.t.le(&other.t)
    }
    fn gt(&self, other: &Self) -> bool {
        self.t.gt(&other.t)
    }
    fn ge(&self, other: &Self) -> bool {
        self.t.ge(&other.t)
    }
}
impl<T: Ord> Ord for CfOrdWrap<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.t.cmp(&other.t)
    }
    // Default implementations for the rest of methods.
}

pub trait Slice<T> {
    fn binary_search_cf(&self, x: &T) -> Result<usize, usize>
    where
        T: CfOrd;
    // @TODO non-binary methods, like contains()
}

impl<T: CfOrd + Ord> Slice<T> for [T] {
    // @TODO factor out part(s) to separate, non-generic functions, to decrease the cost of generic
    // copies
    fn binary_search_cf(&self, given: &T) -> Result<usize, usize> {
        if !T::LOCALITY.has_local() {
            // Any comparison is based on non-local fields only. Hence standard binary search.
            return <[T]>::binary_search(self, given);
        }
        let entry_size = mem::size_of::<T>();
        // TODO runtime: Use https://docs.rs/crossbeam-utils/latest/crossbeam_utils/struct.CachePadded.html && https://docs.rs/cache-size/latest/cache_size.
        let cache_line_size = 128usize; // in bytes
        const CACHE_LINE_START_MASK: usize = 0xFFFFFFFFFFFFFFE0;

        //let cache_line_threshold
        // let ratio
        // let ratio
        let max_entries_per_cache_line = cache_line_size / entry_size;
        // Even if there are EXACTLY 2 entries per cache line, and even if the first entry is NOT
        // aligned to the cache line (and hence the second entry is not fully loaded in cache when
        // we access the first entry only), it MIGHT still be beneficial to access such a (second)
        // entry when performing (partial) "local" comparison, especially if we use `#[repr("C")]`
        // and if we order the "local" fields first.
        //
        // BUT, that would be too complicated.
        if max_entries_per_cache_line < 3 {
            // If the type takes more than half a cache line, then accessing the neighbor would mean
            // loading another cache line(s)! Hence standard binary search.
            //
            // @TODO invoke with full qualification
            return self.binary_search(given);
        }
        //let max_right_neighbors_per_cache_line = max_entries_per_cache_line - 1;
        //
        // TODO: Make these "Bulgarian" constants part of CfOrd trait and/or feature
        //
        // Used with a `<` operator.
        let subslice_size_threshold = 3 * max_entries_per_cache_line + 2;
        // @TODO const
        let max_right_neighbors_in_cache_line = max_entries_per_cache_line - 2;

        // Based on Rust source of `binary_search_by`

        // INVARIANTS:
        // - 0 <= left <= left + size = right <= self.len()
        // - f returns Less for everything in self[..left]
        // - f returns Greater for everything in self[right..]
        let mut size = self.len();
        let mut left = 0;
        let mut right = size;
        #[cfg(debug)]
        let mut local_has_been_equal = false;
        while left < right {
            let mid = left + size / 2;

            // SAFETY: the while condition means `size` is strictly positive, so `size/2 < size`.
            // Thus `left + size/2 < left + size`, which coupled with the `left + size <=
            // self.len()` invariant means we have `left + size/2 < self.len()`, and this is
            // in-bounds.
            let entry = unsafe { self.get_unchecked(mid) };
            let cmp = entry.cmp_local(given);

            // This control flow produces conditional moves, which results in fewer branches and
            // instructions than if/else or matching on cmp::Ordering. This is x86 asm for u8:
            // https://rust.godbolt.org/z/698eYffTx.
            /*if false {
                left = if cmp == Less { mid + 1 } else { left };
                right = if cmp == Greater { mid } else { right };
            }*/
            //let entry_addr = entry as *const _ as usize;
            if cmp != Equal {
                #[cfg(debug)]
                dbg_assert!(!local_has_been_equal);
                left = if cmp == Less {
                    //let max_new_size = right - mid - 1;
                    if right - mid < subslice_size_threshold {
                        // Suppose `entry_addr` is the first (aligned) entry in the cache line.
                        //
                        // let line_start = entry_addr & CACHE_LINE_START_MASK;
                        //
                        // let left_waste = entry_addr % cache_line_size;
                        let mut right_neighbor_distance = 0;
                        let new_left = loop {
                            if right_neighbor_distance == max_right_neighbors_in_cache_line {
                                break mid + 1 + right_neighbor_distance;
                            }

                            let entry =
                                unsafe { self.get_unchecked(mid + 1 + right_neighbor_distance) };

                            if entry.cmp_local(given) == Less {
                                right_neighbor_distance += 1;
                                continue;
                            } else {
                                break mid + 1 + right_neighbor_distance;
                            }
                        };
                        debug_assert!(new_left - (mid + 1) <= max_right_neighbors_in_cache_line);
                        new_left
                    } else {
                        mid + 1
                    }
                } else {
                    left
                };
                right = if cmp == Greater {
                    // NO skimming through the cache - because we're checking ONLY to the "LEFT" of
                    // here.
                    mid
                } else {
                    right
                };
            } else {
                #[cfg(debug)]
                let _ = {
                    local_has_been_equal = true;
                };
                if T::LOCALITY.has_non_local() {
                    // NOT cache-based (or, at least, NOT related to entry's cache line)
                    let cmp = entry.cmp_non_local(given);

                    // From Rust source of `binary_search_by`
                    //
                    // This control flow produces conditional moves, which results in fewer branches
                    // and instructions than if/else or matching on cmp::Ordering. This is x86 asm
                    // for u8: https://rust.godbolt.org/z/698eYffTx.
                    left = if cmp == Less { mid + 1 } else { left };
                    right = if cmp == Greater { mid } else { right };
                    if cmp == Equal {
                        // SAFETY: same as the `get_unchecked` above
                        unsafe { hint::assert_unchecked(mid < self.len()) };
                        return Ok(mid);
                    }
                } else {
                    // SAFETY: same as the `get_unchecked` above
                    unsafe { hint::assert_unchecked(mid < self.len()) };
                    return Ok(mid);
                }
            }

            size = right - left;
        }

        // SAFETY: directly true from the overall invariant. Note that this is `<=`, unlike the
        // assume in the `Ok` path.
        unsafe { hint::assert_unchecked(left <= self.len()) };
        Err(left)
    }
}

impl CfPartialEq for u8 {
    const LOCALITY: Locality = Locality::PureLocal;

    // If unsure, then it's `false`.
    const COMPATIBLE_WITH_PARTIAL_EQ: bool = true;

    fn eq_local(&self, other: &Self) -> bool {
        self == other
    }
    fn eq_non_local(&self, other: &Self) -> bool {
        debug_assert!(false, "unreachable");
        self == other
    }
    fn eq_full(&self, other: &Self) -> bool {
        self == other
    }
}
impl CfOrd for u8 {
    const COMPATIBLE_WITH_ORD: bool = true;

    fn cmp_local(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }

    fn cmp_non_local(&self, other: &Self) -> Ordering {
        unreachable!("NOT to be used")
    }

    fn cmp_full(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u8_bin_search() {
        let v = vec![0u8, 2, 6, 40, 41, 80, 81];
        assert_eq!(v.binary_search_cf(&2), Ok(1));
    }
}
