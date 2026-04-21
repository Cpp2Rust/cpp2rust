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
    let file_ptr: Value<Ptr<::std::fs::File>> = Rc::new(RefCell::new(Default::default()));
    return if !(*file_ptr.borrow()).is_null() {
        match (*file_ptr.borrow()).with_mut(|v| v.sync_all()) {
            Ok(_) => 0,
            Err(_) => -1,
        }
    } else {
        ::std::io::stdout().flush().unwrap();
        ::std::io::stderr().flush().unwrap();
        0
    };
}
