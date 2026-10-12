extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(32)]
pub struct pair {
    #[offset(0)]
    #[byte_size(16)]
    pub a: Value<Box<[i32]>>,
    #[offset(16)]
    #[byte_size(16)]
    pub b: Value<Box<[i32]>>,
}
impl Default for pair {
    fn default() -> Self {
        {
            pair {
                a: Rc::new(RefCell::new((0..4).map(|_| 0_i32).collect::<Box<[i32]>>())),
                b: Rc::new(RefCell::new((0..4).map(|_| 0_i32).collect::<Box<[i32]>>())),
            }
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<pair> = <Value<pair>>::default();
    assert!(
        ((((array_field_ptr!(s.as_pointer(), a) as Ptr::<i32>).offset((4) as isize)
            == (array_field_ptr!(s.as_pointer(), b) as Ptr::<i32>)) as i32)
            != 0)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
