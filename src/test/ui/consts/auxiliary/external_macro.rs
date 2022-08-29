#![feature(allow_internal_unstable)]

// Macro to help ensure CONST_ERR lint errors
// are not silenced in external macros.
// https://github.com/rust-lang/rust/issues/65300

#[macro_export]
macro_rules! static_assert {
    ($test:expr) => {
        #[allow(dead_code)]
        const _: () = [()][!($test as bool) as usize];
    };
}
