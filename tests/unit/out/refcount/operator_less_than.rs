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
impl std::cmp::Ord for Pair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if PairImpl::operator_lt(
                &Rc::new(RefCell::new(Pair {
                    x: self.x.clone(),
                    y: self.y.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(Pair {
                    x: other.x.clone(),
                    y: other.y.clone(),
                }))
                .as_pointer(),
            ) {
                std::cmp::Ordering::Less
            } else if PairImpl::operator_lt(
                &Rc::new(RefCell::new(Pair {
                    x: other.x.clone(),
                    y: other.y.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(Pair {
                    x: self.x.clone(),
                    y: self.y.clone(),
                }))
                .as_pointer(),
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        {
            !(PairImpl::operator_lt(
                &Rc::new(RefCell::new(Pair {
                    x: self.x.clone(),
                    y: self.y.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(Pair {
                    x: other.x.clone(),
                    y: other.y.clone(),
                }))
                .as_pointer(),
            )) && !(PairImpl::operator_lt(
                &Rc::new(RefCell::new(Pair {
                    x: other.x.clone(),
                    y: other.y.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(Pair {
                    x: self.x.clone(),
                    y: self.y.clone(),
                }))
                .as_pointer(),
            ))
        }
    }
}
impl std::cmp::Eq for Pair {}
impl Clone for Pair {
    fn clone(&self) -> Self {
        let __this: Value<Pair> = Rc::new(RefCell::new(Self {
            x: Rc::new(RefCell::new((*self.x.borrow()))),
            y: Rc::new(RefCell::new((*self.y.borrow()))),
        }));
        let this: Ptr<Pair> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Pair {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.x.borrow()).to_bytes(&mut buf[0..4]);
        (*self.y.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            y: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
        }
    }
}
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
    assert!(({ PairImpl::operator_lt(&pair1.as_pointer(), pair2.as_pointer(),) }));
    return 0;
}
pub trait PairImpl {
    fn operator_lt(&self, other: Ptr<Pair>) -> bool;
}
impl PairImpl for Ptr<Pair> {
    fn operator_lt(&self, other: Ptr<Pair>) -> bool {
        return ({
            let _lhs = (*(*(*self).upgrade().deref()).x.borrow());
            _lhs < (*(*other.upgrade().deref()).x.borrow())
        }) || (({
            let _lhs = (*(*(*self).upgrade().deref()).x.borrow());
            _lhs == (*(*other.upgrade().deref()).x.borrow())
        }) && ({
            let _lhs = (*(*(*self).upgrade().deref()).y.borrow());
            _lhs < (*(*other.upgrade().deref()).y.borrow())
        }));
    }
}
