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
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum widget_enum {
    #[default]
    WIDGET_A = 0,
    WIDGET_B = 1,
    WIDGET_C = 2,
}
impl From<i32> for widget_enum {
    fn from(n: i32) -> widget_enum {
        match n {
            0 => widget_enum::WIDGET_A,
            1 => widget_enum::WIDGET_B,
            2 => widget_enum::WIDGET_C,
            _ => panic!("invalid widget_enum value: {}", n),
        }
    }
}
libcc2rs::impl_enum_inc_dec!(widget_enum);
impl ByteRepr for widget_enum {
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self as i32).to_bytes(buf);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        <widget_enum>::from(i32::from_bytes(buf))
    }
}
pub fn b_value_1() -> i32 {
    let w: Value<widget_enum> = Rc::new(RefCell::new(widget_enum::WIDGET_C));
    return ((*w.borrow()) as i32).clone();
}
