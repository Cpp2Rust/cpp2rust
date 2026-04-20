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
    let x: Value<i32> = Rc::new(RefCell::new(1));
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new((x.as_pointer())));
    {
        let __ptr = (*p.borrow()).clone();
        let __tmp = __ptr.read() + 1;
        __ptr.write(__tmp)
    };
    if ((*x.borrow()) == 2) {
        let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2])));
        (*p.borrow_mut()) = ((a.as_pointer() as Ptr<i32>).offset(1 as isize));
        {
            let __ptr = (*p.borrow()).clone();
            let __tmp = __ptr.read() + 1;
            __ptr.write(__tmp)
        };
        if (((*a.borrow())[(0) as usize] == 1) && ((*a.borrow())[(1) as usize] == 3)) {
            (*p.borrow_mut()).prefix_dec();
            {
                let __ptr = (*p.borrow()).clone();
                let __tmp = __ptr.read() + 1;
                __ptr.write(__tmp)
            };
            if (((*a.borrow())[(0) as usize] == 2) && ((*a.borrow())[(1) as usize] == 3)) {
                (*p.borrow_mut()) = (x.as_pointer());
                {
                    let __ptr = (*p.borrow()).clone();
                    let __tmp = __ptr.read() + 1;
                    __ptr.write(__tmp)
                };
                if ((*x.borrow()) == 3) {
                    let p2: Value<Ptr<i32>> = Rc::new(RefCell::new((*p.borrow()).clone()));
                    {
                        let __ptr = (*p2.borrow()).clone();
                        let __tmp = __ptr.read() + 1;
                        __ptr.write(__tmp)
                    };
                    return (((*x.borrow()) == 4) as i32);
                }
            }
        }
    }
    return -1_i32;
}
