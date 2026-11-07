extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(24)]
pub struct Outer {
    #[offset(0)]
    #[byte_size(24)]
    pub values: Vec<i32>,
}
impl Outer {
    pub fn move_from(_a0: Ptr<Outer>) -> Self {
        Self {
            values: field!(_a0, values).with_mut(std::mem::take),
        }
    }
}
pub fn make_0() -> Outer {
    return Outer {
        values: Vec::from([1, 2, 3, 4]),
    };
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(Vec<i32  >  , new, ptr)]
#[byte_size(4)]
pub struct H {
    #[offset(0)]
    pub last: i32,
}
impl H {
    pub fn new(values: Ptr<Vec<i32>>) -> Self {
        Self {
            last: (index!(values, 3_usize).read()),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut h: H = H::new({
        (match Rc::new(RefCell::new(({ make_0() }))).as_pointer() {
            __tmp_0 => field_ref!(__tmp_0, values).as_pointer(),
        })
    });
    println!("{}", h.last);
    return 0;
}
pub trait OuterImpl {
    fn move_assign(&self, _a0: Ptr<Outer>) -> Ptr<Outer>;
}
impl OuterImpl for Ptr<Outer> {
    fn move_assign(&self, _a0: Ptr<Outer>) -> Ptr<Outer> {
        field_ref!((*self), values)
            .as_pointer()
            .write(field!(_a0, values).with_mut(std::mem::take));
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
