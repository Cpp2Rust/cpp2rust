extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn sum_bytes_0(mut buf: Ptr<i8>, mut len: u32) -> i32 {
    let mut sum: i32 = 0;
    let mut i: u32 = 0_u32;
    'loop_: while (i < len) {
        sum += (((elem!(buf, i).read()) as u8) as i32);
        i.postfix_inc();
    }
    return sum;
}
thread_local!(
    pub static g_packet_1: Value<Ptr<i8>> =
        Rc::new(RefCell::new(Ptr::<i8>::from_string_literal(b"\x01\0")));
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut a: i32 = ({ sum_bytes_0(Ptr::<i8>::from_string_literal(b"\x01\0"), 2_u32) });
    let mut b: i32 = ({ sum_bytes_0((*g_packet_1.with(Value::clone).borrow()).clone(), 2_u32) });
    assert!((a == b));
    assert!((a == 1));
    let mut high: i32 = ({ sum_bytes_0(Ptr::<i8>::from_string_literal(b"\xc2\0A"), 3_u32) });
    assert!((high == (194 + 65)));
    let mut c: i32 = ((b"\r\n.\r\n"[(0) as usize] as i32) + (b"\r\n.\r\n"[(3) as usize] as i32));
    assert!((c == ((('\r' as i8) as i32) + (('\r' as i8) as i32))));
    let mut idx: i32 = 1;
    let mut d: i32 = (b"abcd"[(idx) as usize] as i32);
    assert!((d == (('b' as i8) as i32)));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = g_packet_1.with(|_| ());
}
