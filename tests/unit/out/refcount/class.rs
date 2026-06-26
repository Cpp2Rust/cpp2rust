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
    pub first: Value<i32>,
    pub second: Value<i32>,
}
impl Pair {
    pub fn NOP(&self) {}
    pub fn GetFirst(&self) -> i32 {
        return (*self.first.borrow());
    }
    pub fn GetSecond(&self) -> i32 {
        return (*self.second.borrow());
    }
    pub fn Set(&self, field: Ptr<i32>, new_val: i32) -> i32 {
        let new_val: Value<i32> = Rc::new(RefCell::new(new_val));
        ({ self.NOP() });
        let old_val: Value<i32> = Rc::new(RefCell::new((field.read())));
        let __rhs = (*new_val.borrow());
        field.write(__rhs);
        return (*old_val.borrow());
    }
    pub fn SetFirst(&self, new_first: i32) -> i32 {
        let new_first: Value<i32> = Rc::new(RefCell::new(new_first));
        return (({ self.GetFirst() })
            + ({
                let _field: Ptr<i32> = self.first.as_pointer();
                self.Set(_field, (*new_first.borrow()))
            }));
    }
    pub fn SetSecond(&self, new_second: i32) -> i32 {
        let new_second: Value<i32> = Rc::new(RefCell::new(new_second));
        return (({ self.GetSecond() })
            + ({
                let _field: Ptr<i32> = self.second.as_pointer();
                self.Set(_field, (*new_second.borrow()))
            }));
    }
}
impl Clone for Pair {
    fn clone(&self) -> Self {
        let mut this = Self {
            first: Rc::new(RefCell::new((*self.first.borrow()))),
            second: Rc::new(RefCell::new((*self.second.borrow()))),
        };
        this
    }
}
impl ByteRepr for Pair {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.first.borrow()).to_bytes(&mut buf[0..4]);
        (*self.second.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            first: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            second: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
        }
    }
}
#[derive(Default)]
pub struct Route {
    pub path: Value<Pair>,
    pub cost: Value<f64>,
}
impl Route {
    pub fn SetCost(&self, new_cost: f64) -> f64 {
        let new_cost: Value<f64> = Rc::new(RefCell::new(new_cost));
        let old_cost: Value<f64> = Rc::new(RefCell::new((*self.cost.borrow())));
        (*self.cost.borrow_mut()) = (*new_cost.borrow());
        return (*old_cost.borrow());
    }
}
impl Clone for Route {
    fn clone(&self) -> Self {
        let mut this = Self {
            path: Rc::new(RefCell::new((*self.path.borrow()).clone())),
            cost: Rc::new(RefCell::new((*self.cost.borrow()))),
        };
        this
    }
}
impl ByteRepr for Route {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.path.borrow()).to_bytes(&mut buf[0..8]);
        (*self.cost.borrow()).to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            path: Rc::new(RefCell::new(<Pair>::from_bytes(&buf[0..8]))),
            cost: Rc::new(RefCell::new(<f64>::from_bytes(&buf[8..16]))),
        }
    }
}
pub fn RandomRoute_0(route: Ptr<Route>) -> i32 {
    if (((*(*(*route.upgrade().deref()).path.borrow()).first.borrow()) % 2) != 0) {
        return ({
            let _new_first: i32 = ({ (*(*route.upgrade().deref()).path.borrow()).SetSecond(10) });
            (*(*route.upgrade().deref()).path.borrow()).SetFirst(_new_first)
        });
    } else {
        return ({
            let _new_second: i32 =
                ({ (*(*route.upgrade().deref()).path.borrow()).SetFirst(-10_i32) });
            (*(*route.upgrade().deref()).path.borrow()).SetSecond(_new_second)
        });
    }
    panic!("ub: non-void function does not return a value")
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let route1: Value<Route> = Rc::new(RefCell::new(Route {
        path: Rc::new(RefCell::new(Pair {
            first: Rc::new(RefCell::new(0)),
            second: Rc::new(RefCell::new(1)),
        })),
        cost: Rc::new(RefCell::new(5_f64)),
    }));
    let route2: Value<Route> = Rc::new(RefCell::new(Route {
        path: Rc::new(RefCell::new(Pair {
            first: Rc::new(RefCell::new(1)),
            second: Rc::new(RefCell::new(0)),
        })),
        cost: Rc::new(RefCell::new(10_f64)),
    }));
    let old_cost: Value<f64> = Rc::new(RefCell::new(
        ({ (*route1.borrow()).SetCost(({ (*route2.borrow()).SetCost(15_f64) })) }),
    ));
    return ((((({ RandomRoute_0(route1.as_pointer()) }) + ({ RandomRoute_0(route2.as_pointer()) }))
        as f64)
        + (*old_cost.borrow())) as i32);
}
