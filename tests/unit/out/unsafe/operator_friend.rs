extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn operator_eq_0(_a0: *const Defaulted, _a1: *const Defaulted) -> bool {
    return (((*_a0).a) == ((*_a1).a)) && (((*_a0).b) == ((*_a1).b));
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Defaulted {
    pub a: i32,
    pub b: i32,
}
impl std::cmp::PartialEq for Defaulted {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_0(self as *const Defaulted, other as *const Defaulted) }
    }
}
impl std::cmp::Eq for Defaulted {}
pub unsafe fn operator_cmp_1(
    _a0: *const DefaultedOrd,
    _a1: *const DefaultedOrd,
) -> std::cmp::Ordering {
    {
        let mut cmp: std::cmp::Ordering = std::cmp::Ord::cmp(&((*_a0).a), &((*_a1).a));
        if !(cmp == std::cmp::Ordering::Equal) {
            return cmp;
        }
    }
    return std::cmp::Ordering::Equal;
}
pub unsafe fn operator_eq_2(_a0: *const DefaultedOrd, _a1: *const DefaultedOrd) -> bool {
    return (((*_a0).a) == ((*_a1).a));
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct DefaultedOrd {
    pub a: i32,
}
impl std::cmp::Ord for DefaultedOrd {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { operator_cmp_1(self as *const DefaultedOrd, other as *const DefaultedOrd) }
    }
}
impl std::cmp::PartialOrd for DefaultedOrd {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for DefaultedOrd {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_2(self as *const DefaultedOrd, other as *const DefaultedOrd) }
    }
}
impl std::cmp::Eq for DefaultedOrd {}
pub unsafe fn operator_eq_3(x: *const Inline, y: *const Inline) -> bool {
    return (((*x).a) == ((*y).a));
}
pub unsafe fn operator_lt_4(x: *const Inline, y: *const Inline) -> bool {
    return (((*x).a) < ((*y).a));
}
pub unsafe fn operator_add_5(x: *const Inline, y: *const Inline) -> Inline {
    return Inline {
        a: (((*x).a) + ((*y).a)),
    };
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Inline {
    pub a: i32,
}
impl std::cmp::Ord for Inline {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_4(self as *const Inline, other as *const Inline) {
                std::cmp::Ordering::Less
            } else if operator_lt_4(other as *const Inline, self as *const Inline) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Inline {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Inline {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_3(self as *const Inline, other as *const Inline) }
    }
}
impl std::cmp::Eq for Inline {}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct OutOfLine {
    pub a: i32,
}
impl std::cmp::PartialEq for OutOfLine {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_6(self as *const OutOfLine, other as *const OutOfLine) }
    }
}
impl std::cmp::Eq for OutOfLine {}
pub unsafe fn operator_eq_6(x: *const OutOfLine, y: *const OutOfLine) -> bool {
    return (((*x).a) == ((*y).a));
}
pub unsafe fn operator_ne_7(x: *const OutOfLine, y: *const OutOfLine) -> bool {
    return !(unsafe {
        let _x: *const OutOfLine = x;
        let _y: *const OutOfLine = y;
        operator_eq_6(_x, _y)
    });
}
pub unsafe fn operator_eq_8(x: *const Tmpl_int_, y: *const Tmpl_int_) -> bool {
    return (((*x).v) == ((*y).v));
}
pub unsafe fn operator_lt_9(x: *const Tmpl_int_, y: *const Tmpl_int_) -> bool {
    return (((*x).v) < ((*y).v));
}
pub unsafe fn operator_eq_10(x: *const Tmpl_int_, y: *const Tmpl_long_) -> bool {
    return (((*x).v as i64) == ((*y).v));
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Tmpl_int_ {
    pub v: i32,
}
impl std::cmp::Ord for Tmpl_int_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_9(self as *const Tmpl_int_, other as *const Tmpl_int_) {
                std::cmp::Ordering::Less
            } else if operator_lt_9(other as *const Tmpl_int_, self as *const Tmpl_int_) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Tmpl_int_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Tmpl_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_8(self as *const Tmpl_int_, other as *const Tmpl_int_) }
    }
}
impl std::cmp::Eq for Tmpl_int_ {}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Tmpl_long_ {
    pub v: i64,
}
pub unsafe fn operator_eq_11(
    _a0: *const TmplDefaulted_int_,
    _a1: *const TmplDefaulted_int_,
) -> bool {
    return (((*_a0).v) == ((*_a1).v));
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct TmplDefaulted_int_ {
    pub v: i32,
}
impl std::cmp::PartialEq for TmplDefaulted_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_11(
                self as *const TmplDefaulted_int_,
                other as *const TmplDefaulted_int_,
            )
        }
    }
}
impl std::cmp::Eq for TmplDefaulted_int_ {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut d1: Defaulted = Defaulted { a: 1, b: 2 };
    let mut d2: Defaulted = Defaulted { a: 1, b: 2 };
    let mut d3: Defaulted = Defaulted { a: 1, b: 3 };
    assert!(
        (unsafe {
            let _arg0: *const Defaulted = &d1;
            operator_eq_0(_arg0, &d2)
        })
    );
    assert!(
        !(unsafe {
            let _arg0: *const Defaulted = &d1;
            operator_eq_0(_arg0, &d3)
        })
    );
    assert!(
        !(unsafe {
            let _arg0: *const Defaulted = &d1;
            operator_eq_0(_arg0, &d3)
        })
    );
    let mut o1: DefaultedOrd = DefaultedOrd { a: 1 };
    let mut o2: DefaultedOrd = DefaultedOrd { a: 2 };
    assert!(
        (unsafe {
            let _arg0: *const DefaultedOrd = &o1;
            operator_cmp_1(_arg0, &o2)
        }) == std::cmp::Ordering::Less
    );
    assert!(
        (unsafe {
            let _arg0: *const DefaultedOrd = &o2;
            operator_cmp_1(_arg0, &o1)
        }) == std::cmp::Ordering::Greater
    );
    assert!(
        (unsafe {
            let _arg0: *const DefaultedOrd = &o1;
            let _arg1: *const DefaultedOrd = &o1;
            operator_eq_2(_arg0, _arg1)
        })
    );
    assert!(
        (unsafe {
            let _arg0: *const DefaultedOrd = &o1;
            operator_cmp_1(_arg0, &o2)
        }) == std::cmp::Ordering::Less
    );
    let mut i1: Inline = Inline { a: 1 };
    let mut i2: Inline = Inline { a: 2 };
    let mut i3: Inline = Inline { a: 1 };
    assert!(
        (unsafe {
            let _x: *const Inline = &i1;
            operator_eq_3(_x, &i3)
        })
    );
    assert!(
        !(unsafe {
            let _x: *const Inline = &i1;
            operator_eq_3(_x, &i2)
        })
    );
    assert!(
        (unsafe {
            let _x: *const Inline = &i1;
            operator_lt_4(_x, &i2)
        })
    );
    assert!(
        !(unsafe {
            let _x: *const Inline = &i2;
            operator_lt_4(_x, &i1)
        })
    );
    assert!(
        (unsafe {
            let mut _x: Inline = (unsafe {
                let _x: *const Inline = &i1;
                operator_add_5(_x, &i2)
            });
            let mut _y: Inline = Inline { a: 3 };
            operator_eq_3(&mut _x, &mut _y)
        })
    );
    let mut f1: OutOfLine = OutOfLine { a: 4 };
    let mut f2: OutOfLine = OutOfLine { a: 4 };
    let mut f3: OutOfLine = OutOfLine { a: 5 };
    assert!(
        (unsafe {
            let _x: *const OutOfLine = &f1;
            operator_eq_6(_x, &f2)
        })
    );
    assert!(
        (unsafe {
            let _x: *const OutOfLine = &f1;
            operator_ne_7(_x, &f3)
        })
    );
    assert!(
        !(unsafe {
            let _x: *const OutOfLine = &f1;
            operator_eq_6(_x, &f3)
        })
    );
    let mut t1: Tmpl_int_ = Tmpl_int_ { v: 1 };
    let mut t2: Tmpl_int_ = Tmpl_int_ { v: 2 };
    let mut t3: Tmpl_int_ = Tmpl_int_ { v: 1 };
    assert!(
        (unsafe {
            let _x: *const Tmpl_int_ = &t1;
            operator_eq_8(_x, &t3)
        })
    );
    assert!(
        !(unsafe {
            let _x: *const Tmpl_int_ = &t1;
            operator_eq_8(_x, &t2)
        })
    );
    assert!(
        (unsafe {
            let _x: *const Tmpl_int_ = &t1;
            operator_lt_9(_x, &t2)
        })
    );
    let mut u1: Tmpl_long_ = Tmpl_long_ { v: 1_i64 };
    let mut u2: Tmpl_long_ = Tmpl_long_ { v: 2_i64 };
    assert!(
        (unsafe {
            let _x: *const Tmpl_int_ = &t1;
            operator_eq_10(_x, &u1)
        })
    );
    assert!(
        !(unsafe {
            let _x: *const Tmpl_int_ = &t1;
            operator_eq_10(_x, &u2)
        })
    );
    let mut v1: TmplDefaulted_int_ = TmplDefaulted_int_ { v: 7 };
    let mut v2: TmplDefaulted_int_ = TmplDefaulted_int_ { v: 7 };
    let mut v3: TmplDefaulted_int_ = TmplDefaulted_int_ { v: 8 };
    assert!(
        (unsafe {
            let _arg0: *const TmplDefaulted_int_ = &v1;
            operator_eq_11(_arg0, &v2)
        })
    );
    assert!(
        !(unsafe {
            let _arg0: *const TmplDefaulted_int_ = &v1;
            operator_eq_11(_arg0, &v3)
        })
    );
    assert!(
        !(unsafe {
            let _arg0: *const TmplDefaulted_int_ = &v1;
            operator_eq_11(_arg0, &v3)
        })
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
