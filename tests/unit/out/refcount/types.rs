extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let xu8: Value<u8> = Rc::new(RefCell::new(8_u8));
    let xu16: Value<u16> = Rc::new(RefCell::new(16_u16));
    let xu32: Value<u32> = Rc::new(RefCell::new(32_u32));
    let xu64: Value<u64> = Rc::new(RefCell::new(64_u64));
    let xsz1: Value<usize> = Rc::new(RefCell::new(64_usize));
    let xsz2: Value<usize> = Rc::new(RefCell::new(64_usize));
    let xi1: Value<i8> = Rc::new(RefCell::new((-8_i32 as i8)));
    let xi2: Value<i16> = Rc::new(RefCell::new(16_i16));
    let xi3: Value<i32> = Rc::new(RefCell::new(32));
    let xi4: Value<i64> = Rc::new(RefCell::new(64_i64));
    let b: Value<bool> = Rc::new(RefCell::new(((*xu64.borrow()) == 64_u64)));
    let xld: Value<f64> = Rc::new(RefCell::new(1.5E+0));
    let xwc: Value<i32> = Rc::new(RefCell::new(65_i32));
    let xc8: Value<u8> = Rc::new(RefCell::new(66_u8));
    let xc16: Value<u16> = Rc::new(RefCell::new(67_u16));
    let xc32: Value<u32> = Rc::new(RefCell::new(68_u32));
    let xnp: Value<Value<AnyPtr>> = Rc::new(RefCell::new(Default::default()));
    assert!(
        (((((((((((((*xu8.borrow()) as i32) + ((*xu16.borrow()) as i32)) as u32)
            .wrapping_add((*xu32.borrow()))) as u64)
            .wrapping_add((*xu64.borrow())))
        .wrapping_add(((*xsz1.borrow()) as u64)))
        .wrapping_add(((*xsz2.borrow()) as u64)))
        .wrapping_add(((*xi1.borrow()) as u64)))
        .wrapping_add(((*xi2.borrow()) as u64)))
        .wrapping_add(((*xi3.borrow()) as u64)))
        .wrapping_add(((*xi4.borrow()) as u64))
            == 352_u64)
    );
    assert!((((*xld.borrow()) * 2_f64) == 3_f64));
    assert!(
        (((((*xwc.borrow()) + ((*xc8.borrow()) as i32)) + ((*xc16.borrow()) as i32)) as u32)
            .wrapping_add((*xc32.borrow()))
            == 266_u32)
    );
    assert!((<AnyPtr>::default()).is_null());
    return 0;
}
