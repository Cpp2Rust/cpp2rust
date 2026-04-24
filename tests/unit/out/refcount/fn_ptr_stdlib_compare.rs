extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
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
        Rc::new(RefCell::new(FnPtr::<
            fn(AnyPtr, u64, u64, Ptr<::std::fs::File>) -> u64,
        >::new(libcc2rs::fread_refcount)));
    assert!({
        let _lhs = (*fn1.borrow()).clone();
        _lhs == FnPtr::<fn(AnyPtr, u64, u64, Ptr<::std::fs::File>) -> u64>::new(
            libcc2rs::fread_refcount,
        )
    });
    assert!(!((*fn1.borrow()).is_null()));
    let fn2: Value<FnPtr<fn(Ptr<u8>, u64, u64, AnyPtr) -> u64>> = Rc::new(RefCell::new(
        FnPtr::<fn(AnyPtr, u64, u64, Ptr<::std::fs::File>) -> u64>::new(libcc2rs::fread_refcount)
            .cast::<fn(Ptr<u8>, u64, u64, AnyPtr) -> u64>(Some(
            (|a0: Ptr<u8>, a1: u64, a2: u64, a3: AnyPtr| -> u64 {
                libcc2rs::fread_refcount(a0.to_any(), a1, a2, a3.cast::<::std::fs::File>().unwrap())
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
            FnPtr::<fn(Ptr<u8>, u64, u64, AnyPtr) -> u64>::new(my_alternative_fread_0).cast::<fn(
                AnyPtr,
                u64,
                u64,
                Ptr<::std::fs::File>,
            )
                -> u64>(
                Some(
                    (|a0: AnyPtr, a1: u64, a2: u64, a3: Ptr<::std::fs::File>| -> u64 {
                        my_alternative_fread_0(a0.cast::<u8>().unwrap(), a1, a2, a3.to_any())
                    }) as fn(AnyPtr, u64, u64, Ptr<::std::fs::File>) -> u64,
                ),
            ),
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
    'loop_: loop {
        let stream: Value<Ptr<::std::fs::File>> = Rc::new(RefCell::new(
            match Ptr::from_string_literal("rb").to_rust_string() {
                v if v == "rb" => std::fs::OpenOptions::new()
                    .read(true)
                    .open(Ptr::from_string_literal("/dev/zero").to_rust_string())
                    .ok()
                    .map_or(Ptr::null(), |f| Ptr::alloc(f)),
                v if v == "wb" => std::fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(Ptr::from_string_literal("/dev/zero").to_rust_string())
                    .ok()
                    .map_or(Ptr::null(), |f| Ptr::alloc(f)),
                _ => panic!("unsupported mode"),
            },
        ));
        assert!(!((*stream.borrow()).is_null()));
        let buf: Value<Box<[u8]>> = Rc::new(RefCell::new(
            (0..16).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
        ));
        {
            ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any().memset(
                (('X' as u8) as i32) as u8,
                ::std::mem::size_of::<[u8; 16]>() as u64 as usize,
            );
            ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any().clone()
        };
        let n: Value<u64> = Rc::new(RefCell::new(libcc2rs::fread_refcount(
            ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any(),
            1_u64,
            10_u64,
            (*stream.borrow()).clone(),
        )));
        assert!(((*n.borrow()) == 10_u64));
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < 10) {
            assert!((((*buf.borrow())[(*i.borrow()) as usize] as i32) == 0));
            (*i.borrow_mut()).prefix_inc();
        }
        let i: Value<i32> = Rc::new(RefCell::new(10));
        'loop_: while ((*i.borrow()) < 16) {
            assert!((((*buf.borrow())[(*i.borrow()) as usize] as i32) == (('X' as u8) as i32)));
            (*i.borrow_mut()).prefix_inc();
        }
        {
            (*stream.borrow()).delete();
            0
        };
        if !(0 != 0) {
            break;
        }
    }
    'loop_: loop {
        let stream: Value<Ptr<::std::fs::File>> = Rc::new(RefCell::new(
            match Ptr::from_string_literal("rb").to_rust_string() {
                v if v == "rb" => std::fs::OpenOptions::new()
                    .read(true)
                    .open(Ptr::from_string_literal("/dev/zero").to_rust_string())
                    .ok()
                    .map_or(Ptr::null(), |f| Ptr::alloc(f)),
                v if v == "wb" => std::fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(Ptr::from_string_literal("/dev/zero").to_rust_string())
                    .ok()
                    .map_or(Ptr::null(), |f| Ptr::alloc(f)),
                _ => panic!("unsupported mode"),
            },
        ));
        assert!(!((*stream.borrow()).is_null()));
        let buf: Value<Box<[u8]>> = Rc::new(RefCell::new(
            (0..16).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
        ));
        {
            ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any().memset(
                (('X' as u8) as i32) as u8,
                ::std::mem::size_of::<[u8; 16]>() as u64 as usize,
            );
            ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any().clone()
        };
        let n: Value<u64> = Rc::new(RefCell::new(
            ({
                let _arg0: AnyPtr = ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any();
                let _arg1: u64 = 1_u64;
                let _arg2: u64 = 10_u64;
                let _arg3: Ptr<::std::fs::File> = (*stream.borrow()).clone();
                (*(*fn1.borrow()))(_arg0, _arg1, _arg2, _arg3)
            }),
        ));
        assert!(((*n.borrow()) == 10_u64));
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < 10) {
            assert!((((*buf.borrow())[(*i.borrow()) as usize] as i32) == 0));
            (*i.borrow_mut()).prefix_inc();
        }
        let i: Value<i32> = Rc::new(RefCell::new(10));
        'loop_: while ((*i.borrow()) < 16) {
            assert!((((*buf.borrow())[(*i.borrow()) as usize] as i32) == (('X' as u8) as i32)));
            (*i.borrow_mut()).prefix_inc();
        }
        {
            (*stream.borrow()).delete();
            0
        };
        if !(0 != 0) {
            break;
        }
    }
    return 0;
}
