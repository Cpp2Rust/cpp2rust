extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let v: Value<Vec<Vec<i32>>> = Rc::new(RefCell::new(Vec::new()));
    {
        let __a0 = 2_usize as usize;
        (*v.borrow_mut()).resize_with(__a0, || <Vec<i32>>::default())
    };
    {
        let __a0 = 2_usize as usize;
        (*v.borrow_mut())[0_usize].resize_with(__a0, || <i32>::default())
    };
    {
        let __a0 = 1_usize as usize;
        (*v.borrow_mut())[1_usize].resize_with(__a0, || <i32>::default())
    };
    elem!(v.as_pointer().decay().offset(0_usize), 0_usize).write((1 as i32));
    elem!(v.as_pointer().decay().offset(0_usize), 1_usize).write((5 as i32));
    elem!(v.as_pointer().decay().offset(1_usize), 0_usize).write((6 as i32));
    'loop_: for v2 in IterRange::new(v.as_pointer().decay().begin(), v.as_pointer().decay().end()) {
        let v2: Ptr<Vec<i32>> = v2.iter_deref();
        'loop_: for i in IterRange::new(v2.decay().begin(), v2.decay().end()) {
            let i: Ptr<i32> = i.iter_deref();
            i.with_mut(|__v| __v.prefix_inc());
        }
    }
    'loop_: for v2 in IterRange::new(v.as_pointer().decay().begin(), v.as_pointer().decay().end()) {
        let v2: Ptr<Vec<i32>> = v2.iter_deref();
        'loop_: for i in IterRange::new(v2.decay().begin(), v2.decay().end()) {
            let mut i: i32 = (i.iter_deref().read());
            println!("{}", (i + 3));
        }
    }
    'loop_: for v2 in IterRange::new(v.as_pointer().decay().begin(), v.as_pointer().decay().end()) {
        let v2: Value<Vec<i32>> =
            Rc::new(RefCell::new((*v2.iter_deref().upgrade().deref()).clone()));
        'loop_: for i in IterRange::new(
            v2.as_pointer().decay().begin(),
            v2.as_pointer().decay().end(),
        ) {
            let mut i: i32 = (i.iter_deref().read());
            println!("{}", i);
        }
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
