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
        let p: Value<Ptr<i32>> = Rc::new(RefCell::new(
            libcc2rs::malloc_refcount(::std::mem::size_of::<i32>()).reinterpret_cast::<i32>(),
        ));
        (*p.borrow()).write(42);
        assert!((((((*p.borrow()).read()) == 42) as i32) != 0));
        libcc2rs::free_refcount((*p.borrow()).to_any());
        let arr: Value<Ptr<i32>> = Rc::new(RefCell::new(
            libcc2rs::malloc_refcount(
                (4_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)),
            )
            .reinterpret_cast::<i32>(),
        ));
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((((*i.borrow()) < 4) as i32) != 0) {
            elem!((*arr.borrow()), (*i.borrow())).write({ ((*i.borrow()) * 10) });
            (*i.borrow_mut()).postfix_inc();
        }
        assert!(((((elem!((*arr.borrow()), 0).read()) == 0) as i32) != 0));
        assert!(((((elem!((*arr.borrow()), 3).read()) == 30) as i32) != 0));
        libcc2rs::free_refcount((*arr.borrow()).to_any());
        let grow: Value<Ptr<i32>> = Rc::new(RefCell::new(
            libcc2rs::malloc_refcount(
                (2_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)),
            )
            .reinterpret_cast::<i32>(),
        ));
        elem!((*grow.borrow()), 0).write(1);
        elem!((*grow.borrow()), 1).write(2);
        let __rhs = libcc2rs::realloc_refcount(
            (*grow.borrow()).to_any(),
            (4_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)),
        )
        .reinterpret_cast::<i32>();
        (*grow.borrow_mut()) = __rhs;
        elem!((*grow.borrow()), 2).write(3);
        elem!((*grow.borrow()), 3).write(4);
        assert!(((((elem!((*grow.borrow()), 0).read()) == 1) as i32) != 0));
        assert!(((((elem!((*grow.borrow()), 1).read()) == 2) as i32) != 0));
        assert!(((((elem!((*grow.borrow()), 2).read()) == 3) as i32) != 0));
        assert!(((((elem!((*grow.borrow()), 3).read()) == 4) as i32) != 0));
        libcc2rs::free_refcount((*grow.borrow()).to_any());
        let zeros: Value<Ptr<i32>> = Rc::new(RefCell::new(
            libcc2rs::calloc_refcount(4_usize, ::std::mem::size_of::<i32>())
                .reinterpret_cast::<i32>(),
        ));
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((((*i.borrow()) < 4) as i32) != 0) {
            assert!(((((elem!((*zeros.borrow()), (*i.borrow())).read()) == 0) as i32) != 0));
            (*i.borrow_mut()).postfix_inc();
        }
        libcc2rs::free_refcount((*zeros.borrow()).to_any());
    }
    let pmalloc: Value<FnPtr<fn(usize) -> AnyPtr>> =
        Rc::new(RefCell::new(FnPtr::<fn(usize) -> AnyPtr>::new(
            libcc2rs::malloc_refcount,
        )));
    let pfree: Value<FnPtr<fn(AnyPtr)>> = Rc::new(RefCell::new(FnPtr::<fn(AnyPtr)>::new(
        libcc2rs::free_refcount,
    )));
    let prealloc: Value<FnPtr<fn(AnyPtr, usize) -> AnyPtr>> =
        Rc::new(RefCell::new(FnPtr::<fn(AnyPtr, usize) -> AnyPtr>::new(
            libcc2rs::realloc_refcount,
        )));
    let pcalloc: Value<FnPtr<fn(usize, usize) -> AnyPtr>> =
        Rc::new(RefCell::new(FnPtr::<fn(usize, usize) -> AnyPtr>::new(
            libcc2rs::calloc_refcount,
        )));
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let p: Value<Ptr<i32>> = Rc::new(RefCell::new(
            ({ (*pmalloc.borrow()).call(::std::mem::size_of::<i32>()) }).reinterpret_cast::<i32>(),
        ));
        (*p.borrow()).write(42);
        assert!((((((*p.borrow()).read()) == 42) as i32) != 0));
        ({ (*pfree.borrow()).call((*p.borrow()).to_any()) });
        let arr: Value<Ptr<i32>> = Rc::new(RefCell::new(
            ({
                (*pmalloc.borrow())
                    .call((4_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)))
            })
            .reinterpret_cast::<i32>(),
        ));
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((((*i.borrow()) < 4) as i32) != 0) {
            elem!((*arr.borrow()), (*i.borrow())).write({ ((*i.borrow()) * 10) });
            (*i.borrow_mut()).postfix_inc();
        }
        assert!(((((elem!((*arr.borrow()), 0).read()) == 0) as i32) != 0));
        assert!(((((elem!((*arr.borrow()), 3).read()) == 30) as i32) != 0));
        ({ (*pfree.borrow()).call((*arr.borrow()).to_any()) });
        let grow: Value<Ptr<i32>> = Rc::new(RefCell::new(
            ({
                (*pmalloc.borrow())
                    .call((2_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)))
            })
            .reinterpret_cast::<i32>(),
        ));
        elem!((*grow.borrow()), 0).write(1);
        elem!((*grow.borrow()), 1).write(2);
        let __rhs = ({
            (*prealloc.borrow()).call(
                (*grow.borrow()).to_any(),
                (4_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)),
            )
        })
        .reinterpret_cast::<i32>();
        (*grow.borrow_mut()) = __rhs;
        elem!((*grow.borrow()), 2).write(3);
        elem!((*grow.borrow()), 3).write(4);
        assert!(((((elem!((*grow.borrow()), 0).read()) == 1) as i32) != 0));
        assert!(((((elem!((*grow.borrow()), 1).read()) == 2) as i32) != 0));
        assert!(((((elem!((*grow.borrow()), 2).read()) == 3) as i32) != 0));
        assert!(((((elem!((*grow.borrow()), 3).read()) == 4) as i32) != 0));
        ({ (*pfree.borrow()).call((*grow.borrow()).to_any()) });
        let zeros: Value<Ptr<i32>> = Rc::new(RefCell::new(
            ({ (*pcalloc.borrow()).call(4_usize, ::std::mem::size_of::<i32>()) })
                .reinterpret_cast::<i32>(),
        ));
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((((*i.borrow()) < 4) as i32) != 0) {
            assert!(((((elem!((*zeros.borrow()), (*i.borrow())).read()) == 0) as i32) != 0));
            (*i.borrow_mut()).postfix_inc();
        }
        ({ (*pfree.borrow()).call((*zeros.borrow()).to_any()) });
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
