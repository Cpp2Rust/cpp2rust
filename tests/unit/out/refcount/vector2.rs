extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn fn_0(v: Ptr<Vec<i32>>, v3: Vec<i32>) {
    let v3: Value<Vec<i32>> = Rc::new(RefCell::new(v3));
    v.with_mut(|__v: &mut Vec<i32>| __v.push(20));
    let x: Value<i32> = <Value<i32>>::default();
    let v2: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    let v4: Value<Ptr<Vec<i32>>> = Rc::new(RefCell::new((v3.as_pointer())));
    (*v2.borrow_mut()).push(0);
    (*v2.borrow_mut()).push(1);
    (*v2.borrow_mut()).push(3);
    (*x.borrow_mut()) = ((v.to_strong().as_pointer() as Ptr<i32>)
        .offset(2_u64 as isize)
        .read());
    (v2.as_pointer() as Ptr<i32>)
        .offset(0_u64 as isize)
        .write(1);
    ((if true {
        v3.as_pointer()
    } else {
        v.to_strong().as_pointer()
    }) as Ptr<i32>)
        .offset(0_u64 as isize)
        .write(7);
    (v2.as_pointer() as Ptr<Vec<i32>>).write((*v.upgrade().deref()).clone());
    (((*v4.borrow()).to_strong().as_pointer()) as Ptr<i32>)
        .offset(1_u64 as isize)
        .write(13);
    assert!(((*x.borrow()) == 6));
    assert!((((v.to_strong().as_pointer() as Ptr<i32>).read()) == 4));
    assert!(
        (((v.to_strong().as_pointer() as Ptr<i32>)
            .offset(1_u64 as isize)
            .read())
            == 5)
    );
    assert!(
        (((v.to_strong().as_pointer() as Ptr<i32>)
            .offset(2_u64 as isize)
            .read())
            == 6)
    );
    assert!((((v.to_strong().as_pointer() as Ptr<i32>).to_last().read()) == 20));
    assert!((((v2.as_pointer() as Ptr<i32>).offset(0_u64 as isize).read()) == 4));
    assert!((((v2.as_pointer() as Ptr<i32>).offset(1_u64 as isize).read()) == 5));
    assert!((((v2.as_pointer() as Ptr<i32>).offset(2_u64 as isize).read()) == 6));
    assert!((((v3.as_pointer() as Ptr<i32>).offset(0_u64 as isize).read()) == 7));
    assert!((((v3.as_pointer() as Ptr<i32>).offset(1_u64 as isize).read()) == 13));
    v.with_mut(|__v: &mut Vec<i32>| __v.push(20));
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let v: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    let v2: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    (*v.borrow_mut()).push(4);
    (*v.borrow_mut()).push(5);
    (*v.borrow_mut()).push(6);
    (*v2.borrow_mut()).push(8);
    (*v2.borrow_mut()).push(9);
    ({
        let _v: Ptr<Vec<i32>> = v.as_pointer();
        let _v3: Vec<i32> = (*v2.borrow()).clone();
        fn_0(_v, _v3)
    });
    return 0;
}
