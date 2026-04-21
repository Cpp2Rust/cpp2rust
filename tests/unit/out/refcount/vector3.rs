extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let v: Value<Vec<Value<Vec<i32>>>> = Rc::new(RefCell::new(Vec::new()));
    {
        let _a0 = 2_u64 as usize;
        (v.as_pointer() as Ptr<Vec<Value<Vec<i32>>>>).with_mut(|__v: &mut Vec<Value<Vec<i32>>>| {
            __v.resize_with(_a0, <Value<Vec<i32>>>::default)
        })
    };
    {
        let __a0 = 2_u64 as usize;
        (v.as_pointer() as Ptr<Value<Vec<i32>>>)
            .offset(0_u64 as isize)
            .with_mut(|__v: &mut Value<Vec<i32>>| {
                (*__v.borrow_mut()).resize_with(__a0, || <i32>::default())
            })
    };
    {
        let __a0 = 1_u64 as usize;
        (v.as_pointer() as Ptr<Value<Vec<i32>>>)
            .offset(1_u64 as isize)
            .with_mut(|__v: &mut Value<Vec<i32>>| {
                (*__v.borrow_mut()).resize_with(__a0, || <i32>::default())
            })
    };
    ((v.as_pointer() as Ptr<Value<Vec<i32>>>)
        .offset(0_u64 as isize)
        .upgrade()
        .deref()
        .as_pointer() as Ptr<i32>)
        .offset(0_u64 as isize)
        .write(1);
    ((v.as_pointer() as Ptr<Value<Vec<i32>>>)
        .offset(0_u64 as isize)
        .upgrade()
        .deref()
        .as_pointer() as Ptr<i32>)
        .offset(1_u64 as isize)
        .write(5);
    ((v.as_pointer() as Ptr<Value<Vec<i32>>>)
        .offset(1_u64 as isize)
        .upgrade()
        .deref()
        .as_pointer() as Ptr<i32>)
        .offset(0_u64 as isize)
        .write(6);
    'loop_: for mut v2 in v.as_pointer() as Ptr<Value<Vec<i32>>> {
        let v2: Ptr<Vec<i32>> = v2.upgrade().deref().as_pointer();
        'loop_: for mut i in v2.to_strong().as_pointer() as Ptr<i32> {
            i.with_mut(|__v| __v.prefix_inc());
        }
    }
    'loop_: for mut v2 in v.as_pointer() as Ptr<Value<Vec<i32>>> {
        let v2: Ptr<Vec<i32>> = v2.upgrade().deref().as_pointer();
        'loop_: for mut i in v2.to_strong().as_pointer() as Ptr<i32> {
            let i: Value<i32> = Rc::new(RefCell::new(i.read().clone()));
            println!("{}", ((*i.borrow()) + 3));
        }
    }
    'loop_: for mut v2 in v.as_pointer() as Ptr<Value<Vec<i32>>> {
        let v2: Value<Vec<i32>> = Rc::new(RefCell::new(v2.upgrade().deref().borrow().clone()));
        'loop_: for mut i in v2.as_pointer() as Ptr<i32> {
            let i: Value<i32> = Rc::new(RefCell::new(i.read().clone()));
            println!("{}", (*i.borrow()));
        }
    }
    return 0;
}
