extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive()]
pub struct WOFF2Params {
    pub extended_metadata: Value<Vec<u8>>,
    pub brotli_quality: Value<i32>,
    pub allow_transforms: Value<bool>,
}
impl WOFF2Params {
    pub fn WOFF2Params() -> Self {
        let mut this = Self {
            extended_metadata: Rc::new(RefCell::new(
                Ptr::from_string_literal("")
                    .to_c_string_iterator()
                    .chain(std::iter::once(0))
                    .collect::<Vec<u8>>(),
            )),
            brotli_quality: Rc::new(RefCell::new(11)),
            allow_transforms: Rc::new(RefCell::new(true)),
        };
        this
    }
}
impl Clone for WOFF2Params {
    fn clone(&self) -> Self {
        let mut this = Self {
            extended_metadata: Rc::new(RefCell::new((*self.extended_metadata.borrow()).clone())),
            brotli_quality: Rc::new(RefCell::new((*self.brotli_quality.borrow()))),
            allow_transforms: Rc::new(RefCell::new((*self.allow_transforms.borrow()))),
        };
        this
    }
}
impl Default for WOFF2Params {
    fn default() -> Self {
        {
            WOFF2Params::WOFF2Params()
        }
    }
}
impl ByteRepr for WOFF2Params {}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let params: Value<WOFF2Params> = Rc::new(RefCell::new(WOFF2Params::WOFF2Params()));
    assert!((((*(*params.borrow()).extended_metadata.borrow()).len() - 1) as u64 == 0_u64));
    assert!(((*(*params.borrow()).brotli_quality.borrow()) == 11));
    assert!((((*(*params.borrow()).allow_transforms.borrow()) as i32) == (true as i32)));
    return 0;
}
