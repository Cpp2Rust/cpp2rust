extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn foo_0(mut x: i32) -> i32 {
    return x;
}
pub unsafe fn foo_1(mut x: *mut i32) -> i32 {
    return (*x);
}
pub unsafe fn foo_2(mut x: *mut i32, mut y: *mut i32) -> i32 {
    return ((*x) + (*y));
}
pub unsafe fn foo_3(mut x: *mut i32, mut y: *mut i32, z: *mut i32) -> i32 {
    return (((*x) + (*y)) + (*z));
}
pub unsafe fn bar_4(x: *mut i32) -> i32 {
    return (*x);
}
#[derive(Copy, Clone, Default)]
pub struct Foo {}
impl Foo {
    pub unsafe fn foo_const(&self) {}
    pub unsafe fn foo(&mut self) {}
    pub unsafe fn method_i32(&mut self, mut x: i32) {}
    pub unsafe fn method_i32_const(&self, mut x: i32) {}
    pub unsafe fn method2_i32_i32_const(&self, mut x: i32, mut y: i32) {}
    pub unsafe fn method2_f64_f64_const(&self, mut x: f64, mut y: f64) {}
}
pub unsafe fn func_5(mut x: i32) -> i32 {
    return 1;
}
pub unsafe fn func_6(mut x: *mut i32) -> i32 {
    return 1;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut x: i32 = 1;
    let mut out: i32 = 0;
    out += (unsafe {
        let _x: i32 = 0;
        foo_0(_x)
    });
    out += (unsafe {
        let _x: *mut i32 = (&mut x as *mut i32);
        foo_1(_x)
    });
    out += (unsafe {
        let _x: *mut i32 = &mut x as *mut i32;
        bar_4(_x)
    });
    out += (unsafe {
        let _x: *mut i32 = (&mut x as *mut i32);
        let _y: *mut i32 = (&mut x as *mut i32);
        let _z: *mut i32 = &mut x as *mut i32;
        foo_3(_x, _y, _z)
    });
    out += (unsafe {
        let _x: *mut i32 = (&mut x as *mut i32);
        let _y: *mut i32 = (&mut x as *mut i32);
        foo_2(_x, _y)
    });
    let mut bar: i32 = 5;
    out += (((bar)
        + (unsafe {
            let _x: i32 = 0;
            foo_0(_x)
        }))
        + (unsafe {
            let _x: *mut i32 = (&mut x as *mut i32);
            foo_1(_x)
        }));
    let mut foo1: Foo = Foo {};
    let foo2: Foo = Foo {};
    (unsafe { foo1.foo() });
    (unsafe {
        let _x: i32 = 1;
        foo1.method_i32(_x)
    });
    (unsafe { foo2.foo_const() });
    (unsafe {
        let _x: i32 = 2;
        foo2.method_i32_const(_x)
    });
    return out;
}
