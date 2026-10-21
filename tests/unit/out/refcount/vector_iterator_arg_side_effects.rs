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
    let v: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([1, 2, 3, 4])));
    let it: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(v.decay().begin()));
    (*it.borrow_mut()).iter_inc();
    let r: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new({
        let __it = <RandomAccessIterator<i32>>::from((*it.borrow_mut()).iter_post_inc());
        let __off = __it.index();
        v.as_pointer()
            .with_mut(|__v: &mut Vec<i32>| __v.remove(__off));
        __it
    }));
    assert!((((*r.borrow()).iter_deref().read()) == 3));
    assert!(((*v.borrow()).len() == 3_usize));
    let it2: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(v.decay().begin()));
    (*r.borrow_mut()) = {
        let __it = <RandomAccessIterator<i32>>::from((*it2.borrow_mut()).iter_post_inc());
        let __off = __it.index();
        (*v.borrow_mut()).insert(__off, 9);
        __it
    };
    assert!(
        ((((*r.borrow()).iter_deref().read()) == 9) && ({ (*v.borrow())[0_usize] } == 9))
            && ((*v.borrow()).len() == 4_usize)
    );
    let c: Ptr<Vec<i32>> = v.as_pointer();
    let mut i: i32 = 1;
    assert!(
        (({
            let __idx = (i.postfix_inc() as usize);
            if __idx as usize >= (*c.upgrade().deref()).len() {
                panic!("out of bounds access")
            } else {
                c.decay().offset(__idx as isize)
            }
        }
        .read())
            == 1)
            && (i == 2)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
