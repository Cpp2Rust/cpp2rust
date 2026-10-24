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
            let __tmp_0: Value<i32> = Rc::new(RefCell::new(2));
            {
                let mut __it = (*v_begin.borrow()).clone();
                let __last = (*v_end.borrow()).clone();
                let __value = __tmp_0.as_pointer();
                while __it != __last && !CppEq::equal(&__it.iter_deref(), &__value) {
                    __it.iter_inc();
                }
                __it
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
                let __tmp_1: Value<i32> = Rc::new(RefCell::new(2));
                {
                    let mut __it = v.decay().begin();
                    let __last = v.decay().begin();
                    let __value = __tmp_1.as_pointer();
                    while __it != __last && !CppEq::equal(&__it.iter_deref(), &__value) {
                        __it.iter_inc();
                    }
                    __it
                }
            }) == v.decay().begin())
    );
    let w: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([1, 2, 3, 4, 5])));
    assert!(
        ({
            let __tmp_2: Value<i32> = Rc::new(RefCell::new(4));
            {
                let mut __it = w.decay().begin().iter_add(2_isize);
                let __last = w.decay().end();
                let __value = __tmp_2.as_pointer();
                while __it != __last && !CppEq::equal(&__it.iter_deref(), &__value) {
                    __it.iter_inc();
                }
                __it
            }
        }) == w.decay().begin().iter_add(3_isize)
    );
    assert!(
        ({
            let __tmp_3: Value<i32> = Rc::new(RefCell::new(1));
            {
                let mut __it = w.decay().begin().iter_add(2_isize);
                let __last = w.decay().end();
                let __value = __tmp_3.as_pointer();
                while __it != __last && !CppEq::equal(&__it.iter_deref(), &__value) {
                    __it.iter_inc();
                }
                __it
            }
        }) == w.decay().end()
    );
    assert!(
        ({
            let __tmp_4: Value<i32> = Rc::new(RefCell::new(5));
            {
                let mut __it = w.decay().begin().iter_add(2_isize);
                let __last = w.decay().begin().iter_add(4_isize);
                let __value = __tmp_4.as_pointer();
                while __it != __last && !CppEq::equal(&__it.iter_deref(), &__value) {
                    __it.iter_inc();
                }
                __it
            }
        }) == w.decay().begin().iter_add(4_isize)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
