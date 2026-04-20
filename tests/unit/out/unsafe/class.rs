extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(Copy, Clone, Default)]
pub struct Pair {
    pub first: i32,
    pub second: i32,
}
impl Pair {
    pub unsafe fn NOP(&mut self) {}
    pub unsafe fn GetFirst(&self) -> i32 {
        return self.first;
    }
    pub unsafe fn GetSecond(&self) -> i32 {
        return self.second;
    }
    pub unsafe fn Set(&mut self, field: *mut i32, mut new_val: i32) -> i32 {
        (unsafe { self.NOP() });
        let mut old_val: i32 = (*field);
        (*field) = new_val;
        return old_val;
    }
    pub unsafe fn SetFirst(&mut self, mut new_first: i32) -> i32 {
        return ((unsafe { self.GetFirst() })
            + (unsafe {
                let _field: *mut i32 = &mut self.first as *mut i32;
                let _new_val: i32 = new_first;
                self.Set(_field, _new_val)
            }));
    }
    pub unsafe fn SetSecond(&mut self, mut new_second: i32) -> i32 {
        return ((unsafe { self.GetSecond() })
            + (unsafe {
                let _field: *mut i32 = &mut self.second as *mut i32;
                let _new_val: i32 = new_second;
                self.Set(_field, _new_val)
            }));
    }
}
#[derive(Copy, Clone, Default)]
pub struct Route {
    pub path: Pair,
    pub cost: f64,
}
impl Route {
    pub unsafe fn SetCost(&mut self, mut new_cost: f64) -> f64 {
        let mut old_cost: f64 = self.cost;
        self.cost = new_cost;
        return old_cost;
    }
}
pub unsafe fn RandomRoute_0(route: *mut Route) -> i32 {
    if ((((*route).path.first) % (2)) != 0) {
        return (unsafe {
            let _new_first: i32 = (unsafe {
                let _new_second: i32 = 10;
                (*route).path.SetSecond(_new_second)
            });
            (*route).path.SetFirst(_new_first)
        });
    } else {
        return (unsafe {
            let _new_second: i32 = (unsafe {
                let _new_first: i32 = -10_i32;
                (*route).path.SetFirst(_new_first)
            });
            (*route).path.SetSecond(_new_second)
        });
    }
    panic!("ub: non-void function does not return a value")
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut route1: Route = Route {
        path: Pair {
            first: 0,
            second: 1,
        },
        cost: 5_f64,
    };
    let mut route2: Route = Route {
        path: Pair {
            first: 1,
            second: 0,
        },
        cost: 10_f64,
    };
    let mut old_cost: f64 = (unsafe {
        let _new_cost: f64 = (unsafe {
            let _new_cost: f64 = 15_f64;
            route2.SetCost(_new_cost)
        });
        route1.SetCost(_new_cost)
    });
    return (((((unsafe {
        let _route: *mut Route = &mut route1 as *mut Route;
        RandomRoute_0(_route)
    }) + (unsafe {
        let _route: *mut Route = &mut route2 as *mut Route;
        RandomRoute_0(_route)
    })) as f64)
        + (old_cost)) as i32);
}
