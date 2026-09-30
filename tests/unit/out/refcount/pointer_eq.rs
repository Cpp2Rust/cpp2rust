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
    let x: Value<i32> = Rc::new(RefCell::new(5));
    let p1: Value<Ptr<i32>> = Rc::new(RefCell::new((x.as_pointer())));
    let p2: Value<Ptr<i32>> = Rc::new(RefCell::new((x.as_pointer())));
    assert!(({ (*p1.borrow()).clone() } == { (*p2.borrow()).clone() }));
    let y: Value<i32> = Rc::new(RefCell::new(5));
    let p3: Value<Ptr<i32>> = Rc::new(RefCell::new((y.as_pointer())));
    assert!(({ (*p1.borrow()).clone() } != { (*p3.borrow()).clone() }));
    let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3])));
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new((arr.as_pointer() as Ptr<i32>)));
    assert!(({ (*p.borrow()).clone() } == { (arr.as_pointer() as Ptr::<i32>) }));
    assert!(
        ({ (*p.borrow()).offset((1) as isize) } == { ((arr.as_pointer() as Ptr<i32>).offset(1)) })
    );
    assert!(
        ({ (*p.borrow()).offset((2) as isize) } == { ((arr.as_pointer() as Ptr<i32>).offset(2)) })
    );
    let val: Value<i32> = Rc::new(RefCell::new(42));
    let orig: Value<Ptr<i32>> = Rc::new(RefCell::new((val.as_pointer())));
    let as_bytes: Value<Ptr<u8>> = Rc::new(RefCell::new((*orig.borrow()).reinterpret_cast::<u8>()));
    let back: Value<Ptr<i32>> =
        Rc::new(RefCell::new((*as_bytes.borrow()).reinterpret_cast::<i32>()));
    assert!(({ (*orig.borrow()).clone() } == { (*back.borrow()).clone() }));
    let arr_bytes: Value<Ptr<u8>> = Rc::new(RefCell::new(
        (arr.as_pointer() as Ptr<i32>).reinterpret_cast::<u8>(),
    ));
    let arr_back: Value<Ptr<i32>> = Rc::new(RefCell::new(
        (*arr_bytes.borrow()).reinterpret_cast::<i32>(),
    ));
    assert!(({ (*arr_back.borrow()).clone() } == { (arr.as_pointer() as Ptr::<i32>) }));
    assert!(
        ({ (*arr_back.borrow()).offset((1) as isize) } == {
            ((arr.as_pointer() as Ptr<i32>).offset(1))
        })
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
