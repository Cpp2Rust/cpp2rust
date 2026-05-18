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
    let N: Value<i32> = Rc::new(RefCell::new(10000));
    let sentinel: Value<i32> = Rc::new(RefCell::new(((*N.borrow()) / 2)));
    let m: Value<BTreeMap<i32, Value<i32>>> = Rc::new(RefCell::new(BTreeMap::new()));
    let __rhs = (*sentinel.borrow());
    (m.as_pointer() as Ptr<BTreeMap<i32, Value<i32>>>)
        .with_mut(|__v: &mut BTreeMap<i32, Value<i32>>| {
            __v.entry((*sentinel.borrow()).clone())
                .or_insert_with(|| Rc::new(RefCell::new(<i32>::default())))
                .as_pointer()
        })
        .write(__rhs);
    let it: Value<RefcountMapIter<i32, i32>> = Rc::new(RefCell::new(RefcountMapIter::find_key(
        (m.as_pointer() as Ptr<BTreeMap<i32, Value<i32>>>),
        &(*sentinel.borrow()),
    )));
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new(((*it.borrow()).second().as_pointer())));
    assert!(
        ((*(*it.borrow()).second().borrow()) == (*sentinel.borrow()))
            && (!(Ptr::from_string_literal("iterator does not have correct value before insert"))
                .is_null())
    );
    assert!(
        ({
            let _lhs = ((*p.borrow()).read());
            _lhs == (*sentinel.borrow())
        }) && (!(Ptr::from_string_literal("pointer does not have correct value before insert"))
            .is_null())
    );
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*sentinel.borrow())) {
        let __rhs = (*i.borrow());
        (m.as_pointer() as Ptr<BTreeMap<i32, Value<i32>>>)
            .with_mut(|__v: &mut BTreeMap<i32, Value<i32>>| {
                __v.entry((*i.borrow()).clone())
                    .or_insert_with(|| Rc::new(RefCell::new(<i32>::default())))
                    .as_pointer()
            })
            .write(__rhs);
        (*i.borrow_mut()).prefix_inc();
    }
    let i: Value<i32> = Rc::new(RefCell::new(((*sentinel.borrow()) + 1)));
    'loop_: while ((*i.borrow()) <= (*N.borrow())) {
        let __rhs = (*i.borrow());
        (m.as_pointer() as Ptr<BTreeMap<i32, Value<i32>>>)
            .with_mut(|__v: &mut BTreeMap<i32, Value<i32>>| {
                __v.entry((*i.borrow()).clone())
                    .or_insert_with(|| Rc::new(RefCell::new(<i32>::default())))
                    .as_pointer()
            })
            .write(__rhs);
        (*i.borrow_mut()).prefix_inc();
    }
    assert!(
        ((*(*it.borrow()).second().borrow()) != 0)
            && (!(Ptr::from_string_literal(
                "in refcount, iterator points to index 0 instead of sentinel"
            ))
            .is_null())
    );
    assert!(
        ((*(*it.borrow()).second().borrow()) == (*sentinel.borrow()))
            && (!(Ptr::from_string_literal("iterator does not have correct value after insert"))
                .is_null())
    );
    assert!(
        ({
            let _lhs = ((*p.borrow()).read());
            _lhs == (*sentinel.borrow())
        }) && (!(Ptr::from_string_literal("pointer does not have correct value after insert"))
            .is_null())
    );
    (*(*it.borrow()).second().borrow_mut()) = 57005;
    assert!(
        (((m.as_pointer() as Ptr<BTreeMap<i32, Value<i32>>>)
            .with_mut(|__v: &mut BTreeMap<i32, Value<i32>>| {
                __v.entry((*sentinel.borrow()).clone())
                    .or_insert_with(|| Rc::new(RefCell::new(<i32>::default())))
                    .as_pointer()
            })
            .read())
            == 57005)
    );
    assert!((((*p.borrow()).read()) == 57005));
    assert!(((*m.borrow()).len() as u64 == ((((*N.borrow()) + 1) as u32) as u64)));
    let prev: Value<i32> = Rc::new(RefCell::new(-1_i32));
    'loop_: for pair in RefcountMapIter::begin(m.as_pointer()) {
        assert!({
            let _lhs = (*pair.first().borrow());
            _lhs > (*prev.borrow())
        });
        (*prev.borrow_mut()) = (*pair.first().borrow());
    }
    return 0;
}
