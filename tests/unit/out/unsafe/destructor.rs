extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut global_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct S {}
impl S {
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut global_0)).postfix_inc();
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct Defaulted {
    pub s: S,
}
impl Defaulted {
    pub unsafe fn destructor(&mut self) {
        S::destructor(&mut self.s);
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct Middle {
    pub s: S,
}
impl Middle {
    pub unsafe fn destructor(&mut self) {
        S::destructor(&mut self.s);
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct Outer {
    pub m: Middle,
}
impl Outer {
    pub unsafe fn destructor(&mut self) {
        Middle::destructor(&mut self.m);
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct ArrayMember {
    pub items: [S; 3],
}
impl ArrayMember {
    pub unsafe fn destructor(&mut self) {
        for __e in self.items.iter_mut() {
            S::destructor(__e);
        }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct EmptyBody {
    pub s: S,
}
impl EmptyBody {
    pub unsafe fn destructor(&mut self) {
        S::destructor(&mut self.s);
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct Templated_char_ {
    pub v: libc::c_char,
}
impl Templated_char_ {
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut global_0)) =
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut global_0)) as usize)
                .wrapping_add((::std::mem::size_of::<libc::c_char>() as usize))) as i32;
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct Templated_int_ {
    pub v: i32,
}
impl Templated_int_ {
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut global_0)) =
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut global_0)) as usize)
                .wrapping_add((::std::mem::size_of::<i32>() as usize))) as i32;
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct Copied {
    pub v: i32,
}
impl Copied {
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut global_0)).postfix_inc();
    }
}
pub static mut order_1: std::cell::LazyCell<[i32; 3]> =
    std::cell::LazyCell::new(|| unsafe { [0_i32; 3] });
pub static mut order_count_2: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct Tagged {
    pub tag: i32,
}
impl Tagged {
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut order_1))
            [((*std::cell::LazyCell::force_mut(&mut *&raw mut order_count_2)).postfix_inc())
                as usize] = self.tag;
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct Ordered {
    pub first: Tagged,
    pub dummy1: i32,
    pub second: Tagged,
    pub dummy2: i32,
    pub third: Tagged,
}
impl Ordered {
    pub unsafe fn destructor(&mut self) {
        Tagged::destructor(&mut self.third);
        Tagged::destructor(&mut self.second);
        Tagged::destructor(&mut self.first);
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    {
        let mut s: S = <S>::default();
        let _dtor_s = std::mem::DropGuard::new(&raw mut s, |__p| unsafe { (*__p).destructor() });
    }
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut global_0)) == (1)));
    {
        let mut s: S = <S>::default();
        let _dtor_s = std::mem::DropGuard::new(&raw mut s, |__p| unsafe { (*__p).destructor() });
    }
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut global_0)) == (2)));
    {
        let mut d: Defaulted = <Defaulted>::default();
        let _dtor_d = std::mem::DropGuard::new(&raw mut d, |__p| unsafe { (*__p).destructor() });
    }
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut global_0)) == (3)));
    {
        let mut o: Outer = <Outer>::default();
        let _dtor_o = std::mem::DropGuard::new(&raw mut o, |__p| unsafe { (*__p).destructor() });
    }
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut global_0)) == (4)));
    {
        let mut am: ArrayMember = <ArrayMember>::default();
        let _dtor_am = std::mem::DropGuard::new(&raw mut am, |__p| unsafe { (*__p).destructor() });
    }
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut global_0)) == (7)));
    {
        let mut e: EmptyBody = <EmptyBody>::default();
        let _dtor_e = std::mem::DropGuard::new(&raw mut e, |__p| unsafe { (*__p).destructor() });
    }
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut global_0)) == (8)));
    {
        let mut tc: Templated_char_ = <Templated_char_>::default();
        let _dtor_tc = std::mem::DropGuard::new(&raw mut tc, |__p| unsafe { (*__p).destructor() });
        let mut ti: Templated_int_ = <Templated_int_>::default();
        let _dtor_ti = std::mem::DropGuard::new(&raw mut ti, |__p| unsafe { (*__p).destructor() });
    }
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut global_0)) == (13)));
    {
        let mut a: Copied = Copied { v: 5 };
        let _dtor_a = std::mem::DropGuard::new(&raw mut a, |__p| unsafe { (*__p).destructor() });
        let mut b: Copied = a.clone();
        let _dtor_b = std::mem::DropGuard::new(&raw mut b, |__p| unsafe { (*__p).destructor() });
        assert!(((b.v) == (5)));
    }
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut global_0)) == (15)));
    {
        let mut o: Ordered = Ordered {
            first: Tagged { tag: 1 },
            dummy1: 0,
            second: Tagged { tag: 2 },
            dummy2: 0,
            third: Tagged { tag: 3 },
        };
        let _dtor_o = std::mem::DropGuard::new(&raw mut o, |__p| unsafe { (*__p).destructor() });
    }
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut order_count_2)) == (3)));
    assert!((((*std::cell::LazyCell::force_mut(&mut *&raw mut order_1))[(0) as usize]) == (3)));
    assert!((((*std::cell::LazyCell::force_mut(&mut *&raw mut order_1))[(1) as usize]) == (2)));
    assert!((((*std::cell::LazyCell::force_mut(&mut *&raw mut order_1))[(2) as usize]) == (1)));
    let mut before: i32 = (*std::cell::LazyCell::force_mut(&mut *&raw mut global_0));
    {
        let mut cs: S = <S>::default();
        let _dtor_cs = std::mem::DropGuard::new(&raw mut cs, |__p| unsafe { (*__p).destructor() });
        &(cs);
    }
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut global_0)) == ((before) + (1))));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const global_0);
    std::cell::LazyCell::force(&*&raw const order_1);
    std::cell::LazyCell::force(&*&raw const order_count_2);
}
