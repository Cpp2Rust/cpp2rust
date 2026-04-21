extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(Clone, Default)]
pub struct S {
    pub v: Vec<i32>,
    pub a: i32,
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: S = <S>::default();
    s.v.push(1);
    'loop_: for e in 0..(s.v.len()) {
        let mut e = s.v[e].clone();
        s.a.postfix_inc();
    }
    return 0;
}
