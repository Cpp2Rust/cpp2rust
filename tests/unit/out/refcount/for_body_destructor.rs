extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(4)]
pub struct Guard {
    #[offset(0)]
    pub digit: i32,
}
pub fn step_0(i: Ptr<i32>) {
    printf!(b"9 ");
    i.with_mut(|__v| {
        __v.prefix_inc();
    });
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    printf!(b"normal/continue: ");
    {
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < 2) {
            'for_body: {
                let first: Value<Guard> = Rc::new(RefCell::new(Guard { digit: 1 }));
                let _dtor_first =
                    std::mem::DropGuard::new(first.clone(), |__p| __p.as_pointer().destructor());
                let second: Value<Guard> = Rc::new(RefCell::new(Guard { digit: 2 }));
                let _dtor_second =
                    std::mem::DropGuard::new(second.clone(), |__p| __p.as_pointer().destructor());
                if ((*i.borrow()) == 0) {
                    break 'for_body;
                }
            }
            ({ step_0(i.as_pointer()) });
        }
    }
    printf!(b"\nbreak: ");
    {
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < 2) {
            'for_body: {
                let g: Value<Guard> = Rc::new(RefCell::new(Guard { digit: 3 }));
                let _dtor_g =
                    std::mem::DropGuard::new(g.clone(), |__p| __p.as_pointer().destructor());
                break 'loop_;
            }
            ({ step_0(i.as_pointer()) });
        }
    }
    printf!(b"\nnested for: ");
    {
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < 1) {
            'for_body: {
                let outer: Value<Guard> = Rc::new(RefCell::new(Guard { digit: 4 }));
                let _dtor_outer =
                    std::mem::DropGuard::new(outer.clone(), |__p| __p.as_pointer().destructor());
                {
                    let j: Value<i32> = Rc::new(RefCell::new(0));
                    'loop_: while ((*j.borrow()) < 2) {
                        'for_body: {
                            let inner: Value<Guard> = Rc::new(RefCell::new(Guard { digit: 5 }));
                            let _dtor_inner = std::mem::DropGuard::new(inner.clone(), |__p| {
                                __p.as_pointer().destructor()
                            });
                            if ((*j.borrow()) == 0) {
                                break 'for_body;
                            }
                            break 'loop_;
                        }
                        ({ step_0(j.as_pointer()) });
                    }
                }
                break 'for_body;
            }
            ({ step_0(i.as_pointer()) });
        }
    }
    printf!(b"\ninner while/do/range: ");
    {
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < 1) {
            'for_body: {
                let outer: Value<Guard> = Rc::new(RefCell::new(Guard { digit: 4 }));
                let _dtor_outer =
                    std::mem::DropGuard::new(outer.clone(), |__p| __p.as_pointer().destructor());
                let mut j: i32 = 0;
                'loop_: while ((*j.prefix_inc()) < 3) {
                    let inner: Value<Guard> = Rc::new(RefCell::new(Guard { digit: 5 }));
                    let _dtor_inner = std::mem::DropGuard::new(inner.clone(), |__p| {
                        __p.as_pointer().destructor()
                    });
                    if (j == 1) {
                        continue 'loop_;
                    }
                    break;
                }
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    let inner: Value<Guard> = Rc::new(RefCell::new(Guard { digit: 6 }));
                    let _dtor_inner = std::mem::DropGuard::new(inner.clone(), |__p| {
                        __p.as_pointer().destructor()
                    });
                    continue 'loop_;
                }
                let values: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2])));
                'loop_: for value in
                    IterRange::new(values.decay(), values.decay().offset((2_i64) as isize))
                {
                    let mut value: i32 = (value.read());
                    let inner: Value<Guard> = Rc::new(RefCell::new(Guard { digit: value }));
                    let _dtor_inner = std::mem::DropGuard::new(inner.clone(), |__p| {
                        __p.as_pointer().destructor()
                    });
                    if (value == 1) {
                        continue 'loop_;
                    }
                    break;
                }
                break 'for_body;
            }
            ({ step_0(i.as_pointer()) });
        }
    }
    printf!(b"\ncondition: ");
    {
        let mut i: i32 = 2;
        'loop_: loop {
            let condition: Value<Guard> = Rc::new(RefCell::new(Guard { digit: i }));
            let _dtor_condition =
                std::mem::DropGuard::new(condition.clone(), |__p| __p.as_pointer().destructor());
            if !({ GuardImpl::to_bool(&condition.as_pointer()) }) {
                break;
            }
            'for_body: {
                let body: Value<Guard> = Rc::new(RefCell::new(Guard { digit: 4 }));
                let _dtor_body =
                    std::mem::DropGuard::new(body.clone(), |__p| __p.as_pointer().destructor());
                if (i == 2) {
                    break 'for_body;
                }
            }
            {
                i.prefix_dec();
                printf!(b"inc%d ", { (*condition.borrow()).digit })
            };
        }
    }
    printf!(b"\ninit/noncompound: ");
    let i: Value<i32> = Rc::new(RefCell::new(0));
    {
        let init: Value<Guard> = Rc::new(RefCell::new(Guard { digit: 5 }));
        let _dtor_init =
            std::mem::DropGuard::new(init.clone(), |__p| __p.as_pointer().destructor());
        'loop_: while ((*i.borrow()) < 2) {
            'for_body: {
                let body: Value<Guard> = Rc::new(RefCell::new(Guard { digit: 6 }));
                let _dtor_body =
                    std::mem::DropGuard::new(body.clone(), |__p| __p.as_pointer().destructor());
            }
            ({ step_0(i.as_pointer()) });
        }
    }
    printf!(b"\nno increment: ");
    {
        let mut j: i32 = 0;
        'loop_: while (j < 2) {
            let g: Value<Guard> = Rc::new(RefCell::new(Guard {
                digit: (*j.prefix_inc()),
            }));
            let _dtor_g = std::mem::DropGuard::new(g.clone(), |__p| __p.as_pointer().destructor());
            continue 'loop_;
        }
    }
    printf!(b"\n");
    return 0;
}
pub trait GuardImpl {
    fn destructor(&self);
    fn to_bool(&self) -> bool;
}
impl GuardImpl for Ptr<Guard> {
    fn destructor(&self) {
        printf!(b"%d ", (*self).with(|__s| __s.digit));
    }
    fn to_bool(&self) -> bool {
        return ((*self).with(|__s| __s.digit) != 0);
    }
}
pub fn __cpp2rust_init_globals() {}
