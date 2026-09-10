extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut order_0: [i32; 8] = unsafe { [0_i32; 8] };
pub static mut n_1: i32 = unsafe { 0 };
#[repr(C)]
#[derive(Clone, Default)]
pub struct Base {}
impl Base {
    pub unsafe fn destructor(&mut self) {
        order_0[(n_1.postfix_inc()) as usize] = 1;
    }
}
#[repr(C)]
#[derive(Clone, Default)]
pub struct Member {}
impl Member {
    pub unsafe fn destructor(&mut self) {
        order_0[(n_1.postfix_inc()) as usize] = 2;
    }
}
#[repr(C)]
#[derive(Clone, Default)]
pub struct Derived {
    pub base_Base: Base,
    pub m: Member,
}
impl Derived {
    pub unsafe fn destructor(&mut self) {
        order_0[(n_1.postfix_inc()) as usize] = 3;
        Member::destructor(&mut self.m);
        Base::destructor(&mut self.base_Base);
    }
}
#[repr(C)]
#[derive(Clone, Default)]
pub struct Implicit {
    pub base_Base: Base,
    pub m: Member,
}
impl Implicit {
    pub unsafe fn destructor(&mut self) {
        Member::destructor(&mut self.m);
        Base::destructor(&mut self.base_Base);
    }
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    {
        let mut d: Derived = <Derived>::default();
        let _dtor_d = ScopedDestructorUnsafe::new(&raw mut d, Derived::destructor);
    }
    assert!(((n_1) == (3)));
    assert!(
        (((order_0[(0) as usize]) == (3)) && ((order_0[(1) as usize]) == (2)))
            && ((order_0[(2) as usize]) == (1))
    );
    n_1 = 0;
    {
        let mut i: Implicit = <Implicit>::default();
        let _dtor_i = ScopedDestructorUnsafe::new(&raw mut i, Implicit::destructor);
    }
    assert!(((n_1) == (2)));
    assert!(((order_0[(0) as usize]) == (2)) && ((order_0[(1) as usize]) == (1)));
    n_1 = 0;
    let mut p: *mut Derived = (Box::leak(Box::new(<Derived>::default())) as *mut Derived);
    {
        let __p = p;
        Derived::destructor(&mut *__p);
        ::std::mem::drop(Box::from_raw(__p))
    };
    assert!(((n_1) == (3)));
    assert!(
        (((order_0[(0) as usize]) == (3)) && ((order_0[(1) as usize]) == (2)))
            && ((order_0[(2) as usize]) == (1))
    );
    return 0;
}
