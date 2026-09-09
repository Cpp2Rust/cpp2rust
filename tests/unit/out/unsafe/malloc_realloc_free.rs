extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut p: *mut i32 = (libcc2rs::malloc_unsafe(::std::mem::size_of::<i32>()) as *mut i32);
        (*p) = 42;
        assert!(((((*p) == (42)) as i32) != 0));
        libcc2rs::free_unsafe((p as *mut i32 as *mut ::libc::c_void));
        let mut arr: *mut i32 = (libcc2rs::malloc_unsafe(
            (4_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)),
        ) as *mut i32);
        let mut i: i32 = 0;
        'loop_: while ((((i) < (4)) as i32) != 0) {
            (*arr.offset((i) as isize)) = ((i) * (10));
            i.postfix_inc();
        }
        assert!(((((*arr.offset((0) as isize)) == (0)) as i32) != 0));
        assert!(((((*arr.offset((3) as isize)) == (30)) as i32) != 0));
        libcc2rs::free_unsafe((arr as *mut i32 as *mut ::libc::c_void));
        let mut grow: *mut i32 = (libcc2rs::malloc_unsafe(
            (2_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)),
        ) as *mut i32);
        (*grow.offset((0) as isize)) = 1;
        (*grow.offset((1) as isize)) = 2;
        grow = (libcc2rs::realloc_unsafe(
            (grow as *mut i32 as *mut ::libc::c_void),
            (4_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)),
        ) as *mut i32);
        (*grow.offset((2) as isize)) = 3;
        (*grow.offset((3) as isize)) = 4;
        assert!(((((*grow.offset((0) as isize)) == (1)) as i32) != 0));
        assert!(((((*grow.offset((1) as isize)) == (2)) as i32) != 0));
        assert!(((((*grow.offset((2) as isize)) == (3)) as i32) != 0));
        assert!(((((*grow.offset((3) as isize)) == (4)) as i32) != 0));
        libcc2rs::free_unsafe((grow as *mut i32 as *mut ::libc::c_void));
        let mut zeros: *mut i32 =
            (libcc2rs::calloc_unsafe(4_usize, ::std::mem::size_of::<i32>()) as *mut i32);
        let mut i: i32 = 0;
        'loop_: while ((((i) < (4)) as i32) != 0) {
            assert!(((((*zeros.offset((i) as isize)) == (0)) as i32) != 0));
            i.postfix_inc();
        }
        libcc2rs::free_unsafe((zeros as *mut i32 as *mut ::libc::c_void));
    }
    let mut pmalloc: Option<unsafe fn(usize) -> *mut ::libc::c_void> =
        Some(libcc2rs::malloc_unsafe);
    let mut pfree: Option<unsafe fn(*mut ::libc::c_void)> = Some(libcc2rs::free_unsafe);
    let mut prealloc: Option<unsafe fn(*mut ::libc::c_void, usize) -> *mut ::libc::c_void> =
        Some(libcc2rs::realloc_unsafe);
    let mut pcalloc: Option<unsafe fn(usize, usize) -> *mut ::libc::c_void> =
        Some(libcc2rs::calloc_unsafe);
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut p: *mut i32 =
            ((unsafe { (pmalloc).unwrap()(::std::mem::size_of::<i32>()) }) as *mut i32);
        (*p) = 42;
        assert!(((((*p) == (42)) as i32) != 0));
        (unsafe { (pfree).unwrap()((p as *mut i32 as *mut ::libc::c_void)) });
        let mut arr: *mut i32 = ((unsafe {
            (pmalloc).unwrap()((4_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)))
        }) as *mut i32);
        let mut i: i32 = 0;
        'loop_: while ((((i) < (4)) as i32) != 0) {
            (*arr.offset((i) as isize)) = ((i) * (10));
            i.postfix_inc();
        }
        assert!(((((*arr.offset((0) as isize)) == (0)) as i32) != 0));
        assert!(((((*arr.offset((3) as isize)) == (30)) as i32) != 0));
        (unsafe { (pfree).unwrap()((arr as *mut i32 as *mut ::libc::c_void)) });
        let mut grow: *mut i32 = ((unsafe {
            (pmalloc).unwrap()((2_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)))
        }) as *mut i32);
        (*grow.offset((0) as isize)) = 1;
        (*grow.offset((1) as isize)) = 2;
        grow = ((unsafe {
            (prealloc).unwrap()(
                (grow as *mut i32 as *mut ::libc::c_void),
                (4_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)),
            )
        }) as *mut i32);
        (*grow.offset((2) as isize)) = 3;
        (*grow.offset((3) as isize)) = 4;
        assert!(((((*grow.offset((0) as isize)) == (1)) as i32) != 0));
        assert!(((((*grow.offset((1) as isize)) == (2)) as i32) != 0));
        assert!(((((*grow.offset((2) as isize)) == (3)) as i32) != 0));
        assert!(((((*grow.offset((3) as isize)) == (4)) as i32) != 0));
        (unsafe { (pfree).unwrap()((grow as *mut i32 as *mut ::libc::c_void)) });
        let mut zeros: *mut i32 =
            ((unsafe { (pcalloc).unwrap()(4_usize, ::std::mem::size_of::<i32>()) }) as *mut i32);
        let mut i: i32 = 0;
        'loop_: while ((((i) < (4)) as i32) != 0) {
            assert!(((((*zeros.offset((i) as isize)) == (0)) as i32) != 0));
            i.postfix_inc();
        }
        (unsafe { (pfree).unwrap()((zeros as *mut i32 as *mut ::libc::c_void)) });
    }
    return 0;
}
