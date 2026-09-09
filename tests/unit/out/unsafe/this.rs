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
        let mut this = Self {
            a_: a,
            self__: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn S2(mut a: i32, mut other: *mut S) -> Self {
        let mut this = Self {
            a_: a,
            self__: std::ptr::null_mut(),
        };
        if ((this) == (other)) {
            this.self__ = std::ptr::null_mut();
        } else {
            this.self__ = other;
        }
        this
    }
    pub unsafe fn returns_this_reference(&mut self) -> *mut S {
        return &mut (*self) as *mut S;
    }
    pub unsafe fn returns_this_pointer(&mut self) -> *mut S {
        return self;
    }
    pub unsafe fn inc(&mut self) -> *mut S {
        self.a_.postfix_inc();
        return &mut (*self) as *mut S;
    }
    pub unsafe fn set_from_this(&mut self) {
        self.a_ = ((self.a_) + (1));
    }
    pub unsafe fn get(&mut self) -> i32 {
        return self.a_;
    }
    pub unsafe fn twice(&mut self) -> i32 {
        return ((unsafe { S::get(self) }) * (2));
    }
    pub unsafe fn link(&mut self) {
        self.self__ = self;
    }
    pub unsafe fn bump_me(&mut self) {
        (unsafe { bump_0(self) });
    }
    pub unsafe fn cref(&self) -> *const S {
        return &(*self) as *const S;
    }
    pub unsafe fn is(&self, mut o: *const S) -> bool {
        return ((o) == (self));
    }
    pub unsafe fn destroy(&mut self) {
        ::std::mem::drop(Box::from_raw(self));
    }
    pub unsafe fn reset(&mut self) {
        (*self) = S::S1({ 0 });
    }
    pub unsafe fn copy_if_different_const(&mut self, mut other: *const S) -> bool {
        if ((self) == (other)) {
            return false;
        }
        self.a_ = (*other).a_;
        self.self__ = (*other).self__;
        return true;
    }
    pub unsafe fn copy_if_different(&mut self, mut other: *mut S) -> bool {
        if ((self) == (other)) {
            return false;
        }
        self.a_ = (*other).a_;
        self.self__ = (*other).self__;
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
        let mut this = Self { a_: a };
        this.a_ *= 2;
        this
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
