extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Pair {
    pub x: Value<i32>,
    pub y: Value<i32>,
}
impl Pair {
    pub fn lt(&self, other: Ptr<Pair>) -> bool {
        return {
            let _lhs = {
                let _lhs = (*self.x.borrow());
                _lhs < (*(*other.upgrade().deref()).x.borrow())
            };
            _lhs || ({
                let _lhs = {
                    let _lhs = (*self.x.borrow());
                    _lhs == (*(*other.upgrade().deref()).x.borrow())
                };
                _lhs && {
                    let _lhs = (*self.y.borrow());
                    _lhs < (*(*other.upgrade().deref()).y.borrow())
                }
            })
        };
    }
}
impl Ord for Pair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if self.lt(Rc::new(RefCell::new(other.clone())).as_pointer()) {
                std::cmp::Ordering::Less
            } else if other.lt(Rc::new(RefCell::new(self.clone())).as_pointer()) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        {
            !(self.lt(Rc::new(RefCell::new(other.clone())).as_pointer()))
                && !(other.lt(Rc::new(RefCell::new(self.clone())).as_pointer()))
        }
    }
}
impl Eq for Pair {}
impl Clone for Pair {
    fn clone(&self) -> Self {
        let mut this = Self {
            x: Rc::new(RefCell::new((*self.x.borrow()))),
            y: Rc::new(RefCell::new((*self.y.borrow()))),
        };
        this
    }
}
impl ByteRepr for Pair {}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let pair1: Value<Pair> = Rc::new(RefCell::new(Pair {
        x: Rc::new(RefCell::new(1)),
        y: Rc::new(RefCell::new(2)),
    }));
    let pair2: Value<Pair> = Rc::new(RefCell::new(Pair {
        x: Rc::new(RefCell::new(1)),
        y: Rc::new(RefCell::new(3)),
    }));
    return ((*pair1.borrow()).lt(pair2.as_pointer()) as i32);
}
