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
    let v: Value<Vec<Vec<Vec<Vec<i8>>>>> = Rc::new(RefCell::new(Vec::new()));
    {
        let __init = Vec::new();
        (*v.borrow_mut()).push(__init)
    };
    {
        let __init = Vec::new();
        (*v.borrow_mut()).push(__init)
    };
    let row: Value<Vec<Vec<i8>>> = Rc::new(RefCell::new(Vec::new()));
    {
        let __a1 = {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"a").to_c_bytes();
            __bytes.push(0);
            __bytes
        };
        (*row.borrow_mut()).push(__a1)
    };
    {
        let __a1 = {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"bc").to_c_bytes();
            __bytes.push(0);
            __bytes
        };
        (*row.borrow_mut()).push(__a1)
    };
    elem!(v.decay(), 0_usize)
        .with_mut(|__v: &mut Vec<Vec<Vec<i8>>>| __v.push((*row.borrow()).clone()));
    {
        let __a1 = {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"def").to_c_bytes();
            __bytes.push(0);
            __bytes
        };
        (*row.borrow_mut()).push(__a1)
    };
    elem!(v.decay(), 1_usize)
        .with_mut(|__v: &mut Vec<Vec<Vec<i8>>>| __v.push((*row.borrow()).clone()));
    {
        let __init = Vec::new();
        elem!(v.decay(), 1_usize).with_mut(|__v: &mut Vec<Vec<Vec<i8>>>| __v.push(__init))
    };
    match (v.decay().offset(1_usize).decay().to_last(),) {
        (__tmp_0,) => {
            let __a1 = {
                let mut r = (*v
                    .decay()
                    .offset(0_usize)
                    .decay()
                    .decay()
                    .offset(1_usize)
                    .upgrade()
                    .deref())
                .clone();
                r.pop();
                Ptr::<i8>::from_string_literal(b"x").with_c_str(|__s| r.extend_from_slice(__s));
                r.push(0);
                r
            };
            __tmp_0.with_mut(|__v: &mut Vec<Vec<i8>>| __v.push(__a1))
        }
    };
    assert!(((*v.decay().offset(0_usize).decay().upgrade().deref()).len() == 2_usize));
    assert!(
        ((*v.decay().offset(1_usize).decay().upgrade().deref()).len() == 3_usize)
            && (match (v.decay().offset(1_usize).decay().decay().offset(2_usize),) {
                (__tmp_1,) => {
                    Ptr::<i8>::from_string_literal(b"def").with_c_str(|__s| {
                        (*__tmp_1.upgrade().deref())
                            [..(*__tmp_1.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            })
    );
    assert!(match (v
        .decay()
        .offset(1_usize)
        .decay()
        .to_last()
        .decay()
        .offset(0_usize),)
    {
        (__tmp_2,) => {
            Ptr::<i8>::from_string_literal(b"bcx").with_c_str(|__s| {
                (*__tmp_2.upgrade().deref())[..(*__tmp_2.upgrade().deref()).len().saturating_sub(1)]
                    == *__s
            })
        }
    });
    let copy: Value<Vec<Vec<Vec<Vec<i8>>>>> = Rc::new(RefCell::new((*v.borrow()).clone()));
    match (Rc::new(RefCell::new({
        let mut __bytes = Ptr::<i8>::from_string_literal(b"z").to_c_bytes();
        __bytes.push(0);
        __bytes
    }))
    .as_pointer(),)
    {
        (__tmp_3,) => copy
            .decay()
            .offset(0_usize)
            .decay()
            .decay()
            .offset(0_usize)
            .write(__tmp_3.with_mut(std::mem::take)),
    };
    match (v.decay().offset(0_usize).decay().decay().offset(1_usize),) {
        (__tmp_4,) => {
            if 0_usize as usize >= (*__tmp_4.upgrade().deref()).len().saturating_sub(1) {
                panic!("out of bounds access")
            } else {
                __tmp_4.decay().offset(0_usize as isize)
            }
        }
    }
    .write(('B' as i8));
    assert!(
        (match (v.decay().offset(0_usize).decay().decay().offset(0_usize),) {
            (__tmp_5,) => {
                Ptr::<i8>::from_string_literal(b"a").with_c_str(|__s| {
                    (*__tmp_5.upgrade().deref())
                        [..(*__tmp_5.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        }) && (match (copy.decay().offset(0_usize).decay().decay().offset(0_usize),) {
            (__tmp_6,) => {
                Ptr::<i8>::from_string_literal(b"z").with_c_str(|__s| {
                    (*__tmp_6.upgrade().deref())
                        [..(*__tmp_6.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        })
    );
    assert!(
        (match (v.decay().offset(0_usize).decay().decay().offset(1_usize),) {
            (__tmp_7,) => {
                Ptr::<i8>::from_string_literal(b"Bc").with_c_str(|__s| {
                    (*__tmp_7.upgrade().deref())
                        [..(*__tmp_7.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        }) && (match (copy.decay().offset(0_usize).decay().decay().offset(1_usize),) {
            (__tmp_8,) => {
                Ptr::<i8>::from_string_literal(b"bc").with_c_str(|__s| {
                    (*__tmp_8.upgrade().deref())
                        [..(*__tmp_8.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        })
    );
    elem!(v.decay(), 1_usize).with_mut(|__v: &mut Vec<Vec<Vec<i8>>>| __v.remove(0));
    assert!(
        match (v.decay().offset(1_usize).decay().decay().offset(0_usize),) {
            (__tmp_9,) => {
                Ptr::<i8>::from_string_literal(b"bcx").with_c_str(|__s| {
                    (*__tmp_9.upgrade().deref())
                        [..(*__tmp_9.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                })
            }
        }
    );
    assert!(((*copy.decay().offset(1_usize).decay().upgrade().deref()).len() == 3_usize));
    assert!(((*row.borrow()).len() == 3_usize));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
