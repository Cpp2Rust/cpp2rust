extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
pub struct StackArray {
    #[offset(0)]
    #[byte_size(24)]
    pub arr: Value<Box<[Ptr<i32>]>>,
}
impl Default for StackArray {
    fn default() -> Self {
        StackArray {
            arr: Rc::new(RefCell::new(
                (0..3)
                    .map(|_| Ptr::<i32>::null())
                    .collect::<Box<[Ptr<i32>]>>(),
            )),
        }
    }
}
pub fn IncrementAll_0(s: Ptr<StackArray>) {
    let mut i: i32 = 0;
    'loop_: while (i < 3) {
        {
            let _ptr = (elem!((array_field_ptr!(s, arr) as Ptr<Ptr::<i32>>), i).read()).clone();
            _ptr.write(_ptr.read() + 1)
        };
        i.prefix_inc();
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(0));
    let s: Value<StackArray> = Rc::new(RefCell::new(StackArray {
        arr: Rc::new(RefCell::new(Box::new([
            (x.as_pointer()),
            (x.as_pointer()),
            (x.as_pointer()),
        ]))),
    }));
    ({ IncrementAll_0(s.as_pointer()) });
    assert!(((*x.borrow()) == 3));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
