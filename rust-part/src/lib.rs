use std::ffi::{c_char, CStr};

use data_structure::vec_wrapper::VecWrapper;
use ex_structs::{StructA, StructB};

mod data_structure;
mod ex_structs;

#[no_mangle]
pub extern "C" fn test_hello() {
    println!("Hello, from Rust!")
}

#[no_mangle]
pub extern "C" fn print_strings(strings: VecWrapper<*const c_char>) {
    let string_vec: Vec<String> = strings
        .to_vec()
        .iter()
        .map(|cchar| unsafe {
            CStr::from_ptr(*cchar)
                .to_str()
                .expect("CStr to &str failed.")
                .to_string()
        })
        .collect();

    println!("{}", string_vec.join(" "))
}

#[no_mangle]
// A dummy function to make cbindgen generate C representation of StructA, Struct B
pub extern "C" fn _dummy(_a: &StructA, _b: &StructB) {
    // it's a dummy function, don't call this function.
    panic!("This function shouldn't be called.")
}
