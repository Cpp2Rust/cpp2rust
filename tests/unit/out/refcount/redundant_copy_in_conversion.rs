extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn sink_0(it: BidirectionalIterator<(Value<i32>, Value<i32>)>) -> i32 {
    let it: Value<BidirectionalIterator<(Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(it));
    let cit: Value<BidirectionalIterator<(Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(
        <BidirectionalIterator<(Value<i32>, Value<i32>)>>::from((*it.borrow()).clone()),
    ));
    return if (*cit.borrow())
        == <BidirectionalIterator<(Value<i32>, Value<i32>)>>::from((*it.borrow()).clone())
    {
        (*(*(*it.borrow()).iter_deref().upgrade().deref()).1.borrow())
    } else {
        0
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let m: Value<Map<i32, (Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(Map::new()));
    Map::subscript_move(
        m.as_pointer(),
        Rc::new(RefCell::new(0)).as_pointer(),
        |__k| cpp_move!(__k),
    )
    .write(1);
    let end: Value<BidirectionalIterator<(Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(
        m.as_pointer()
            .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end()),
    ));
    let it0: Value<BidirectionalIterator<(Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(
        ({
            let __tmp_0: Value<i32> = Rc::new(RefCell::new(0));
            m.as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.find(&__tmp_0.as_pointer()))
        }),
    ));
    let const_it: Value<BidirectionalIterator<(Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(
        <BidirectionalIterator<(Value<i32>, Value<i32>)>>::from((*it0.borrow()).clone()),
    ));
    let mut r: i32 = if (*const_it.borrow())
        == <BidirectionalIterator<(Value<i32>, Value<i32>)>>::from((*end.borrow()).clone())
    {
        0
    } else {
        1
    };
    r += ({ sink_0((*it0.borrow()).clone()) });
    r += if (*end.borrow()) == (*end.borrow()) {
        0
    } else {
        1
    };
    assert!((r == 2));
    let values: Value<Vec<u32>> = Rc::new(RefCell::new(Vec::from([7_u32, 8_u32, 9_u32])));
    let middle: Value<RandomAccessIterator<u32>> =
        Rc::new(RefCell::new(values.decay().begin().iter_add(1_isize)));
    let const_middle: Value<RandomAccessIterator<u32>> = Rc::new(RefCell::new(
        <RandomAccessIterator<u32>>::from((*middle.borrow()).clone()),
    ));
    assert!((((*const_middle.borrow()).iter_deref().read()) == 8_u32));
    assert!((*const_middle.borrow()) == (*middle.borrow()));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
