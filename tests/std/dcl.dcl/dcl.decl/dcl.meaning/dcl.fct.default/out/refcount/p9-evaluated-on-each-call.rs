extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static next_ticket_calls_0: Value<i32> = Rc::new(RefCell::new(0_i32));
);
pub fn next_ticket_1() -> i32 {
    return ((*next_ticket_calls_0.with(Value::clone).borrow_mut()).prefix_inc() * 100);
}
pub fn issue_2(ticket: Option<i32>) -> i32 {
    let mut ticket: i32 = ticket.unwrap_or_else(|| ({ next_ticket_1() }));
    return ticket;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((next_ticket_calls_0.with(|rc| *rc.borrow()) == 0));
    assert!((({ issue_2(None,) }) == 100));
    assert!((next_ticket_calls_0.with(|rc| *rc.borrow()) == 1));
    assert!((({ issue_2(Some(7),) }) == 7));
    assert!((next_ticket_calls_0.with(|rc| *rc.borrow()) == 1));
    assert!((({ issue_2(None,) }) == 200));
    assert!((next_ticket_calls_0.with(|rc| *rc.borrow()) == 2));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = next_ticket_calls_0.with(|_| ());
}
