extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn select_rows_0(mut rows: Ptr<Vec<Vec<i32>>>, mut calls: Ptr<i32>) -> Ptr<Vec<Vec<i32>>> {
    calls.with_mut(|__v| {
        __v.prefix_inc();
    });
    index!(index!(rows, 0_usize), 2_usize).write(9);
    return rows;
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    {
        let mut bound: i32 = 0;
        'loop_: while (bound < 2) {
            'for_body: {
                let rows: Value<Vec<Vec<i32>>> =
                    Rc::new(RefCell::new(Vec::from([Vec::from([1, 2, 3])])));
                rows.with_mut(|__b0| {
                    if 2_usize > (*__b0).capacity() as usize {
                        let len_0 = (*__b0).len();
                        (*__b0).reserve_exact(2_usize - len_0 as usize);
                    }
                });
                let calls: Value<i32> = Rc::new(RefCell::new(0));
                if (bound != 0) {
                    let mut receiver: Ptr<Vec<Vec<i32>>> =
                        ({ select_rows_0((rows.as_pointer()), (calls.as_pointer())) });
                    {
                        let mut __tmp_2 = {
                            {
                                let mut __tmp_0 = { rows.decay().offset(0_usize).decay() };
                                let mut __tmp_1 =
                                    { rows.decay().offset(0_usize).decay().offset((3) as isize) };
                                let mut __v = Vec::new();
                                let mut __it = (__tmp_0).clone();
                                while __it != __tmp_1 {
                                    __v.push((__it.iter_deref().read()));
                                    __it.iter_inc();
                                }
                                __v
                            }
                        };
                        receiver.with_mut(|__v: &mut Vec<Vec<i32>>| __v.push(__tmp_2))
                    };
                } else {
                    {
                        let mut __tmp_3 =
                            { ({ select_rows_0((rows.as_pointer()), (calls.as_pointer())) }) };
                        let mut __tmp_6 = {
                            {
                                let mut __tmp_4 = { rows.decay().offset(0_usize).decay() };
                                let mut __tmp_5 =
                                    { rows.decay().offset(0_usize).decay().offset((3) as isize) };
                                let mut __v = Vec::new();
                                let mut __it = (__tmp_4).clone();
                                while __it != __tmp_5 {
                                    __v.push((__it.iter_deref().read()));
                                    __it.iter_inc();
                                }
                                __v
                            }
                        };
                        __tmp_3.with_mut(|__v: &mut Vec<Vec<i32>>| __v.push(__tmp_6))
                    };
                }
                printf!(
                    b"source=%d inserted=%d calls=%d\n",
                    (index!(elem!(rows.decay(), 0_usize), 2_usize).read()),
                    (index!(elem!(rows.decay(), 1_usize), 2_usize).read()),
                    (*calls.borrow())
                );
            }
            bound.prefix_inc();
        }
    }
    0
}
pub fn __cpp2rust_init_globals() {}
