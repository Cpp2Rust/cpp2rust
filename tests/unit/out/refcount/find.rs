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
    let v: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    {
        let __a1 = 1;
        (*v.borrow_mut()).push(__a1)
    };
    {
        let __a1 = 2;
        (*v.borrow_mut()).push(__a1)
    };
    {
        let __a1 = 3;
        (*v.borrow_mut()).push(__a1)
    };
    let v_begin: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(v.decay().begin()));
    let v_end: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(v.decay().end()));
    let it: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(
        ({
            let __tmp_1: Value<i32> = Rc::new(RefCell::new(2));
            match ({ (*v_end.borrow()).clone() },) {
                (mut __tmp_0,) => {
                    let mut __it = ({ (*v_begin.borrow()).clone() });
                    while __it != __tmp_0
                        && !CppEq::equal(&__it.iter_deref(), &__tmp_1.as_pointer())
                    {
                        __it.iter_inc();
                    }
                    __it
                }
            }
        }),
    ));
    let mut v_result_true: bool = (*it.borrow()) != v.decay().end();
    let m: Value<Map<i32, (Value<i32>, Value<f64>)>> = Rc::new(RefCell::new(Map::new()));
    Map::subscript_move(
        m.as_pointer(),
        Rc::new(RefCell::new(1)).as_pointer(),
        |__k| cpp_move!(__k),
    )
    .write(1_f64);
    Map::subscript_move(
        m.as_pointer(),
        Rc::new(RefCell::new(2)).as_pointer(),
        |__k| cpp_move!(__k),
    )
    .write(2_f64);
    Map::subscript_move(
        m.as_pointer(),
        Rc::new(RefCell::new(3)).as_pointer(),
        |__k| cpp_move!(__k),
    )
    .write(3_f64);
    let m_begin: Value<BidirectionalIterator<(Value<i32>, Value<f64>)>> = Rc::new(RefCell::new(
        m.as_pointer()
            .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
    ));
    let m_end: Value<BidirectionalIterator<(Value<i32>, Value<f64>)>> = Rc::new(RefCell::new(
        m.as_pointer()
            .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.end()),
    ));
    let mut m_result_true: bool = (*m_begin.borrow()) != (*m_end.borrow());
    assert!(
        ((v_result_true) && (m_result_true))
            && (({
                let __tmp_4: Value<i32> = Rc::new(RefCell::new(2));
                match ({ v.decay().begin() }, { v.decay().begin() }) {
                    (mut __tmp_2, mut __tmp_3) => {
                        let mut __it = (__tmp_2).clone();
                        while __it != __tmp_3
                            && !CppEq::equal(&__it.iter_deref(), &__tmp_4.as_pointer())
                        {
                            __it.iter_inc();
                        }
                        __it
                    }
                }
            }) == v.decay().begin())
    );
    let w: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([1, 2, 3, 4, 5])));
    assert!(
        ({
            let __tmp_7: Value<i32> = Rc::new(RefCell::new(4));
            match ({ w.decay().begin().iter_add(2_isize) }, { w.decay().end() }) {
                (mut __tmp_5, mut __tmp_6) => {
                    let mut __it = (__tmp_5).clone();
                    while __it != __tmp_6
                        && !CppEq::equal(&__it.iter_deref(), &__tmp_7.as_pointer())
                    {
                        __it.iter_inc();
                    }
                    __it
                }
            }
        }) == w.decay().begin().iter_add(3_isize)
    );
    assert!(
        ({
            let __tmp_10: Value<i32> = Rc::new(RefCell::new(1));
            match ({ w.decay().begin().iter_add(2_isize) }, { w.decay().end() }) {
                (mut __tmp_8, mut __tmp_9) => {
                    let mut __it = (__tmp_8).clone();
                    while __it != __tmp_9
                        && !CppEq::equal(&__it.iter_deref(), &__tmp_10.as_pointer())
                    {
                        __it.iter_inc();
                    }
                    __it
                }
            }
        }) == w.decay().end()
    );
    assert!(
        ({
            let __tmp_13: Value<i32> = Rc::new(RefCell::new(5));
            match ({ w.decay().begin().iter_add(2_isize) }, {
                w.decay().begin().iter_add(4_isize)
            }) {
                (mut __tmp_11, mut __tmp_12) => {
                    let mut __it = (__tmp_11).clone();
                    while __it != __tmp_12
                        && !CppEq::equal(&__it.iter_deref(), &__tmp_13.as_pointer())
                    {
                        __it.iter_inc();
                    }
                    __it
                }
            }
        }) == w.decay().begin().iter_add(4_isize)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
