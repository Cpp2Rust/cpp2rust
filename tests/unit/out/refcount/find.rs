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
    let v_begin: Value<Ptr<i32>> = Rc::new(RefCell::new((v.as_pointer() as Ptr<i32>)));
    let v_end: Value<Ptr<i32>> = Rc::new(RefCell::new((v.as_pointer() as Ptr<i32>).to_end()));
    let it: Value<Ptr<i32>> = Rc::new(RefCell::new({
        let count = ((*v_end.borrow()).get_offset() - (*v_begin.borrow()).get_offset()) as usize;
        (*v_begin.borrow()).offset(
            (*v_begin.borrow())
                .clone()
                .into_iter()
                .take(count)
                .position(|value_0| value_0.read() == 2)
                .unwrap_or(count) as isize,
        )
    }));
    let mut v_result_true: bool = (*it.borrow()) != (v.as_pointer() as Ptr<i32>).to_end();
    let m: Value<BTreeMap<i32, Value<f64>>> = Rc::new(RefCell::new(BTreeMap::new()));
    (m.as_pointer() as Ptr<BTreeMap<i32, Value<f64>>>)
        .with_mut(|__v: &mut BTreeMap<i32, Value<f64>>| {
            __v.entry(1)
                .or_insert_with(|| Rc::new(RefCell::new(<f64>::default())))
                .as_pointer()
        })
        .write(1_f64);
    (m.as_pointer() as Ptr<BTreeMap<i32, Value<f64>>>)
        .with_mut(|__v: &mut BTreeMap<i32, Value<f64>>| {
            __v.entry(2)
                .or_insert_with(|| Rc::new(RefCell::new(<f64>::default())))
                .as_pointer()
        })
        .write(2_f64);
    (m.as_pointer() as Ptr<BTreeMap<i32, Value<f64>>>)
        .with_mut(|__v: &mut BTreeMap<i32, Value<f64>>| {
            __v.entry(3)
                .or_insert_with(|| Rc::new(RefCell::new(<f64>::default())))
                .as_pointer()
        })
        .write(3_f64);
    let m_begin: Value<RefcountMapIter<i32, f64>> = Rc::new(RefCell::new(RefcountMapIter::begin(
        (m.as_pointer() as Ptr<BTreeMap<i32, Value<f64>>>),
    )));
    let m_end: Value<RefcountMapIter<i32, f64>> = Rc::new(RefCell::new(RefcountMapIter::end(
        (m.as_pointer() as Ptr<BTreeMap<i32, Value<f64>>>),
    )));
    let mut m_result_true: bool = (*m_begin.borrow()) != (*m_end.borrow());
    assert!(
        ((v_result_true) && (m_result_true))
            && ({
                let count = ((v.as_pointer() as Ptr<i32>).get_offset()
                    - (v.as_pointer() as Ptr<i32>).get_offset())
                    as usize;
                (v.as_pointer() as Ptr<i32>).offset(
                    (v.as_pointer() as Ptr<i32>)
                        .clone()
                        .into_iter()
                        .take(count)
                        .position(|value_0| value_0.read() == 2)
                        .unwrap_or(count) as isize,
                )
            } == (v.as_pointer() as Ptr<i32>))
    );
    let w: Value<Vec<i32>> = Rc::new(RefCell::new(vec![1, 2, 3, 4, 5]));
    assert!(
        {
            let count = ((w.as_pointer() as Ptr<i32>).to_end().get_offset()
                - (w.as_pointer() as Ptr<i32>)
                    .offset(2_i64 as isize)
                    .get_offset()) as usize;
            (w.as_pointer() as Ptr<i32>).offset(2_i64 as isize).offset(
                (w.as_pointer() as Ptr<i32>)
                    .offset(2_i64 as isize)
                    .clone()
                    .into_iter()
                    .take(count)
                    .position(|value_0| value_0.read() == 4)
                    .unwrap_or(count) as isize,
            )
        } == (w.as_pointer() as Ptr<i32>).offset(3_i64 as isize)
    );
    assert!(
        {
            let count = ((w.as_pointer() as Ptr<i32>).to_end().get_offset()
                - (w.as_pointer() as Ptr<i32>)
                    .offset(2_i64 as isize)
                    .get_offset()) as usize;
            (w.as_pointer() as Ptr<i32>).offset(2_i64 as isize).offset(
                (w.as_pointer() as Ptr<i32>)
                    .offset(2_i64 as isize)
                    .clone()
                    .into_iter()
                    .take(count)
                    .position(|value_0| value_0.read() == 1)
                    .unwrap_or(count) as isize,
            )
        } == (w.as_pointer() as Ptr<i32>).to_end()
    );
    assert!(
        {
            let count = ((w.as_pointer() as Ptr<i32>)
                .offset(4_i64 as isize)
                .get_offset()
                - (w.as_pointer() as Ptr<i32>)
                    .offset(2_i64 as isize)
                    .get_offset()) as usize;
            (w.as_pointer() as Ptr<i32>).offset(2_i64 as isize).offset(
                (w.as_pointer() as Ptr<i32>)
                    .offset(2_i64 as isize)
                    .clone()
                    .into_iter()
                    .take(count)
                    .position(|value_0| value_0.read() == 5)
                    .unwrap_or(count) as isize,
            )
        } == (w.as_pointer() as Ptr<i32>).offset(4_i64 as isize)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
