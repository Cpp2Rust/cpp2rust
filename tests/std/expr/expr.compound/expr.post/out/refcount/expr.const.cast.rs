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
pub struct p4_reference_const_cast_Pair {
    #[offset(0)]
    pub x: i32,
}
pub fn take_0(r: Ptr<i32>) {
    r.write(8);
}
pub fn test_1() {
    let v: Value<i32> = Rc::new(RefCell::new(1));
    let cr: Ptr<i32> = v.as_pointer();
    cr.write(2);
    assert!(({ ((*v.borrow()) == 2) }));
    ({ take_0((cr).clone()) });
    assert!(({ ((*v.borrow()) == 8) }));
    let pair: Value<p4_reference_const_cast_Pair> =
        Rc::new(RefCell::new(p4_reference_const_cast_Pair { x: 1 }));
    let cpair: Ptr<p4_reference_const_cast_Pair> = pair.as_pointer();
    field!(cpair, x).write(2);
    assert!(({ ({ (*pair.borrow()).x } == 2) }));
    let vec_: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([1])));
    let cvec: Ptr<Vec<i32>> = vec_.as_pointer();
    {
        let __a1 = 2;
        cvec.with_mut(|__v: &mut Vec<i32>| __v.push(__a1))
    };
    assert!(({ ((*vec_.borrow()).len() == 2_usize) }));
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    ({ test_1() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
