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
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let count: Value<i32> = Rc::new(RefCell::new(0));
    let c: Value<color> = Rc::new(RefCell::new(color::RED));
    'loop_: while (((((*c.borrow()) as u32) < ((color::COLOR_LAST as i32) as u32)) as i32) != 0) {
        (*count.borrow_mut()).postfix_inc();
        (*c.borrow_mut()).postfix_inc();
    }
    assert!(((((*count.borrow()) == 3) as i32) != 0));
    let c: Value<color> = Rc::new(RefCell::new(color::RED));
    assert!(
        (((((*c.borrow_mut()).postfix_inc() as u32) == ((color::RED as i32) as u32)) as i32) != 0)
    );
    assert!(
        (((((*c.borrow_mut()).prefix_inc() as u32) == ((color::BLUE as i32) as u32)) as i32) != 0)
    );
    assert!(
        (((((*c.borrow_mut()).postfix_dec() as u32) == ((color::BLUE as i32) as u32)) as i32) != 0)
    );
    assert!(
        (((((*c.borrow_mut()).prefix_dec() as u32) == ((color::RED as i32) as u32)) as i32) != 0)
    );
    return 0;
}
