use std::ffi::{c_char, CStr};

use data_structure::vec_wrapper::VecWrapper;

mod data_structure;

#[no_mangle]
pub fn test_hello() {
    println!("Hello, from Rust!")
}

#[no_mangle]
pub fn print_strings(strings: &VecWrapper<*const c_char>) {
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

    for string in string_vec {
        println!("{string}")
    }
}
