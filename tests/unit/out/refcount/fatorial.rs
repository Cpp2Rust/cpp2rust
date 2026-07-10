extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn fatorial_0(n: i32) -> i32 {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    if ((*n.borrow()) == 0) {
        return 1;
    }
    return ((*n.borrow()) * ({ fatorial_0(((*n.borrow()) - 1)) }));
}
pub fn fatorial_by_ref_1(n: Ptr<i32>) {
    if ((n.read()) == 1) {
        {
            let _ptr = n.clone();
            _ptr.write(_ptr.read() * 1)
        };
        return;
    }
    let n_1: Value<i32> = Rc::new(RefCell::new(((n.read()) - 1)));
    ({ fatorial_by_ref_1(n_1.as_pointer()) });
    let __rhs = (*n_1.borrow());
    {
        let _ptr = n.clone();
        _ptr.write(_ptr.read() * __rhs)
    };
}
pub fn fatorial_by_ptr_2(n: Ptr<i32>) {
    let n: Value<Ptr<i32>> = Rc::new(RefCell::new(n));
    if (((*n.borrow()).read()) == 1) {
        {
            let _ptr = (*n.borrow()).clone();
            _ptr.write(_ptr.read() * 1)
        };
        return;
    }
    let n_1: Value<i32> = Rc::new(RefCell::new((((*n.borrow()).read()) - 1)));
    ({ fatorial_by_ptr_2((n_1.as_pointer())) });
    let __rhs = (*n_1.borrow());
    {
        let _ptr = (*n.borrow()).clone();
        _ptr.write(_ptr.read() * __rhs)
    };
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let n1: Value<i32> = Rc::new(RefCell::new(2));
    ({ fatorial_by_ptr_2((n1.as_pointer())) });
    let n: Value<i32> = Rc::new(RefCell::new(((*n1.borrow()) + 1)));
    ({ fatorial_by_ref_1(n.as_pointer()) });
    return ({ fatorial_0((*n.borrow())) });
}
