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
    let mut N: i32 = 10000;
    let sentinel: Value<i32> = Rc::new(RefCell::new((N / 2)));
    let m: Value<Map<i32, (Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(Map::new()));
    let __rhs = (*sentinel.borrow());
    Map::subscript(m.as_pointer(), sentinel.as_pointer()).write(__rhs);
    let it: Value<BidirectionalIterator<(Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(
        m.as_pointer()
            .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.find(&sentinel.as_pointer())),
    ));
    let mut p: Ptr<i32> = ((*(*it.borrow()).iter_deref().upgrade().deref())
        .1
        .as_pointer());
    assert!(
        ((*(*(*it.borrow()).iter_deref().upgrade().deref()).1.borrow()) == (*sentinel.borrow()))
            && (!(Ptr::<i8>::from_string_literal(
                b"iterator does not have correct value before insert"
            ))
            .is_null())
    );
    assert!(
        ({ (p.read()) } == { (*sentinel.borrow()) })
            && (!(Ptr::<i8>::from_string_literal(
                b"pointer does not have correct value before insert"
            ))
            .is_null())
    );
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*sentinel.borrow())) {
        let __rhs = (*i.borrow());
        Map::subscript(m.as_pointer(), i.as_pointer()).write(__rhs);
        (*i.borrow_mut()).prefix_inc();
    }
    let i: Value<i32> = Rc::new(RefCell::new(((*sentinel.borrow()) + 1)));
    'loop_: while ((*i.borrow()) <= N) {
        let __rhs = (*i.borrow());
        Map::subscript(m.as_pointer(), i.as_pointer()).write(__rhs);
        (*i.borrow_mut()).prefix_inc();
    }
    assert!(
        ((*(*(*it.borrow()).iter_deref().upgrade().deref()).1.borrow()) != 0)
            && (!(Ptr::<i8>::from_string_literal(
                b"in refcount, iterator points to index 0 instead of sentinel"
            ))
            .is_null())
    );
    assert!(
        ((*(*(*it.borrow()).iter_deref().upgrade().deref()).1.borrow()) == (*sentinel.borrow()))
            && (!(Ptr::<i8>::from_string_literal(
                b"iterator does not have correct value after insert"
            ))
            .is_null())
    );
    assert!(
        ({ (p.read()) } == { (*sentinel.borrow()) })
            && (!(Ptr::<i8>::from_string_literal(
                b"pointer does not have correct value after insert"
            ))
            .is_null())
    );
    (*(*(*it.borrow()).iter_deref().upgrade().deref())
        .1
        .borrow_mut()) = 57005;
    assert!(((Map::subscript(m.as_pointer(), sentinel.as_pointer()).read()) == 57005));
    assert!(((p.read()) == 57005));
    assert!(((*m.borrow()).len() == (((N + 1) as u32) as usize)));
    let mut prev: i32 = -1_i32;
    'loop_: for pair in IterRange::new(
        m.as_pointer()
            .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin()),
        m.as_pointer()
            .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end()),
    ) {
        let pair: Ptr<(Value<i32>, Value<i32>)> = pair.iter_deref();
        assert!(({ (*(*pair.upgrade().deref()).0.borrow()) } > { prev }));
        prev = (*(*pair.upgrade().deref()).0.borrow());
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
