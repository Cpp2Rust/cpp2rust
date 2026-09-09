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
pub struct S {
    pub a_: i32,
    pub self__: *mut S,
}
impl S {
    pub unsafe fn S1(mut a: i32) -> Self {
        let mut __this = Self {
            a_: a,
            self__: std::ptr::null_mut(),
        };
        let this = &raw mut __this;
        __this
    }
    pub unsafe fn S2(mut a: i32, mut other: *mut S) -> Self {
        let mut __this = Self {
            a_: a,
            self__: std::ptr::null_mut(),
        };
        let this = &raw mut __this;
        if ((this) == (other)) {
            (*this).self__ = std::ptr::null_mut();
        } else {
            (*this).self__ = other;
        }
        __this
    }
    pub unsafe fn returns_this_reference(&mut self) -> *mut S {
        let this = self as *mut S;
        return &mut (*this) as *mut S;
    }
    pub unsafe fn returns_this_pointer(&mut self) -> *mut S {
        let this = self as *mut S;
        return this;
    }
    pub unsafe fn inc(&mut self) -> *mut S {
        let this = self as *mut S;
        (*this).a_.postfix_inc();
        return &mut (*this) as *mut S;
    }
    pub unsafe fn set_from_this(&mut self) {
        let this = self as *mut S;
        (*this).a_ = (((*this).a_) + (1));
    }
    pub unsafe fn get(&mut self) -> i32 {
        let this = self as *mut S;
        return (*this).a_;
    }
    pub unsafe fn twice(&mut self) -> i32 {
        let this = self as *mut S;
        return ((unsafe { S::get(self) }) * (2));
    }
    pub unsafe fn link(&mut self) {
        let this = self as *mut S;
        (*this).self__ = this;
    }
    pub unsafe fn bump_me(&mut self) {
        let this = self as *mut S;
        (unsafe { bump_0(this) });
    }
    pub unsafe fn cref(&self) -> *const S {
        let this = self as *const S;
        return &(*this) as *const S;
    }
    pub unsafe fn is(&self, mut o: *const S) -> bool {
        let this = self as *const S;
        return ((o) == (this));
    }
    pub unsafe fn destroy(&mut self) {
        let this = self as *mut S;
        ::std::mem::drop(Box::from_raw(this));
    }
    pub unsafe fn reset(&mut self) {
        let this = self as *mut S;
        (*this) = S::S1({ 0 });
    }
    pub unsafe fn copy_if_different_const(&mut self, mut other: *const S) -> bool {
        let this = self as *mut S;
        if (((this).cast_const()) == (other)) {
            return false;
        }
        (*this).a_ = (*other).a_;
        (*this).self__ = (*other).self__;
        return true;
    }
    pub unsafe fn copy_if_different(&mut self, mut other: *mut S) -> bool {
        let this = self as *mut S;
        if ((this) == (other)) {
            return false;
        }
        (*this).a_ = (*other).a_;
        (*this).self__ = (*other).self__;
        return true;
    }
}
pub unsafe fn bump_0(mut p: *mut S) {
    (*p).a_.postfix_inc();
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct D {
    pub a_: i32,
}
impl D {
    pub unsafe fn D(mut a: i32) -> Self {
        let mut __this = Self { a_: a };
        let this = &raw mut __this;
        (*this).a_ *= 2;
        __this
    }
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: S = S::S1({ 1 });
    let ref_: *mut S = (unsafe { S::returns_this_reference(&mut s) });
    (*ref_).a_.postfix_inc();
    assert!(((s.a_) == (2)));
    let mut ptr: *mut S = (unsafe { S::returns_this_pointer(&mut s) });
    (*ptr).a_.postfix_inc();
    assert!(((s.a_) == (3)));
    (unsafe { S::inc(&mut (*(unsafe { S::inc(&mut (*(unsafe { S::inc(&mut s) }))) }))) });
    assert!(((s.a_) == (6)));
    (unsafe { S::set_from_this(&mut s) });
    assert!(((s.a_) == (7)));
    assert!(((unsafe { S::twice(&mut s,) }) == (14)));
    (unsafe { S::link(&mut s) });
    assert!(((s.self__) == (&mut s as *mut S)));
    (*s.self__).a_.postfix_inc();
    assert!(((s.a_) == (8)));
    (unsafe { S::bump_me(&mut s) });
    assert!(((s.a_) == (9)));
    let mut d: D = D::D({ 3 });
    assert!(((d.a_) == (6)));
    let cr: *const S = (unsafe { S::cref(&s) });
    assert!((((*cr).a_) == (9)));
    let mut t: S = S::S1({ 0 });
    assert!(
        (unsafe {
            let _o: *const S = (&mut s as *mut S).cast_const();
            S::is(&s, _o)
        })
    );
    assert!(!(unsafe { S::is(&s, (&mut t as *mut S).cast_const(),) }));
    let mut p: *mut S = (Box::leak(Box::new(S::S1({ 1 }))) as *mut S);
    let mut q: *mut S = (unsafe { S::returns_this_pointer(&mut (*p)) });
    (*q).a_.postfix_inc();
    assert!((((*p).a_) == (2)));
    ::std::mem::drop(Box::from_raw(p));
    let mut h: *mut S = (Box::leak(Box::new(S::S1({ 5 }))) as *mut S);
    (unsafe { S::destroy(&mut (*h)) });
    (unsafe { S::reset(&mut s) });
    assert!(((s.a_) == (0)));
    assert!((s.self__).is_null());
    assert!(
        (((unsafe {
            let _other: *mut S = (&mut s as *mut S);
            S::copy_if_different(&mut s, _other)
        }) as i32)
            == (false as i32))
    );
    assert!(
        (((unsafe {
            let _other: *const S = (&mut s as *mut S).cast_const();
            S::copy_if_different_const(&mut s, _other)
        }) as i32)
            == (false as i32))
    );
    let mut other: S = S::S1({ 22 });
    assert!(
        (((unsafe { S::copy_if_different(&mut s, (&mut other as *mut S),) }) as i32)
            == (true as i32))
    );
    assert!(((s.a_) == (other.a_)));
    assert!(((s.self__) == (other.self__)));
    let mut u: S = S::S2({ 1 }, { (&mut s as *mut S) });
    assert!(((u.self__) == (&mut s as *mut S)));
    return 0;
}
