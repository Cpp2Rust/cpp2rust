extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::Seek;
use std::io::{Read, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn my_alternative_fread_0(p: Ptr<u8>, n: u64, m: u64, f: AnyPtr) -> u64 {
    let p: Value<Ptr<u8>> = Rc::new(RefCell::new(p));
    let n: Value<u64> = Rc::new(RefCell::new(n));
    let m: Value<u64> = Rc::new(RefCell::new(m));
    let f: Value<AnyPtr> = Rc::new(RefCell::new(f));
    return 22_u64;
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let fn1: Value<FnPtr<fn(AnyPtr, u64, u64, Ptr<::std::fs::File>) -> u64>> =
        Rc::new(RefCell::new(fn_ptr!(
            rules::stdio_tgt_refcount::f5,
            fn(AnyPtr, u64, u64, Ptr<::std::fs::File>) -> u64
        )));
    assert!({
        let _lhs = (*fn1.borrow()).clone();
        _lhs == fn_ptr!(
            rules::stdio_tgt_refcount::f5,
            fn(AnyPtr, u64, u64, Ptr<::std::fs::File>) -> u64
        )
    });
    assert!(!((*fn1.borrow()).is_null()));
    let fn2: Value<FnPtr<fn(Ptr<u8>, u64, u64, AnyPtr) -> u64>> = Rc::new(RefCell::new(
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
        let _lhs = (*fn1.borrow()).clone();
        _lhs == ((*fn2.borrow()).cast::<fn(AnyPtr, u64, u64, Ptr<::std::fs::File>) -> u64>(None))
            .clone()
    });
    let f3: Value<FnPtr<fn(AnyPtr, u64, u64, Ptr<::std::fs::File>) -> u64>> =
        Rc::new(RefCell::new(
            fn_ptr!(
                my_alternative_fread_0,
                fn(Ptr::<u8>, u64, u64, AnyPtr) -> u64
            )
            .cast::<fn(AnyPtr, u64, u64, Ptr<::std::fs::File>) -> u64>(Some(
                (|a0: AnyPtr, a1: u64, a2: u64, a3: Ptr<::std::fs::File>| -> u64 {
                    my_alternative_fread_0(a0.cast::<u8>().unwrap(), a1, a2, a3.to_any())
                }) as fn(AnyPtr, u64, u64, Ptr<::std::fs::File>) -> u64,
            )),
        ));
    assert!(
        (({
            let _arg0: AnyPtr = Default::default();
            let _arg1: u64 = 0_u64;
            let _arg2: u64 = 0_u64;
            let _arg3: Ptr<::std::fs::File> = Default::default();
            (*(*f3.borrow()))(_arg0, _arg1, _arg2, _arg3)
        }) == 22_u64)
    );
    return 0;
}
