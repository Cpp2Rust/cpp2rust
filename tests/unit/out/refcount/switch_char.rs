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
            v if v == (('a' as u8) as i32) => {
                return 1;
            }
            v if v == (('b' as u8) as i32) => {
                return 2;
            }
            v if v == (('\n' as u8) as i32) => {
                return 3;
            }
            v if v == (('\0' as u8) as i32) => {
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
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(
        (({
            let _c: u8 = ('a' as u8);
            switch_char_0(_c)
        }) == 1)
    );
    assert!(
        (({
            let _c: u8 = ('b' as u8);
            switch_char_0(_c)
        }) == 2)
    );
    assert!(
        (({
            let _c: u8 = ('\n' as u8);
            switch_char_0(_c)
        }) == 3)
    );
    assert!(
        (({
            let _c: u8 = ('\0' as u8);
            switch_char_0(_c)
        }) == 4)
    );
    assert!(
        (({
            let _c: u8 = ('z' as u8);
            switch_char_0(_c)
        }) == 0)
    );
    return 0;
}
