extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::Seek;
use std::io::{Read, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn identity_0(mut x: i32) -> i32 {
    return x;
}
pub unsafe fn swap_by_ptr_1(mut a: *mut i32, mut b: *mut i32) {
    let mut tmp: i32 = (*a);
    (*a) = (*b);
    (*b) = tmp;
}
pub unsafe fn swap_by_ref_2(a: *mut i32, b: *mut i32) {
    let mut tmp: i32 = (*a);
    (*a) = (*b);
    (*b) = tmp;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut x: i32 = 0;
    write!(
        std::fs::File::from_raw_fd(
            std::io::stdout()
                .as_fd()
                .try_clone_to_owned()
                .unwrap()
                .into_raw_fd(),
        ),
        "{:}\n",
        x,
    );
    let mut a: i32 = 1;
    let mut b: i32 = (unsafe {
        let _x: i32 = a;
        identity_0(_x)
    });
    write!(
        std::fs::File::from_raw_fd(
            std::io::stdout()
                .as_fd()
                .try_clone_to_owned()
                .unwrap()
                .into_raw_fd(),
        ),
        "{:}\n",
        b,
    );
    let mut c: i32 = 2;
    let mut p: *mut i32 = (&mut c as *mut i32);
    write!(
        std::fs::File::from_raw_fd(
            std::io::stdout()
                .as_fd()
                .try_clone_to_owned()
                .unwrap()
                .into_raw_fd(),
        ),
        "{:}\n",
        (*p),
    );
    let mut d: i32 = 3;
    let mut e: i32 = 4;
    (unsafe {
        let _a: *mut i32 = (&mut d as *mut i32);
        let _b: *mut i32 = (&mut e as *mut i32);
        swap_by_ptr_1(_a, _b)
    });
    let mut f: i32 = 4;
    let mut g: i32 = 5;
    (unsafe {
        let _a: *mut i32 = &mut f as *mut i32;
        let _b: *mut i32 = &mut g as *mut i32;
        swap_by_ref_2(_a, _b)
    });
    let mut h: *mut i32 = (Box::leak(Box::new(6)) as *mut i32);
    write!(
        std::fs::File::from_raw_fd(
            std::io::stdout()
                .as_fd()
                .try_clone_to_owned()
                .unwrap()
                .into_raw_fd(),
        ),
        "{:}\n",
        (*h),
    );
    ::std::mem::drop(Box::from_raw(h));
    let mut i: *mut i32 = Box::leak(Box::new([7, 8, 0 as i32])).as_mut_ptr();
    write!(
        std::fs::File::from_raw_fd(
            std::io::stdout()
                .as_fd()
                .try_clone_to_owned()
                .unwrap()
                .into_raw_fd(),
        ),
        "{:} {:}\n",
        (*i.offset((0) as isize)),
        (*i.offset((1) as isize)),
    );

    ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
        i,
        libcc2rs::malloc_usable_size(i as *mut ::libc::c_void) / ::std::mem::size_of::<i32>(),
    )));
    (unsafe {
        let _a: *mut i32 = (Box::leak(Box::new(7)) as *mut i32);
        let _b: *mut i32 = (Box::leak(Box::new(8)) as *mut i32);
        swap_by_ptr_1(_a, _b)
    });
    (unsafe {
        let _a: *mut i32 = (Box::leak(Box::new(7)) as *mut i32).offset((0) as isize);
        let _b: *mut i32 = (Box::leak(Box::new(8)) as *mut i32).offset((0) as isize);
        swap_by_ptr_1(_a, _b)
    });
    (unsafe {
        let _a: *mut i32 = &mut (*(Box::leak(Box::new(9)) as *mut i32)) as *mut i32;
        let _b: *mut i32 = &mut (*(Box::leak(Box::new(10)) as *mut i32)) as *mut i32;
        swap_by_ref_2(_a, _b)
    });
    (unsafe {
        let _a: *mut i32 =
            &mut (*(Box::leak(Box::new(9)) as *mut i32).offset((0) as isize)) as *mut i32;
        let _b: *mut i32 =
            &mut (*(Box::leak(Box::new(10)) as *mut i32).offset((0) as isize)) as *mut i32;
        swap_by_ref_2(_a, _b)
    });
    let mut j: Option<Box<i32>> = Some(Box::from_raw((Box::leak(Box::new(11)) as *mut i32)));
    let mut k: *mut i32 = j
        .as_deref_mut()
        .map_or(::std::ptr::null_mut(), |v| v as *mut i32);
    write!(
        std::fs::File::from_raw_fd(
            std::io::stdout()
                .as_fd()
                .try_clone_to_owned()
                .unwrap()
                .into_raw_fd(),
        ),
        "{:}\n",
        (*k),
    );
    let mut l: Option<Box<i32>> = Some(Box::new(11));
    let mut m: *mut i32 = l
        .as_deref_mut()
        .map_or(::std::ptr::null_mut(), |v| v as *mut i32);
    write!(
        std::fs::File::from_raw_fd(
            std::io::stdout()
                .as_fd()
                .try_clone_to_owned()
                .unwrap()
                .into_raw_fd(),
        ),
        "{:}\n",
        (*m),
    );
    return c;
}
