extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
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
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut c: color = color::BLUE;
    c.postfix_inc();
    return if ((((c as u32) == ((color::RED as i32) as u32)) as i32) != 0) {
        0
    } else {
        1
    };
}
