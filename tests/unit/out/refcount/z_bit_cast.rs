extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn decay_cast_0(mut a1: Ptr<u32>) {}
pub fn bit_cast_1(mut p: AnyPtr) {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a1: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([1_u32, 2_u32, 3_u32])));
    ({ decay_cast_0((a1.as_pointer() as Ptr<u32>)) });
    ({ decay_cast_0(((a1.as_pointer() as Ptr<u32>).offset(0))) });
    ({ bit_cast_1(((a1.as_pointer() as Ptr<u32>) as Ptr<u32>).to_any()) });
    ({ bit_cast_1((((a1.as_pointer() as Ptr<u32>).offset(0)) as Ptr<u32>).to_any()) });
    ({ bit_cast_1(((a1.as_pointer()) as Ptr<u32>).to_any()) });
    let mut ptr: AnyPtr = ((a1.as_pointer() as Ptr<u32>) as Ptr<u32>).to_any();
    assert!(({ (ptr).clone() } == { ((a1.as_pointer() as Ptr::<u32>) as Ptr::<u32>).to_any() }));
    assert!(
        ({ (elem!((ptr.reinterpret_cast::<u32>()), 0).read()) } == {
            (*a1.borrow())[(0) as usize]
        })
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
