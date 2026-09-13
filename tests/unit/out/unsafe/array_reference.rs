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
pub unsafe fn len5_1(mut s: *const libc::c_char) -> i32 {
    let mut n: i32 = 0;
    'loop_: while (((*s.offset((n) as isize)) as i32) != (('\0' as libc::c_char) as i32)) {
        n.prefix_inc();
    }
    return n;
}
pub unsafe fn sum_2(a: *const [i32; 3]) -> i32 {
    return ((((*a)[(0) as usize]) + ((*a)[(1) as usize])) + ((*a)[(2) as usize]));
}
pub unsafe fn fill_3(a: *mut [i32; 3], mut v: i32) {
    let mut i: i32 = 0;
    'loop_: while ((i) < (3)) {
        (*a)[(i) as usize] = v;
        i.prefix_inc();
    }
}
pub unsafe fn sum_twice_4(a: *const [i32; 3]) -> i32 {
    return ((unsafe { sum_2(a) }) + (unsafe { sum_2(a) }));
}
pub unsafe fn fill_and_sum_5(a: *mut [i32; 3], mut v: i32, out: *mut i32) {
    (unsafe {
        let _a: *mut [i32; 3] = a;
        let _v: i32 = v;
        fill_3(_a, _v)
    });
    (*out) = (unsafe { sum_twice_4(a) });
}
pub unsafe fn pick_6(s: *const [libc::c_char; 5]) -> *const [libc::c_char; 5] {
    return s;
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
pub unsafe fn sum_points_7(p: *const [Point; 2]) -> i32 {
    return (((((*p)[(0) as usize].x) + ((*p)[(0) as usize].y)) + ((*p)[(1) as usize].x))
        + ((*p)[(1) as usize].y));
}
pub unsafe fn shift_points_8(p: *mut [Point; 2], mut d: i32) {
    (*p)[(0) as usize].x += d;
    (*p)[(1) as usize].y += d;
}
pub unsafe fn total_len_9(names: *mut [*const libc::c_char; 2]) -> i32 {
    return ((unsafe { len5_1((*names)[(0) as usize]) })
        + (unsafe { len5_1((*names)[(1) as usize]) }));
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
    assert!(((unsafe { sum_2(&arr as *const [i32; 3],) }) == (6)));
    (unsafe { fill_3(&mut arr as *mut [i32; 3], 7) });
    assert!(((unsafe { sum_2(&arr as *const [i32; 3],) }) == (21)));
    assert!(((unsafe { sum_twice_4(&arr as *const [i32; 3],) }) == (42)));
    let mut out: i32 = 0;
    (unsafe { fill_and_sum_5(&mut arr as *mut [i32; 3], 2, &mut out as *mut i32) });
    assert!(((out) == (12)));
    assert!(((arr[(0) as usize]) == (2)));
    let lit: *const [libc::c_char; 5] =
        &std::mem::transmute(*b"beta\0") as *const [libc::c_char; 5];
    assert!(((unsafe { len_0(lit,) }) == (4)));
    assert!(
        (((*(unsafe { pick_6(&std::mem::transmute(*b"beta\0") as *const [libc::c_char; 5],) }))
            [(0) as usize] as i32)
            == (('b' as libc::c_char) as i32))
    );
    assert!(((unsafe { len_0((unsafe { pick_6(&buf as *const [libc::c_char; 5],) }),) }) == (4)));
    let mut pts: [Point; 2] = [Point { x: 1, y: 2 }, Point { x: 3, y: 4 }];
    assert!(((unsafe { sum_points_7(&pts as *const [Point; 2],) }) == (10)));
    (unsafe { shift_points_8(&mut pts as *mut [Point; 2], 10) });
    assert!(((pts[(0) as usize].x) == (11)));
    assert!(((pts[(1) as usize].y) == (14)));
    assert!(((unsafe { sum_points_7(&pts as *const [Point; 2],) }) == (30)));
    let mut names: [*const libc::c_char; 2] = [c"ab".as_ptr(), c"cde".as_ptr()];
    assert!(((unsafe { total_len_9(&mut names as *mut [*const libc::c_char; 2],) }) == (5)));
    return 0;
}
