use lazy_static::lazy_static;
use libc::{c_char, fdopen};
use regex::Regex;
use std::{ffi::CStr, fs::File, os::unix::prelude::IntoRawFd};

lazy_static! {
    static ref METALBEAR_PREFIX: Regex =
        Regex::new("metalbear_").expect("Failed creating metalbear_ prefix regex!");
}

/// NOTE(alex): Produces "libmetalbear_lib.so".
#[no_mangle]
pub extern "C" fn fopen(filename: *const c_char, mode: *const c_char) -> *mut libc::FILE {
    let filename_str = unsafe { CStr::from_ptr(filename) }
        .to_str()
        .expect("Failed converting filename from *c_char!");

    if METALBEAR_PREFIX.is_match(filename_str) {
        let modified_file = format!("{filename_str}.bkp");

        let file = File::open(modified_file).expect("Failed to open file {modified_file}");
        unsafe { fdopen(file.into_raw_fd(), mode) }
    } else {
        let file = File::open(filename_str).expect("Failed to open file {filename_str}");
        unsafe { fdopen(file.into_raw_fd(), mode) }
    }
}
