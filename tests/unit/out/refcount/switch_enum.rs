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
pub fn switch_enum_0(c: Color) -> i32 {
    let c: Value<Color> = Rc::new(RefCell::new(c));
    'switch: {
        let __match_cond = ((*c.borrow()) as i32);
        match __match_cond {
            v if v == (Color::kRed as i32) => {
                return 10;
            }
            v if v == (Color::kGreen as i32) => {
                return 20;
            }
            v if v == (Color::kBlue as i32) => {
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
    assert!(
        (({
            let _c: Color = Color::kRed;
            switch_enum_0(_c)
        }) == 10)
    );
    assert!(
        (({
            let _c: Color = Color::kGreen;
            switch_enum_0(_c)
        }) == 20)
    );
    assert!(
        (({
            let _c: Color = Color::kBlue;
            switch_enum_0(_c)
        }) == 30)
    );
    return 0;
}
