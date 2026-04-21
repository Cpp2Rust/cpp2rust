extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(Clone)]
pub struct WOFF2Params {
    pub extended_metadata: Vec<u8>,
    pub brotli_quality: i32,
    pub allow_transforms: bool,
}
impl WOFF2Params {
    pub unsafe fn WOFF2Params() -> Self {
        let mut this = Self {
            extended_metadata: {
                let s = b"\0".as_ptr();
                std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1)
                    .to_vec()
            },
            brotli_quality: 11,
            allow_transforms: true,
        };
        this
    }
}
impl Default for WOFF2Params {
    fn default() -> Self {
        unsafe { WOFF2Params::WOFF2Params() }
    }
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut params: WOFF2Params = WOFF2Params::WOFF2Params();
    assert!((((params.extended_metadata.len() - 1) as u64) == (0_u64)));
    assert!(((params.brotli_quality) == (11)));
    assert!(((params.allow_transforms as i32) == (true as i32)));
    return 0;
}
