extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_promotions_0(count: i32, __args: &[VaArg]) -> i32 {
    let count: Value<i32> = Rc::new(RefCell::new(count));
    let ap: Value<VaList> = Rc::new(RefCell::new(VaList::default()));
    (*ap.borrow_mut()) = VaList::new(__args);
    let mut a: i32 = (*ap.borrow_mut()).arg::<i32>();
    let mut b: i32 = (*ap.borrow_mut()).arg::<i32>();
    let mut c: f64 = (*ap.borrow_mut()).arg::<f64>();
    assert!((((a == 65) as i32) != 0));
    assert!((((b == 10) as i32) != 0));
    assert!((((c == 3_f64) as i32) != 0));
    return ((a + b) + (c as i32));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut x: i8 = (('A' as i32) as i8);
    let mut y: i16 = 10_i16;
    let mut z: f32 = 3_f32;
    assert!(
        (((({
            test_promotions_0(
                3,
                &[(x as i32).into(), (y as i32).into(), (z as f64).into()],
            )
        }) == 78) as i32)
            != 0)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
