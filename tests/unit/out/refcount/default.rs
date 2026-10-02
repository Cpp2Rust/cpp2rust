extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(144)]
pub struct Pointers {
    #[offset(0)]
    #[byte_size(8)]
    pub x1: Ptr<i32>,
    #[offset(8)]
    #[byte_size(8)]
    pub x2: Ptr<i32>,
    #[offset(16)]
    #[byte_size(40)]
    pub x3: Value<Box<[Ptr<i32>]>>,
    #[offset(56)]
    #[byte_size(80)]
    pub x4: Value<Box<[Ptr<i32>]>>,
    #[offset(136)]
    pub x5: i32,
}
impl Default for Pointers {
    fn default() -> Self {
        Pointers {
            x1: Ptr::<i32>::null(),
            x2: Ptr::<i32>::null(),
            x3: Rc::new(RefCell::new(
                (0..5)
                    .map(|_| Ptr::<i32>::null())
                    .collect::<Box<[Ptr<i32>]>>(),
            )),
            x4: Rc::new(RefCell::new(
                (0..10)
                    .map(|_| Ptr::<i32>::null())
                    .collect::<Box<[Ptr<i32>]>>(),
            )),
            x5: 0_i32,
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let default_pointers: Value<Ptr<Pointers>> = Rc::new(RefCell::new(Ptr::alloc_array(
        (0..10_usize)
            .map(|_| <Pointers>::default())
            .collect::<Box<[Pointers]>>(),
    )));
    (*default_pointers.borrow()).delete();
    return 0;
}
pub fn __cpp2rust_init_globals() {}
