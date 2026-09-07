extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type color = u32;
pub const color_RED: color = 0;
pub const color_GREEN: color = 1;
pub const color_BLUE: color = 2;
pub const color_COLOR_LAST: color = 3;
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let count: Value<i32> = Rc::new(RefCell::new(0));
    let c: Value<color> = Rc::new(RefCell::new(color_RED));
    'loop_: while (((((*c.borrow()) as u32) < ((color_COLOR_LAST as i32) as u32)) as i32) != 0) {
        (*count.borrow_mut()).postfix_inc();
        (*c.borrow_mut()).postfix_inc();
    }
    assert!(((((*count.borrow()) == 3) as i32) != 0));
    let c: Value<color> = Rc::new(RefCell::new(color_RED));
    assert!(
        (((((*c.borrow_mut()).postfix_inc() as u32) == ((color_RED as i32) as u32)) as i32) != 0)
    );
    assert!(
        (((((*c.borrow_mut()).prefix_inc() as u32) == ((color_BLUE as i32) as u32)) as i32) != 0)
    );
    assert!(
        (((((*c.borrow_mut()).postfix_dec() as u32) == ((color_BLUE as i32) as u32)) as i32) != 0)
    );
    assert!(
        (((((*c.borrow_mut()).prefix_dec() as u32) == ((color_RED as i32) as u32)) as i32) != 0)
    );
    return 0;
}
