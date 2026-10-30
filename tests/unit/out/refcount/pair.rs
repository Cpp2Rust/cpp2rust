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
    let p1: Value<(Value<f64>, Value<i32>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new((1 as f64))),
        Rc::new(RefCell::new(2)),
    )));
    assert!(((*(*p1.borrow()).0.borrow()) == 1_f64));
    assert!(((*(*p1.borrow()).1.borrow()) == 2));
    let p2: Value<(Value<f64>, Value<i32>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new((3 as f64))),
        Rc::new(RefCell::new(4)),
    )));
    assert!(((*(*p2.borrow()).0.borrow()) == 3_f64));
    assert!(((*(*p2.borrow()).1.borrow()) == 4));
    let p3: Value<(Value<f64>, Value<i32>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(5)),
        Rc::new(RefCell::new(6)),
    )));
    assert!(((*(*p3.borrow()).0.borrow()) == 5_f64));
    assert!(((*(*p3.borrow()).1.borrow()) == 6));
    let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2])));
    let mut n: u32 = 2_u32;
    let const_ptr_pairs: Value<Vec<(Value<Ptr<i32>>, Value<u64>)>> = Rc::new(RefCell::new(
        (0..(1_usize) as usize)
            .map(|_| {
                (
                    Rc::new(RefCell::new(Ptr::<i32>::null())),
                    Rc::new(RefCell::new(0_u64)),
                )
            })
            .collect::<Vec<_>>(),
    ));
    (*const_ptr_pairs.borrow_mut())[0_usize] = (
        Rc::new(RefCell::new(
            (arr.as_pointer() as Ptr<i32>).offset((1) as isize),
        )),
        Rc::new(RefCell::new(n)),
    );
    assert!(
        (((*(*const_ptr_pairs.borrow())[0_usize].0.borrow()).read()) == 2)
            && ((*(*const_ptr_pairs.borrow())[0_usize].1.borrow()) == 2_u64)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
