extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(20)]
pub struct S {
    #[offset(0)]
    pub head: i32,
    #[offset(4)]
    #[byte_size(12)]
    pub tail: Box<[i32]>,
    #[offset(16)]
    #[byte_size(4)]
    pub buf: Box<[i8]>,
}
impl Default for S {
    fn default() -> Self {
        {
            S {
                head: 0_i32,
                tail: (0..3).map(|_| 0_i32).collect::<Box<[i32]>>(),
                buf: (0..4).map(|_| 0_i8).collect::<Box<[i8]>>(),
            }
        }
    }
}
thread_local!(
    pub static s_0: Value<S> = Rc::new(RefCell::new(S {
        head: 5,
        tail: Box::new([0; 3]),
        buf: Box::new([0; 4]),
    }));
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(({ ((({ (*s_0.with(Value::clone).borrow()).head } == 5) as i32) != 0) }));
    let mut i: i32 = 0;
    'loop_: while (((i < 3) as i32) != 0) {
        assert!(((((array_elem!(s_0.with(|v| v.as_pointer()), tail, i).read()) == 0) as i32) != 0));
        i.postfix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while (((i < 4) as i32) != 0) {
        assert!(
            (((((array_elem!(s_0.with(|v| v.as_pointer()), buf, i).read()) as i32) == 0) as i32)
                != 0)
        );
        i.postfix_inc();
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = s_0.with(|_| ());
}
