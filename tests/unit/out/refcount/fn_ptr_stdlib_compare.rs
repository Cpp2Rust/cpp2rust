extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn my_alternative_fread_0(p: Ptr<core::ffi::c_char>, n: usize, m: usize, f: AnyPtr) -> usize {
    let p: Value<Ptr<core::ffi::c_char>> = Rc::new(RefCell::new(p));
    let n: Value<usize> = Rc::new(RefCell::new(n));
    let m: Value<usize> = Rc::new(RefCell::new(m));
    let f: Value<AnyPtr> = Rc::new(RefCell::new(f));
    return 22_usize;
}
pub fn my_alternative_fwrite_1(p: Ptr<core::ffi::c_char>, n: usize, m: usize, f: AnyPtr) -> usize {
    let p: Value<Ptr<core::ffi::c_char>> = Rc::new(RefCell::new(p));
    let n: Value<usize> = Rc::new(RefCell::new(n));
    let m: Value<usize> = Rc::new(RefCell::new(m));
    let f: Value<AnyPtr> = Rc::new(RefCell::new(f));
    return 33_usize;
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let fn1: Value<FnPtr<fn(AnyPtr, usize, usize, Ptr<::std::fs::File>) -> usize>> =
        Rc::new(RefCell::new(FnPtr::<
            fn(AnyPtr, usize, usize, Ptr<::std::fs::File>) -> usize,
        >::new(libcc2rs::fread_refcount)));
    assert!({
        let _lhs = (*fn1.borrow()).clone();
        _lhs == FnPtr::<fn(AnyPtr, usize, usize, Ptr<::std::fs::File>) -> usize>::new(
            libcc2rs::fread_refcount,
        )
    });
    assert!(!((*fn1.borrow()).is_null()));
    let fn2: Value<FnPtr<fn(Ptr<core::ffi::c_char>, usize, usize, AnyPtr) -> usize>> =
        Rc::new(RefCell::new(
            FnPtr::<fn(AnyPtr, usize, usize, Ptr<::std::fs::File>) -> usize>::new(
                libcc2rs::fread_refcount,
            )
            .cast::<fn(Ptr<core::ffi::c_char>, usize, usize, AnyPtr) -> usize>(Some(
                (|a0: Ptr<core::ffi::c_char>, a1: usize, a2: usize, a3: AnyPtr| -> usize {
                    libcc2rs::fread_refcount(
                        a0.to_any(),
                        a1,
                        a2,
                        a3.cast::<::std::fs::File>().unwrap(),
                    )
                }) as fn(Ptr<core::ffi::c_char>, usize, usize, AnyPtr) -> usize,
            )),
        ));
    assert!({
        let _lhs = (*fn1.borrow()).clone();
        _lhs == ((*fn2.borrow())
            .cast::<fn(AnyPtr, usize, usize, Ptr<::std::fs::File>) -> usize>(None))
        .clone()
    });
    let f3: Value<FnPtr<fn(AnyPtr, usize, usize, Ptr<::std::fs::File>) -> usize>> =
        Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<core::ffi::c_char>, usize, usize, AnyPtr) -> usize>::new(
                my_alternative_fread_0,
            )
            .cast::<fn(AnyPtr, usize, usize, Ptr<::std::fs::File>) -> usize>(Some(
                (|a0: AnyPtr, a1: usize, a2: usize, a3: Ptr<::std::fs::File>| -> usize {
                    my_alternative_fread_0(
                        a0.cast::<core::ffi::c_char>().unwrap(),
                        a1,
                        a2,
                        a3.to_any(),
                    )
                }) as fn(AnyPtr, usize, usize, Ptr<::std::fs::File>) -> usize,
            )),
        ));
    assert!(
        (({
            let _arg0: AnyPtr = AnyPtr::default();
            let _arg3: Ptr<::std::fs::File> = Ptr::null();
            (*(*f3.borrow()))(_arg0, 0_usize, 0_usize, _arg3)
        }) == 22_usize)
    );
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let stream: Value<Ptr<::std::fs::File>> = Rc::new(RefCell::new(
            match Ptr::from_string_literal(b"rb").to_rust_string() {
                v if v == "rb" => std::fs::OpenOptions::new()
                    .read(true)
                    .open(Ptr::from_string_literal(b"/dev/zero").to_rust_string())
                    .ok()
                    .map_or(Ptr::null(), |f| Ptr::alloc(f)),
                v if v == "wb" => std::fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(Ptr::from_string_literal(b"/dev/zero").to_rust_string())
                    .ok()
                    .map_or(Ptr::null(), |f| Ptr::alloc(f)),
                _ => panic!("unsupported mode"),
            },
        ));
        assert!(!((*stream.borrow()).is_null()));
        let buf: Value<Box<[core::ffi::c_char]>> = Rc::new(RefCell::new(
            (0..16)
                .map(|_| <core::ffi::c_char>::default())
                .collect::<Box<[core::ffi::c_char]>>(),
        ));
        {
            ((buf.as_pointer() as Ptr<core::ffi::c_char>) as Ptr<core::ffi::c_char>)
                .to_any()
                .memset(
                    (('X' as core::ffi::c_char) as i32) as u8,
                    ::std::mem::size_of::<[core::ffi::c_char; 16]>() as usize,
                );
            ((buf.as_pointer() as Ptr<core::ffi::c_char>) as Ptr<core::ffi::c_char>)
                .to_any()
                .clone()
        };
        let n: Value<usize> = Rc::new(RefCell::new({
            let __a0 =
                ((buf.as_pointer() as Ptr<core::ffi::c_char>) as Ptr<core::ffi::c_char>).to_any();
            let __a1 = 1_usize;
            let __a2 = 10_usize;
            let __a3 = (*stream.borrow()).clone();
            libcc2rs::fread_refcount(__a0, __a1, __a2, __a3)
        }));
        assert!(((*n.borrow()) == 10_usize));
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < 10) {
            assert!((((*buf.borrow())[(*i.borrow()) as usize] as i32) == 0));
            (*i.borrow_mut()).prefix_inc();
        }
        let i: Value<i32> = Rc::new(RefCell::new(10));
        'loop_: while ((*i.borrow()) < 16) {
            assert!(
                (((*buf.borrow())[(*i.borrow()) as usize] as i32)
                    == (('X' as core::ffi::c_char) as i32))
            );
            (*i.borrow_mut()).prefix_inc();
        }
        {
            (*stream.borrow()).delete();
            0
        };
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let stream: Value<Ptr<::std::fs::File>> = Rc::new(RefCell::new(
            match Ptr::from_string_literal(b"rb").to_rust_string() {
                v if v == "rb" => std::fs::OpenOptions::new()
                    .read(true)
                    .open(Ptr::from_string_literal(b"/dev/zero").to_rust_string())
                    .ok()
                    .map_or(Ptr::null(), |f| Ptr::alloc(f)),
                v if v == "wb" => std::fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(Ptr::from_string_literal(b"/dev/zero").to_rust_string())
                    .ok()
                    .map_or(Ptr::null(), |f| Ptr::alloc(f)),
                _ => panic!("unsupported mode"),
            },
        ));
        assert!(!((*stream.borrow()).is_null()));
        let buf: Value<Box<[core::ffi::c_char]>> = Rc::new(RefCell::new(
            (0..16)
                .map(|_| <core::ffi::c_char>::default())
                .collect::<Box<[core::ffi::c_char]>>(),
        ));
        {
            ((buf.as_pointer() as Ptr<core::ffi::c_char>) as Ptr<core::ffi::c_char>)
                .to_any()
                .memset(
                    (('X' as core::ffi::c_char) as i32) as u8,
                    ::std::mem::size_of::<[core::ffi::c_char; 16]>() as usize,
                );
            ((buf.as_pointer() as Ptr<core::ffi::c_char>) as Ptr<core::ffi::c_char>)
                .to_any()
                .clone()
        };
        let n: Value<usize> = Rc::new(RefCell::new(
            ({
                let _arg0: AnyPtr = ((buf.as_pointer() as Ptr<core::ffi::c_char>)
                    as Ptr<core::ffi::c_char>)
                    .to_any();
                let _arg3: Ptr<::std::fs::File> = (*stream.borrow()).clone();
                (*(*fn1.borrow()))(_arg0, 1_usize, 10_usize, _arg3)
            }),
        ));
        assert!(((*n.borrow()) == 10_usize));
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < 10) {
            assert!((((*buf.borrow())[(*i.borrow()) as usize] as i32) == 0));
            (*i.borrow_mut()).prefix_inc();
        }
        let i: Value<i32> = Rc::new(RefCell::new(10));
        'loop_: while ((*i.borrow()) < 16) {
            assert!(
                (((*buf.borrow())[(*i.borrow()) as usize] as i32)
                    == (('X' as core::ffi::c_char) as i32))
            );
            (*i.borrow_mut()).prefix_inc();
        }
        {
            (*stream.borrow()).delete();
            0
        };
    }
    let gn1: Value<FnPtr<fn(AnyPtr, usize, usize, Ptr<::std::fs::File>) -> usize>> =
        Rc::new(RefCell::new(FnPtr::<
            fn(AnyPtr, usize, usize, Ptr<::std::fs::File>) -> usize,
        >::new(libcc2rs::fwrite_refcount)));
    assert!({
        let _lhs = (*gn1.borrow()).clone();
        _lhs == FnPtr::<fn(AnyPtr, usize, usize, Ptr<::std::fs::File>) -> usize>::new(
            libcc2rs::fwrite_refcount,
        )
    });
    assert!(!((*gn1.borrow()).is_null()));
    let gn2: Value<FnPtr<fn(Ptr<core::ffi::c_char>, usize, usize, AnyPtr) -> usize>> =
        Rc::new(RefCell::new(
            FnPtr::<fn(AnyPtr, usize, usize, Ptr<::std::fs::File>) -> usize>::new(
                libcc2rs::fwrite_refcount,
            )
            .cast::<fn(Ptr<core::ffi::c_char>, usize, usize, AnyPtr) -> usize>(Some(
                (|a0: Ptr<core::ffi::c_char>, a1: usize, a2: usize, a3: AnyPtr| -> usize {
                    libcc2rs::fwrite_refcount(
                        a0.to_any(),
                        a1,
                        a2,
                        a3.cast::<::std::fs::File>().unwrap(),
                    )
                }) as fn(Ptr<core::ffi::c_char>, usize, usize, AnyPtr) -> usize,
            )),
        ));
    assert!({
        let _lhs = (*gn1.borrow()).clone();
        _lhs == ((*gn2.borrow())
            .cast::<fn(AnyPtr, usize, usize, Ptr<::std::fs::File>) -> usize>(None))
        .clone()
    });
    let g3: Value<FnPtr<fn(AnyPtr, usize, usize, Ptr<::std::fs::File>) -> usize>> =
        Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<core::ffi::c_char>, usize, usize, AnyPtr) -> usize>::new(
                my_alternative_fwrite_1,
            )
            .cast::<fn(AnyPtr, usize, usize, Ptr<::std::fs::File>) -> usize>(Some(
                (|a0: AnyPtr, a1: usize, a2: usize, a3: Ptr<::std::fs::File>| -> usize {
                    my_alternative_fwrite_1(
                        a0.cast::<core::ffi::c_char>().unwrap(),
                        a1,
                        a2,
                        a3.to_any(),
                    )
                }) as fn(AnyPtr, usize, usize, Ptr<::std::fs::File>) -> usize,
            )),
        ));
    assert!(
        (({
            let _arg0: AnyPtr = AnyPtr::default();
            let _arg3: Ptr<::std::fs::File> = Ptr::null();
            (*(*g3.borrow()))(_arg0, 0_usize, 0_usize, _arg3)
        }) == 33_usize)
    );
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let stream: Value<Ptr<::std::fs::File>> = Rc::new(RefCell::new(
            match Ptr::from_string_literal(b"wb").to_rust_string() {
                v if v == "rb" => std::fs::OpenOptions::new()
                    .read(true)
                    .open(Ptr::from_string_literal(b"/dev/null").to_rust_string())
                    .ok()
                    .map_or(Ptr::null(), |f| Ptr::alloc(f)),
                v if v == "wb" => std::fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(Ptr::from_string_literal(b"/dev/null").to_rust_string())
                    .ok()
                    .map_or(Ptr::null(), |f| Ptr::alloc(f)),
                _ => panic!("unsupported mode"),
            },
        ));
        assert!(!((*stream.borrow()).is_null()));
        let buf: Value<Box<[core::ffi::c_char]>> = Rc::new(RefCell::new(
            (0..10)
                .map(|_| <core::ffi::c_char>::default())
                .collect::<Box<[core::ffi::c_char]>>(),
        ));
        {
            ((buf.as_pointer() as Ptr<core::ffi::c_char>) as Ptr<core::ffi::c_char>)
                .to_any()
                .memset(
                    (('Y' as core::ffi::c_char) as i32) as u8,
                    ::std::mem::size_of::<[core::ffi::c_char; 10]>() as usize,
                );
            ((buf.as_pointer() as Ptr<core::ffi::c_char>) as Ptr<core::ffi::c_char>)
                .to_any()
                .clone()
        };
        let n: Value<usize> = Rc::new(RefCell::new({
            let __a0 =
                ((buf.as_pointer() as Ptr<core::ffi::c_char>) as Ptr<core::ffi::c_char>).to_any();
            let __a1 = 1_usize;
            let __a2 = 10_usize;
            let __a3 = (*stream.borrow()).clone();
            libcc2rs::fwrite_refcount(__a0, __a1, __a2, __a3)
        }));
        assert!(((*n.borrow()) == 10_usize));
        {
            (*stream.borrow()).delete();
            0
        };
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let stream: Value<Ptr<::std::fs::File>> = Rc::new(RefCell::new(
            match Ptr::from_string_literal(b"wb").to_rust_string() {
                v if v == "rb" => std::fs::OpenOptions::new()
                    .read(true)
                    .open(Ptr::from_string_literal(b"/dev/null").to_rust_string())
                    .ok()
                    .map_or(Ptr::null(), |f| Ptr::alloc(f)),
                v if v == "wb" => std::fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(Ptr::from_string_literal(b"/dev/null").to_rust_string())
                    .ok()
                    .map_or(Ptr::null(), |f| Ptr::alloc(f)),
                _ => panic!("unsupported mode"),
            },
        ));
        assert!(!((*stream.borrow()).is_null()));
        let buf: Value<Box<[core::ffi::c_char]>> = Rc::new(RefCell::new(
            (0..10)
                .map(|_| <core::ffi::c_char>::default())
                .collect::<Box<[core::ffi::c_char]>>(),
        ));
        {
            ((buf.as_pointer() as Ptr<core::ffi::c_char>) as Ptr<core::ffi::c_char>)
                .to_any()
                .memset(
                    (('Y' as core::ffi::c_char) as i32) as u8,
                    ::std::mem::size_of::<[core::ffi::c_char; 10]>() as usize,
                );
            ((buf.as_pointer() as Ptr<core::ffi::c_char>) as Ptr<core::ffi::c_char>)
                .to_any()
                .clone()
        };
        let n: Value<usize> = Rc::new(RefCell::new(
            ({
                let _arg0: AnyPtr = ((buf.as_pointer() as Ptr<core::ffi::c_char>)
                    as Ptr<core::ffi::c_char>)
                    .to_any();
                let _arg3: Ptr<::std::fs::File> = (*stream.borrow()).clone();
                (*(*gn1.borrow()))(_arg0, 1_usize, 10_usize, _arg3)
            }),
        ));
        assert!(((*n.borrow()) == 10_usize));
        {
            (*stream.borrow()).delete();
            0
        };
    }
    return 0;
}
