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
pub unsafe fn sum_ptr_5(mut p: *const i32) -> i32 {
    return (((*p.offset((0) as isize)) + (*p.offset((1) as isize))) + (*p.offset((2) as isize)));
}
pub unsafe fn sum_decayed_6(a: *const [i32; 3]) -> i32 {
    return (unsafe { sum_ptr_5((*a).as_ptr()) });
}
pub unsafe fn bump_ptr_7(mut p: *mut i32) {
    (*p.offset((0) as isize)) += 1;
}
pub unsafe fn bump_decayed_8(a: *mut [i32; 3]) {
    (unsafe { bump_ptr_7((*a).as_mut_ptr()) });
}
pub unsafe fn fill_and_sum_9(a: *mut [i32; 3], mut v: i32, out: *mut i32) {
    (unsafe {
        let _a: *mut [i32; 3] = a;
        let _v: i32 = v;
        fill_3(_a, _v)
    });
    (*out) = (unsafe { sum_twice_4(a) });
}
pub unsafe fn pick_10(s: *const [libc::c_char; 5]) -> *const [libc::c_char; 5] {
    return s;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
pub unsafe fn sum_points_11(p: *const [Point; 2]) -> i32 {
    return (((((*p)[(0) as usize].x) + ((*p)[(0) as usize].y)) + ((*p)[(1) as usize].x))
        + ((*p)[(1) as usize].y));
}
pub unsafe fn shift_points_12(p: *mut [Point; 2], mut d: i32) {
    (*p)[(0) as usize].x += d;
    (*p)[(1) as usize].y += d;
}
pub unsafe fn total_len_13(names: *mut [*const libc::c_char; 2]) -> i32 {
    return ((unsafe { len5_1((*names)[(0) as usize]) })
        + (unsafe { len5_1((*names)[(1) as usize]) }));
}
pub static mut kTable_14: std::cell::LazyCell<[[i32; 3]; 2]> =
    std::cell::LazyCell::new(|| unsafe { [[1, 2, 3], [4, 5, 6]] });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((unsafe { len_0(&std::mem::transmute(*b"beta\0"),) }) == (4)));
    let mut buf: [libc::c_char; 5] = std::mem::transmute(*b"abcd\0");
    assert!(((unsafe { len_0(&buf,) }) == (4)));
    let mut arr: [i32; 3] = [1, 2, 3];
    assert!(((unsafe { sum_2(&arr,) }) == (6)));
    assert!(
        ((unsafe {
            let mut _a: [i32; 3] = [1, 2, 3];
            sum_2(&mut _a)
        }) == (6))
    );
    (unsafe { fill_3(&mut arr, 7) });
    assert!(((unsafe { sum_2(&arr,) }) == (21)));
    assert!(((unsafe { sum_twice_4(&arr,) }) == (42)));
    let mut out: i32 = 0;
    (unsafe { fill_and_sum_9(&mut arr, 2, &mut out) });
    assert!(((out) == (12)));
    assert!(((arr[(0) as usize]) == (2)));
    let lit: *const [libc::c_char; 5] = &std::mem::transmute(*b"beta\0");
    assert!(((unsafe { len_0(lit,) }) == (4)));
    assert!(
        (((*(unsafe { pick_10(&std::mem::transmute(*b"beta\0"),) }))[(0) as usize] as i32)
            == (('b' as libc::c_char) as i32))
    );
    assert!(((unsafe { len_0((unsafe { pick_10(&buf,) }),) }) == (4)));
    let mut pts: [Point; 2] = [Point { x: 1, y: 2 }, Point { x: 3, y: 4 }];
    assert!(((unsafe { sum_points_11(&pts,) }) == (10)));
    (unsafe { shift_points_12(&mut pts, 10) });
    assert!(((pts[(0) as usize].x) == (11)));
    assert!(((pts[(1) as usize].y) == (14)));
    assert!(((unsafe { sum_points_11(&pts,) }) == (30)));
    assert!(((unsafe { sum_decayed_6(&arr,) }) == (unsafe { sum_2(&arr,) })));
    (unsafe { bump_decayed_8(&mut arr) });
    assert!(((arr[(0) as usize]) == (3)));
    let mut names: [*const libc::c_char; 2] = [c"ab".as_ptr(), c"cde".as_ptr()];
    assert!(((unsafe { total_len_13(&mut names,) }) == (5)));
    let mut rows: [[i32; 3]; 2] = [[1, 2, 3], [4, 5, 6]];
    let mut p: *mut [i32; 3] = rows.as_mut_ptr();
    p.prefix_inc();
    assert!((((*p).as_mut_ptr()) == (&mut rows[(1) as usize][(0) as usize] as *mut i32)));
    let r: *mut [i32; 3] = &mut (*p);
    assert!((((*r)[(0) as usize]) == (4)));
    assert!(((unsafe { sum_2(r,) }) == (15)));
    assert!(((unsafe { sum_2(&(*p),) }) == (15)));
    let mut q: *mut [i32; 3] = rows.as_mut_ptr();
    assert!(((unsafe { sum_2(&(*q.offset((1) as isize)),) }) == (15)));
    assert!(
        (((*q.offset((1) as isize)).as_mut_ptr())
            == (&mut rows[(1) as usize][(0) as usize] as *mut i32))
    );
    (*q.offset((1) as isize))[(2) as usize] = 8;
    assert!(
        ((rows[(1) as usize][(2) as usize]) == (8)) && ((rows[(0) as usize][(2) as usize]) == (3))
    );
    rows[(0) as usize][(1) as usize] = rows[(0) as usize][(2) as usize];
    (*q.offset((1) as isize))[(0) as usize] = (*q.offset((0) as isize))[(1) as usize];
    (*q.offset((1) as isize))[(1) as usize] += (*q.offset((0) as isize))[(2) as usize];
    assert!(
        (((rows[(0) as usize][(1) as usize]) == (3))
            && ((rows[(1) as usize][(0) as usize]) == (3)))
            && ((rows[(1) as usize][(1) as usize]) == (8))
    );
    assert!(
        (((((&mut rows[(1) as usize][(2) as usize] as *mut i32) as usize
            - (&mut rows[(1) as usize][(0) as usize] as *mut i32) as usize)
            / ::std::mem::size_of::<i32>()) as i64)
            == (2_i64))
    );
    let mut key: [i32; 4] = [0, 4, 5, 6];
    assert!(
        (({
            let sa = core::slice::from_raw_parts(
                ((&mut key[(1) as usize] as *mut i32) as *const ::libc::c_void) as *const u8,
                ::std::mem::size_of::<[i32; 3]>() as usize,
            );
            let sb = core::slice::from_raw_parts(
                ((*std::cell::LazyCell::force_mut(&mut *&raw mut kTable_14))[(1) as usize].as_ptr()
                    as *const ::libc::c_void) as *const u8,
                ::std::mem::size_of::<[i32; 3]>() as usize,
            );
            let mut diff = 0_i32;
            for (x, y) in sa.iter().zip(sb.iter()) {
                if x != y {
                    diff = (*x as i32) - (*y as i32);
                    break;
                }
            }
            diff
        }) == (0))
    );
    assert!(
        (({
            let sa = core::slice::from_raw_parts(
                ((&mut key[(1) as usize] as *mut i32) as *const ::libc::c_void) as *const u8,
                ::std::mem::size_of::<[i32; 3]>() as usize,
            );
            let sb = core::slice::from_raw_parts(
                ((*std::cell::LazyCell::force_mut(&mut *&raw mut kTable_14))[(0) as usize].as_ptr()
                    as *const ::libc::c_void) as *const u8,
                ::std::mem::size_of::<[i32; 3]>() as usize,
            );
            let mut diff = 0_i32;
            for (x, y) in sa.iter().zip(sb.iter()) {
                if x != y {
                    diff = (*x as i32) - (*y as i32);
                    break;
                }
            }
            diff
        }) != (0))
    );
    assert!(
        (((*std::cell::LazyCell::force_mut(&mut *&raw mut kTable_14))[(1) as usize][(2) as usize])
            == (6))
            && (((*std::cell::LazyCell::force_mut(&mut *&raw mut kTable_14))[(0) as usize]
                [(1) as usize])
                == (2))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const kTable_14);
}
