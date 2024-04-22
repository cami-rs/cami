use crate::COrd;
use core::cmp::Ordering;
#[cfg(feature = "hint_assert_unchecked")]
use core::hint;
use core::mem;

#[cfg(test)]
mod se_tests_2d;
#[cfg(test)]
mod se_tests_3d;
#[cfg(test)]
mod se_tests_bin_search;
#[cfg(test)]
mod se_tests_strs;

pub trait SliceExt<T> {
    fn binary_search_ca(&self, x: &T) -> Result<usize, usize>
    where
        T: COrd;
    // @TODO ?? non-binary methods, like contains()
}

impl<T: COrd + Ord> SliceExt<T> for [T] {
    // @TODO factor out part(s) to separate, non-generic functions, to decrease the cost of generic
    // copies
    fn binary_search_ca(&self, given: &T) -> Result<usize, usize> {
        if !T::LOCALITY.has_local() {
            // Any comparison is based on non-local fields only. Hence standard binary search.
            return <[T]>::binary_search(self, given);
        }
        // @TODO change from `let` to `const`, wherever possible.
        let entry_size = mem::size_of::<T>();
        // TODO runtime: Use https://docs.rs/crossbeam-utils/latest/crossbeam_utils/struct.CachePadded.html && https://docs.rs/cache-size/latest/cache_size.
        let cache_line_size = 128usize; // in bytes
        const _CACHE_LINE_START_MASK: usize = 0xFFFFFFFFFFFFFFE0;

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
        // TODO: Make these constants part of COrd trait and/or feature
        //
        // Used with a `<` operator.
        let subslice_size_threshold = 3 * max_entries_per_cache_line + 2;
        // @TODO const
        let max_right_neighbors_in_cache_line = max_entries_per_cache_line - 2;

        // Based on Rust source of `binary_search`
        // (https://doc.rust-lang.org/nightly/core/primitive.slice.html#method.binary_search) ->
        // `binary_search_by`
        // https://doc.rust-lang.org/nightly/core/primitive.slice.html#method.binary_search_by

        // INVARIANTS:
        // - 0 <= left <= left + size = right <= self.len()
        // - f returns Ordering::Less for everything in self[..left]
        // - f returns Ordering::Greater for everything in self[right..]
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
            #[cfg(feature = "unsafe_from_rust_source")]
            let entry = unsafe { self.get_unchecked(mid) };
            #[cfg(not(feature = "unsafe"))]
            let entry = &self[mid];

            let cmp = entry.cmp_local(given);

            // This control flow produces conditional moves, which results in fewer branches and
            // instructions than if/else or matching on cmp::Ordering. This is x86 asm for u8:
            // https://rust.godbolt.org/z/698eYffTx.
            /*if false {
                left = if cmp == Ordering::Less { mid + 1 } else { left };

                right = if cmp == Ordering::Greater { mid } else { right };
            }*/
            //let entry_addr = entry as *const _ as usize;
            if cmp != Ordering::Equal {
                #[cfg(debug)]
                dbg_assert!(!local_has_been_equal);
                left = if cmp == Ordering::Less {
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

                            #[cfg(feature = "unsafe")] // NOT from Rust source
                            let entry =
                                unsafe { self.get_unchecked(mid + 1 + right_neighbor_distance) };
                            #[cfg(not(feature = "unsafe"))]
                            let entry = &self[mid + 1 + right_neighbor_distance];

                            if entry.cmp_local(given) == Ordering::Less {
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
                right = if cmp == Ordering::Greater {
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
                    left = if cmp == Ordering::Less { mid + 1 } else { left };
                    right = if cmp == Ordering::Greater { mid } else { right };
                    if cmp == Ordering::Equal {
                        // SAFETY: same as the `get_unchecked` above
                        #[cfg(feature = "hint_assert_unchecked")]
                        // @TODO new feature w/ unsafe_from_rust_source
                        unsafe {
                            hint::assert_unchecked(mid < self.len())
                        };
                        #[cfg(not(feature = "hint_assert_unchecked"))]
                        debug_assert!(mid < self.len());

                        return Ok(mid);
                    }
                } else {
                    // SAFETY: same as the `get_unchecked` above
                    #[cfg(feature = "hint_assert_unchecked")] //@TODO w/ unsafe_from_rust_source
                    unsafe {
                        hint::assert_unchecked(mid < self.len())
                    };
                    #[cfg(not(feature = "hint_assert_unchecked"))]
                    debug_assert!(mid < self.len());

                    return Ok(mid);
                }
            }

            size = right - left;
        }

        // SAFETY: directly true from the overall invariant. Note that this is `<=`, unlike the
        // assume in the `Ok` path.
        #[cfg(feature = "hint_assert_unchecked")] // @TODO w/ unsafe_from_rust_source
        unsafe {
            hint::assert_unchecked(left <= self.len())
        };
        #[cfg(not(feature = "hint_assert_unchecked"))]
        debug_assert!(left <= self.len());

        Err(left)
    }
}
