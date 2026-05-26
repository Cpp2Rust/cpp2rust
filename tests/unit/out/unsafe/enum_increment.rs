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
    COLOR_LAST = 3,
}
impl From<i32> for color {
    fn from(n: i32) -> color {
        match n {
            0 => color::RED,
            1 => color::GREEN,
            2 => color::BLUE,
            3 => color::COLOR_LAST,
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
    let mut count: i32 = 0;
    let mut c: color = color::RED;
    'loop_: while ((((c as u32) < ((color::COLOR_LAST as i32) as u32)) as i32) != 0) {
        count.postfix_inc();
        c.postfix_inc();
    }
    assert!(((((count) == (3)) as i32) != 0));
    let mut c: color = color::RED;
    assert!(((((c.postfix_inc() as u32) == ((color::RED as i32) as u32)) as i32) != 0));
    assert!(((((c.prefix_inc() as u32) == ((color::BLUE as i32) as u32)) as i32) != 0));
    assert!(((((c.postfix_dec() as u32) == ((color::BLUE as i32) as u32)) as i32) != 0));
    assert!(((((c.prefix_dec() as u32) == ((color::RED as i32) as u32)) as i32) != 0));
    return 0;
}
