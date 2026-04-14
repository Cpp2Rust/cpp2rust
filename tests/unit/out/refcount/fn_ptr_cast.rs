extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::Seek;
use std::io::{Read, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn double_it_0(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return ((*x.borrow()) * 2);
}
pub fn test_roundtrip_1() {
    let fn_: Value<FnPtr<fn(i32) -> i32>> =
        Rc::new(RefCell::new(fn_ptr!(double_it_0, fn(i32) -> i32)));
    assert!(
        (({
            let _arg0: i32 = 5;
            (*fn_.borrow()).call()(_arg0)
        }) == 10)
    );
    let gfn: Value<FnPtr<fn()>> =
        Rc::new(RefCell::new(((*fn_.borrow()).cast::<fn()>(None)).clone()));
    assert!(!((*gfn.borrow()).is_null()));
    let fn2: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(
        ((*gfn.borrow()).cast::<fn(i32) -> i32>(None)).clone(),
    ));
    assert!(
        (({
            let _arg0: i32 = 5;
            (*fn2.borrow()).call()(_arg0)
        }) == 10)
    );
    assert!({
        let _lhs = (*fn2.borrow()).clone();
        _lhs == (*fn_.borrow()).clone()
    });
}
pub fn test_double_cast_2() {
    let fn_: Value<FnPtr<fn(i32) -> i32>> =
        Rc::new(RefCell::new(fn_ptr!(double_it_0, fn(i32) -> i32)));
    let fn2: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(
        ((*fn_.borrow())
            .cast::<fn()>(None)
            .cast::<fn(i32) -> i32>(None))
        .clone(),
    ));
    assert!(
        (({
            let _arg0: i32 = 5;
            (*fn2.borrow()).call()(_arg0)
        }) == 10)
    );
    assert!({
        let _lhs = (*fn2.borrow()).clone();
        _lhs == (*fn_.borrow()).clone()
    });
}
#[derive(Default)]
pub struct Command {
    pub data: Value<AnyPtr>,
}
impl Clone for Command {
    fn clone(&self) -> Self {
        let mut this = Self {
            data: Rc::new(RefCell::new((*self.data.borrow()).clone())),
        };
        this
    }
}
impl ByteRepr for Command {}
pub fn test_void_ptr_to_fn_3() {
    let cmd: Value<Command> = Rc::new(RefCell::new(<Command>::default()));
    (*(*cmd.borrow()).data.borrow_mut()) = fn_ptr!(double_it_0, fn(i32) -> i32).to_any();
    let fn_: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(
        ((*(*cmd.borrow()).data.borrow())
            .cast_fn::<fn(i32) -> i32>()
            .expect("ub:wrong fn type"))
        .clone(),
    ));
    assert!(
        (({
            let _arg0: i32 = 5;
            (*fn_.borrow()).call()(_arg0)
        }) == 10)
    );
}
pub fn add_offset_4(base: Ptr<i32>, offset: i32) -> i32 {
    let base: Value<Ptr<i32>> = Rc::new(RefCell::new(base));
    let offset: Value<i32> = Rc::new(RefCell::new(offset));
    return {
        let _lhs = ((*base.borrow()).read());
        _lhs + (*offset.borrow())
    };
}
pub fn test_call_through_cast_5() {
    let gfn: Value<FnPtr<fn(AnyPtr, i32) -> i32>> = Rc::new(RefCell::new(
        fn_ptr!(add_offset_4, fn(Ptr::<i32>, i32) -> i32).cast::<fn(AnyPtr, i32) -> i32>(Some(
            (|a0: AnyPtr, a1: i32| -> i32 { add_offset_4(a0.cast::<i32>().unwrap(), a1) })
                as fn(AnyPtr, i32) -> i32,
        )),
    ));
    let val: Value<i32> = Rc::new(RefCell::new(100));
    let result: Value<i32> = Rc::new(RefCell::new(
        ({
            let _arg0: AnyPtr = ((val.as_pointer()) as Ptr<i32>).to_any();
            let _arg1: i32 = 42;
            (*gfn.borrow()).call()(_arg0, _arg1)
        }),
    ));
    assert!(((*result.borrow()) == 142));
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test_roundtrip_1() });
    ({ test_double_cast_2() });
    ({ test_void_ptr_to_fn_3() });
    ({ test_call_through_cast_5() });
    return 0;
}
