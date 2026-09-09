extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Sized {}
impl Clone for Sized {
    fn clone(&self) -> Self {
        let __this: Value<Sized> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<Sized> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Sized {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
pub fn is_small_0() -> bool {
    return true;
}
pub fn is_small_1() -> bool {
    return false;
}
pub fn has_size_2() -> bool {
    return true;
}
pub fn has_size_3() -> bool {
    return false;
}
pub fn pick_4(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    if (true) && (true) {
        return 1;
    }
    return 2;
}
pub fn pick_5(x: i64) -> i32 {
    let x: Value<i64> = Rc::new(RefCell::new(x));
    if (true) && (false) {
        return 1;
    }
    return 2;
}
pub fn pick_6(x: f32) -> i32 {
    let x: Value<f32> = Rc::new(RefCell::new(x));
    if (false) && (true) {
        return 1;
    }
    return 2;
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(({ is_small_0() }));
    assert!(!({ is_small_1() }));
    assert!(true);
    assert!(!false);
    assert!(({ has_size_2() }));
    assert!(!({ has_size_3() }));
    assert!((({ pick_4(1,) }) == 1));
    assert!((({ pick_5(1_i64,) }) == 2));
    assert!((({ pick_6(1.0E+0,) }) == 2));
    return 0;
}
pub trait SizedImpl {
    fn size(&self) -> i32;
}
impl SizedImpl for Ptr<Sized> {
    fn size(&self) -> i32 {
        return 4;
    }
}
