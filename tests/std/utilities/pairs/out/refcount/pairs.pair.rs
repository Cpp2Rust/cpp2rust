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
    let target: Value<(Value<Option<Value<i32>>>, Value<i32>)> = Rc::new(RefCell::new({
        let __first: Ptr<Option<Value<i32>>> = (*source.borrow()).0.as_pointer();
        let __second: Ptr<i32> = (*source.borrow()).1.as_pointer();
        (
            Rc::new(RefCell::new(cpp_move!(__first))),
            Rc::new(RefCell::new(cpp_move!(__second))),
        )
    }));
    assert!(({ (((*(*target.borrow()).0.borrow()).as_pointer().read()) == 1) }));
    assert!(({ ((*(*target.borrow()).1.borrow()) == 2) }));
    assert!(({ (*(*source.borrow()).0.borrow()).is_none() }));
    assert!(({ ((*(*source.borrow()).1.borrow()) == 2) }));
}
pub fn test_1() {
    let owner: Value<Option<Value<i32>>> = Rc::new(RefCell::new(Ptr::alloc(3).to_owned_opt()));
    let moved: Value<(Value<Option<Value<i32>>>, Value<i32>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(cpp_move!(owner.as_pointer()))),
        Rc::new(RefCell::new(1)),
    )));
    assert!(({ ((*owner.borrow()).as_pointer()).is_null() }));
    assert!(({ (((*(*moved.borrow()).0.borrow()).as_pointer().read()) == 3) }));
    assert!(({ ((*(*moved.borrow()).1.borrow()) == 1) }));
    let text: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = Ptr::<i8>::from_string_literal(b"abc").to_c_bytes();
        __bytes.push(0);
        __bytes
    }));
    let moved_text: Value<(Value<Vec<i8>>, Value<i32>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(
            text.as_pointer()
                .with_mut(|__v| std::mem::replace(__v, vec![0])),
        )),
        Rc::new(RefCell::new(2)),
    )));
    assert!(
        ({
            Ptr::<i8>::from_string_literal(b"abc").with_c_str(|__s| {
                (*(*moved_text.borrow()).0.borrow())
                    [..(*(*moved_text.borrow()).0.borrow()).len().saturating_sub(1)]
                    == *__s
            })
        })
    );
    let mut values: Vec<i32> = vec![7; 2_usize];
    let copied: Value<(Value<Vec<i32>>, Value<i32>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new((values).clone())),
        Rc::new(RefCell::new(3)),
    )));
    assert!((values.len() == 2_usize));
    assert!(
        ({
            ((*(*copied.borrow()).0.borrow()).len() == 2_usize)
                && ((elem!((*copied.borrow()).0.decay(), 1_usize).read()) == 7)
        })
    );
    let referent: Value<i32> = Rc::new(RefCell::new(5));
    let bound: Value<(Value<Ptr<i32>>, Value<i8>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(referent.as_pointer())),
        Rc::new(RefCell::new(('c' as i8))),
    )));
    (*referent.borrow_mut()) = 6;
    assert!(({ (((*bound.borrow()).0.borrow().clone().read()) == 6) }));
    assert!(
        ({ ({ ((*bound.borrow()).0.borrow().clone()).clone() } == { (referent.as_pointer()) }) })
    );
    assert!(({ (((*(*bound.borrow()).1.borrow()) as i32) == (('c' as i8) as i32)) }));
}
pub fn test_2() {
    let source: Value<(Value<i32>, Value<Option<Value<i32>>>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(0)),
        Rc::new(RefCell::new(Ptr::alloc(81).to_owned_opt())),
    )));
    let mut original: Ptr<i32> = (*(*source.borrow()).1.borrow()).as_pointer();
    let converted: Value<(Value<i64>, Value<Option<Value<i32>>>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(
            (((*source.borrow()).0.as_pointer().read()) as i64),
        )),
        Rc::new(RefCell::new(cpp_move!((*source.borrow()).1.as_pointer()))),
    )));
    assert!(({ ((*(*converted.borrow()).0.borrow()) == 0_i64) }));
    assert!(({ ({ (*(*converted.borrow()).1.borrow()).as_pointer() } == { (original).clone() }) }));
    assert!(({ (((*(*converted.borrow()).1.borrow()).as_pointer().read()) == 81) }));
    assert!(({ ((*(*source.borrow()).1.borrow()).as_pointer()).is_null() }));
}
pub fn test_3() {
    let source: Value<(Value<i32>, Value<Option<Value<i32>>>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(4)),
        Rc::new(RefCell::new(Ptr::alloc(9).to_owned_opt())),
    )));
    let mut original: Ptr<i32> = (*(*source.borrow()).1.borrow()).as_pointer();
    let target: Value<(Value<i64>, Value<Option<Value<i32>>>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(0_i64)),
        Rc::new(RefCell::new(None)),
    )));
    {
        let __first = (((*source.borrow()).0.as_pointer().read()) as i64);
        let __second = cpp_move!((*source.borrow()).1.as_pointer());
        target.as_pointer().with(|__dst| {
            *__dst.0.borrow_mut() = __first;
            *__dst.1.borrow_mut() = __second;
        });
        target.as_pointer()
    };
    assert!(({ ((*(*target.borrow()).0.borrow()) == 4_i64) }));
    assert!(({ ({ (*(*target.borrow()).1.borrow()).as_pointer() } == { (original).clone() }) }));
    assert!(({ (((*(*target.borrow()).1.borrow()).as_pointer().read()) == 9) }));
    assert!(({ ((*(*source.borrow()).1.borrow()).as_pointer()).is_null() }));
}
pub fn test_4() {
    let a: Value<(Value<i32>, Value<i32>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(1)),
        Rc::new(RefCell::new(2)),
    )));
    let b: Value<(Value<i32>, Value<i32>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(1)),
        Rc::new(RefCell::new(3)),
    )));
    let c: Value<(Value<i32>, Value<i32>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(2)),
        Rc::new(RefCell::new(0)),
    )));
    assert!((*a.borrow()) != (*b.borrow()));
    assert!(({ !((*a.borrow()) != (*a.borrow())) }));
    assert!((*a.borrow()) < (*b.borrow()));
    assert!(({ !((*b.borrow()) < (*a.borrow())) }));
    assert!((*b.borrow()) < (*c.borrow()));
    assert!((*a.borrow()) <= (*a.borrow()));
    assert!(({ !((*c.borrow()) <= (*b.borrow())) }));
    assert!((*c.borrow()) > (*b.borrow()));
    assert!(({ !((*a.borrow()) > (*a.borrow())) }));
    assert!((*c.borrow()) >= (*a.borrow()));
    assert!(({ !((*a.borrow()) >= (*b.borrow())) }));
    let mut ordered: bool = ((*a.borrow()) < (*b.borrow())) && ((*b.borrow()) < (*c.borrow()));
    assert!(ordered);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test_0() });
    ({ test_1() });
    ({ test_2() });
    ({ test_3() });
    ({ test_4() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
