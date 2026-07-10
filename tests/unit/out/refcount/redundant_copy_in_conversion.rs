extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn sink_0(it: RefcountMapIter<i32, i32>) -> i32 {
    let it: Value<RefcountMapIter<i32, i32>> = Rc::new(RefCell::new(it));
    let cit: Value<RefcountMapIter<i32, i32>> = Rc::new(RefCell::new((*it.borrow()).clone()));
    return if (*cit.borrow()) == (*it.borrow()).clone() {
        (*(*it.borrow()).second().borrow())
    } else {
        0
    };
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let m: Value<BTreeMap<i32, Value<i32>>> = Rc::new(RefCell::new(BTreeMap::new()));
    (m.as_pointer() as Ptr<BTreeMap<i32, Value<i32>>>)
        .with_mut(|__v: &mut BTreeMap<i32, Value<i32>>| {
            __v.entry(0.clone())
                .or_insert_with(|| Rc::new(RefCell::new(<i32>::default())))
                .as_pointer()
        })
        .write(1);
    let end: Value<RefcountMapIter<i32, i32>> = Rc::new(RefCell::new(RefcountMapIter::end(
        (m.as_pointer() as Ptr<BTreeMap<i32, Value<i32>>>),
    )));
    let it0: Value<RefcountMapIter<i32, i32>> = Rc::new(RefCell::new(RefcountMapIter::find_key(
        (m.as_pointer() as Ptr<BTreeMap<i32, Value<i32>>>),
        &0,
    )));
    let const_it: Value<RefcountMapIter<i32, i32>> = Rc::new(RefCell::new((*it0.borrow()).clone()));
    let r: Value<i32> = Rc::new(RefCell::new(
        if (*const_it.borrow()) == (*end.borrow()).clone() {
            0
        } else {
            1
        },
    ));
    (*r.borrow_mut()) += ({ sink_0((*it0.borrow()).clone()) });
    (*r.borrow_mut()) += if (*end.borrow()) == (*end.borrow()) {
        0
    } else {
        1
    };
    return (*r.borrow());
}
