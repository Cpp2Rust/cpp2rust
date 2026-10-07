extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn my_alternative_fread_0(mut p: Ptr<i8>, mut n: usize, mut m: usize, mut f: AnyPtr) -> usize {
    return 22_usize;
}
pub fn my_alternative_fwrite_1(mut p: Ptr<i8>, mut n: usize, mut m: usize, mut f: AnyPtr) -> usize {
    return 33_usize;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut fn1: FnPtr<fn(AnyPtr, usize, usize, Ptr<CFile>) -> usize> =
        FnPtr::<fn(AnyPtr, usize, usize, Ptr<CFile>) -> usize>::new(libcc2rs::fread_refcount);
    assert!(
        ({ (fn1).clone() } == {
            FnPtr::<fn(AnyPtr, usize, usize, Ptr<CFile>) -> usize>::new(libcc2rs::fread_refcount)
        })
    );
    assert!(!((fn1).is_null()));
    let mut fn2: FnPtr<fn(Ptr<i8>, usize, usize, AnyPtr) -> usize> =
        FnPtr::<fn(AnyPtr, usize, usize, Ptr<CFile>) -> usize>::new(libcc2rs::fread_refcount)
            .cast::<fn(Ptr<i8>, usize, usize, AnyPtr) -> usize>();
    assert!(({ (fn1).clone() } == { fn2.cast::<fn(AnyPtr, usize, usize, Ptr<CFile>) -> usize>() }));
    let mut f3: FnPtr<fn(AnyPtr, usize, usize, Ptr<CFile>) -> usize> = FnPtr::<
        fn(Ptr<i8>, usize, usize, AnyPtr) -> usize,
    >::new(
        my_alternative_fread_0
    )
    .cast::<fn(AnyPtr, usize, usize, Ptr<CFile>) -> usize>();
    assert!((({ (f3).call(AnyPtr::default(), 0_usize, 0_usize, Ptr::null(),) }) == 22_usize));
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut stream: Ptr<CFile> = match CFile::open(
            &Ptr::<i8>::from_string_literal(b"/dev/zero").to_rust_string(),
            &Ptr::<i8>::from_string_literal(b"rb").to_rust_string(),
        ) {
            Some(__f) => Ptr::alloc(__f),
            None => Ptr::null(),
        };
        assert!(!((stream).is_null()));
        let buf: Value<Box<[i8]>> =
            Rc::new(RefCell::new((0..16).map(|_| 0_i8).collect::<Box<[i8]>>()));
        {
            ((buf.as_pointer() as Ptr<i8>) as Ptr<i8>)
                .to_any()
                .memset((('X' as i8) as i32) as u8, 16usize as usize);
            ((buf.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any()
        };
        let mut n: usize = {
            let __a0 = ((buf.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any();
            let __a1 = 1_usize;
            let __a2 = 10_usize;
            let __a3 = (stream).clone();
            libcc2rs::fread_refcount(__a0, __a1, __a2, __a3)
        };
        assert!((n == 10_usize));
        let mut i: i32 = 0;
        'loop_: while (i < 10) {
            assert!((((*buf.borrow())[(i) as usize] as i32) == 0));
            i.prefix_inc();
        }
        let mut i: i32 = 10;
        'loop_: while (i < 16) {
            assert!((((*buf.borrow())[(i) as usize] as i32) == (('X' as i8) as i32)));
            i.prefix_inc();
        }
        {
            let __r = stream.with(|__f| __f.close());
            stream.delete();
            __r
        };
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut stream: Ptr<CFile> = match CFile::open(
            &Ptr::<i8>::from_string_literal(b"/dev/zero").to_rust_string(),
            &Ptr::<i8>::from_string_literal(b"rb").to_rust_string(),
        ) {
            Some(__f) => Ptr::alloc(__f),
            None => Ptr::null(),
        };
        assert!(!((stream).is_null()));
        let buf: Value<Box<[i8]>> =
            Rc::new(RefCell::new((0..16).map(|_| 0_i8).collect::<Box<[i8]>>()));
        {
            ((buf.as_pointer() as Ptr<i8>) as Ptr<i8>)
                .to_any()
                .memset((('X' as i8) as i32) as u8, 16usize as usize);
            ((buf.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any()
        };
        let mut n: usize = ({
            (fn1).call(
                ((buf.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any(),
                1_usize,
                10_usize,
                (stream).clone(),
            )
        });
        assert!((n == 10_usize));
        let mut i: i32 = 0;
        'loop_: while (i < 10) {
            assert!((((*buf.borrow())[(i) as usize] as i32) == 0));
            i.prefix_inc();
        }
        let mut i: i32 = 10;
        'loop_: while (i < 16) {
            assert!((((*buf.borrow())[(i) as usize] as i32) == (('X' as i8) as i32)));
            i.prefix_inc();
        }
        {
            let __r = stream.with(|__f| __f.close());
            stream.delete();
            __r
        };
    }
    let mut gn1: FnPtr<fn(AnyPtr, usize, usize, Ptr<CFile>) -> usize> =
        FnPtr::<fn(AnyPtr, usize, usize, Ptr<CFile>) -> usize>::new(libcc2rs::fwrite_refcount);
    assert!(
        ({ (gn1).clone() } == {
            FnPtr::<fn(AnyPtr, usize, usize, Ptr<CFile>) -> usize>::new(libcc2rs::fwrite_refcount)
        })
    );
    assert!(!((gn1).is_null()));
    let mut gn2: FnPtr<fn(Ptr<i8>, usize, usize, AnyPtr) -> usize> =
        FnPtr::<fn(AnyPtr, usize, usize, Ptr<CFile>) -> usize>::new(libcc2rs::fwrite_refcount)
            .cast::<fn(Ptr<i8>, usize, usize, AnyPtr) -> usize>();
    assert!(({ (gn1).clone() } == { gn2.cast::<fn(AnyPtr, usize, usize, Ptr<CFile>) -> usize>() }));
    let mut g3: FnPtr<fn(AnyPtr, usize, usize, Ptr<CFile>) -> usize> =
        FnPtr::<fn(Ptr<i8>, usize, usize, AnyPtr) -> usize>::new(my_alternative_fwrite_1)
            .cast::<fn(AnyPtr, usize, usize, Ptr<CFile>) -> usize>();
    assert!((({ (g3).call(AnyPtr::default(), 0_usize, 0_usize, Ptr::null(),) }) == 33_usize));
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut stream: Ptr<CFile> = match CFile::open(
            &Ptr::<i8>::from_string_literal(b"/dev/null").to_rust_string(),
            &Ptr::<i8>::from_string_literal(b"wb").to_rust_string(),
        ) {
            Some(__f) => Ptr::alloc(__f),
            None => Ptr::null(),
        };
        assert!(!((stream).is_null()));
        let buf: Value<Box<[i8]>> =
            Rc::new(RefCell::new((0..10).map(|_| 0_i8).collect::<Box<[i8]>>()));
        {
            ((buf.as_pointer() as Ptr<i8>) as Ptr<i8>)
                .to_any()
                .memset((('Y' as i8) as i32) as u8, 10usize as usize);
            ((buf.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any()
        };
        let mut n: usize = {
            let __a0 = ((buf.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any();
            let __a1 = 1_usize;
            let __a2 = 10_usize;
            let __a3 = (stream).clone();
            libcc2rs::fwrite_refcount(__a0, __a1, __a2, __a3)
        };
        assert!((n == 10_usize));
        {
            let __r = stream.with(|__f| __f.close());
            stream.delete();
            __r
        };
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut stream: Ptr<CFile> = match CFile::open(
            &Ptr::<i8>::from_string_literal(b"/dev/null").to_rust_string(),
            &Ptr::<i8>::from_string_literal(b"wb").to_rust_string(),
        ) {
            Some(__f) => Ptr::alloc(__f),
            None => Ptr::null(),
        };
        assert!(!((stream).is_null()));
        let buf: Value<Box<[i8]>> =
            Rc::new(RefCell::new((0..10).map(|_| 0_i8).collect::<Box<[i8]>>()));
        {
            ((buf.as_pointer() as Ptr<i8>) as Ptr<i8>)
                .to_any()
                .memset((('Y' as i8) as i32) as u8, 10usize as usize);
            ((buf.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any()
        };
        let mut n: usize = ({
            (gn1).call(
                ((buf.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any(),
                1_usize,
                10_usize,
                (stream).clone(),
            )
        });
        assert!((n == 10_usize));
        {
            let __r = stream.with(|__f| __f.close());
            stream.delete();
            __r
        };
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
