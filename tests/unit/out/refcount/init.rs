extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct X {
    #[offset(0)]
    pub x: i32,
}
impl ByteRepr for X {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.x.to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x: <i32>::from_bytes(&buf[0..4]),
        }
    }
}
pub fn func_0() -> i32 {
    return 42;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(0_i32));
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::<i32>::null()));
    let g: Ptr<i32> = x.as_pointer();
    let q: Value<Ptr<i32>> = Rc::new(RefCell::new((x.as_pointer())));
    let z: Value<Ptr<i32>> = Rc::new(RefCell::new((*p.borrow()).clone()));
    let xx: Value<X> = Rc::new(RefCell::new(<X>::default()));
    let zz: Value<Ptr<X>> = Rc::new(RefCell::new((xx.as_pointer())));
    (*xx.borrow_mut()).x = 1;
    (*q.borrow_mut()) = (field_ptr!(xx.as_pointer(), x));
    (*q.borrow_mut()) = (field_ptr!((*zz.borrow()), x));
    field!((*zz.borrow()), x).write(2);
    let ww: Value<X> = Rc::new(RefCell::new((*xx.borrow()).clone()));
    (*ww.borrow_mut()) = (*xx.borrow()).clone();
    let aa: Value<i32> = Rc::new(RefCell::new(({ func_0() })));
    (*aa.borrow_mut()) = ({ func_0() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
