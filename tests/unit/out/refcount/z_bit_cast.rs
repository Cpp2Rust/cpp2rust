extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn decay_cast_0(a1: Ptr<u32>) {
    let a1: Value<Ptr<u32>> = Rc::new(RefCell::new(a1));
}
pub fn bit_cast_1(p: AnyPtr) {
    let p: Value<AnyPtr> = Rc::new(RefCell::new(p));
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a1: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([1_u32, 2_u32, 3_u32])));
    ({
        let _a1: Ptr<u32> = (a1.as_pointer() as Ptr<u32>);
        decay_cast_0(_a1)
    });
    ({
        let _a1: Ptr<u32> = ((a1.as_pointer() as Ptr<u32>).offset(0 as isize));
        decay_cast_0(_a1)
    });
    ({
        let _p: AnyPtr = ((a1.as_pointer() as Ptr<u32>) as Ptr<u32>).to_any();
        bit_cast_1(_p)
    });
    ({
        let _p: AnyPtr = (((a1.as_pointer() as Ptr<u32>).offset(0 as isize)) as Ptr<u32>).to_any();
        bit_cast_1(_p)
    });
    ({
        let _p: AnyPtr = ((a1.as_pointer()) as Ptr<u32>).to_any();
        bit_cast_1(_p)
    });
    let ptr: Value<AnyPtr> = Rc::new(RefCell::new(
        ((a1.as_pointer() as Ptr<u32>) as Ptr<u32>).to_any(),
    ));
    assert!({
        let _lhs = (*ptr.borrow()).clone();
        _lhs == ((a1.as_pointer() as Ptr<u32>) as Ptr<u32>).to_any()
    });
    assert!({
        let _lhs = (((*ptr.borrow()).cast::<u32>().expect("ub:wrong type"))
            .offset((0) as isize)
            .read());
        _lhs == (*a1.borrow())[(0) as usize]
    });
    return 0;
}
