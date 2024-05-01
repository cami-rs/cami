use core::hint;
use fastrand::Rng;

// For purging the L1, L2..., in bytes.
const MAX_CACHE_SIZE: usize = 2_080_000;

//const USIZE_MAX_HALF: usize = usize::MAX / 2;

pub fn purge_cache(rng: &mut Rng) {
    let mut vec = Vec::<u8>::with_capacity(MAX_CACHE_SIZE);

    for _ in [0..MAX_CACHE_SIZE] {
        vec.push(rng.u8(..));
    }
    hint::black_box(vec);
}
