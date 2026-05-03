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
    let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([10, 11, 12, 13, 14, 15, 16, 17])));
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new((arr.as_pointer() as Ptr<i32>)));
    let q: Value<Ptr<i32>> = Rc::new(RefCell::new((*p.borrow()).offset((1) as isize)));
    assert!((((*q.borrow()).read()) == 11));
    let r: Value<Ptr<i32>> = Rc::new(RefCell::new((*p.borrow()).offset((3) as isize)));
    assert!((((*r.borrow()).read()) == 13));
    let s: Value<Ptr<i32>> = Rc::new(RefCell::new((*r.borrow()).offset(-((2) as isize))));
    assert!((((*s.borrow()).read()) == 11));
    let diff: Value<i64> = Rc::new(RefCell::new(
        ((*r.borrow()).clone() - (*p.borrow()).clone()) as i64,
    ));
    assert!(((*diff.borrow()) == 3_i64));
    let idx: Value<u64> = Rc::new(RefCell::new(
        (((((*r.borrow()).clone() - (*p.borrow()).clone()) as i64) as i64) as u64),
    ));
    assert!(((*idx.borrow()) == 3_u64));
    let q2: Value<Ptr<i32>> = Rc::new(RefCell::new((*p.borrow()).clone()));
    (*q2.borrow_mut()).prefix_inc();
    assert!((((*q2.borrow()).read()) == 11));
    (*q2.borrow_mut()).postfix_inc();
    assert!((((*q2.borrow()).read()) == 12));
    (*q2.borrow_mut()).prefix_dec();
    assert!((((*q2.borrow()).read()) == 11));
    (*q2.borrow_mut()).postfix_dec();
    assert!((((*q2.borrow()).read()) == 10));
    assert!({
        let _lhs = (*q2.borrow()).clone();
        _lhs == (*p.borrow()).clone()
    });
    let q3: Value<Ptr<i32>> = Rc::new(RefCell::new((*p.borrow()).clone()));
    (*q3.borrow_mut()) += 4;
    assert!((((*q3.borrow()).read()) == 14));
    (*q3.borrow_mut()) -= 2;
    assert!((((*q3.borrow()).read()) == 12));
    let step: Value<u64> = Rc::new(RefCell::new(2_u64));
    let q4: Value<Ptr<i32>> = Rc::new(RefCell::new(
        (*p.borrow()).offset((*step.borrow()) as isize),
    ));
    assert!((((*q4.borrow()).read()) == 12));
    let v: Value<i32> = Rc::new(RefCell::new(((*p.borrow()).offset((3) as isize).read())));
    assert!(((*v.borrow()) == 13));
    let v2: Value<i32> = Rc::new(RefCell::new((((*p.borrow()).offset((4) as isize)).read())));
    assert!(((*v2.borrow()) == 14));
    ((*p.borrow()).offset((5) as isize)).write(99);
    assert!((((*p.borrow()).offset((5) as isize).read()) == 99));
    assert!(((*arr.borrow())[(5) as usize] == 99));
    let end: Value<Ptr<i32>> = Rc::new(RefCell::new((*p.borrow()).offset((8) as isize)));
    let sum: Value<i32> = Rc::new(RefCell::new(0));
    let it: Value<Ptr<i32>> = Rc::new(RefCell::new((*p.borrow()).clone()));
    'loop_: while {
        let _lhs = (*it.borrow()).clone();
        _lhs != (*end.borrow()).clone()
    } {
        let __rhs = ((*it.borrow()).read());
        (*sum.borrow_mut()) += __rhs;
        (*it.borrow_mut()).prefix_inc();
    }
    assert!(((*sum.borrow()) == (((((((10 + 11) + 12) + 13) + 14) + 99) + 16) + 17)));
    let bytes: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        0_u8, 1_u8, 2_u8, 3_u8, 4_u8, 5_u8, 6_u8, 7_u8,
    ])));
    let bp: Value<Ptr<u8>> = Rc::new(RefCell::new((bytes.as_pointer() as Ptr<u8>)));
    let bq: Value<Ptr<u8>> = Rc::new(RefCell::new((*bp.borrow()).offset((4) as isize)));
    assert!(((((*bq.borrow()).read()) as i32) == 4));
    let bdiff: Value<i64> = Rc::new(RefCell::new(
        ((*bq.borrow()).clone() - (*bp.borrow()).clone()) as i64,
    ));
    assert!(((*bdiff.borrow()) == 4_i64));
    let cp: Value<Ptr<i32>> = Rc::new(RefCell::new((arr.as_pointer() as Ptr<i32>)));
    let cq: Value<Ptr<i32>> = Rc::new(RefCell::new((*cp.borrow()).offset((2) as isize)));
    assert!((((*cq.borrow()).read()) == 12));
    let cdiff: Value<i64> = Rc::new(RefCell::new(
        ((*cq.borrow()).clone() - (*cp.borrow()).clone()) as i64,
    ));
    assert!(((*cdiff.borrow()) == 2_i64));
    let n: Value<u64> = Rc::new(RefCell::new(3_u64));
    let q5: Value<Ptr<i32>> = Rc::new(RefCell::new(
        (arr.as_pointer() as Ptr<i32>).offset((*n.borrow()) as isize),
    ));
    assert!((((*q5.borrow()).read()) == 13));
    let q6: Value<Ptr<i32>> = Rc::new(RefCell::new(
        ((arr.as_pointer() as Ptr<i32>).offset((*n.borrow()) as isize)),
    ));
    assert!({
        let _lhs = (*q6.borrow()).clone();
        _lhs == (*q5.borrow()).clone()
    });
    let matrix: Value<Box<[Value<Box<[i32]>>]>> = Rc::new(RefCell::new(Box::new([
        Rc::new(RefCell::new(Box::new([0, 1, 2, 3]))),
        Rc::new(RefCell::new(Box::new([4, 5, 6, 7]))),
        Rc::new(RefCell::new(Box::new([8, 9, 10, 11]))),
    ])));
    let row1: Value<Ptr<i32>> = Rc::new(RefCell::new(
        ((((matrix.as_pointer() as Ptr<Value<Box<[i32]>>>)
            .offset(1 as isize)
            .read()
            .as_pointer()) as Ptr<i32>)
            .offset(0 as isize)),
    ));
    assert!((((*row1.borrow()).offset((2) as isize).read()) == 6));
    let back: Value<Ptr<i32>> = Rc::new(RefCell::new((*end.borrow()).offset(-((1) as isize))));
    assert!((((*back.borrow()).read()) == 17));
    return 0;
}
