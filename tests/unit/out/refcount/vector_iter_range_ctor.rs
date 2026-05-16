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
    let src: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([1_u32, 2_u32, 3_u32])));
    let v1: Value<Vec<u32>> = Rc::new(RefCell::new({
        let mut __a0 = (src.as_pointer() as Ptr<u32>).clone();
        let mut __out = Vec::with_capacity(
            (src.as_pointer() as Ptr<u32>)
                .offset((3) as isize)
                .get_offset()
                - __a0.get_offset(),
        );
        while __a0 != (src.as_pointer() as Ptr<u32>).offset((3) as isize) {
            __out.push(u32::try_from(__a0.read()).ok().unwrap());
            __a0 += 1;
        }
        __out
    }));
    assert!(((*v1.borrow()).len() as u64 == 3_u64));
    assert!(
        ((((v1.as_pointer() as Ptr<u32>).offset(0_u64 as isize).read()) == 1_u32)
            && (((v1.as_pointer() as Ptr<u32>).offset(1_u64 as isize).read()) == 2_u32))
            && (((v1.as_pointer() as Ptr<u32>).offset(2_u64 as isize).read()) == 3_u32)
    );
    let v2: Value<Vec<u64>> = Rc::new(RefCell::new({
        let mut __a0 = (src.as_pointer() as Ptr<u32>).clone();
        let mut __out = Vec::with_capacity(
            (src.as_pointer() as Ptr<u32>)
                .offset((3) as isize)
                .get_offset()
                - __a0.get_offset(),
        );
        while __a0 != (src.as_pointer() as Ptr<u32>).offset((3) as isize) {
            __out.push(u64::try_from(__a0.read()).ok().unwrap());
            __a0 += 1;
        }
        __out
    }));
    assert!(((*v2.borrow()).len() as u64 == 3_u64));
    assert!(
        ((((v2.as_pointer() as Ptr<u64>).offset(0_u64 as isize).read()) == 1_u64)
            && (((v2.as_pointer() as Ptr<u64>).offset(1_u64 as isize).read()) == 2_u64))
            && (((v2.as_pointer() as Ptr<u64>).offset(2_u64 as isize).read()) == 3_u64)
    );
    let v3: Value<Vec<i32>> = Rc::new(RefCell::new({
        let mut __a0 = (src.as_pointer() as Ptr<u32>).clone();
        let mut __out = Vec::with_capacity(
            (src.as_pointer() as Ptr<u32>)
                .offset((3) as isize)
                .get_offset()
                - __a0.get_offset(),
        );
        while __a0 != (src.as_pointer() as Ptr<u32>).offset((3) as isize) {
            __out.push(i32::try_from(__a0.read()).ok().unwrap());
            __a0 += 1;
        }
        __out
    }));
    assert!(((*v3.borrow()).len() as u64 == 3_u64));
    assert!(
        ((((v3.as_pointer() as Ptr<i32>).offset(0_u64 as isize).read()) == 1)
            && (((v3.as_pointer() as Ptr<i32>).offset(1_u64 as isize).read()) == 2))
            && (((v3.as_pointer() as Ptr<i32>).offset(2_u64 as isize).read()) == 3)
    );
    return 0;
}
