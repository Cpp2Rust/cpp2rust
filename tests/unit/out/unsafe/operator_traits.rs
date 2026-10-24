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
pub struct Lt {
    pub v: i32,
}
impl Lt {
    pub unsafe fn operator_lt(&self, o: *const Lt) -> bool {
        return ((self.v) < ((*o).v));
    }
}
impl std::cmp::Ord for Lt {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if Lt::operator_lt(self, other as *const Lt) {
                std::cmp::Ordering::Less
            } else if Lt::operator_lt(other, self as *const Lt) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Lt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Lt {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            !(Lt::operator_lt(self, other as *const Lt))
                && !(Lt::operator_lt(other, self as *const Lt))
        }
    }
}
impl std::cmp::Eq for Lt {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Eq {
    pub v: i32,
}
impl Eq {
    pub unsafe fn operator_eq(&self, o: *const Eq) -> bool {
        return ((self.v) == ((*o).v));
    }
}
impl std::cmp::PartialEq for Eq {
    fn eq(&self, other: &Self) -> bool {
        unsafe { Eq::operator_eq(self, other as *const Eq) }
    }
}
impl std::cmp::Eq for Eq {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Cmp {
    pub v: i32,
}
impl Cmp {
    pub unsafe fn operator_cmp(&self, o: *const Cmp) -> std::cmp::Ordering {
        return std::cmp::Ord::cmp(&(self.v), &((*o).v));
    }
    pub unsafe fn operator_eq(&self, o: *const Cmp) -> bool {
        return ((self.v) == ((*o).v));
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
        unsafe { Cmp::operator_eq(self, other as *const Cmp) }
    }
}
impl std::cmp::Eq for Cmp {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Free {
    pub v: i32,
}
impl std::cmp::Ord for Free {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_0(self as *const Free, other as *const Free) {
                std::cmp::Ordering::Less
            } else if operator_lt_0(other as *const Free, self as *const Free) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Free {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Free {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_1(self as *const Free, other as *const Free) }
    }
}
impl std::cmp::Eq for Free {}
pub unsafe fn operator_lt_0(a: *const Free, b: *const Free) -> bool {
    return (((*a).v) < ((*b).v));
}
pub unsafe fn operator_eq_1(a: *const Free, b: *const Free) -> bool {
    return (((*a).v) == ((*b).v));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Wrapped_int_ {
    pub v: i32,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut w: Wrapped_int_ = Wrapped_int_ { v: 2 };
    assert!(((w.v) == (2)));
    let mut lts: Vec<Lt> = Vec::from([Lt { v: 3 }, Lt { v: 1 }, Lt { v: 2 }]);
    {
        let __first = lts.begin();
        let __n = lts.end().iter_distance(&__first);
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| (*__first.iter_at(__x)).cmp(&*__first.iter_at(__y)));
        let __values: Vec<_> = __order
            .iter()
            .map(|&__i| std::ptr::read(__first.iter_at(__i)))
            .collect();
        for (__i, __v) in (0..__n).zip(__values) {
            std::ptr::write(__first.iter_at(__i), __v);
        }
    };
    assert!(
        (((lts[(0_usize)].v) == (1)) && ((lts[(1_usize)].v) == (2))) && ((lts[(2_usize)].v) == (3))
    );
    let mut eqs: Vec<Eq> = Vec::from([Eq { v: 1 }, Eq { v: 2 }, Eq { v: 3 }]);
    let mut two: Eq = Eq { v: 2 };
    let mut nine: Eq = Eq { v: 9 };
    assert!(
        (((*{
            let mut __it = eqs.begin();
            let __last = eqs.end();
            let __value = &two;
            while __it != __last && *__it.iter_deref() != *__value {
                __it.iter_inc();
            }
            __it
        }
        .iter_deref())
        .v) == (2))
    );
    assert!(
        {
            let mut __it = eqs.begin();
            let __last = eqs.end();
            let __value = &nine;
            while __it != __last && *__it.iter_deref() != *__value {
                __it.iter_inc();
            }
            __it
        } == eqs.end()
    );
    let mut cmps: Vec<Cmp> = Vec::from([Cmp { v: 3 }, Cmp { v: 1 }, Cmp { v: 2 }]);
    {
        let __first = cmps.begin();
        let __n = cmps.end().iter_distance(&__first);
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| (*__first.iter_at(__x)).cmp(&*__first.iter_at(__y)));
        let __values: Vec<_> = __order
            .iter()
            .map(|&__i| std::ptr::read(__first.iter_at(__i)))
            .collect();
        for (__i, __v) in (0..__n).zip(__values) {
            std::ptr::write(__first.iter_at(__i), __v);
        }
    };
    assert!(((cmps[(0_usize)].v) == (1)) && ((cmps[(2_usize)].v) == (3)));
    let mut three: Cmp = Cmp { v: 3 };
    assert!(
        (((*{
            let mut __it = cmps.begin();
            let __last = cmps.end();
            let __value = &three;
            while __it != __last && *__it.iter_deref() != *__value {
                __it.iter_inc();
            }
            __it
        }
        .iter_deref())
        .v) == (3))
    );
    let mut frees: Vec<Free> = Vec::from([Free { v: 2 }, Free { v: 1 }]);
    {
        let __first = frees.begin();
        let __n = frees.end().iter_distance(&__first);
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| (*__first.iter_at(__x)).cmp(&*__first.iter_at(__y)));
        let __values: Vec<_> = __order
            .iter()
            .map(|&__i| std::ptr::read(__first.iter_at(__i)))
            .collect();
        for (__i, __v) in (0..__n).zip(__values) {
            std::ptr::write(__first.iter_at(__i), __v);
        }
    };
    assert!(((frees[(0_usize)].v) == (1)));
    let mut ftwo: Free = Free { v: 2 };
    assert!(
        (((*{
            let mut __it = frees.begin();
            let __last = frees.end();
            let __value = &ftwo;
            while __it != __last && *__it.iter_deref() != *__value {
                __it.iter_inc();
            }
            __it
        }
        .iter_deref())
        .v) == (2))
    );
    let mut m: UnsafeMap<Lt, (Lt, i32)> = UnsafeMap::new();
    (*m.subscript_move(&mut Lt { v: 2 }, |__k| cpp_move_unsafe!(__k))) = 20;
    (*m.subscript_move(&mut Lt { v: 1 }, |__k| cpp_move_unsafe!(__k))) = 10;
    assert!((((*m.begin().iter_deref()).1) == (10)));
    assert!(((*m.subscript_move(&mut Lt { v: 2 }, |__k| cpp_move_unsafe!(__k))) == (20)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
