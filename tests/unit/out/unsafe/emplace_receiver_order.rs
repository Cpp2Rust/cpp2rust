extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn select_rows_0(
    mut rows: *mut Vec<Vec<i32>>,
    mut calls: *mut i32,
) -> *mut Vec<Vec<i32>> {
    (*calls).prefix_inc();
    (&mut (*rows))[(0_usize)][(2_usize)] = 9;
    return rows;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    {
        let mut bound: i32 = 0;
        'loop_: while ((bound) < (2)) {
            let mut rows: Vec<Vec<i32>> = Vec::from([Vec::from([1, 2, 3])]);
            if 2_usize > rows.capacity() as usize {
                let len_0 = rows.len();
                rows.reserve_exact(2_usize - len_0 as usize);
            };
            let mut calls: i32 = 0;
            if (bound != 0) {
                let mut receiver: *mut Vec<Vec<i32>> = (unsafe {
                    select_rows_0((&mut rows as *mut Vec<Vec<i32>>), (&mut calls as *mut i32))
                });
                (*receiver).push({
                    let mut __tmp_0 = { rows[(0_usize)].as_mut_ptr() };
                    let mut __tmp_1 = { rows[(0_usize)].as_mut_ptr().offset((3) as isize) };
                    let mut __v = Vec::new();
                    let mut __it = __tmp_0;
                    while __it != __tmp_1 {
                        __v.push((*__it.iter_deref()));
                        __it.iter_inc();
                    }
                    __v
                });
            } else {
                {
                    let mut __tmp_2 = {
                        (unsafe {
                            select_rows_0(
                                (&mut rows as *mut Vec<Vec<i32>>),
                                (&mut calls as *mut i32),
                            )
                        })
                    };
                    let mut __tmp_5 = {
                        {
                            let mut __tmp_3 = { rows[(0_usize)].as_mut_ptr() };
                            let mut __tmp_4 = { rows[(0_usize)].as_mut_ptr().offset((3) as isize) };
                            let mut __v = Vec::new();
                            let mut __it = __tmp_3;
                            while __it != __tmp_4 {
                                __v.push((*__it.iter_deref()));
                                __it.iter_inc();
                            }
                            __v
                        }
                    };
                    (*__tmp_2).push(__tmp_5)
                };
            }
            (unsafe {
                libc::printf(
                    c"source=%d inserted=%d calls=%d\n".as_ptr() as *const libc::c_char,
                    (rows[(0_usize)][(2_usize)]),
                    (rows[(1_usize)][(2_usize)]),
                    (calls),
                )
            });
            bound.prefix_inc();
        }
    }
    0
}
pub unsafe fn __cpp2rust_init_globals() {}
