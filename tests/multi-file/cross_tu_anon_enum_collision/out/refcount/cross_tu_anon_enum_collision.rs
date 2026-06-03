extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum anon_0 {
    #[default]
    ALPHA = 7,
}
impl From<i32> for anon_0 {
    fn from(n: i32) -> anon_0 {
        match n {
            7 => anon_0::ALPHA,
            _ => panic!("invalid anon_0 value: {}", n),
        }
    }
}
libcc2rs::impl_enum_inc_dec!(anon_0);
pub fn a_value_1() -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(0));
    (*x.borrow_mut()) |= (anon_0::ALPHA as i32);
    return (*x.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((((({ a_value_1() }) == 7) as i32) != 0));
    assert!((((({ b_value_2() }) == 9) as i32) != 0));
    return 0;
}
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum anon_3 {
    #[default]
    BETA = 9,
}
impl From<i32> for anon_3 {
    fn from(n: i32) -> anon_3 {
        match n {
            9 => anon_3::BETA,
            _ => panic!("invalid anon_3 value: {}", n),
        }
    }
}
libcc2rs::impl_enum_inc_dec!(anon_3);
pub fn b_value_2() -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(0));
    (*x.borrow_mut()) |= (anon_3::BETA as i32);
    return (*x.borrow());
}
