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
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<Map<i32, (Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(Map::new()));
    match (Rc::new(RefCell::new(1)).as_pointer(),) {
        (__tmp_0,) => ({ (*a.borrow_mut()).subscript_move(__tmp_0, |__k| cpp_move!(__k)) }),
    }
    .write(10);
    let b: Value<Map<i32, (Value<i32>, Value<i32>)>> = Rc::new(RefCell::new((*a.borrow()).clone()));
    match (Rc::new(RefCell::new(1)).as_pointer(),) {
        (__tmp_1,) => ({ (*b.borrow_mut()).subscript_move(__tmp_1, |__k| cpp_move!(__k)) }),
    }
    .write(20);
    assert!(
        ((match (Rc::new(RefCell::new(1)).as_pointer(),) {
            (__tmp_2,) => {
                ({ (*a.borrow_mut()).subscript_move(__tmp_2, |__k| cpp_move!(__k)) })
            }
        }
        .read())
            == 10)
    );
    let x: Value<Map<i32, (Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(Map::new()));
    match (Rc::new(RefCell::new(2)).as_pointer(),) {
        (__tmp_3,) => ({ (*x.borrow_mut()).subscript_move(__tmp_3, |__k| cpp_move!(__k)) }),
    }
    .write(3);
    let y: Value<Map<i32, (Value<i32>, Value<i32>)>> = Rc::new(RefCell::new((*x.borrow()).clone()));
    match (Rc::new(RefCell::new(2)).as_pointer(),) {
        (__tmp_4,) => ({ (*y.borrow_mut()).subscript_move(__tmp_4, |__k| cpp_move!(__k)) }),
    }
    .write(4);
    assert!(
        ((match (Rc::new(RefCell::new(2)).as_pointer(),) {
            (__tmp_5,) => {
                ({ (*x.borrow_mut()).subscript_move(__tmp_5, |__k| cpp_move!(__k)) })
            }
        }
        .read())
            == 3)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
