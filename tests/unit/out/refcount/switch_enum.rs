extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
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
pub fn switch_enum_0(c: Color) -> i32 {
    let c: Value<Color> = Rc::new(RefCell::new(c));
    'switch: {
        let __match_cond = ((*c.borrow()) as i32);
        match __match_cond {
            __v if __v == (Color::kRed as i32) => {
                return 10;
            }
            __v if __v == (Color::kGreen as i32) => {
                return 20;
            }
            __v if __v == (Color::kBlue as i32) => {
                return 30;
            }
            _ => {}
        }
    };
    return -1_i32;
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((({ switch_enum_0(Color::kRed,) }) == 10));
    assert!((({ switch_enum_0(Color::kGreen,) }) == 20));
    assert!((({ switch_enum_0(Color::kBlue,) }) == 30));
    return 0;
}
