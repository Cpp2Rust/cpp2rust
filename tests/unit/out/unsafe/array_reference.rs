extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn len_0(s: *const [libc::c_char; 5]) -> i32 {
    let mut n: i32 = 0;
    'loop_: while (((*s)[(n) as usize] as i32) != (('\0' as libc::c_char) as i32)) {
        n.prefix_inc();
    }
    return n;
}
pub unsafe fn sum_1(a: *const [i32; 3]) -> i32 {
    return ((((*a)[(0) as usize]) + ((*a)[(1) as usize])) + ((*a)[(2) as usize]));
}
pub unsafe fn fill_2(a: *mut [i32; 3], mut v: i32) {
    let mut i: i32 = 0;
    'loop_: while ((i) < (3)) {
        (*a)[(i) as usize] = v;
        i.prefix_inc();
    }
}
pub unsafe fn sum_twice_3(a: *const [i32; 3]) -> i32 {
    return ((unsafe { sum_1(a) }) + (unsafe { sum_1(a) }));
}
pub unsafe fn fill_and_sum_4(a: *mut [i32; 3], mut v: i32, out: *mut i32) {
    (unsafe {
        let _a: *mut [i32; 3] = a;
        let _v: i32 = v;
        fill_2(_a, _v)
    });
    (*out) = (unsafe { sum_twice_3(a) });
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(
        ((unsafe { len_0(&std::mem::transmute(*b"beta\0") as *const [libc::c_char; 5],) }) == (4))
    );
    let mut buf: [libc::c_char; 5] = std::mem::transmute(*b"abcd\0");
    assert!(((unsafe { len_0(&buf as *const [libc::c_char; 5],) }) == (4)));
    let mut arr: [i32; 3] = [1, 2, 3];
    assert!(((unsafe { sum_1(&arr as *const [i32; 3],) }) == (6)));
    (unsafe { fill_2(&mut arr as *mut [i32; 3], 7) });
    assert!(((unsafe { sum_1(&arr as *const [i32; 3],) }) == (21)));
    assert!(((unsafe { sum_twice_3(&arr as *const [i32; 3],) }) == (42)));
    let mut out: i32 = 0;
    (unsafe { fill_and_sum_4(&mut arr as *mut [i32; 3], 2, &mut out as *mut i32) });
    assert!(((out) == (12)));
    assert!(((arr[(0) as usize]) == (2)));
    return 0;
}
