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
    return rows;
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let rows: Value<Vec<Vec<i32>>> = Rc::new(RefCell::new(Vec::from([Vec::from([1, 2, 3])])));
    rows.with_mut(|__b0| {
        if 2_usize > (*__b0).capacity() as usize {
            let len_0 = (*__b0).len();
            (*__b0).reserve_exact(2_usize - len_0 as usize);
        }
    });
    let calls: Value<i32> = Rc::new(RefCell::new(0));
    let mut alias: Ptr<Vec<Vec<i32>>> = (rows.as_pointer());
    {
        let mut __tmp_0 = { ({ select_rows_0((alias).clone(), (calls.as_pointer())) }) };
        let mut __tmp_3 = {
            {
                let mut __tmp_1 = { (alias).decay().offset(0_usize).decay() };
                let mut __tmp_2 = { (alias).decay().offset(0_usize).decay().offset((3) as isize) };
                let mut __v = Vec::new();
                let mut __it = (__tmp_1).clone();
                while __it != __tmp_2 {
                    __v.push((__it.iter_deref().read()));
                    __it.iter_inc();
                }
                __v
            }
        };
        __tmp_0.with_mut(|__v: &mut Vec<Vec<i32>>| __v.push(__tmp_3))
    };
    printf!(
        b"calls=%d size=%zu last=%d\n",
        (*calls.borrow()),
        (*rows.borrow()).len(),
        (index!(elem!(rows.decay(), 1_usize), 2_usize).read())
    );
    0
}
pub fn __cpp2rust_init_globals() {}
