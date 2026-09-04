extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct widget {
    pub id: Value<i32>,
}
impl Clone for widget {
    fn clone(&self) -> Self {
        Self {
            id: Rc::new(RefCell::new((*self.id.borrow()).clone())),
        }
    }
}
impl ByteRepr for widget {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.id.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            id: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
pub fn a_value_0() -> i32 {
    let w: Value<widget> = <Value<widget>>::default();
    (*(*w.borrow()).id.borrow_mut()) = 11;
    return (*(*w.borrow()).id.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((((({ a_value_0() }) == 11) as i32) != 0));
    assert!((((({ b_value_1() }) == 2) as i32) != 0));
    return 0;
}
pub type widget_enum = u32;
pub const widget_enum_WIDGET_A: widget_enum = 0;
pub const widget_enum_WIDGET_B: widget_enum = 1;
pub const widget_enum_WIDGET_C: widget_enum = 2;
pub fn b_value_1() -> i32 {
    let w: Value<widget_enum> = Rc::new(RefCell::new(widget_enum_WIDGET_C));
    return ((*w.borrow()) as i32);
}
