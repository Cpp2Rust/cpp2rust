extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct p11_defaulted_move_constructor_is_trivial_MoveOnly {
    #[offset(0)]
    pub value: i32,
}
impl p11_defaulted_move_constructor_is_trivial_MoveOnly {
    pub fn new(mut value: i32) -> Self {
        Self { value: value }
    }
    pub fn move_from(_a0: Ptr<p11_defaulted_move_constructor_is_trivial_MoveOnly>) -> Self {
        Self {
            value: { (*_a0.upgrade().deref()).value },
        }
    }
}
pub fn pass_0(
    object: p11_defaulted_move_constructor_is_trivial_MoveOnly,
) -> p11_defaulted_move_constructor_is_trivial_MoveOnly {
    let object: Value<p11_defaulted_move_constructor_is_trivial_MoveOnly> =
        Rc::new(RefCell::new(object));
    return p11_defaulted_move_constructor_is_trivial_MoveOnly::move_from({ object.as_pointer() });
}
pub fn test_1() {
    let mut result: p11_defaulted_move_constructor_is_trivial_MoveOnly = ({
        pass_0(p11_defaulted_move_constructor_is_trivial_MoveOnly::new({
            7
        }))
    });
    assert!((result.value == 7));
}
pub fn read_by_value_2(it: RandomAccessIterator<i32>) -> i32 {
    let it: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(it));
    return ((*it.borrow()).iter_deref().read());
}
pub fn test_3() {
    let source: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([1, 2, 3])));
    let moved: Value<Vec<i32>> = Rc::new(RefCell::new((*source.borrow_mut()).clone()));
    assert!(({ ((*moved.borrow()).len() == 3_usize) }));
    assert!(({ ((*source.borrow()).len() == 3_usize) }));
    let values: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([7, 8])));
    let it: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(values.decay().begin()));
    assert!(({ (({ read_by_value_2((*it.borrow()).clone(),) }) == 7) }));
    assert!(({ (((*it.borrow()).iter_deref().read()) == 7) }));
    let moved_it: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new((*it.borrow()).clone()));
    assert!(({ (((*moved_it.borrow()).iter_deref().read()) == 7) }));
    assert!(({ (((*it.borrow()).iter_deref().read()) == 7) }));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test_1() });
    ({ test_3() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
