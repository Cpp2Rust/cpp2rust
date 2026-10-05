extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Point {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
pub fn agg_0(n: i32) -> i32 {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    let buf40: Value<Box<[i8]>> =
        Rc::new(RefCell::new((0..40).map(|_| 0_i8).collect::<Box<[i8]>>()));
    let buf256: Value<Box<[u8]>> =
        Rc::new(RefCell::new((0..256).map(|_| 0_u8).collect::<Box<[u8]>>()));
    let arr64: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..64).map(|_| 0_i32).collect::<Box<[i32]>>()));
    let longs: Value<Box<[i64]>> =
        Rc::new(RefCell::new((0..33).map(|_| 0_i64).collect::<Box<[i64]>>()));
    let p: Value<Point> = <Value<Point>>::default();
    let ptr: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::<i32>::null()));
    let fp: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(FnPtr::<fn(i32) -> i32>::null()));
    let file: Value<Ptr<CFile>> = Rc::new(RefCell::new(Ptr::null()));
    let total: Value<i32> = Rc::new(RefCell::new(0_i32));
    goto_block!({
        '__entry: {
            *total.borrow_mut() = 0;
            if ((((*n.borrow()) < 0) as i32) != 0) {
                goto!('out);
            }
            (*total.borrow_mut()) = 1;
        }
        'out: {
            return (*total.borrow());
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((((({ agg_0(-1_i32,) }) == 0) as i32) != 0));
    assert!((((({ agg_0(1,) }) == 1) as i32) != 0));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
