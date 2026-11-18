extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0() {
    let mut source: (Option<Box<i32>>, i32) =
        (Some(Box::from_raw((Box::leak(Box::new(1)) as *mut i32))), 2);
    let mut target: (Option<Box<i32>>, i32) = (
        cpp_move_unsafe!(&mut source.0),
        cpp_move_unsafe!(&mut source.1),
    );
    assert!(
        ((*target
            .0
            .as_deref()
            .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
            == (1))
    );
    assert!(((target.1) == (2)));
    assert!(source.0.is_none());
    assert!(((source.1) == (2)));
}
pub unsafe fn test_1() {
    let mut owner: Option<Box<i32>> = Some(Box::from_raw((Box::leak(Box::new(3)) as *mut i32)));
    let mut moved: (Option<Box<i32>>, i32) = (cpp_move_unsafe!(&mut owner), 1);
    assert!(
        (owner
            .as_deref_mut()
            .map_or(::std::ptr::null_mut(), |v| v as *mut i32))
        .is_null()
    );
    assert!(
        ((*moved
            .0
            .as_deref()
            .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
            == (3))
    );
    assert!(((moved.1) == (1)));
    let mut text: Vec<libc::c_char> = std::slice::from_raw_parts(
        c"abc".as_ptr(),
        (0..).take_while(|&i| *c"abc".as_ptr().add(i) != 0).count() + 1,
    )
    .to_vec();
    let mut moved_text: (Vec<libc::c_char>, i32) = (std::mem::replace(&mut *&mut text, vec![0]), 2);
    assert!(
        moved_text.0 == {
            std::slice::from_raw_parts(
                c"abc".as_ptr(),
                (0..).take_while(|&i| *c"abc".as_ptr().add(i) != 0).count() + 1,
            )
            .to_vec()
        }
    );
    let values: Vec<i32> = vec![7; 2_usize];
    let mut copied: (Vec<i32>, i32) = ((values).clone(), 3);
    assert!(((values.len()) == (2_usize)));
    assert!(((copied.0.len()) == (2_usize)) && ((copied.0[(1_usize)]) == (7)));
    let mut referent: i32 = 5;
    let mut bound: (*const i32, libc::c_char) = (&mut referent, ('c' as libc::c_char));
    referent = 6;
    assert!(((*bound.0) == (6)));
    assert!(((bound.0) == ((&mut referent as *mut i32).cast_const())));
    assert!(((bound.1 as i32) == (('c' as libc::c_char) as i32)));
}
pub unsafe fn test_2() {
    let mut source: (i32, Option<Box<i32>>) = (
        0,
        Some(Box::from_raw((Box::leak(Box::new(81)) as *mut i32))),
    );
    let mut original: *mut i32 = source
        .1
        .as_deref_mut()
        .map_or(::std::ptr::null_mut(), |v| v as *mut i32);
    let mut converted: (i64, Option<Box<i32>>) =
        (((*&mut source.0) as i64), cpp_move_unsafe!(&mut source.1));
    assert!(((converted.0) == (0_i64)));
    assert!(
        ((converted
            .1
            .as_deref_mut()
            .map_or(::std::ptr::null_mut(), |v| v as *mut i32))
            == (original))
    );
    assert!(
        ((*converted
            .1
            .as_deref()
            .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
            == (81))
    );
    assert!(
        (source
            .1
            .as_deref_mut()
            .map_or(::std::ptr::null_mut(), |v| v as *mut i32))
        .is_null()
    );
}
pub unsafe fn test_3() {
    let mut source: (i32, Option<Box<i32>>) =
        (4, Some(Box::from_raw((Box::leak(Box::new(9)) as *mut i32))));
    let mut original: *mut i32 = source
        .1
        .as_deref_mut()
        .map_or(::std::ptr::null_mut(), |v| v as *mut i32);
    let mut target: (i64, Option<Box<i32>>) = <(i64, Option<Box<i32>>)>::default();
    {
        *(&mut target as *mut (i64, Option<Box<i32>>)) =
            (((*&mut source.0) as i64), cpp_move_unsafe!(&mut source.1));
        (&mut target as *mut (i64, Option<Box<i32>>))
    };
    assert!(((target.0) == (4_i64)));
    assert!(
        ((target
            .1
            .as_deref_mut()
            .map_or(::std::ptr::null_mut(), |v| v as *mut i32))
            == (original))
    );
    assert!(
        ((*target
            .1
            .as_deref()
            .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
            == (9))
    );
    assert!(
        (source
            .1
            .as_deref_mut()
            .map_or(::std::ptr::null_mut(), |v| v as *mut i32))
        .is_null()
    );
}
pub unsafe fn test_4() {
    let mut a: (i32, i32) = (1, 2);
    let mut b: (i32, i32) = (1, 3);
    let mut c: (i32, i32) = (2, 0);
    assert!(a != b);
    assert!(!(a != a));
    assert!(a < b);
    assert!(!(b < a));
    assert!(b < c);
    assert!(a <= a);
    assert!(!(c <= b));
    assert!(c > b);
    assert!(!(a > a));
    assert!(c >= a);
    assert!(!(a >= b));
    let mut ordered: bool = (a < b) && (b < c);
    assert!(ordered);
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_0() });
    (unsafe { test_1() });
    (unsafe { test_2() });
    (unsafe { test_3() });
    (unsafe { test_4() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
