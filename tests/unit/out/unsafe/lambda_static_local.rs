extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut a_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(
        ((unsafe {
            (|| {
                static mut n_1: std::cell::LazyCell<i32> =
                    std::cell::LazyCell::new(|| unsafe { 0 });;
                return (*std::cell::LazyCell::force_mut(&mut *&raw mut n_1)).prefix_inc();
            })()
        }) == (1))
    );
    assert!(
        ((unsafe {
            (|| {
                static mut n_1: std::cell::LazyCell<i32> =
                    std::cell::LazyCell::new(|| unsafe { 0 });;
                return (*std::cell::LazyCell::force_mut(&mut *&raw mut n_1)).prefix_inc();
            })()
        }) == (2))
    );
    let mut copy: _ = (|| {
        static mut n_1: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });;
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut n_1)).prefix_inc();
    })
    .clone();
    assert!(((unsafe { copy() }) == (3)));
    assert!(
        ((unsafe {
            (|| {
                static mut n_1: std::cell::LazyCell<i32> =
                    std::cell::LazyCell::new(|| unsafe { 0 });;
                return (*std::cell::LazyCell::force_mut(&mut *&raw mut n_1)).prefix_inc();
            })()
        }) == (4))
    );
    assert!(( ( ( unsafe { ( ( | x : , | { static mut calls_2 : std::cell::LazyCell< i32 > = std::cell::LazyCell::new(|| unsafe { 0 } ) ;
  ;
 ;
 (*std::cell::LazyCell::force_mut(&mut *&raw mut calls_2)) .postfix_inc() ;
 return (*std::cell::LazyCell::force_mut(&mut *&raw mut calls_2))  ;
 } ) ) ( 1 , ) } ) ) == ( 1 ) ) );
    assert!(( ( ( unsafe { ( ( | x : , | { static mut calls_2 : std::cell::LazyCell< i32 > = std::cell::LazyCell::new(|| unsafe { 0 } ) ;
  ;
 ;
 (*std::cell::LazyCell::force_mut(&mut *&raw mut calls_2)) .postfix_inc() ;
 return (*std::cell::LazyCell::force_mut(&mut *&raw mut calls_2))  ;
 } ) ) ( 2 , ) } ) ) == ( 2 ) ) );
    assert!(( ( ( unsafe { ( ( | x : , | { static mut calls_2 : std::cell::LazyCell< i32 > = std::cell::LazyCell::new(|| unsafe { 0 } ) ;
  ;
 ;
 (*std::cell::LazyCell::force_mut(&mut *&raw mut calls_2)) .postfix_inc() ;
 return (*std::cell::LazyCell::force_mut(&mut *&raw mut calls_2))  ;
 } ) ) ( 1.0E+0 , ) } ) ) == ( 1 ) ) );
    assert!(( ( ( unsafe { ( ( | x : , | { static mut calls_2 : std::cell::LazyCell< i32 > = std::cell::LazyCell::new(|| unsafe { 0 } ) ;
  ;
 ;
 (*std::cell::LazyCell::force_mut(&mut *&raw mut calls_2)) .postfix_inc() ;
 return (*std::cell::LazyCell::force_mut(&mut *&raw mut calls_2))  ;
 } ) ) ( 3 , ) } ) ) == ( 3 ) ) );
    assert!(
        ((unsafe {
            (|_a0: i32| {
                return (*std::cell::LazyCell::force_mut(&mut *&raw mut a_0));
            })(0)
        }) == (0))
    );
    (*std::cell::LazyCell::force_mut(&mut *&raw mut a_0)) = 1;
    assert!(
        ((unsafe {
            (|_a0: i32| {
                return (*std::cell::LazyCell::force_mut(&mut *&raw mut a_0));
            })(0)
        }) == (1))
    );
    assert!(
        ((unsafe {
            (|| {
                let mut p: P_3 = P_3 { x: 2, y: 3 };
                return ((p.x) * (p.y));
            })()
        }) == (6))
    );
    return 0;
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct P_3 {
    pub x: i32,
    pub y: i32,
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const a_0);
}
