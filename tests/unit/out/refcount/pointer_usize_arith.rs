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
    let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([10, 11, 12, 13, 14, 15, 16, 17])));
    let mut p: Ptr<i32> = (arr.as_pointer() as Ptr<i32>);
    let mut q: Ptr<i32> = p.offset((1) as isize);
    assert!(((q.read()) == 11));
    let mut r: Ptr<i32> = p.offset((3) as isize);
    assert!(((r.read()) == 13));
    let mut s: Ptr<i32> = r.offset(-((2) as isize));
    assert!(((s.read()) == 11));
    let mut diff: i64 = ((r).clone() - (p).clone()) as i64;
    assert!((diff == 3_i64));
    let mut idx: usize = ((((r).clone() - (p).clone()) as i64) as usize);
    assert!((idx == 3_usize));
    let mut q2: Ptr<i32> = (p).clone();
    q2.prefix_inc();
    assert!(((q2.read()) == 11));
    q2.postfix_inc();
    assert!(((q2.read()) == 12));
    q2.prefix_dec();
    assert!(((q2.read()) == 11));
    q2.postfix_dec();
    assert!(((q2.read()) == 10));
    assert!(({ (q2).clone() } == { (p).clone() }));
    let mut q3: Ptr<i32> = (p).clone();
    q3 += 4;
    assert!(((q3.read()) == 14));
    q3 -= 2;
    assert!(((q3.read()) == 12));
    let mut step: usize = 2_usize;
    let mut q4: Ptr<i32> = p.offset((step) as isize);
    assert!(((q4.read()) == 12));
    let mut v: i32 = (elem!(p, 3).read());
    assert!((v == 13));
    let mut v2: i32 = ((p.offset((4) as isize)).read());
    assert!((v2 == 14));
    (p.offset((5) as isize)).write(99);
    assert!(((elem!(p, 5).read()) == 99));
    assert!(((*arr.borrow())[(5) as usize] == 99));
    let mut end: Ptr<i32> = p.offset((8) as isize);
    let mut sum: i32 = 0;
    let mut it: Ptr<i32> = (p).clone();
    'loop_: while ({ (it).clone() } != { (end).clone() }) {
        sum += { (it.read()) };
        it.prefix_inc();
    }
    assert!((sum == (((((((10 + 11) + 12) + 13) + 14) + 99) + 16) + 17)));
    let bytes: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        0_u8, 1_u8, 2_u8, 3_u8, 4_u8, 5_u8, 6_u8, 7_u8,
    ])));
    let mut bp: Ptr<u8> = (bytes.as_pointer() as Ptr<u8>);
    let mut bq: Ptr<u8> = bp.offset((4) as isize);
    assert!((((bq.read()) as i32) == 4));
    let mut bdiff: i64 = ((bq).clone() - (bp).clone()) as i64;
    assert!((bdiff == 4_i64));
    let mut cp: Ptr<i32> = (arr.as_pointer() as Ptr<i32>);
    let mut cq: Ptr<i32> = cp.offset((2) as isize);
    assert!(((cq.read()) == 12));
    let mut cdiff: i64 = ((cq).clone() - (cp).clone()) as i64;
    assert!((cdiff == 2_i64));
    let mut n: usize = 3_usize;
    let mut q5: Ptr<i32> = (arr.as_pointer() as Ptr<i32>).offset((n) as isize);
    assert!(((q5.read()) == 13));
    let mut q6: Ptr<i32> = ((arr.as_pointer() as Ptr<i32>).offset(n));
    assert!(({ (q6).clone() } == { (q5).clone() }));
    let matrix: Value<Box<[Value<Box<[i32]>>]>> = Rc::new(RefCell::new(Box::new([
        Rc::new(RefCell::new(Box::new([0, 1, 2, 3]))),
        Rc::new(RefCell::new(Box::new([4, 5, 6, 7]))),
        Rc::new(RefCell::new(Box::new([8, 9, 10, 11]))),
    ])));
    let mut row1: Ptr<i32> = ((((matrix.as_pointer() as Ptr<Value<Box<[i32]>>>)
        .offset(1)
        .read()
        .as_pointer()) as Ptr<i32>)
        .offset(0));
    assert!(((elem!(row1, 2).read()) == 6));
    let mut back: Ptr<i32> = end.offset(-((1) as isize));
    assert!(((back.read()) == 17));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
