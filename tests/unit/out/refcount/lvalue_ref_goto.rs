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
    let __tmp_0: Value<i32> = Rc::new(RefCell::new(5));
    let mut r: Ptr<i32> = <Ptr<i32>>::default();
    goto_block!({
        '__entry: {
            r = __tmp_0.as_pointer();
            goto!('body);
        }
        'body: {
            assert!(((r.read()) == 5));
            return 0;
        }
    });
    panic!("ub: non-void function does not return a value")
}
