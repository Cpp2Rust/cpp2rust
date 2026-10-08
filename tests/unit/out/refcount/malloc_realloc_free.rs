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
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut p: Ptr<i32> =
            libcc2rs::malloc_refcount(::std::mem::size_of::<i32>()).reinterpret_cast::<i32>();
        p.write(42);
        assert!(((((p.read()) == 42) as i32) != 0));
        libcc2rs::free_refcount((p).to_any());
        let mut arr: Ptr<i32> = libcc2rs::malloc_refcount(
            (4_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)),
        )
        .reinterpret_cast::<i32>();
        let mut i: i32 = 0;
        'loop_: while (((i < 4) as i32) != 0) {
            elem!(arr, i).write({ (i * 10) });
            i.postfix_inc();
        }
        assert!(((((elem!(arr, 0).read()) == 0) as i32) != 0));
        assert!(((((elem!(arr, 3).read()) == 30) as i32) != 0));
        libcc2rs::free_refcount((arr).to_any());
        let mut grow: Ptr<i32> = libcc2rs::malloc_refcount(
            (2_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)),
        )
        .reinterpret_cast::<i32>();
        elem!(grow, 0).write(1);
        elem!(grow, 1).write(2);
        let __rhs = libcc2rs::realloc_refcount(
            (grow).to_any(),
            (4_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)),
        )
        .reinterpret_cast::<i32>();
        grow = __rhs;
        elem!(grow, 2).write(3);
        elem!(grow, 3).write(4);
        assert!(((((elem!(grow, 0).read()) == 1) as i32) != 0));
        assert!(((((elem!(grow, 1).read()) == 2) as i32) != 0));
        assert!(((((elem!(grow, 2).read()) == 3) as i32) != 0));
        assert!(((((elem!(grow, 3).read()) == 4) as i32) != 0));
        libcc2rs::free_refcount((grow).to_any());
        let mut zeros: Ptr<i32> = libcc2rs::calloc_refcount(4_usize, ::std::mem::size_of::<i32>())
            .reinterpret_cast::<i32>();
        let mut i: i32 = 0;
        'loop_: while (((i < 4) as i32) != 0) {
            assert!(((((elem!(zeros, i).read()) == 0) as i32) != 0));
            i.postfix_inc();
        }
        libcc2rs::free_refcount((zeros).to_any());
    }
    let mut pmalloc: FnPtr<fn(usize) -> AnyPtr> =
        FnPtr::<fn(usize) -> AnyPtr>::new(libcc2rs::malloc_refcount);
    let mut pfree: FnPtr<fn(AnyPtr)> = FnPtr::<fn(AnyPtr)>::new(libcc2rs::free_refcount);
    let mut prealloc: FnPtr<fn(AnyPtr, usize) -> AnyPtr> =
        FnPtr::<fn(AnyPtr, usize) -> AnyPtr>::new(libcc2rs::realloc_refcount);
    let mut pcalloc: FnPtr<fn(usize, usize) -> AnyPtr> =
        FnPtr::<fn(usize, usize) -> AnyPtr>::new(libcc2rs::calloc_refcount);
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut p: Ptr<i32> =
            ({ pmalloc.call(::std::mem::size_of::<i32>()) }).reinterpret_cast::<i32>();
        p.write(42);
        assert!(((((p.read()) == 42) as i32) != 0));
        ({ pfree.call((p).to_any()) });
        let mut arr: Ptr<i32> =
            ({ pmalloc.call((4_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize))) })
                .reinterpret_cast::<i32>();
        let mut i: i32 = 0;
        'loop_: while (((i < 4) as i32) != 0) {
            elem!(arr, i).write({ (i * 10) });
            i.postfix_inc();
        }
        assert!(((((elem!(arr, 0).read()) == 0) as i32) != 0));
        assert!(((((elem!(arr, 3).read()) == 30) as i32) != 0));
        ({ pfree.call((arr).to_any()) });
        let mut grow: Ptr<i32> =
            ({ pmalloc.call((2_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize))) })
                .reinterpret_cast::<i32>();
        elem!(grow, 0).write(1);
        elem!(grow, 1).write(2);
        let __rhs = ({
            prealloc.call(
                (grow).to_any(),
                (4_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)),
            )
        })
        .reinterpret_cast::<i32>();
        grow = __rhs;
        elem!(grow, 2).write(3);
        elem!(grow, 3).write(4);
        assert!(((((elem!(grow, 0).read()) == 1) as i32) != 0));
        assert!(((((elem!(grow, 1).read()) == 2) as i32) != 0));
        assert!(((((elem!(grow, 2).read()) == 3) as i32) != 0));
        assert!(((((elem!(grow, 3).read()) == 4) as i32) != 0));
        ({ pfree.call((grow).to_any()) });
        let mut zeros: Ptr<i32> =
            ({ pcalloc.call(4_usize, ::std::mem::size_of::<i32>()) }).reinterpret_cast::<i32>();
        let mut i: i32 = 0;
        'loop_: while (((i < 4) as i32) != 0) {
            assert!(((((elem!(zeros, i).read()) == 0) as i32) != 0));
            i.postfix_inc();
        }
        ({ pfree.call((zeros).to_any()) });
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
