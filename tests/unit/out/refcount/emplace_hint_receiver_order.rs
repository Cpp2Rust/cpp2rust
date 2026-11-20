extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn receiver_0(
    mut m: Ptr<Map<i32, (Value<i32>, Value<Vec<i32>>)>>,
    mut v: Ptr<Vec<i32>>,
    mut calls: Ptr<i32>,
) -> Ptr<Map<i32, (Value<i32>, Value<Vec<i32>>)>> {
    calls.with_mut(|__v| {
        __v.prefix_inc();
    });
    index!(v, 2_usize).write(9);
    return m;
}
pub fn hint_1(
    mut m: Ptr<Map<i32, (Value<i32>, Value<Vec<i32>>)>>,
    mut v: Ptr<Vec<i32>>,
    mut calls: Ptr<i32>,
) -> BidirectionalIterator<(Value<i32>, Value<Vec<i32>>)> {
    calls.with_mut(|__v| {
        __v.prefix_inc();
    });
    index!(v, 2_usize).write(7);
    return m.with(|__m: &Map<i32, (Value<i32>, Value<Vec<i32>>)>| __m.end());
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    {
        let mut select_receiver: i32 = 1;
        'loop_: while (select_receiver >= 0) {
            let m: Value<Map<i32, (Value<i32>, Value<Vec<i32>>)>> =
                Rc::new(RefCell::new(Map::new()));
            let v: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([1, 2, 3])));
            let a: Value<i32> = Rc::new(RefCell::new(0));
            let b: Value<i32> = Rc::new(RefCell::new(0));
            if (select_receiver != 0) {
                {
                    let mut __tmp_0 =
                        { ({ receiver_0((m.as_pointer()), (v.as_pointer()), (a.as_pointer())) }) };
                    let mut __tmp_1 = {
                        <BidirectionalIterator<(Value<i32>, Value<Vec<i32>>)>>::from(
                            ({ hint_1((m.as_pointer()), (v.as_pointer()), (b.as_pointer())) }),
                        )
                    };
                    let mut __tmp_2 = {
                        (
                            Rc::new(RefCell::new(1)),
                            Rc::new(RefCell::new(({ (*v.borrow()).clone() }))),
                        )
                    };
                    let _ = (__tmp_1).clone();
                    Map::emplace((__tmp_0).clone(), &__tmp_2.0.as_pointer(), || __tmp_2).0
                };
            } else {
                {
                    let mut __tmp_3 = {
                        <BidirectionalIterator<(Value<i32>, Value<Vec<i32>>)>>::from(
                            ({ hint_1((m.as_pointer()), (v.as_pointer()), (b.as_pointer())) }),
                        )
                    };
                    let mut __tmp_4 = {
                        (
                            Rc::new(RefCell::new(1)),
                            Rc::new(RefCell::new(({ (*v.borrow()).clone() }))),
                        )
                    };
                    let _ = (__tmp_3).clone();
                    Map::emplace(m.as_pointer(), &__tmp_4.0.as_pointer(), || __tmp_4).0
                };
            }
            let inserted: Ptr<Vec<i32>> = ({
                let __tmp_5: Value<i32> = Rc::new(RefCell::new(1));
                (*m.borrow())
                    .get(&__tmp_5.as_pointer())
                    .expect("out of range!")
                    .borrow()
                    .1
                    .as_pointer()
            });
            printf!(
                b"receiver=%d hint=%d source=%d inserted=%d\n",
                (*a.borrow()),
                (*b.borrow()),
                { (*v.borrow())[2_usize] },
                (index!(inserted, 2_usize).read())
            );
            select_receiver.prefix_dec();
        }
    }
    0
}
pub fn __cpp2rust_init_globals() {}
