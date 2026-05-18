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
                let _new_val: i32 = (*new_first.borrow());
                self.Set(_field, _new_val)
            }));
    }
    pub fn SetSecond(&self, new_second: i32) -> i32 {
        let new_second: Value<i32> = Rc::new(RefCell::new(new_second));
        return (({ self.GetSecond() })
            + ({
                let _field: Ptr<i32> = self.second.as_pointer();
                let _new_val: i32 = (*new_second.borrow());
                self.Set(_field, _new_val)
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
impl ByteRepr for Pair {}
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
impl ByteRepr for Route {}
pub fn RandomRoute_0(route: Ptr<Route>) -> i32 {
    if (((*(*(*route.upgrade().deref()).path.borrow()).first.borrow()) % 2) != 0) {
        return ({
            let _new_first: i32 = ({
                let _new_second: i32 = 10;
                (*(*route.upgrade().deref()).path.borrow()).SetSecond(_new_second)
            });
            (*(*route.upgrade().deref()).path.borrow()).SetFirst(_new_first)
        });
    } else {
        return ({
            let _new_second: i32 = ({
                let _new_first: i32 = -10_i32;
                (*(*route.upgrade().deref()).path.borrow()).SetFirst(_new_first)
            });
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
        ({
            let _new_cost: f64 = ({
                let _new_cost: f64 = 15_f64;
                (*route2.borrow()).SetCost(_new_cost)
            });
            (*route1.borrow()).SetCost(_new_cost)
        }),
    ));
    return ((((({
        let _route: Ptr<Route> = route1.as_pointer();
        RandomRoute_0(_route)
    }) + ({
        let _route: Ptr<Route> = route2.as_pointer();
        RandomRoute_0(_route)
    })) as f64)
        + (*old_cost.borrow())) as i32);
}
