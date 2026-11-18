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
    libcc2rs::cpp2rust_exit(main_0());
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
        .with_mut(|__v: &mut Vec<Vec<Vec<i8>>>| __v.push(({ (*row.borrow()).clone() })));
    {
        let __a1 = {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"def").to_c_bytes();
            __bytes.push(0);
            __bytes
        };
        (*row.borrow_mut()).push(__a1)
    };
    elem!(v.decay(), 1_usize)
        .with_mut(|__v: &mut Vec<Vec<Vec<i8>>>| __v.push(({ (*row.borrow()).clone() })));
    {
        let __init = Vec::new();
        elem!(v.decay(), 1_usize).with_mut(|__v: &mut Vec<Vec<Vec<i8>>>| __v.push(__init))
    };
    match (match (v.decay().offset(1_usize),) {
        (__tmp_1,) => {
            let __n = (*__tmp_1.upgrade().deref()).len() - 1;
            __tmp_1.decay().offset((__n) as isize)
        }
    },)
    {
        (__tmp_0,) => {
            let __a1 = {
                let mut r = ({
                    (*match (v.decay().offset(0_usize),) {
                        (__tmp_2,) => __tmp_2.decay(),
                    }
                    .decay()
                    .offset(1_usize)
                    .upgrade()
                    .deref())
                    .clone()
                });
                r.pop();
                Ptr::<i8>::from_string_literal(b"x").with_c_str(|__s| r.extend_from_slice(__s));
                r.push(0);
                r
            };
            __tmp_0.with_mut(|__v: &mut Vec<Vec<i8>>| __v.push(__a1))
        }
    };
    assert!(
        ({
            ((*match (v.decay().offset(0_usize),) {
                (__tmp_3,) => __tmp_3.decay(),
            }
            .upgrade()
            .deref())
            .len()
                == 2_usize)
        })
    );
    assert!(
        ({
            ((*match (v.decay().offset(1_usize),) {
                (__tmp_4,) => __tmp_4.decay(),
            }
            .upgrade()
            .deref())
            .len()
                == 3_usize)
                && (match (match (v.decay().offset(1_usize),) {
                    (__tmp_6,) => __tmp_6.decay(),
                }
                .decay()
                .offset(2_usize),)
                {
                    (__tmp_5,) => Ptr::<i8>::from_string_literal(b"def").with_c_str(|__s| {
                        (*__tmp_5.upgrade().deref())
                            [..(*__tmp_5.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                })
        })
    );
    assert!(
        ({
            match (match (v.decay().offset(1_usize),) {
                (__tmp_8,) => {
                    let __n = (*__tmp_8.upgrade().deref()).len() - 1;
                    __tmp_8.decay().offset((__n) as isize)
                }
            }
            .decay()
            .offset(0_usize),)
            {
                (__tmp_7,) => Ptr::<i8>::from_string_literal(b"bcx").with_c_str(|__s| {
                    (*__tmp_7.upgrade().deref())
                        [..(*__tmp_7.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                }),
            }
        })
    );
    let copy: Value<Vec<Vec<Vec<Vec<i8>>>>> = Rc::new(RefCell::new((*v.borrow()).clone()));
    match (
        match (copy.decay().offset(0_usize),) {
            (__tmp_10,) => __tmp_10.decay(),
        }
        .decay()
        .offset(0_usize),
        Rc::new(RefCell::new({
            let mut __bytes = Ptr::<i8>::from_string_literal(b"z").to_c_bytes();
            __bytes.push(0);
            __bytes
        }))
        .as_pointer(),
    ) {
        (__tmp_9, __tmp_11) => __tmp_9.with_mut(|__b0| {
            (*__b0) = __tmp_11.with_mut(|__v| std::mem::replace(__v, vec![0]));
        }),
    };
    match (match (v.decay().offset(0_usize),) {
        (__tmp_13,) => __tmp_13.decay(),
    }
    .decay()
    .offset(1_usize),)
    {
        (__tmp_12,) => {
            if 0_usize >= (*__tmp_12.upgrade().deref()).len() - 1 {
                panic!("out of bounds access")
            }
            __tmp_12.decay().offset((0_usize) as isize)
        }
    }
    .write(('B' as i8));
    assert!(
        ({
            (match (match (v.decay().offset(0_usize),) {
                (__tmp_15,) => __tmp_15.decay(),
            }
            .decay()
            .offset(0_usize),)
            {
                (__tmp_14,) => Ptr::<i8>::from_string_literal(b"a").with_c_str(|__s| {
                    (*__tmp_14.upgrade().deref())
                        [..(*__tmp_14.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                }),
            }) && (match (match (copy.decay().offset(0_usize),) {
                (__tmp_17,) => __tmp_17.decay(),
            }
            .decay()
            .offset(0_usize),)
            {
                (__tmp_16,) => Ptr::<i8>::from_string_literal(b"z").with_c_str(|__s| {
                    (*__tmp_16.upgrade().deref())
                        [..(*__tmp_16.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                }),
            })
        })
    );
    assert!(
        ({
            (match (match (v.decay().offset(0_usize),) {
                (__tmp_19,) => __tmp_19.decay(),
            }
            .decay()
            .offset(1_usize),)
            {
                (__tmp_18,) => Ptr::<i8>::from_string_literal(b"Bc").with_c_str(|__s| {
                    (*__tmp_18.upgrade().deref())
                        [..(*__tmp_18.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                }),
            }) && (match (match (copy.decay().offset(0_usize),) {
                (__tmp_21,) => __tmp_21.decay(),
            }
            .decay()
            .offset(1_usize),)
            {
                (__tmp_20,) => Ptr::<i8>::from_string_literal(b"bc").with_c_str(|__s| {
                    (*__tmp_20.upgrade().deref())
                        [..(*__tmp_20.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                }),
            })
        })
    );
    elem!(v.decay(), 1_usize).with_mut(|__v: &mut Vec<Vec<Vec<i8>>>| __v.remove(0));
    assert!(
        ({
            match (match (v.decay().offset(1_usize),) {
                (__tmp_23,) => __tmp_23.decay(),
            }
            .decay()
            .offset(0_usize),)
            {
                (__tmp_22,) => Ptr::<i8>::from_string_literal(b"bcx").with_c_str(|__s| {
                    (*__tmp_22.upgrade().deref())
                        [..(*__tmp_22.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                }),
            }
        })
    );
    assert!(
        ({
            ((*match (copy.decay().offset(1_usize),) {
                (__tmp_24,) => __tmp_24.decay(),
            }
            .upgrade()
            .deref())
            .len()
                == 3_usize)
        })
    );
    assert!(({ ((*row.borrow()).len() == 3_usize) }));
    let d: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    let mut first: i32 = 1;
    let mut last: i32 = 2;
    (*d.borrow_mut()).push(first);
    (*d.borrow_mut()).push(last);
    let r: Ptr<i32> = d.decay();
    r.write(5);
    let c: Ptr<i32> = {
        let __n = (*d.borrow()).len() - 1;
        d.decay().offset((__n) as isize)
    };
    assert!(((d.decay().read()) == 5) && ((c.read()) == 2));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
