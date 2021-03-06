#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

extern "C" {
    #[link_name = "\u{1}_ZL1k"]
    pub static k: bool;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct A {
    pub _address: u8,
}
extern "C" {
    #[link_name = "\u{1}_ZN1A1kE"]
    pub static A_k: bool;
}
#[test]
fn bindgen_test_layout_A() {
    assert_eq!(
        ::std::mem::size_of::<A>(),
        1usize,
        concat!("Size of: ", stringify!(A))
    );
    assert_eq!(
        ::std::mem::align_of::<A>(),
        1usize,
        concat!("Alignment of ", stringify!(A))
    );
}
pub type foo = bool;
extern "C" {
    #[link_name = "\u{1}_ZL2k2"]
    pub static k2: foo;
}
