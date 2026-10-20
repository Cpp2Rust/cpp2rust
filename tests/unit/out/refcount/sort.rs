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
    {
        let __n = (v.as_pointer() as Ptr<i32>)
            .end()
            .iter_distance(&(v.as_pointer() as Ptr<i32>).begin());
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| {
            (v.as_pointer() as Ptr<i32>)
                .begin()
                .iter_at(__x)
                .with(|__a| {
                    (v.as_pointer() as Ptr<i32>)
                        .begin()
                        .iter_at(__y)
                        .with(|__b| __a.cmp(__b))
                })
        });
        reorder(__order, |__i| {
            (v.as_pointer() as Ptr<i32>).begin().iter_at(__i)
        });
    };
    let mut i: u32 = 0_u32;
    'loop_: while ((i as usize) < ((*v.borrow()).len()).wrapping_sub(1_usize)) {
        assert!(
            ({ (*v.borrow())[(i as usize)] } < {
                (*v.borrow())[(((i).wrapping_add(1_u32)) as usize)]
            })
        );
        i.prefix_inc();
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
