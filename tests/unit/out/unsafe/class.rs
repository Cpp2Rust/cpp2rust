extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Pair {
    pub first: i32,
    pub second: i32,
}
impl Pair {
    pub unsafe fn NOP(&mut self) {
        let this = self as *mut Pair;
    }
    pub unsafe fn GetFirst(&self) -> i32 {
        let this = self as *const Pair;
        return (*this).first;
    }
    pub unsafe fn GetSecond(&self) -> i32 {
        let this = self as *const Pair;
        return (*this).second;
    }
    pub unsafe fn Set(&mut self, field: *mut i32, mut new_val: i32) -> i32 {
        let this = self as *mut Pair;
        (unsafe { Pair::NOP(self) });
        let mut old_val: i32 = (*field);
        (*field) = new_val;
        return old_val;
    }
    pub unsafe fn SetFirst(&mut self, mut new_first: i32) -> i32 {
        let this = self as *mut Pair;
        return ((unsafe { Pair::GetFirst(self) })
            + (unsafe {
                let _field: *mut i32 = &mut (*this).first as *mut i32;
                Pair::Set(self, _field, new_first)
            }));
    }
    pub unsafe fn SetSecond(&mut self, mut new_second: i32) -> i32 {
        let this = self as *mut Pair;
        return ((unsafe { Pair::GetSecond(self) })
            + (unsafe {
                let _field: *mut i32 = &mut (*this).second as *mut i32;
                Pair::Set(self, _field, new_second)
            }));
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Route {
    pub path: Pair,
    pub cost: f64,
}
impl Route {
    pub unsafe fn SetCost(&mut self, mut new_cost: f64) -> f64 {
        let this = self as *mut Route;
        let mut old_cost: f64 = (*this).cost;
        (*this).cost = new_cost;
        return old_cost;
    }
}
pub unsafe fn RandomRoute_0(route: *mut Route) -> i32 {
    if ((((*route).path.first) % (2)) != 0) {
        return (unsafe {
            let _new_first: i32 = (unsafe { Pair::SetSecond(&mut (*route).path, 10) });
            Pair::SetFirst(&mut (*route).path, _new_first)
        });
    } else {
        return (unsafe {
            let _new_second: i32 = (unsafe { Pair::SetFirst(&mut (*route).path, -10_i32) });
            Pair::SetSecond(&mut (*route).path, _new_second)
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
        Route::SetCost(
            &mut route1,
            (unsafe { Route::SetCost(&mut route2, 15_f64) }),
        )
    });
    assert!(
        (((((unsafe { RandomRoute_0(&mut route1 as *mut Route,) })
            + (unsafe { RandomRoute_0(&mut route2 as *mut Route,) })) as f64)
            + (old_cost))
            == (9_f64))
    );
    return 0;
}
