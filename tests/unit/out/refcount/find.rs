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
    let v_begin: Value<RandomAccessIterator<i32>> =
        Rc::new(RefCell::new((v.as_pointer() as Ptr<i32>).begin()));
    let v_end: Value<RandomAccessIterator<i32>> =
        Rc::new(RefCell::new((v.as_pointer() as Ptr<i32>).end()));
    let it: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new({
        let mut __it = (*v_begin.borrow()).clone();
        while __it != (*v_end.borrow()).clone() && __it.iter_deref().with(|__v| *__v != 2) {
            __it.iter_inc();
        }
        __it
    }));
    let mut v_result_true: bool = (*it.borrow()) != (v.as_pointer() as Ptr<i32>).end();
    let m: Value<Map<i32, (Value<i32>, Value<f64>)>> = Rc::new(RefCell::new(Map::new()));
    (m.as_pointer() as Ptr<Map<i32, (Value<i32>, Value<f64>)>>)
        .with_mut(|__m: &mut Map<i32, (Value<i32>, Value<f64>)>| {
            __m.get_or_insert_with(1.clone(), || {
                (
                    Rc::new(RefCell::new(1)),
                    Rc::new(RefCell::new(<f64>::default())),
                )
            })
            .borrow()
            .1
            .as_pointer()
        })
        .write(1_f64);
    (m.as_pointer() as Ptr<Map<i32, (Value<i32>, Value<f64>)>>)
        .with_mut(|__m: &mut Map<i32, (Value<i32>, Value<f64>)>| {
            __m.get_or_insert_with(2.clone(), || {
                (
                    Rc::new(RefCell::new(2)),
                    Rc::new(RefCell::new(<f64>::default())),
                )
            })
            .borrow()
            .1
            .as_pointer()
        })
        .write(2_f64);
    (m.as_pointer() as Ptr<Map<i32, (Value<i32>, Value<f64>)>>)
        .with_mut(|__m: &mut Map<i32, (Value<i32>, Value<f64>)>| {
            __m.get_or_insert_with(3.clone(), || {
                (
                    Rc::new(RefCell::new(3)),
                    Rc::new(RefCell::new(<f64>::default())),
                )
            })
            .borrow()
            .1
            .as_pointer()
        })
        .write(3_f64);
    let m_begin: Value<BidirectionalIterator<(Value<i32>, Value<f64>)>> = Rc::new(RefCell::new(
        (m.as_pointer() as Ptr<Map<i32, (Value<i32>, Value<f64>)>>)
            .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
    ));
    let m_end: Value<BidirectionalIterator<(Value<i32>, Value<f64>)>> = Rc::new(RefCell::new(
        (m.as_pointer() as Ptr<Map<i32, (Value<i32>, Value<f64>)>>)
            .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.end()),
    ));
    let mut m_result_true: bool = (*m_begin.borrow()) != (*m_end.borrow());
    assert!(
        ((v_result_true) && (m_result_true))
            && ({
                let mut __it = (v.as_pointer() as Ptr<i32>).begin();
                while __it != (v.as_pointer() as Ptr<i32>).begin()
                    && __it.iter_deref().with(|__v| *__v != 2)
                {
                    __it.iter_inc();
                }
                __it
            } == (v.as_pointer() as Ptr<i32>).begin())
    );
    let w: Value<Vec<i32>> = Rc::new(RefCell::new(vec![1, 2, 3, 4, 5]));
    assert!(
        {
            let mut __it = (w.as_pointer() as Ptr<i32>).begin().iter_add(2_isize);
            while __it != (w.as_pointer() as Ptr<i32>).end()
                && __it.iter_deref().with(|__v| *__v != 4)
            {
                __it.iter_inc();
            }
            __it
        } == (w.as_pointer() as Ptr<i32>).begin().iter_add(3_isize)
    );
    assert!(
        {
            let mut __it = (w.as_pointer() as Ptr<i32>).begin().iter_add(2_isize);
            while __it != (w.as_pointer() as Ptr<i32>).end()
                && __it.iter_deref().with(|__v| *__v != 1)
            {
                __it.iter_inc();
            }
            __it
        } == (w.as_pointer() as Ptr<i32>).end()
    );
    assert!(
        {
            let mut __it = (w.as_pointer() as Ptr<i32>).begin().iter_add(2_isize);
            while __it != (w.as_pointer() as Ptr<i32>).begin().iter_add(4_isize)
                && __it.iter_deref().with(|__v| *__v != 5)
            {
                __it.iter_inc();
            }
            __it
        } == (w.as_pointer() as Ptr<i32>).begin().iter_add(4_isize)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
