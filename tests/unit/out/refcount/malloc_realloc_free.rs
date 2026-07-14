extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
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
        libcc2rs::free_refcount(((*p.borrow()).clone() as Ptr<i32>).to_any());
        let arr: Value<Ptr<i32>> = Rc::new(RefCell::new(
            libcc2rs::malloc_refcount(
                (4_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)),
            )
            .reinterpret_cast::<i32>(),
        ));
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((((*i.borrow()) < 4) as i32) != 0) {
            let __rhs = ((*i.borrow()) * 10);
            (*arr.borrow()).offset((*i.borrow()) as isize).write(__rhs);
            (*i.borrow_mut()).postfix_inc();
        }
        assert!((((((*arr.borrow()).offset((0) as isize).read()) == 0) as i32) != 0));
        assert!((((((*arr.borrow()).offset((3) as isize).read()) == 30) as i32) != 0));
        libcc2rs::free_refcount(((*arr.borrow()).clone() as Ptr<i32>).to_any());
        let grow: Value<Ptr<i32>> = Rc::new(RefCell::new(
            libcc2rs::malloc_refcount(
                (2_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)),
            )
            .reinterpret_cast::<i32>(),
        ));
        (*grow.borrow()).offset((0) as isize).write(1);
        (*grow.borrow()).offset((1) as isize).write(2);
        let __rhs = libcc2rs::realloc_refcount(
            ((*grow.borrow()).clone() as Ptr<i32>).to_any(),
            (4_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)),
        )
        .reinterpret_cast::<i32>();
        (*grow.borrow_mut()) = __rhs;
        (*grow.borrow()).offset((2) as isize).write(3);
        (*grow.borrow()).offset((3) as isize).write(4);
        assert!((((((*grow.borrow()).offset((0) as isize).read()) == 1) as i32) != 0));
        assert!((((((*grow.borrow()).offset((1) as isize).read()) == 2) as i32) != 0));
        assert!((((((*grow.borrow()).offset((2) as isize).read()) == 3) as i32) != 0));
        assert!((((((*grow.borrow()).offset((3) as isize).read()) == 4) as i32) != 0));
        libcc2rs::free_refcount(((*grow.borrow()).clone() as Ptr<i32>).to_any());
        let zeros: Value<Ptr<i32>> = Rc::new(RefCell::new(
            libcc2rs::calloc_refcount(4_usize, ::std::mem::size_of::<i32>())
                .reinterpret_cast::<i32>(),
        ));
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((((*i.borrow()) < 4) as i32) != 0) {
            assert!(
                (((((*zeros.borrow()).offset((*i.borrow()) as isize).read()) == 0) as i32) != 0)
            );
            (*i.borrow_mut()).postfix_inc();
        }
        libcc2rs::free_refcount(((*zeros.borrow()).clone() as Ptr<i32>).to_any());
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
            ({ (*(*pmalloc.borrow()))(::std::mem::size_of::<i32>()) }).reinterpret_cast::<i32>(),
        ));
        (*p.borrow()).write(42);
        assert!((((((*p.borrow()).read()) == 42) as i32) != 0));
        ({ (*(*pfree.borrow()))(((*p.borrow()).clone() as Ptr<i32>).to_any()) });
        let arr: Value<Ptr<i32>> = Rc::new(RefCell::new(
            ({
                (*(*pmalloc.borrow()))(
                    (4_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)),
                )
            })
            .reinterpret_cast::<i32>(),
        ));
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((((*i.borrow()) < 4) as i32) != 0) {
            let __rhs = ((*i.borrow()) * 10);
            (*arr.borrow()).offset((*i.borrow()) as isize).write(__rhs);
            (*i.borrow_mut()).postfix_inc();
        }
        assert!((((((*arr.borrow()).offset((0) as isize).read()) == 0) as i32) != 0));
        assert!((((((*arr.borrow()).offset((3) as isize).read()) == 30) as i32) != 0));
        ({ (*(*pfree.borrow()))(((*arr.borrow()).clone() as Ptr<i32>).to_any()) });
        let grow: Value<Ptr<i32>> = Rc::new(RefCell::new(
            ({
                (*(*pmalloc.borrow()))(
                    (2_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)),
                )
            })
            .reinterpret_cast::<i32>(),
        ));
        (*grow.borrow()).offset((0) as isize).write(1);
        (*grow.borrow()).offset((1) as isize).write(2);
        let __rhs = ({
            (*(*prealloc.borrow()))(
                ((*grow.borrow()).clone() as Ptr<i32>).to_any(),
                (4_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)),
            )
        })
        .reinterpret_cast::<i32>();
        (*grow.borrow_mut()) = __rhs;
        (*grow.borrow()).offset((2) as isize).write(3);
        (*grow.borrow()).offset((3) as isize).write(4);
        assert!((((((*grow.borrow()).offset((0) as isize).read()) == 1) as i32) != 0));
        assert!((((((*grow.borrow()).offset((1) as isize).read()) == 2) as i32) != 0));
        assert!((((((*grow.borrow()).offset((2) as isize).read()) == 3) as i32) != 0));
        assert!((((((*grow.borrow()).offset((3) as isize).read()) == 4) as i32) != 0));
        ({ (*(*pfree.borrow()))(((*grow.borrow()).clone() as Ptr<i32>).to_any()) });
        let zeros: Value<Ptr<i32>> = Rc::new(RefCell::new(
            ({ (*(*pcalloc.borrow()))(4_usize, ::std::mem::size_of::<i32>()) })
                .reinterpret_cast::<i32>(),
        ));
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((((*i.borrow()) < 4) as i32) != 0) {
            assert!(
                (((((*zeros.borrow()).offset((*i.borrow()) as isize).read()) == 0) as i32) != 0)
            );
            (*i.borrow_mut()).postfix_inc();
        }
        ({ (*(*pfree.borrow()))(((*zeros.borrow()).clone() as Ptr<i32>).to_any()) });
    }
    return 0;
}
