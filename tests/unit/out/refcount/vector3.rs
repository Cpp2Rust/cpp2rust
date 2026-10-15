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
        elem!((v.as_pointer() as Ptr<Vec<i32>>), 0_usize)
            .with_mut(|__v: &mut Vec<i32>| __v.resize_with(__a0, || <i32>::default()))
    };
    {
        let __a0 = 1_usize as usize;
        elem!((v.as_pointer() as Ptr<Vec<i32>>), 1_usize)
            .with_mut(|__v: &mut Vec<i32>| __v.resize_with(__a0, || <i32>::default()))
    };
    index!(elem!((v.as_pointer() as Ptr<Vec<i32>>), 0_usize), 0_usize).write(1);
    index!(elem!((v.as_pointer() as Ptr<Vec<i32>>), 0_usize), 1_usize).write(5);
    index!(elem!((v.as_pointer() as Ptr<Vec<i32>>), 1_usize), 0_usize).write(6);
    'loop_: for mut v2 in v.as_pointer() as Ptr<Vec<i32>> {
        'loop_: for mut i in Ptr::<Vec<i32>>::decay(&(v2)) as Ptr<i32> {
            i.with_mut(|__v| __v.prefix_inc());
        }
    }
    'loop_: for mut v2 in v.as_pointer() as Ptr<Vec<i32>> {
        'loop_: for mut i in Ptr::<Vec<i32>>::decay(&(v2)) as Ptr<i32> {
            let mut i: i32 = i.read();
            println!("{}", (i + 3));
        }
    }
    'loop_: for mut v2 in v.as_pointer() as Ptr<Vec<i32>> {
        let v2: Value<Vec<i32>> = Rc::new(RefCell::new(v2.upgrade().deref().clone()));
        'loop_: for mut i in v2.as_pointer() as Ptr<i32> {
            let mut i: i32 = i.read();
            println!("{}", i);
        }
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
