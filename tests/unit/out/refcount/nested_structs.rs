extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Level0_Level1_1_Level2_1_Level3_1 {
    pub x1: Value<i32>,
}
impl Clone for Level0_Level1_1_Level2_1_Level3_1 {
    fn clone(&self) -> Self {
        let mut this = Self {
            x1: Rc::new(RefCell::new((*self.x1.borrow()))),
        };
        this
    }
}
impl ByteRepr for Level0_Level1_1_Level2_1_Level3_1 {}
#[derive(Default)]
pub struct Level0_Level1_1_Level2_1_Level3_2 {
    pub x1: Value<i32>,
    pub x2: Value<i32>,
}
impl Clone for Level0_Level1_1_Level2_1_Level3_2 {
    fn clone(&self) -> Self {
        let mut this = Self {
            x1: Rc::new(RefCell::new((*self.x1.borrow()))),
            x2: Rc::new(RefCell::new((*self.x2.borrow()))),
        };
        this
    }
}
impl ByteRepr for Level0_Level1_1_Level2_1_Level3_2 {}
#[derive(Default)]
pub struct Level0_Level1_1_Level2_1 {
    pub x1: Value<i32>,
}
impl Clone for Level0_Level1_1_Level2_1 {
    fn clone(&self) -> Self {
        let mut this = Self {
            x1: Rc::new(RefCell::new((*self.x1.borrow()))),
        };
        this
    }
}
impl ByteRepr for Level0_Level1_1_Level2_1 {}
#[derive(Default)]
pub struct Level0_Level1_1 {
    pub x1: Value<i32>,
}
impl Clone for Level0_Level1_1 {
    fn clone(&self) -> Self {
        let mut this = Self {
            x1: Rc::new(RefCell::new((*self.x1.borrow()))),
        };
        this
    }
}
impl ByteRepr for Level0_Level1_1 {}
#[derive(Default)]
pub struct Level0_Level1_2 {
    pub x1: Value<i32>,
    pub x2: Value<i32>,
}
impl Clone for Level0_Level1_2 {
    fn clone(&self) -> Self {
        let mut this = Self {
            x1: Rc::new(RefCell::new((*self.x1.borrow()))),
            x2: Rc::new(RefCell::new((*self.x2.borrow()))),
        };
        this
    }
}
impl ByteRepr for Level0_Level1_2 {}
#[derive(Default)]
pub struct Level0 {}
impl Clone for Level0 {
    fn clone(&self) -> Self {
        let mut this = Self {};
        this
    }
}
impl ByteRepr for Level0 {}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x1: Value<Level0_Level1_1> = Rc::new(RefCell::new(Level0_Level1_1 {
        x1: Rc::new(RefCell::new(0)),
    }));
    let x2: Value<Level0_Level1_2> = Rc::new(RefCell::new(Level0_Level1_2 {
        x1: Rc::new(RefCell::new(1)),
        x2: Rc::new(RefCell::new(2)),
    }));
    let x3: Value<Level0_Level1_1_Level2_1> = Rc::new(RefCell::new(Level0_Level1_1_Level2_1 {
        x1: Rc::new(RefCell::new(3)),
    }));
    let x4: Value<Level0_Level1_1_Level2_1_Level3_1> =
        Rc::new(RefCell::new(Level0_Level1_1_Level2_1_Level3_1 {
            x1: Rc::new(RefCell::new(4)),
        }));
    let x5: Value<Level0_Level1_1_Level2_1_Level3_2> =
        Rc::new(RefCell::new(Level0_Level1_1_Level2_1_Level3_2 {
            x1: Rc::new(RefCell::new(5)),
            x2: Rc::new(RefCell::new(6)),
        }));
    return 0;
}
