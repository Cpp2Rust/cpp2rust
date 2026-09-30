extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct Outer_RunInfo {
    #[offset(0)]
    pub block_idx: i32,
    #[offset(4)]
    pub num_extra_zero_runs: i32,
}
impl ByteRepr for Outer_RunInfo {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.block_idx.to_bytes(&mut buf[0..4]);
        self.num_extra_zero_runs.to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            block_idx: <i32>::from_bytes(&buf[0..4]),
            num_extra_zero_runs: <i32>::from_bytes(&buf[4..8]),
        }
    }
}
#[derive(Record, VaArg, FnPtrArg, Default)]
pub struct Outer {
    #[offset(0)]
    pub runs: Value<Vec<Outer_RunInfo>>,
}
impl Clone for Outer {
    fn clone(&self) -> Self {
        let __this: Value<Outer> = Rc::new(RefCell::new(Self {
            runs: { Rc::new(RefCell::new((*self.runs.borrow()).clone())) },
        }));
        let this: Ptr<Outer> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
<<<<<<< HEAD
impl ByteRepr for Outer {}
=======
impl ByteRepr for Outer {
    fn byte_size() -> usize {
        24
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.runs.borrow()).to_bytes(&mut buf[0..24]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            runs: Rc::new(RefCell::new(<Vec<Outer_RunInfo>>::from_bytes(&buf[0..24]))),
        }
    }
}
>>>>>>> 3ed38b58 (Remove Value<> boxing from struct fields)
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let o: Value<Outer> = Rc::new(RefCell::new(<Outer>::default()));
    let info: Value<Outer_RunInfo> = Rc::new(RefCell::new(<Outer_RunInfo>::default()));
    (*info.borrow_mut()).block_idx = 1;
    (*info.borrow_mut()).num_extra_zero_runs = 2;
    {
        let a0_clone = (*info.borrow()).clone();
        (*(*o.borrow()).runs.borrow_mut()).push(a0_clone)
    };
    assert!(((*(*o.borrow()).runs.borrow()).len() == 1_usize));
    assert!(({ (*(*o.borrow()).runs.borrow())[(0_usize) as usize].block_idx } == 1));
    assert!(({ (*(*o.borrow()).runs.borrow())[(0_usize) as usize].num_extra_zero_runs } == 2));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
