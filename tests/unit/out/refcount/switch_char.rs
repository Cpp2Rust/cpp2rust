extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn switch_char_0(c: u8) -> i32 {
    let c: Value<u8> = Rc::new(RefCell::new(c));
    'switch: {
        let __match_cond = ((*c.borrow()) as i32);
        match __match_cond {
            __v if __v == (('a' as u8) as i32) => {
                return 1;
            }
            __v if __v == (('b' as u8) as i32) => {
                return 2;
            }
            __v if __v == (('\n' as u8) as i32) => {
                return 3;
            }
            __v if __v == (('\0' as u8) as i32) => {
                return 4;
            }
            _ => {
                return 0;
            }
        }
    };
    panic!("ub: non-void function does not return a value")
}
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum Color {
    #[default]
    kRed = 0,
    kGreen = 1,
    kBlue = 2,
}
impl From<i32> for Color {
    fn from(n: i32) -> Color {
        match n {
            0 => Color::kRed,
            1 => Color::kGreen,
            2 => Color::kBlue,
            _ => panic!("invalid Color value: {}", n),
        }
    }
}
libcc2rs::impl_enum_inc_dec!(Color);
impl ByteRepr for Color {
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self as i32).to_bytes(buf);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        <Color>::from(i32::from_bytes(buf))
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((({ switch_char_0(('a' as u8),) }) == 1));
    assert!((({ switch_char_0(('b' as u8),) }) == 2));
    assert!((({ switch_char_0(('\n' as u8),) }) == 3));
    assert!((({ switch_char_0(('\0' as u8),) }) == 4));
    assert!((({ switch_char_0(('z' as u8),) }) == 0));
    return 0;
}
