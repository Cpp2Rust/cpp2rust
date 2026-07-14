extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn fill_row_0(mut row: *mut libc::c_char, mut c: libc::c_char) {
    (*row.offset((0) as isize)) = c;
    (*row.offset((1) as isize)) = (('\0' as i32) as libc::c_char);
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut grid: [[libc::c_char; 6]; 3] = [[(0 as libc::c_char); 6]; 3];
    let mut i: i32 = 0;
    'loop_: while ((((i) < (3)) as i32) != 0) {
        (unsafe {
            let _row: *mut libc::c_char = grid[(i) as usize].as_mut_ptr();
            let _c: libc::c_char = ((('a' as i32) + (i)) as libc::c_char);
            fill_row_0(_row, _c)
        });
        i.postfix_inc();
    }
    assert!(((((grid[(0) as usize][(0) as usize] as i32) == ('a' as i32)) as i32) != 0));
    assert!(((((grid[(1) as usize][(0) as usize] as i32) == ('b' as i32)) as i32) != 0));
    assert!(((((grid[(2) as usize][(0) as usize] as i32) == ('c' as i32)) as i32) != 0));
    assert!(((((grid[(1) as usize][(1) as usize] as i32) == ('\0' as i32)) as i32) != 0));
    grid[(2) as usize][(5) as usize] = (('z' as i32) as libc::c_char);
    assert!(((((grid[(2) as usize][(5) as usize] as i32) == ('z' as i32)) as i32) != 0));
    return 0;
}
