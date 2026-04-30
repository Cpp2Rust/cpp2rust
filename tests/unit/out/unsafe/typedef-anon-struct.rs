extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Outer_RunInfo {
    pub block_idx: i32,
    pub num_extra_zero_runs: i32,
}
#[repr(C)]
#[derive(Clone, Default)]
pub struct Outer {
    pub runs: Vec<Outer_RunInfo>,
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut o: Outer = <Outer>::default();
    let mut info: Outer_RunInfo = <Outer_RunInfo>::default();
    info.block_idx = 1;
    info.num_extra_zero_runs = 2;
    {
        let a0_clone = info.clone();
        o.runs.push(a0_clone)
    };
    assert!(((o.runs.len() as u64) == (1_u64)));
    assert!(((o.runs[(0_u64) as usize].block_idx) == (1)));
    assert!(((o.runs[(0_u64) as usize].num_extra_zero_runs) == (2)));
    return 0;
}
