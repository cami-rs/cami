#[cfg(not(feature = "std"))]
const _: () = {
    panic!("All benches require at least 'std' feature. See benches/README.md.");
};

fn main() {}
