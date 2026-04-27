extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub struct basic {
    __buf: Rc<RefCell<Vec<u8>>>,
    pub i: Value<i32>,
    pub f: Value<f32>,
}
impl Default for basic {
    fn default() -> Self {
        let __buf = Rc::new(RefCell::new(vec![0u8; 4]));
        Self {
            __buf: __buf.clone(),
            i: Value::new_reinterpreted(
                Rc::new(SliceOriginalAlloc {
                    weak: Rc::downgrade(&__buf),
                }),
                0,
            ),
            f: Value::new_reinterpreted(
                Rc::new(SliceOriginalAlloc {
                    weak: Rc::downgrade(&__buf),
                }),
                0,
            ),
        }
    }
}
impl ByteRepr for basic {}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let u: Value<basic> = <Value<basic>>::default();
    (*u.borrow()).i.write(42);
    assert!(((*u.borrow()).i.read() == 42));
    (*u.borrow()).f.write(3.140000105E+0);
    assert!(((*u.borrow()).f.read() == 3.140000105E+0));
    return 0;
}
