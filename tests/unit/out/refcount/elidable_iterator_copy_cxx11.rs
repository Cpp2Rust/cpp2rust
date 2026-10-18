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
        let __a1 = 4;
        (*v.borrow_mut()).push(__a1)
    };
    {
        let __a1 = 7;
        (*v.borrow_mut()).push(__a1)
    };
    let it: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new({
        let mut __it = RandomAccessIterator::Vec((v.as_pointer() as Ptr<i32>));
        while __it != RandomAccessIterator::Vec((v.as_pointer() as Ptr<i32>).to_end())
            && __it.iter_deref().with(|__v| *__v != 7)
        {
            __it.iter_inc();
        }
        __it
    }));
    assert!((*it.borrow()) != RandomAccessIterator::Vec((v.as_pointer() as Ptr<i32>).to_end()));
    assert!((((*it.borrow()).iter_deref().read()) == 7));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
