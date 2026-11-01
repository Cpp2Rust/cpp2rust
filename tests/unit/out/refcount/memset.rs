extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut N: i32 = 3;
    let mut arr: Ptr<i32> =
        Ptr::alloc_array((0..(N as usize)).map(|_| 0_i32).collect::<Box<[i32]>>());
    match ({ (arr).to_any() },) {
        (mut __tmp_0,) => {
            __tmp_0.memset(
                (1) as u8,
                (::std::mem::size_of::<i32>() as usize).wrapping_mul((N as usize)) as usize,
            );
            (__tmp_0).clone()
        }
    };
    let mut sum: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while (i < N) {
        sum += { (elem!(arr, i).read()) };
        i.prefix_inc();
    }
    arr.delete();
    assert!((sum == 50529027));
    let bytes: Value<Box<[u8]>> =
        Rc::new(RefCell::new((0..4).map(|_| 0_u8).collect::<Box<[u8]>>()));
    let mut fill: u8 = 7_u8;
    let mut n: i32 = 2;
    match ({ (((bytes.as_pointer() as Ptr<u8>).offset(0)) as Ptr<u8>).to_any() },) {
        (mut __tmp_1,) => {
            __tmp_1.memset(((fill as i32) + 1) as u8, (n as usize) as usize);
            (__tmp_1).clone()
        }
    };
    match ({ (((bytes.as_pointer() as Ptr<u8>).offset(n)) as Ptr<u8>).to_any() },) {
        (mut __tmp_2,) => {
            __tmp_2.memset((fill as i32) as u8, ((4 - n) as usize) as usize);
            (__tmp_2).clone()
        }
    };
    assert!(
        ({
            (((((*bytes.borrow())[(0) as usize] as i32) == 8)
                && (((*bytes.borrow())[(1) as usize] as i32) == 8))
                && (((*bytes.borrow())[(2) as usize] as i32) == 7))
                && (((*bytes.borrow())[(3) as usize] as i32) == 7)
        })
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
