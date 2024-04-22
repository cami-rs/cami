## Internal conventions

### Internal package names & filenames

No `mod.rs` files.

It's OK to have repetition ("stuttering") in internal package names. For example,
`locality::loc_tests`, `macros::mac_c::mac_c_tests_basic`.

That helps when switching between files with no need to see a file path.