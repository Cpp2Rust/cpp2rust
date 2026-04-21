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
    let input: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3])));
    let output: Value<Box<[i32]>> = Rc::new(RefCell::new(
        (0..3).map(|_| <i32>::default()).collect::<Box<[i32]>>(),
    ));
    {
        let mut outptr = (output.as_pointer() as Ptr<i32>).clone();
        let mut curr = (input.as_pointer() as Ptr<i32>).clone();
        while curr < (input.as_pointer() as Ptr<i32>).offset((3) as isize) {
            outptr.write((curr.read()).clone().into());
            curr += 1;
            outptr += 1;
        }
        outptr
    };
    return 0;
}
