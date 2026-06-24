extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum color {
    #[default]
    RED = 0,
    GREEN = 1,
    BLUE = 2,
}
impl From<i32> for color {
    fn from(n: i32) -> color {
        match n {
            0 => color::RED,
            1 => color::GREEN,
            2 => color::BLUE,
            _ => panic!("invalid color value: {}", n),
        }
    }
}
libcc2rs::impl_enum_inc_dec!(color);
impl ByteRepr for color {
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self as i32).to_bytes(buf);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        <color>::from(i32::from_bytes(buf))
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let c: Value<color> = Rc::new(RefCell::new(color::BLUE));
    (*c.borrow_mut()).postfix_inc();
    return if (((((*c.borrow()) as u32) == ((color::RED as i32) as u32)) as i32) != 0) {
        0
    } else {
        1
    };
}
