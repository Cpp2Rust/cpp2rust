extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[repr(C)]
#[derive(Default)]
pub struct Outer_RunInfo {
    pub block_idx: i32,
    pub num_extra_zero_runs: i32,
}
impl Clone for Outer_RunInfo {
    fn clone(&self) -> Self {
        let mut this = Self {
            block_idx: self.block_idx,
            num_extra_zero_runs: self.num_extra_zero_runs,
        };
        this
    }
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
#[repr(C)]
#[derive(Default)]
pub struct Outer {
    pub runs: Vec<Outer_RunInfo>,
}
impl Clone for Outer {
    fn clone(&self) -> Self {
        let mut this = Self {
            runs: (self.runs).clone(),
        };
        this
    }
}
impl ByteRepr for Outer {
    fn byte_size() -> usize {
        24
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.runs.to_bytes(&mut buf[0..24]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            runs: <Vec<Outer_RunInfo>>::from_bytes(&buf[0..24]),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let o: Value<Outer> = Rc::new(RefCell::new(<Outer>::default()));
    let info: Value<Outer_RunInfo> = Rc::new(RefCell::new(<Outer_RunInfo>::default()));
    (*info.borrow_mut()).block_idx = 1;
    (*info.borrow_mut()).num_extra_zero_runs = 2;
    {
        let a0_clone = (*info.borrow()).clone();
        (*o.borrow_mut()).runs.push(a0_clone)
    };
    assert!(((*o.borrow()).runs.len() == 1_usize));
    assert!(
        ((o.as_pointer().field_ptr(
            0,
            |__v: &Outer| &__v.runs[..],
            |__v: &mut Outer| &mut __v.runs[..]
        ) as Ptr<Outer_RunInfo>)
            .offset(0_usize)
            .with(|__v| (*__v).block_idx)
            == 1)
    );
    assert!(
        ((o.as_pointer().field_ptr(
            0,
            |__v: &Outer| &__v.runs[..],
            |__v: &mut Outer| &mut __v.runs[..]
        ) as Ptr<Outer_RunInfo>)
            .offset(0_usize)
            .with(|__v| (*__v).num_extra_zero_runs)
            == 2)
    );
    return 0;
}
