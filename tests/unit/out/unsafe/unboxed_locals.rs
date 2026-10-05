extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn set_0(mut p: *mut i32, mut v: i32) {
    (*p) = v;
}
pub unsafe fn inc_1(r: *mut i32) {
    (*r).prefix_inc();
}
pub unsafe fn sum_2(mut arr: *const i32, mut n: i32) -> i32 {
    let mut s: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while ((i) < (n)) {
        s += (*arr.offset((i) as isize));
        i.prefix_inc();
    }
    return s;
}
pub unsafe fn countdown_3(mut n: i32, mut step: Option<i32>) -> i32 {
    let mut step: i32 = step.unwrap_or(1);
    let mut steps: i32 = 0;
    'loop_: while ((n) > (0)) {
        n -= step;
        steps.postfix_inc();
    }
    return steps;
}
pub unsafe trait Shape {
    unsafe fn scale(&mut self, factor: i32) -> i32;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Square {
    pub side: i32,
}
impl Default for Square {
    fn default() -> Self {
        Square { side: 2 }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut plain: i32 = 1;
    plain += 2;
    plain.postfix_inc();
    assert!(((plain) == (4)));
    &(::std::mem::size_of::<i32>());
    let mut addr: i32 = 0;
    (unsafe { set_0((&mut addr as *mut i32), 5) });
    assert!(((addr) == (5)));
    let mut ref_: i32 = 1;
    (unsafe { inc_1(&mut ref_) });
    let alias: *mut i32 = &mut ref_;
    (*alias).postfix_inc();
    assert!(((ref_) == (3)));
    let mut captured: i32 = 7;
    let mut get: FnPtr<fn() -> i32> = lambda_unsafe!(
        {
            let captured: *mut i32 = &mut captured;
        },
        || -> i32 {
            return (*captured);
        }
    );
    captured = 8;
    assert!(((unsafe { get.call() }) == (8)));
    let mut arr: [i32; 4] = [1, 2, 3, 4];
    arr[(0) as usize] = ((arr[(3) as usize]) * (2));
    arr[(1) as usize].postfix_inc();
    assert!((((arr[(0) as usize]) + (arr[(1) as usize])) == (11)));
    let mut elem: [i32; 2] = [0, 0];
    (unsafe { set_0((&mut elem[(1) as usize] as *mut i32), 6) });
    assert!(((elem[(1) as usize]) == (6)));
    let mut decayed: [i32; 3] = [1, 2, 3];
    assert!(((unsafe { sum_2((decayed.as_mut_ptr()).cast_const(), 3,) }) == (6)));
    let mut elem_ref: [i32; 2] = [1, 1];
    (unsafe { inc_1(&mut elem_ref[(0) as usize]) });
    assert!(((elem_ref[(0) as usize]) == (2)));
    let mut p: *mut i32 = (&mut addr as *mut i32);
    p = (&mut elem[(0) as usize] as *mut i32);
    (*p) = 9;
    assert!(((elem[(0) as usize]) == (9)));
    let mut str: [libc::c_char; 8] = std::mem::transmute(*b"abc\0\0\0\0\0");
    str[(0) as usize] = ('x' as libc::c_char);
    assert!(
        ((str[(0) as usize] as i32) == (('x' as libc::c_char) as i32))
            && ((str[(3) as usize] as i32) == (('\0' as libc::c_char) as i32))
    );
    let mut zeros: [f64; 16] = [
        0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64,
        0_f64, 0_f64, 0_f64,
    ];
    zeros[(15) as usize] = 1.5E+0;
    assert!(((zeros[(0) as usize]) == (0_f64)) && ((zeros[(15) as usize]) == (1.5E+0)));
    assert!(((unsafe { countdown_3(10, None,) }) == (10)));
    assert!(((unsafe { countdown_3(10, Some(3),) }) == (4)));
    let mut square: Square = <Square>::default();
    let mut shape: *mut dyn Shape = (&mut square as *mut Square);
    assert!(((unsafe { (*shape).scale(3,) }) == (6)));
    return 0;
}
unsafe impl Shape for Square {
    unsafe fn scale(&mut self, factor: i32) -> i32 {
        return ((factor) * (self.side));
    }
}
pub unsafe fn __cpp2rust_init_globals() {}
