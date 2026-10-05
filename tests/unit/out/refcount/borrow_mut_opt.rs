extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn convert_without_rhs_0() {
    let x: Value<i32> = Rc::new(RefCell::new(0));
    let mut y: i32 = 1;
    (*x.borrow_mut()) = 0;
    (*x.borrow_mut()) = (y + 1);
    y = 0;
    y = ((*x.borrow()) + 1);
    (*x.borrow_mut()) += 1;
    y += 1;
    y = 0;
    (*x.borrow_mut()) = 0;
    let mut z: i32 = ((*x.borrow()) + y);
    z = (((*x.borrow()) + y) + 1);
    let mut arr: [i32; 2] = [1, 2];
    let mut w: i32 = (arr[(y) as usize] + arr[(*x.borrow()) as usize]);
    w += ((z + y) + (*x.borrow()));
    let mut arr2: [i8; 3] = [('a' as i8), ('b' as i8), ('c' as i8)];
    let mut p1: Ptr<i32> = (x.as_pointer());
    let mut c: i8 = arr2[(p1.read()) as usize];
    c = arr2[(p1.read()) as usize];
    let mut p2: Ptr<i32> = (x.as_pointer());
    p2.write(1);
    let r: Ptr<i32> = x.as_pointer();
    r.write(1);
}
pub fn convert_with_rhs_1() {
    let x: Value<i32> = Rc::new(RefCell::new(0));
    (*x.borrow_mut()) = { ((*x.borrow()) + 1) };
    let mut y: i32 = 0;
    y = { (y + 1) };
    let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2])));
    (*arr.borrow_mut())[(y) as usize] = { (y + 1) };
    (*arr.borrow_mut())[(*x.borrow()) as usize] = { ((*x.borrow()) + 1) };
    (*arr.borrow_mut())[(*x.borrow()) as usize] = { ((*arr.borrow())[(y) as usize] + 1) };
    let z: Ptr<i32> = x.as_pointer();
    (*x.borrow_mut()) += { (z.read()) };
    y += { (z.read()) };
    let mut p: Ptr<i32> = (x.as_pointer());
    (*x.borrow_mut()) += { (p.read()) };
    y += { (p.read()) };
    p = ((arr.as_pointer() as Ptr<i32>).offset(0));
    (*arr.borrow_mut())[(0) as usize] = { (p.read()) };
    {
        let _ptr = z.clone();
        _ptr.write(_ptr.read() + { (*x.borrow()) })
    };
    {
        let _ptr = z.clone();
        _ptr.write(_ptr.read() + { y })
    };
    {
        let _ptr = z.clone();
        _ptr.write(_ptr.read() + { (p.read()) })
    };
    {
        let _ptr = p.clone();
        _ptr.write(_ptr.read() + { (y + (*x.borrow())) })
    };
    {
        let _ptr = p.clone();
        _ptr.write(_ptr.read() + { ({ (*x.borrow()) } + { (z.read()) }) })
    };
    {
        let _ptr = p.clone();
        _ptr.write(_ptr.read() + { ({ y } + { (z.read()) }) })
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ convert_without_rhs_0() });
    ({ convert_with_rhs_1() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
