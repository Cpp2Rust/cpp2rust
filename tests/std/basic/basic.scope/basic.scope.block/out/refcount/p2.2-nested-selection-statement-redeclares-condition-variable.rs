extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static inner_then_runs_0: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static inner_else_runs_1: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static outer_else_runs_2: Value<i32> = Rc::new(RefCell::new(0_i32));
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    {
        let mut choice: i32 = 4;
        if (choice != 0) {
            {
                let mut choice: i32 = 0;
                if (choice != 0) {
                    (*inner_then_runs_0.with(Value::clone).borrow_mut()).prefix_inc();
                } else {
                    (*inner_else_runs_1.with(Value::clone).borrow_mut()).prefix_inc();
                    assert!((choice == 0));
                }
            }
        } else {
            (*outer_else_runs_2.with(Value::clone).borrow_mut()).prefix_inc();
            assert!((choice == 0));
        }
    }
    assert!((inner_then_runs_0.with(|rc| *rc.borrow()) == 0));
    assert!((inner_else_runs_1.with(|rc| *rc.borrow()) == 1));
    assert!((outer_else_runs_2.with(|rc| *rc.borrow()) == 0));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = inner_then_runs_0.with(|_| ());
    let _ = inner_else_runs_1.with(|_| ());
    let _ = outer_else_runs_2.with(|_| ());
}
