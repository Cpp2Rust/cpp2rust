extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Copy, Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct P {
    #[offset(0)]
    pub x: i32,
}
impl CppEq for P {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        PImpl::operator_eq(__lhs, __rhs.clone())
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
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
            {
                let mut __tmp_0 = { (*v_end.borrow()).clone() };
                let mut __it = ({ (*v_begin.borrow()).clone() });
                while __it != __tmp_0 && !CppEq::equal(&__it.iter_deref(), &__tmp_1.as_pointer()) {
                    __it.iter_inc();
                }
                __it
            }
        }),
    ));
    let mut v_result_true: bool = (*it.borrow()) != v.decay().end();
    let m: Value<Map<i32, (Value<i32>, Value<f64>)>> = Rc::new(RefCell::new(Map::new()));
    match (Rc::new(RefCell::new(1)).as_pointer(),) {
        (__tmp_2,) => ({ (*m.borrow_mut()).subscript_move(__tmp_2, |__k| cpp_move!(__k)) }),
    }
    .write(1_f64);
    match (Rc::new(RefCell::new(2)).as_pointer(),) {
        (__tmp_3,) => ({ (*m.borrow_mut()).subscript_move(__tmp_3, |__k| cpp_move!(__k)) }),
    }
    .write(2_f64);
    match (Rc::new(RefCell::new(3)).as_pointer(),) {
        (__tmp_4,) => ({ (*m.borrow_mut()).subscript_move(__tmp_4, |__k| cpp_move!(__k)) }),
    }
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
                let __tmp_7: Value<i32> = Rc::new(RefCell::new(2));
                {
                    let mut __tmp_5 = { v.decay().begin() };
                    let mut __tmp_6 = { v.decay().begin() };
                    let mut __it = (__tmp_5).clone();
                    while __it != __tmp_6
                        && !CppEq::equal(&__it.iter_deref(), &__tmp_7.as_pointer())
                    {
                        __it.iter_inc();
                    }
                    __it
                }
            }) == v.decay().begin())
    );
    let w: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([1, 2, 3, 4, 5])));
    assert!(
        ({
            let __tmp_10: Value<i32> = Rc::new(RefCell::new(4));
            {
                let mut __tmp_8 = { w.decay().begin().iter_add(2_isize) };
                let mut __tmp_9 = { w.decay().end() };
                let mut __it = (__tmp_8).clone();
                while __it != __tmp_9 && !CppEq::equal(&__it.iter_deref(), &__tmp_10.as_pointer()) {
                    __it.iter_inc();
                }
                __it
            }
        }) == w.decay().begin().iter_add(3_isize)
    );
    assert!(
        ({
            let __tmp_13: Value<i32> = Rc::new(RefCell::new(1));
            {
                let mut __tmp_11 = { w.decay().begin().iter_add(2_isize) };
                let mut __tmp_12 = { w.decay().end() };
                let mut __it = (__tmp_11).clone();
                while __it != __tmp_12 && !CppEq::equal(&__it.iter_deref(), &__tmp_13.as_pointer())
                {
                    __it.iter_inc();
                }
                __it
            }
        }) == w.decay().end()
    );
    assert!(
        ({
            let __tmp_16: Value<i32> = Rc::new(RefCell::new(5));
            {
                let mut __tmp_14 = { w.decay().begin().iter_add(2_isize) };
                let mut __tmp_15 = { w.decay().begin().iter_add(4_isize) };
                let mut __it = (__tmp_14).clone();
                while __it != __tmp_15 && !CppEq::equal(&__it.iter_deref(), &__tmp_16.as_pointer())
                {
                    __it.iter_inc();
                }
                __it
            }
        }) == w.decay().begin().iter_add(4_isize)
    );
    let p: Value<Vec<P>> = Rc::new(RefCell::new(Vec::from([
        P { x: 0 },
        P { x: 1 },
        P { x: 2 },
    ])));
    assert!(
        ({
            let __tmp_19: Value<P> = Rc::new(RefCell::new(P { x: 1 }));
            {
                let mut __tmp_17 = { p.decay().begin() };
                let mut __tmp_18 = { p.decay().end() };
                let mut __it = (__tmp_17).clone();
                while __it != __tmp_18
                    && !CppEq::equal(&__it.iter_deref(), &(__tmp_19.as_pointer()))
                {
                    __it.iter_inc();
                }
                __it
            }
        }) == p.decay().begin().iter_add(1_isize)
    );
    return 0;
}
pub trait PImpl {
    fn operator_eq(&self, o: Ptr<P>) -> bool;
}
impl PImpl for Ptr<P> {
    fn operator_eq(&self, o: Ptr<P>) -> bool {
        return ({ (*self).with(|__s| __s.x) } == { o.with(|__s| __s.x) });
    }
}
pub fn __cpp2rust_init_globals() {}
