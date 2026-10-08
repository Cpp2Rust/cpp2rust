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
pub struct X {
    pub v: i32,
}
impl std::cmp::Ord for X {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if ((operator_lt_0(self.clone(), other.clone())) != 0) {
                std::cmp::Ordering::Less
            } else if ((operator_lt_0(other.clone(), self.clone())) != 0) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for X {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for X {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ((operator_eq_1(self.clone(), other.clone())) != 0) }
    }
}
impl std::cmp::Eq for X {}
pub unsafe fn operator_eq_1(mut a: X, mut b: X) -> i32 {
    return if ((a.v) == (b.v)) { 2 } else { 0 };
}
pub unsafe fn operator_ne_2(mut a: X, mut b: X) -> i32 {
    return if ((a.v) != (b.v)) { 3 } else { 0 };
}
pub unsafe fn operator_lt_0(mut a: X, mut b: X) -> i32 {
    return if ((a.v) < (b.v)) { 4 } else { 0 };
}
pub unsafe fn operator_gt_3(mut a: X, mut b: X) -> i32 {
    return if ((a.v) > (b.v)) { 5 } else { 0 };
}
pub unsafe fn operator_le_4(mut a: X, mut b: X) -> i32 {
    return if ((a.v) <= (b.v)) { 6 } else { 0 };
}
pub unsafe fn operator_ge_5(mut a: X, mut b: X) -> i32 {
    return if ((a.v) >= (b.v)) { 7 } else { 0 };
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Result {
    pub r: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Custom {
    pub v: i32,
}
impl Custom {
    pub unsafe fn operator_cmp(&self, o: *const Custom) -> Result {
        return Result {
            r: ((self.v) - ((*o).v)),
        };
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Mixed {
    pub v: i32,
}
impl Mixed {
    pub unsafe fn operator_cmp(&self, o: *const Mixed) -> Result {
        return Result {
            r: ((self.v) - ((*o).v)),
        };
    }
    pub unsafe fn operator_lt(&self, o: *const Mixed) -> i32 {
        return if ((self.v) < ((*o).v)) { 8 } else { 0 };
    }
    pub unsafe fn operator_eq(&self, o: *const Mixed) -> i32 {
        return if ((self.v) == ((*o).v)) { 9 } else { 0 };
    }
}
impl std::cmp::Ord for Mixed {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if ((Mixed::operator_lt(self, other as *const Mixed)) != 0) {
                std::cmp::Ordering::Less
            } else if ((Mixed::operator_lt(other, self as *const Mixed)) != 0) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Mixed {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Mixed {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ((Mixed::operator_eq(self, other as *const Mixed)) != 0) }
    }
}
impl std::cmp::Eq for Mixed {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct MyBool {
    pub value: bool,
}
impl MyBool {
    pub unsafe fn new(mut v: bool) -> Self {
        let mut this = Self { value: v };
        this
    }
    pub unsafe fn to_bool(&self) -> bool {
        return self.value;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Boolish {
    pub v: i32,
}
impl std::cmp::Ord for Boolish {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if {
                let __cmp = &operator_lt_6(self.clone(), other.clone());
                MyBool::to_bool(__cmp)
            } {
                std::cmp::Ordering::Less
            } else if {
                let __cmp = &operator_lt_6(other.clone(), self.clone());
                MyBool::to_bool(__cmp)
            } {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Boolish {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Boolish {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            {
                let __cmp = &operator_eq_7(self.clone(), other.clone());
                MyBool::to_bool(__cmp)
            }
        }
    }
}
impl std::cmp::Eq for Boolish {}
pub unsafe fn operator_eq_7(mut a: Boolish, mut b: Boolish) -> MyBool {
    return MyBool::new({ ((a.v) == (b.v)) });
}
pub unsafe fn operator_lt_6(mut a: Boolish, mut b: Boolish) -> MyBool {
    return MyBool::new({ ((a.v) < (b.v)) });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: X = X { v: 1 };
    let mut b: X = X { v: 2 };
    let mut c: X = X { v: 1 };
    assert!(
        ((unsafe {
            let _a: X = a;
            operator_eq_1(_a, c)
        }) == (2))
    );
    assert!(
        ((unsafe {
            let _a: X = a;
            operator_eq_1(_a, b)
        }) == (0))
    );
    assert!(
        ((unsafe {
            let _a: X = a;
            operator_ne_2(_a, b)
        }) == (3))
    );
    assert!(
        ((unsafe {
            let _a: X = a;
            operator_lt_0(_a, b)
        }) == (4))
    );
    assert!(
        ((unsafe {
            let _a: X = b;
            operator_gt_3(_a, a)
        }) == (5))
    );
    assert!(
        ((unsafe {
            let _a: X = a;
            operator_le_4(_a, c)
        }) == (6))
    );
    assert!(
        ((unsafe {
            let _a: X = a;
            operator_ge_5(_a, c)
        }) == (7))
    );
    let mut xs: [X; 3] = [X { v: 3 }, X { v: 1 }, X { v: 2 }];
    {
        let len = xs
            .as_mut_ptr()
            .offset((3) as isize)
            .offset_from(xs.as_mut_ptr()) as usize;
        ::std::slice::from_raw_parts_mut(xs.as_mut_ptr(), len).sort()
    };
    assert!(
        (((xs[(0) as usize].v) == (1)) && ((xs[(1) as usize].v) == (2)))
            && ((xs[(2) as usize].v) == (3))
    );
    let mut p: Custom = Custom { v: 5 };
    let mut q: Custom = Custom { v: 2 };
    let mut r: Result = (unsafe { Custom::operator_cmp(&p, &q) });
    assert!(((r.r) == (3)));
    let mut m: Mixed = Mixed { v: 4 };
    let mut n: Mixed = Mixed { v: 4 };
    assert!(((unsafe { Mixed::operator_eq(&m, &n,) }) == (9)));
    assert!((((unsafe { Mixed::operator_cmp(&m, &n,) }).r) == (0)));
    let mut ms: [Mixed; 3] = [Mixed { v: 6 }, Mixed { v: 4 }, Mixed { v: 5 }];
    {
        let len = ms
            .as_mut_ptr()
            .offset((3) as isize)
            .offset_from(ms.as_mut_ptr()) as usize;
        ::std::slice::from_raw_parts_mut(ms.as_mut_ptr(), len).sort()
    };
    assert!(
        (((ms[(0) as usize].v) == (4)) && ((ms[(1) as usize].v) == (5)))
            && ((ms[(2) as usize].v) == (6))
    );
    assert!(
        ((unsafe {
            let _o: *const Mixed = &ms[(1) as usize];
            Mixed::operator_lt(&ms[(0) as usize], _o)
        }) == (8))
    );
    let mut b1: Boolish = Boolish { v: 1 };
    let mut b2: Boolish = Boolish { v: 2 };
    assert!(
        (unsafe {
            MyBool::to_bool(
                &(unsafe {
                    let _a: Boolish = b1;
                    operator_eq_7(_a, Boolish { v: 1 })
                }),
            )
        })
    );
    assert!(
        !(unsafe {
            MyBool::to_bool(
                &(unsafe {
                    let _a: Boolish = b2;
                    operator_lt_6(_a, b1)
                }),
            )
        })
    );
    let mut lt: MyBool = (unsafe {
        let _a: Boolish = b1;
        operator_lt_6(_a, b2)
    });
    assert!(lt.value);
    let mut bs: [Boolish; 3] = [Boolish { v: 3 }, Boolish { v: 1 }, Boolish { v: 2 }];
    {
        let len = bs
            .as_mut_ptr()
            .offset((3) as isize)
            .offset_from(bs.as_mut_ptr()) as usize;
        ::std::slice::from_raw_parts_mut(bs.as_mut_ptr(), len).sort()
    };
    assert!(
        (((bs[(0) as usize].v) == (1)) && ((bs[(1) as usize].v) == (2)))
            && ((bs[(2) as usize].v) == (3))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
