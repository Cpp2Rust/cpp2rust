extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::Seek;
use std::io::{Read, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let rfn: Value<FnPtr<fn(AnyPtr, u64, u64, Ptr<::std::fs::File>) -> u64>> =
        Rc::new(RefCell::new(fn_ptr!(
            rules::stdio_tgt_refcount::f5,
            fn(AnyPtr, u64, u64, Ptr<::std::fs::File>) -> u64
        )));
    assert!({
        let _lhs = (*rfn.borrow()).clone();
        _lhs == fn_ptr!(
            rules::stdio_tgt_refcount::f5,
            fn(AnyPtr, u64, u64, Ptr<::std::fs::File>) -> u64
        )
    });
    assert!(!((*rfn.borrow()).is_null()));
    let rfn2: Value<FnPtr<fn(Ptr<u8>, u64, u64, AnyPtr) -> u64>> = Rc::new(RefCell::new(
        fn_ptr!(
            rules::stdio_tgt_refcount::f5,
            fn(AnyPtr, u64, u64, Ptr<::std::fs::File>) -> u64
        )
        .cast::<fn(Ptr<u8>, u64, u64, AnyPtr) -> u64>(Some(
            (|a0: Ptr<u8>, a1: u64, a2: u64, a3: AnyPtr| -> u64 {
                rules::stdio_tgt_refcount::f5(
                    a0.to_any(),
                    a1,
                    a2,
                    a3.cast::<::std::fs::File>().unwrap(),
                )
            }) as fn(Ptr<u8>, u64, u64, AnyPtr) -> u64,
        )),
    ));
    assert!({
        let _lhs = (*rfn.borrow()).clone();
        _lhs == ((*rfn2.borrow()).cast::<fn(AnyPtr, u64, u64, Ptr<::std::fs::File>) -> u64>(None))
            .clone()
    });
    return 0;
}
