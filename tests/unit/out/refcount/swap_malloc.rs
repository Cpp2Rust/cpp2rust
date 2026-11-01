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
    let mut buf: Ptr<i32> =
        libcc2rs::malloc_refcount((3_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)))
            .reinterpret_cast::<i32>();
    elem!(buf, 0).write(1);
    elem!(buf, 1).write(2);
    elem!(buf, 2).write(3);
    cpp_swap!(buf.offset((0) as isize), buf.offset((2) as isize));
    assert!(
        (((elem!(buf, 0).read()) == 3) && ((elem!(buf, 1).read()) == 2))
            && ((elem!(buf, 2).read()) == 1)
    );
    cpp_swap!(buf.offset((1) as isize), buf.offset((1) as isize));
    assert!(((elem!(buf, 1).read()) == 2));
    let local: Value<i32> = Rc::new(RefCell::new(4));
    cpp_swap!(buf.offset((1) as isize), local.as_pointer());
    assert!(({ ((elem!(buf, 1).read()) == 4) && ((*local.borrow()) == 2) }));
    let mut other: Ptr<i32> =
        libcc2rs::malloc_refcount(::std::mem::size_of::<i32>()).reinterpret_cast::<i32>();
    other.write(5);
    cpp_swap!((buf).clone(), (other).clone());
    assert!(((elem!(buf, 0).read()) == 5) && ((other.read()) == 3));
    let mut bytes: Ptr<u8> = buf.reinterpret_cast::<u8>();
    elem!(bytes, 0).write(170_u8);
    elem!(bytes, 4).write(187_u8);
    cpp_swap!(bytes.offset((0) as isize), bytes.offset((4) as isize));
    assert!(
        (((elem!(bytes, 0).read()) as i32) == 187) && (((elem!(bytes, 4).read()) as i32) == 170)
    );
    let words: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([16909060_u32, 84281096_u32])));
    let mut halves: Ptr<u16> = (words.as_pointer() as Ptr<u32>).reinterpret_cast::<u16>();
    cpp_swap!(halves.offset((0) as isize), halves.offset((3) as isize));
    assert!(
        ({
            ((*words.borrow())[(0) as usize] == 16909574_u32)
                && ((*words.borrow())[(1) as usize] == 50595592_u32)
        })
    );
    libcc2rs::free_refcount((other).to_any());
    libcc2rs::free_refcount((buf).to_any());
    return 0;
}
pub fn __cpp2rust_init_globals() {}
