extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() {
    let source: Value<(Value<Option<Value<i32>>>, Value<i32>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(Ptr::alloc(1).to_owned_opt())),
        Rc::new(RefCell::new(2)),
    )));
    let pairs: Value<Vec<(Value<Option<Value<i32>>>, Value<i32>)>> =
        Rc::new(RefCell::new(Vec::new()));
    {
        let mut __tmp_0 = {
            <RandomAccessIterator<(Value<Option<Value<i32>>>, Value<i32>)>>::from(
                pairs.decay().end(),
            )
        };
        (*pairs.borrow_mut()).insert(__tmp_0.index(), cpp_move!(source.as_pointer()));
        (__tmp_0).clone()
    };
    assert!(((({ (*(*pairs.borrow())[0_usize].0.borrow()).as_pointer() }).read()) == 1));
    assert!(({ (*(*source.borrow()).0.borrow()).is_none() }));
    assert!(({ ((*(*source.borrow()).1.borrow()) == 2) }));
}
pub fn test_1() {
    let source: Value<(Value<Option<Value<i32>>>, Value<i32>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(Ptr::alloc(1).to_owned_opt())),
        Rc::new(RefCell::new(2)),
    )));
    let pairs: Value<Vec<(Value<Option<Value<i32>>>, Value<i32>)>> =
        Rc::new(RefCell::new(Vec::new()));
    {
        let __a1 = ({ std::mem::take(&mut (*source.borrow_mut())) });
        (*pairs.borrow_mut()).push(__a1)
    };
    assert!(((({ (*(*pairs.borrow())[0_usize].0.borrow()).as_pointer() }).read()) == 1));
    assert!(({ (*(*source.borrow()).0.borrow()).is_none() }));
    assert!(({ ((*(*source.borrow()).1.borrow()) == 2) }));
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    ({ test_0() });
    ({ test_1() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
