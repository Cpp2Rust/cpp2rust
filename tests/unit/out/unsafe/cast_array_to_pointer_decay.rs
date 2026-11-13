extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn deref_0(mut p: *mut i32) -> i32 {
    return (*p);
}
pub unsafe fn strlen_1(mut s: *mut libc::c_char) -> i32 {
    let mut c: i32 = 0;
    'loop_: while ((*s.postfix_inc()) != 0) {
        c.prefix_inc();
    }
    return c;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Row {
    pub a: [i32; 3],
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: [i32; 2] = [1, 2];
    let mut s: [libc::c_char; 4] = [
        ('a' as libc::c_char),
        ('b' as libc::c_char),
        ('c' as libc::c_char),
        ('\0' as libc::c_char),
    ];
    assert!(
        (((unsafe { deref_0(a.as_mut_ptr(),) }) + (unsafe { strlen_1(s.as_mut_ptr(),) })) == (4))
    );
    let mut m: [[i32; 3]; 2] = [[1, 2, 3], [4, 5, 6]];
    let mut r: Row = Row { a: [7, 8, 9] };
    let mut row: *mut [i32; 3] = (&mut m[(1) as usize] as *mut [i32; 3]);
    let mut field: *mut [i32; 3] = (&mut r.a as *mut [i32; 3]);
    let mut whole: *mut [i32; 3] = (&mut m[(0) as usize] as *mut [i32; 3]);
    assert!(
        ((((*row)[(2) as usize]) == (6)) && (((*field)[(1) as usize]) == (8)))
            && (((*whole)[(0) as usize]) == (1))
    );
    assert!(
        (((row as *mut i32) == (m[(1) as usize].as_mut_ptr())) && ((*(field as *mut i32)) == (7)))
            && ((*((&mut r.a as *mut [i32; 3]) as *mut i32).offset((2) as isize)) == (9))
    );
    let mut v: *mut ::libc::c_void = ((&mut r.a as *mut [i32; 3]) as *mut ::libc::c_void);
    assert!(
        ((v) == (r.a.as_mut_ptr() as *mut ::libc::c_void)) && ((v) != (row as *mut ::libc::c_void))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
