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
        (*(*it.borrow()).iter_deref().1.borrow())
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
    (m.as_pointer() as Ptr<Map<i32, (Value<i32>, Value<i32>)>>)
        .with_mut(|__m: &mut Map<i32, (Value<i32>, Value<i32>)>| {
            __m.get_or_insert_with(0.clone(), || {
                (
                    Rc::new(RefCell::new(0)),
                    Rc::new(RefCell::new(<i32>::default())),
                )
            })
            .borrow()
            .1
            .as_pointer()
        })
        .write(1);
    let end: Value<BidirectionalIterator<(Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(
        (m.as_pointer() as Ptr<Map<i32, (Value<i32>, Value<i32>)>>)
            .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end()),
    ));
    let it0: Value<BidirectionalIterator<(Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(
        (m.as_pointer() as Ptr<Map<i32, (Value<i32>, Value<i32>)>>)
            .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.find(&0)),
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
    return 0;
}
pub fn __cpp2rust_init_globals() {}
