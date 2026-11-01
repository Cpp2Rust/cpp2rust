extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct Degrees {
    pub value: i32,
}
impl Degrees {
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self { value: v };
        this
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut input: [i32; 3] = [1, 2, 3];
    let mut output: [i32; 3] = [0_i32; 3];
    match ({ input.as_mut_ptr().offset((3) as isize) },) {
        (mut __tmp_0,) => {
            let mut __first = input.as_mut_ptr();
            let mut __out = output.as_mut_ptr();
            while __first != __tmp_0 {
                *__out.iter_deref() = (*__first.iter_deref());
                __first.iter_inc();
                __out.iter_inc();
            }
            __out
        }
    };
    let mut widened: Vec<i64> = vec![0_i64; 3_usize as usize];
    match ({ input.as_mut_ptr().offset((3) as isize) },) {
        (mut __tmp_1,) => {
            let mut __first = input.as_mut_ptr();
            let mut __out = widened.begin();
            while __first != __tmp_1 {
                *__out.iter_deref() = ((*__first.iter_deref()) as i64);
                __first.iter_inc();
                __out.iter_inc();
            }
            __out
        }
    };
    assert!(((widened[(0_usize)]) == (1_i64)) && ((widened[(2_usize)]) == (3_i64)));
    let mut constructed: Vec<Degrees> = vec![Degrees::new({ 0 },); 3_usize as usize];
    match ({ input.as_mut_ptr().offset((3) as isize) },) {
        (mut __tmp_2,) => {
            let mut __first = input.as_mut_ptr();
            let mut __out = constructed.begin();
            while __first != __tmp_2 {
                *__out.iter_deref() =
                    <Degrees as CppFromUnsafe<i32>>::cpp_from(__first.iter_deref());
                __first.iter_inc();
                __out.iter_inc();
            }
            __out
        }
    };
    assert!(((constructed[(0_usize)].value) == (1)) && ((constructed[(2_usize)].value) == (3)));
    let mut wide: [i64; 2] = [7_i64, 8_i64];
    let mut floating: Vec<f64> = vec![0_f64; 2_usize as usize];
    match ({ wide.as_mut_ptr().offset((2) as isize) },) {
        (mut __tmp_3,) => {
            let mut __first = wide.as_mut_ptr();
            let mut __out = floating.begin();
            while __first != __tmp_3 {
                *__out.iter_deref() = ((*__first.iter_deref()) as f64);
                __first.iter_inc();
                __out.iter_inc();
            }
            __out
        }
    };
    assert!(((floating[(1_usize)]) == (8_f64)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
