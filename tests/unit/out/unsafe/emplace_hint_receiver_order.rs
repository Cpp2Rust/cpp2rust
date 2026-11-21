extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn receiver_0(
    mut m: *mut UnsafeMap<i32, (i32, Vec<i32>)>,
    mut v: *mut Vec<i32>,
    mut calls: *mut i32,
) -> *mut UnsafeMap<i32, (i32, Vec<i32>)> {
    (*calls).prefix_inc();
    (&mut (*v))[(2_usize)] = 9;
    return m;
}
pub unsafe fn hint_1(
    mut m: *mut UnsafeMap<i32, (i32, Vec<i32>)>,
    mut v: *mut Vec<i32>,
    mut calls: *mut i32,
) -> UnsafeBidirectionalIterator<(i32, Vec<i32>)> {
    (*calls).prefix_inc();
    (&mut (*v))[(2_usize)] = 7;
    return (*m).end();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    {
        let mut select_receiver: i32 = 1;
        'loop_: while ((select_receiver) >= (0)) {
            let mut m: UnsafeMap<i32, (i32, Vec<i32>)> = UnsafeMap::new();
            let mut v: Vec<i32> = Vec::from([1, 2, 3]);
            let mut a: i32 = 0;
            let mut b: i32 = 0;
            if (select_receiver != 0) {
                {
                    let mut __tmp_0 = {
                        (unsafe {
                            receiver_0(
                                (&mut m as *mut UnsafeMap<i32, (i32, Vec<i32>)>),
                                (&mut v as *mut Vec<i32>),
                                (&mut a as *mut i32),
                            )
                        })
                    };
                    let mut __tmp_1 = {
                        <UnsafeBidirectionalIterator<(i32, Vec<i32>)>>::from(
                            (unsafe {
                                hint_1(
                                    (&mut m as *mut UnsafeMap<i32, (i32, Vec<i32>)>),
                                    (&mut v as *mut Vec<i32>),
                                    (&mut b as *mut i32),
                                )
                            }),
                        )
                    };
                    let mut __tmp_2 = { (1, (v).clone()) };
                    let _ = (__tmp_1).clone();
                    (*__tmp_0).emplace(__tmp_2).0
                };
            } else {
                {
                    let _ = <UnsafeBidirectionalIterator<(i32, Vec<i32>)>>::from(
                        (unsafe {
                            hint_1(
                                (&mut m as *mut UnsafeMap<i32, (i32, Vec<i32>)>),
                                (&mut v as *mut Vec<i32>),
                                (&mut b as *mut i32),
                            )
                        }),
                    );
                    m.emplace((1, (v).clone())).0
                };
            }
            let inserted: *const Vec<i32> = &raw const (*m.get(&1).expect("out of range!")).1;
            (unsafe {
                libc::printf(
                    c"receiver=%d hint=%d source=%d inserted=%d\n".as_ptr() as *const libc::c_char,
                    (a),
                    (b),
                    (v[(2_usize)]),
                    ((&(*inserted))[(2_usize)]),
                )
            });
            select_receiver.prefix_dec();
        }
    }
    0
}
pub unsafe fn __cpp2rust_init_globals() {}
