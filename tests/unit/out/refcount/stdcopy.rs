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
        let count = (input.as_pointer() as Ptr<i32>)
            .offset((3) as isize)
            .get_offset()
            - (input.as_pointer() as Ptr<i32>).get_offset();
        let mut outptr = (output.as_pointer() as Ptr<i32>).clone();
        for value in PtrValueIter::new(&(input.as_pointer() as Ptr<i32>), count) {
            outptr.write(value.into());
            outptr += 1;
        }
        outptr
    };
    return 0;
}
