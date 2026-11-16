extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct HasIntArray {
    #[offset(0)]
    pub before: i32,
    #[offset(4)]
    #[byte_size(12)]
    pub elements: Box<[i32]>,
}
impl Default for HasIntArray {
    fn default() -> Self {
        {
            HasIntArray {
                before: 0_i32,
                elements: (0..3).map(|_| 0_i32).collect::<Box<[i32]>>(),
            }
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut pm: isize = (<HasIntArray as Record>::OFFSETS.elements as isize);
    let obj: Value<HasIntArray> = Rc::new(RefCell::new(HasIntArray {
        before: 1,
        elements: Box::new([2, 3, 4]),
    }));
    assert!((((obj.as_pointer()).member::<Box<[i32]>>(pm).read())[(0) as usize] == 2));
    assert!((((obj.as_pointer()).member::<Box<[i32]>>(pm).read())[(2) as usize] == 4));
    index!((obj.as_pointer()).member::<Box<[i32]>>(pm), 1).write(7);
    assert!(((array_elem!(obj.as_pointer(), elements, 1).read()) == 7));
    assert!(({ ({ (*obj.borrow()).before } == 1) }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
