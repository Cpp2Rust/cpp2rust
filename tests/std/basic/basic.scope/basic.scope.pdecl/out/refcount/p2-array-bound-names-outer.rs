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
    let length: Value<i32> = Rc::new(RefCell::new(2));
    {
        let mut length: [i32; 2] = [7, 8];
        assert!(
            ((::std::mem::size_of::<[i32; 2]>() as usize)
                .wrapping_div((::std::mem::size_of::<i32>() as usize))
                == 2_usize)
        );
        assert!((length[(0) as usize] == 7));
        assert!((length[(1) as usize] == 8));
    }
    assert!(({ ((*length.borrow()) == 2) }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
