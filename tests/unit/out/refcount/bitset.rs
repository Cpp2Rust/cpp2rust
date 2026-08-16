extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let b1: Value<Vec<bool>> = Rc::new(RefCell::new(
        (0..u64::BITS).map(|i| (5_u64 >> i) & 1 == 1).collect(),
    ));
    let b2: Value<Vec<bool>> = Rc::new(RefCell::new({
        let mut bits = vec![false; 2_usize];
        bits.extend((*b1.borrow()).iter());
        bits
    }));
    let b3: Value<Vec<bool>> = Rc::new(RefCell::new(
        (*b2.borrow()).iter().skip(1_usize).copied().collect(),
    ));
    assert!(
        ((*b1.borrow())
            .iter()
            .rev()
            .fold(0u64, |acc, &b| (acc << 1) | b as u64)
            == 5_u64)
    );
    assert!(
        ((*b2.borrow())
            .iter()
            .rev()
            .fold(0u64, |acc, &b| (acc << 1) | b as u64)
            == 20_u64)
    );
    assert!(
        ((*b3.borrow())
            .iter()
            .rev()
            .fold(0u64, |acc, &b| (acc << 1) | b as u64)
            == 10_u64)
    );
    return 0;
}
