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
    pub unsafe fn operator_eq(&self, __p0: *const Eq) -> bool {
        return (((*(self as *const Eq)).a) == ((*__p0).a))
            && (((*(self as *const Eq)).b) == ((*__p0).b));
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
    pub unsafe fn operator_cmp(&self, __p0: *const Cmp) -> std::cmp::Ordering {
        {
            let mut cmp: std::cmp::Ordering = ((*(self as *const Cmp)).a).cmp(&((*__p0).a));
            if !(cmp == std::cmp::Ordering::Equal) {
                return cmp;
            }
        }
        {
            let mut cmp: std::cmp::Ordering = ((*(self as *const Cmp)).b).cmp(&((*__p0).b));
            if !(cmp == std::cmp::Ordering::Equal) {
                return cmp;
            }
        }
        return std::cmp::Ordering::Equal;
    }
    pub unsafe fn operator_eq(&self, __p0: *const Cmp) -> bool {
        return (((*(self as *const Cmp)).a) == ((*__p0).a))
            && (((*(self as *const Cmp)).b) == ((*__p0).b));
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
    pub unsafe fn operator_eq(&self, __p0: *const Both) -> bool {
        return (((*(self as *const Both)).a) == ((*__p0).a));
    }
    pub unsafe fn operator_cmp(&self, __p0: *const Both) -> std::cmp::Ordering {
        {
            let mut cmp: std::cmp::Ordering = ((*(self as *const Both)).a).cmp(&((*__p0).a));
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
    return 0;
}
