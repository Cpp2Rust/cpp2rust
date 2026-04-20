#![feature(rustc_private)]
extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::{prepostfix::*, rc::*};
use std::cell::RefCell;
use std::rc::{Rc, Weak};
pub fn decay_cast(a1: Value<Pointer<u32>>) {}
pub fn bit_cast(p: Value<Pointer<dyn std::any::Any>>) {}
pub fn main() {
    std::process::exit(*main_0().borrow() as i32);
}
pub fn main_0() -> Value<i32> {
    let a1: Value<Box<[Value<u32>]>> = Rc::new(RefCell::new(Box::new([
        Rc::new(RefCell::new((1_i32 as u32))),
        Rc::new(RefCell::new((2_i32 as u32))),
        Rc::new(RefCell::new((3_i32 as u32))),
    ])));
    decay_cast(Rc::new(RefCell::new(a1.as_pointer().clone())));
    decay_cast(Rc::new(RefCell::new(
        a1.as_pointer().offset(0_i32 as isize),
    )));
    bit_cast(Rc::new(RefCell::new(
        (a1.as_pointer().clone() as Pointer<u32>)
            .as_any()
            .offset(0_isize),
    )));
    bit_cast(Rc::new(RefCell::new(
        (a1.as_pointer().offset(0_i32 as isize).clone() as Pointer<u32>).as_any(),
    )));
    bit_cast(Rc::new(RefCell::new(
        (a1.as_pointer().clone() as Pointer<u32>).as_any(),
    )));
    let ptr: Value<Pointer<dyn std::any::Any>> = Rc::new(RefCell::new(
        (a1.as_pointer().clone() as Pointer<u32>)
            .as_any()
            .offset(0_isize),
    ));
    (if ((!((*ptr.borrow()) == (a1.as_pointer().clone() as Pointer<u32>).as_any()) as i64) != 0) {
        panic!("assertion failed: {}", unsafe {
            ::std::ffi::CStr::from_ptr(b"ptr == a1\0" as *const u8 as *const i8)
                .to_str()
                .unwrap()
        })
    } else {
    });
    (if ((!((*((*ptr.borrow()).downcast::<u32>())
        .offset(0_i32 as isize)
        .as_reference()
        .upgrade()
        .expect("ub: dangling pointer")
        .borrow())
        == (*(*a1.borrow())[0_i32 as usize].borrow())) as i64)
        != 0)
    {
        panic!("assertion failed: {}", unsafe {
            ::std::ffi::CStr::from_ptr(
                b"((unsigned int*)ptr)[0] == a1[0]\0" as *const u8 as *const i8,
            )
            .to_str()
            .unwrap()
        })
    } else {
    });
    return Rc::new(RefCell::new(0_i32));
}
