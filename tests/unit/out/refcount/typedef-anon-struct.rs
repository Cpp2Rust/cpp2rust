extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Outer_RunInfo {
    pub block_idx: Value<i32>,
    pub num_extra_zero_runs: Value<i32>,
}
impl Clone for Outer_RunInfo {
    fn clone(&self) -> Self {
        let mut this = Self {
            block_idx: Rc::new(RefCell::new((*self.block_idx.borrow()))),
            num_extra_zero_runs: Rc::new(RefCell::new((*self.num_extra_zero_runs.borrow()))),
        };
        this
    }
}
impl ByteRepr for Outer_RunInfo {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.block_idx.borrow()).to_bytes(&mut buf[0..4]);
        (*self.num_extra_zero_runs.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            block_idx: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            num_extra_zero_runs: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
        }
    }
}
#[derive(Default)]
pub struct Outer {
    pub runs: Value<Vec<Outer_RunInfo>>,
}
impl Clone for Outer {
    fn clone(&self) -> Self {
        let mut this = Self {
            runs: Rc::new(RefCell::new((*self.runs.borrow()).clone())),
        };
        this
    }
}
impl ByteRepr for Outer {}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let o: Value<Outer> = Rc::new(RefCell::new(<Outer>::default()));
    let info: Value<Outer_RunInfo> = Rc::new(RefCell::new(<Outer_RunInfo>::default()));
    (*(*info.borrow()).block_idx.borrow_mut()) = 1;
    (*(*info.borrow()).num_extra_zero_runs.borrow_mut()) = 2;
    {
        let a0_clone = (*info.borrow()).clone();
        (*(*o.borrow()).runs.borrow_mut()).push(a0_clone)
    };
    assert!(((*(*o.borrow()).runs.borrow()).len() == 1_usize));
    assert!(
        ((*(*((*o.borrow()).runs.as_pointer() as Ptr<Outer_RunInfo>)
            .offset(0_usize)
            .upgrade()
            .deref())
        .block_idx
        .borrow())
            == 1)
    );
    assert!(
        ((*(*((*o.borrow()).runs.as_pointer() as Ptr<Outer_RunInfo>)
            .offset(0_usize)
            .upgrade()
            .deref())
        .num_extra_zero_runs
        .borrow())
            == 2)
    );
    return 0;
}
