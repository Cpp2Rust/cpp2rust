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
pub struct Eq {
    pub a: i32,
    pub b: i32,
}
impl Eq {
    pub unsafe fn operator_eq(&self, _arg0: *const Eq) -> bool {
        return (((*(self as *const Eq)).a) == ((*_arg0).a))
            && (((*(self as *const Eq)).b) == ((*_arg0).b));
    }
}
impl std::cmp::PartialEq for Eq {
    fn eq(&self, other: &Self) -> bool {
        unsafe { Eq::operator_eq(self, other as *const Eq) }
    }
}
impl std::cmp::Eq for Eq {}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Cmp {
    pub a: i32,
    pub b: i32,
}
impl Cmp {
    pub unsafe fn operator_cmp(&self, _arg0: *const Cmp) -> std::cmp::Ordering {
        {
            let mut cmp: std::cmp::Ordering = ((*(self as *const Cmp)).a).cmp(&((*_arg0).a));
            if !(cmp == std::cmp::Ordering::Equal) {
                return cmp;
            }
        }
        {
            let mut cmp: std::cmp::Ordering = ((*(self as *const Cmp)).b).cmp(&((*_arg0).b));
            if !(cmp == std::cmp::Ordering::Equal) {
                return cmp;
            }
        }
        return std::cmp::Ordering::Equal;
    }
    pub unsafe fn operator_eq(&self, _arg0: *const Cmp) -> bool {
        return (((*(self as *const Cmp)).a) == ((*_arg0).a))
            && (((*(self as *const Cmp)).b) == ((*_arg0).b));
    }
}
impl std::cmp::Ord for Cmp {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { Cmp::operator_cmp(self, other as *const Cmp) }
    }
}
impl std::cmp::PartialOrd for Cmp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Cmp {
    fn eq(&self, other: &Self) -> bool {
        unsafe { Cmp::operator_cmp(self, other as *const Cmp) == std::cmp::Ordering::Equal }
    }
}
impl std::cmp::Eq for Cmp {}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Both {
    pub a: i32,
}
impl Both {
    pub unsafe fn operator_eq(&self, _arg0: *const Both) -> bool {
        return (((*(self as *const Both)).a) == ((*_arg0).a));
    }
    pub unsafe fn operator_cmp(&self, _arg0: *const Both) -> std::cmp::Ordering {
        {
            let mut cmp: std::cmp::Ordering = ((*(self as *const Both)).a).cmp(&((*_arg0).a));
            if !(cmp == std::cmp::Ordering::Equal) {
                return cmp;
            }
        }
        return std::cmp::Ordering::Equal;
    }
}
impl std::cmp::Ord for Both {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { Both::operator_cmp(self, other as *const Both) }
    }
}
impl std::cmp::PartialOrd for Both {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Both {
    fn eq(&self, other: &Self) -> bool {
        unsafe { Both::operator_eq(self, other as *const Both) }
    }
}
impl std::cmp::Eq for Both {}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct OrdOnly {
    pub a: i32,
}
impl OrdOnly {
    pub unsafe fn operator_cmp(&self, _arg0: *const OrdOnly) -> std::cmp::Ordering {
        {
            let mut cmp: std::cmp::Ordering = ((*(self as *const OrdOnly)).a).cmp(&((*_arg0).a));
            if !(cmp == std::cmp::Ordering::Equal) {
                return cmp;
            }
        }
        return std::cmp::Ordering::Equal;
    }
}
impl std::cmp::Ord for OrdOnly {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { OrdOnly::operator_cmp(self, other as *const OrdOnly) }
    }
}
impl std::cmp::PartialOrd for OrdOnly {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for OrdOnly {
    fn eq(&self, other: &Self) -> bool {
        unsafe { OrdOnly::operator_cmp(self, other as *const OrdOnly) == std::cmp::Ordering::Equal }
    }
}
impl std::cmp::Eq for OrdOnly {}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Inner {
    pub x: i32,
}
impl Inner {
    pub unsafe fn operator_cmp(&self, _arg0: *const Inner) -> std::cmp::Ordering {
        {
            let mut cmp: std::cmp::Ordering = ((*(self as *const Inner)).x).cmp(&((*_arg0).x));
            if !(cmp == std::cmp::Ordering::Equal) {
                return cmp;
            }
        }
        return std::cmp::Ordering::Equal;
    }
    pub unsafe fn operator_eq(&self, _arg0: *const Inner) -> bool {
        return (((*(self as *const Inner)).x) == ((*_arg0).x));
    }
}
impl std::cmp::Ord for Inner {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { Inner::operator_cmp(self, other as *const Inner) }
    }
}
impl std::cmp::PartialOrd for Inner {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Inner {
    fn eq(&self, other: &Self) -> bool {
        unsafe { Inner::operator_cmp(self, other as *const Inner) == std::cmp::Ordering::Equal }
    }
}
impl std::cmp::Eq for Inner {}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Outer {
    pub i: Inner,
    pub y: i32,
}
impl Outer {
    pub unsafe fn operator_cmp(&self, _arg0: *const Outer) -> std::cmp::Ordering {
        {
            let mut cmp: std::cmp::Ordering = (unsafe {
                let _arg0: *const Inner = &(*_arg0).i as *const Inner;
                Inner::operator_cmp(&(*(self as *const Outer)).i, _arg0)
            });
            if !(cmp == std::cmp::Ordering::Equal) {
                return cmp;
            }
        }
        {
            let mut cmp: std::cmp::Ordering = ((*(self as *const Outer)).y).cmp(&((*_arg0).y));
            if !(cmp == std::cmp::Ordering::Equal) {
                return cmp;
            }
        }
        return std::cmp::Ordering::Equal;
    }
    pub unsafe fn operator_eq(&self, _arg0: *const Outer) -> bool {
        return (unsafe {
            let _arg0: *const Inner = &(*_arg0).i as *const Inner;
            Inner::operator_eq(&(*(self as *const Outer)).i, _arg0)
        }) && (((*(self as *const Outer)).y) == ((*_arg0).y));
    }
}
impl std::cmp::Ord for Outer {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { Outer::operator_cmp(self, other as *const Outer) }
    }
}
impl std::cmp::PartialOrd for Outer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Outer {
    fn eq(&self, other: &Self) -> bool {
        unsafe { Outer::operator_cmp(self, other as *const Outer) == std::cmp::Ordering::Equal }
    }
}
impl std::cmp::Eq for Outer {}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Secondary {
    pub a: i32,
}
impl Secondary {
    pub unsafe fn operator_eq(&self, _arg0: *const Secondary) -> bool {
        return (((*(self as *const Secondary)).a) == ((*_arg0).a));
    }
    pub unsafe fn operator_ne(&self, _arg0: *const Secondary) -> bool {
        return !(unsafe {
            let _arg0: *const Secondary = _arg0;
            Secondary::operator_eq(&(*(self as *const Secondary)), _arg0)
        });
    }
    pub unsafe fn operator_cmp(&self, _arg0: *const Secondary) -> std::cmp::Ordering {
        {
            let mut cmp: std::cmp::Ordering = ((*(self as *const Secondary)).a).cmp(&((*_arg0).a));
            if !(cmp == std::cmp::Ordering::Equal) {
                return cmp;
            }
        }
        return std::cmp::Ordering::Equal;
    }
    pub unsafe fn operator_lt(&self, _arg0: *const Secondary) -> bool {
        return (unsafe {
            let _arg0: *const Secondary = _arg0;
            Secondary::operator_cmp(&(*(self as *const Secondary)), _arg0)
        }) == std::cmp::Ordering::Less;
    }
    pub unsafe fn operator_ge(&self, _arg0: *const Secondary) -> bool {
        return (unsafe {
            let _arg0: *const Secondary = _arg0;
            Secondary::operator_cmp(&(*(self as *const Secondary)), _arg0)
        }) != std::cmp::Ordering::Less;
    }
}
impl std::cmp::Ord for Secondary {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { Secondary::operator_cmp(self, other as *const Secondary) }
    }
}
impl std::cmp::PartialOrd for Secondary {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Secondary {
    fn eq(&self, other: &Self) -> bool {
        unsafe { Secondary::operator_eq(self, other as *const Secondary) }
    }
}
impl std::cmp::Eq for Secondary {}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut e1: Eq = Eq { a: 1, b: 2 };
    let mut e2: Eq = Eq { a: 1, b: 2 };
    let mut e3: Eq = Eq { a: 1, b: 3 };
    assert!((unsafe { Eq::operator_eq(&e1, &e2 as *const Eq,) }));
    assert!(!(unsafe { Eq::operator_eq(&e1, &e3 as *const Eq,) }));
    let mut c1: Cmp = Cmp { a: 1, b: 2 };
    let mut c2: Cmp = Cmp { a: 1, b: 3 };
    let mut c3: Cmp = Cmp { a: 2, b: 0 };
    let mut c4: Cmp = Cmp { a: 1, b: 9 };
    assert!((unsafe { Cmp::operator_cmp(&c1, &c2 as *const Cmp,) }) == std::cmp::Ordering::Less);
    assert!((unsafe { Cmp::operator_cmp(&c3, &c4 as *const Cmp,) }) == std::cmp::Ordering::Greater);
    assert!(
        (unsafe {
            let _arg0: *const Cmp = &c1 as *const Cmp;
            Cmp::operator_eq(&c1, _arg0)
        })
    );
    assert!((unsafe { Cmp::operator_cmp(&c1, &c2 as *const Cmp,) }) == std::cmp::Ordering::Less);
    let mut b1: Both = Both { a: 1 };
    let mut b2: Both = Both { a: 2 };
    assert!((unsafe { Both::operator_cmp(&b1, &b2 as *const Both,) }) == std::cmp::Ordering::Less);
    assert!(
        (unsafe {
            let _arg0: *const Both = &b2 as *const Both;
            Both::operator_eq(&b2, _arg0)
        })
    );
    let mut o1: OrdOnly = OrdOnly { a: 1 };
    let mut o2: OrdOnly = OrdOnly { a: 2 };
    assert!(
        (unsafe { OrdOnly::operator_cmp(&o1, &o2 as *const OrdOnly,) }) == std::cmp::Ordering::Less
    );
    assert!(
        (unsafe { OrdOnly::operator_cmp(&o2, &o1 as *const OrdOnly,) })
            == std::cmp::Ordering::Greater
    );
    let mut x1: Outer = Outer {
        i: Inner { x: 1 },
        y: 9,
    };
    let mut x2: Outer = Outer {
        i: Inner { x: 2 },
        y: 0,
    };
    let mut x3: Outer = Outer {
        i: Inner { x: 1 },
        y: 9,
    };
    assert!(
        (unsafe { Outer::operator_cmp(&x1, &x2 as *const Outer,) }) == std::cmp::Ordering::Less
    );
    assert!((unsafe { Outer::operator_eq(&x1, &x3 as *const Outer,) }));
    assert!(
        (unsafe { Outer::operator_cmp(&x2, &x1 as *const Outer,) }) == std::cmp::Ordering::Greater
    );
    let mut s1: Secondary = Secondary { a: 1 };
    let mut s2: Secondary = Secondary { a: 2 };
    assert!((unsafe { Secondary::operator_ne(&s1, &s2 as *const Secondary,) }));
    assert!((unsafe { Secondary::operator_lt(&s1, &s2 as *const Secondary,) }));
    assert!((unsafe { Secondary::operator_ge(&s2, &s1 as *const Secondary,) }));
    assert!(!(unsafe { Secondary::operator_lt(&s2, &s1 as *const Secondary,) }));
    return 0;
}
