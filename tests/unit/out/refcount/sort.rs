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
    let v: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    {
        let __a1 = 10;
        (*v.borrow_mut()).push(__a1)
    };
    {
        let __a1 = 1;
        (*v.borrow_mut()).push(__a1)
    };
    {
        let __a1 = 9;
        (*v.borrow_mut()).push(__a1)
    };
    {
        let __a1 = 2;
        (*v.borrow_mut()).push(__a1)
    };
    {
        let __a1 = 8;
        (*v.borrow_mut()).push(__a1)
    };
    {
        let __a1 = 3;
        (*v.borrow_mut()).push(__a1)
    };
    {
        let __a1 = 7;
        (*v.borrow_mut()).push(__a1)
    };
    {
        let __a1 = 4;
        (*v.borrow_mut()).push(__a1)
    };
    {
        let __a1 = 5;
        (*v.borrow_mut()).push(__a1)
    };
    {
        let __a1 = 6;
        (*v.borrow_mut()).push(__a1)
    };
    (v.as_pointer() as Ptr<i32>).sort((v.as_pointer() as Ptr<i32>).to_end().get_offset());
    let mut i: u32 = 0_u32;
    'loop_: while ((i as usize) < ((*v.borrow()).len()).wrapping_sub(1_usize)) {
        assert!(
            ((elem!((v.as_pointer() as Ptr<i32>), (i as usize)).read())
                < (elem!(
                    (v.as_pointer() as Ptr<i32>),
                    (((i).wrapping_add(1_u32)) as usize)
                )
                .read()))
        );
        i.prefix_inc();
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
