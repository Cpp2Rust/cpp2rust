extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn sum_bytes_0(buf: Ptr<u8>, len: u32) -> i32 {
    let buf: Value<Ptr<u8>> = Rc::new(RefCell::new(buf));
    let len: Value<u32> = Rc::new(RefCell::new(len));
    let sum: Value<i32> = Rc::new(RefCell::new(0));
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while ((*i.borrow()) < (*len.borrow())) {
        (*sum.borrow_mut()) += (((*buf.borrow()).offset((*i.borrow()) as isize).read()) as i32);
        (*i.borrow_mut()).postfix_inc();
    }
    return (*sum.borrow());
}
thread_local!(
    pub static g_packet_1: Value<Ptr<u8>> =
        Rc::new(RefCell::new(Ptr::from_string_literal(b"\x01\0")));
);
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<i32> = Rc::new(RefCell::new(
        ({ sum_bytes_0(Ptr::from_string_literal(b"\x01\0"), 2_u32) }),
    ));
    let b: Value<i32> = Rc::new(RefCell::new(
        ({ sum_bytes_0((*g_packet_1.with(Value::clone).borrow()).clone(), 2_u32) }),
    ));
    assert!(((*a.borrow()) == (*b.borrow())));
    assert!(((*a.borrow()) == 1));
    let c: Value<i32> = Rc::new(RefCell::new(
        ((b"\r\n.\r\n"[(0) as usize] as i32) + (b"\r\n.\r\n"[(3) as usize] as i32)),
    ));
    assert!(((*c.borrow()) == ((('\r' as u8) as i32) + (('\r' as u8) as i32))));
    let idx: Value<i32> = Rc::new(RefCell::new(1));
    let d: Value<i32> = Rc::new(RefCell::new((b"abcd"[(*idx.borrow()) as usize] as i32)));
    assert!(((*d.borrow()) == (('b' as u8) as i32)));
    return 0;
}
