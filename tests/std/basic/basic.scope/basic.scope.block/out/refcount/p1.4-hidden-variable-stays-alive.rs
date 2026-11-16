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
    let depth: Value<i32> = Rc::new(RefCell::new(1));
    let mut outermost: Ptr<i32> = (depth.as_pointer());
    {
        let mut depth: i32 = 2;
        {
            outermost.with_mut(|__v| *__v = *__v + 10)
        };
        assert!((depth == 2));
        {
            let mut depth: i32 = 3;
            {
                outermost.with_mut(|__v| *__v = *__v + 100)
            };
            assert!((depth == 3));
        }
        assert!((depth == 2));
    }
    assert!(({ ((*depth.borrow()) == 111) }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
