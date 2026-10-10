extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
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
        (unsafe { Pair::NOP(self) });
        let mut old_val: i32 = (*field);
        (*field) = new_val;
        return old_val;
    }
    pub unsafe fn SetFirst(&mut self, mut new_first: i32) -> i32 {
        return ((unsafe { Pair::GetFirst(self) })
            + (unsafe {
                let _field: *mut i32 = &mut self.first;
                Pair::Set(self, _field, new_first)
            }));
    }
    pub unsafe fn SetSecond(&mut self, mut new_second: i32) -> i32 {
        return ((unsafe { Pair::GetSecond(self) })
            + (unsafe {
                let _field: *mut i32 = &mut self.second;
                Pair::Set(self, _field, new_second)
            }));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Counter {
    pub v: i32,
    pub calls: i32,
}
impl Counter {
    pub unsafe fn Get(&self) -> i32 {
        (*(&raw const self.calls).cast_mut()).prefix_inc();
        return self.v;
    }
    pub unsafe fn operator_eq(&self, o: *const Counter) -> bool {
        (*(&raw const self.calls).cast_mut()).prefix_inc();
        return ((self.v) == ((*o).v));
    }
}
impl std::cmp::PartialEq for Counter {
    fn eq(&self, other: &Self) -> bool {
        unsafe { Counter::operator_eq(self, other as *const Counter) }
    }
}
impl std::cmp::Eq for Counter {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Tally {
    pub calls: i32,
}
impl Tally {
    pub unsafe fn SameNameBetweenConstAndNonConst_1(&mut self) -> i32 {
        self.calls += 1;
        return 1;
    }
    pub unsafe fn SameNameBetweenConstAndNonConst_2(&self) -> i32 {
        (*(&raw const self.calls).cast_mut()) += 10;
        return 2;
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
        __cpp2rust_init_globals();
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
        (((((unsafe { RandomRoute_0(&mut route1,) }) + (unsafe { RandomRoute_0(&mut route2,) }))
            as f64)
            + (old_cost))
            == (9_f64))
    );
    let mut c1: Counter = Counter { v: 3, calls: 0 };
    let c2: Counter = Counter { v: 3, calls: 0 };
    let mut pc: *const Counter = (&mut c1 as *mut Counter).cast_const();
    assert!(((unsafe { Counter::Get(&c1,) }) == (3)));
    assert!(((unsafe { Counter::Get(&c2,) }) == (3)));
    assert!(((unsafe { Counter::Get(&(*pc),) }) == (3)));
    assert!((unsafe { Counter::operator_eq(&c1, &c2,) }));
    assert!((unsafe { Counter::operator_eq(&c2, &c1,) }));
    assert!(((c1.calls) == (3)));
    assert!(((c2.calls) == (2)));
    let mut t: Tally = Tally { calls: 0 };
    let ct: Tally = Tally { calls: 0 };
    let rt: *const Tally = &t;
    assert!(((unsafe { Tally::SameNameBetweenConstAndNonConst_1(&mut t,) }) == (1)));
    assert!(((unsafe { Tally::SameNameBetweenConstAndNonConst_2(&ct,) }) == (2)));
    assert!(((unsafe { Tally::SameNameBetweenConstAndNonConst_2(&(*rt),) }) == (2)));
    assert!(((t.calls) == (11)));
    assert!(((ct.calls) == (10)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
