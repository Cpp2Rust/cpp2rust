extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Default)]
pub struct Base {
    pub a: i32,
    pub copies: i32,
}
impl Base {
    pub unsafe fn Base(mut x: i32) -> Self {
        let mut this = Self { a: x, copies: 0 };
        this
    }
    pub unsafe fn Base_pconstBase(o: *const Base) -> Self {
        let mut this = Self {
            a: (*o).a,
            copies: (((*o).copies) + (1)),
        };
        this
    }
    pub unsafe fn operator_assign(&mut self, o: *const Base) -> *mut Base {
        self.a = (*o).a;
        self.copies = (((*o).copies) + (1));
        return &mut (*(self as *mut Base)) as *mut Base;
    }
}
impl Clone for Base {
    fn clone(&self) -> Self {
        unsafe { Base::Base_pconstBase(self as *const Base) }
    }
}
#[repr(C)]
#[derive(Clone, Default)]
pub struct Derived {
    pub base_Base: Base,
    pub b: i32,
}
impl Derived {
    pub unsafe fn Derived(mut x: i32) -> Self {
        let mut this = Self {
            base_Base: Base::Base({ x }),
            b: ((x) * (10)),
        };
        this
    }
}
pub unsafe fn take_0(mut v: Base) -> i32 {
    return ((v.a) + (v.copies));
}
pub unsafe fn make_1(mut x: i32) -> Base {
    let mut d: Derived = Derived::Derived({ x });
    return Base::Base_pconstBase({ &d.base_Base as *const Base });
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut d: Derived = Derived::Derived({ 1 });
    let mut init: Base = Base::Base_pconstBase({ &d.base_Base as *const Base });
    assert!(((init.a) == (1)));
    assert!(((init.copies) == (1)));
    let mut assigned: Base = Base::Base({ 9 });
    (unsafe { Base::operator_assign(&mut assigned, &d.base_Base as *const Base) });
    assert!(((assigned.a) == (1)));
    assert!(((assigned.copies) == (1)));
    assert!(((unsafe { take_0(Base::Base_pconstBase({ &d.base_Base as *const Base },),) }) == (2)));
    let mut made: Base = (unsafe { make_1(3) });
    assert!(((made.a) == (3)));
    assert!(((made.copies) >= (1)));
    d.base_Base.a = 7;
    assert!(((init.a) == (1)));
    assert!(((assigned.a) == (1)));
    return 0;
}
