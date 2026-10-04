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
    let y: Value<i32> = Rc::new(RefCell::new(1));
    (*x.borrow_mut()) = 0;
    (*x.borrow_mut()) = ((*y.borrow()) + 1);
    (*y.borrow_mut()) = 0;
    (*y.borrow_mut()) = ((*x.borrow()) + 1);
    (*x.borrow_mut()) += 1;
    (*y.borrow_mut()) += 1;
    (*y.borrow_mut()) = 0;
    (*x.borrow_mut()) = 0;
    let z: Value<i32> = Rc::new(RefCell::new(((*x.borrow()) + (*y.borrow()))));
    (*z.borrow_mut()) = (((*x.borrow()) + (*y.borrow())) + 1);
    let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2])));
    let w: Value<i32> = Rc::new(RefCell::new(
        ((*arr.borrow())[(*y.borrow()) as usize] + (*arr.borrow())[(*x.borrow()) as usize]),
    ));
    (*w.borrow_mut()) += (((*z.borrow()) + (*y.borrow())) + (*x.borrow()));
    let arr2: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
        ('a' as i8),
        ('b' as i8),
        ('c' as i8),
    ])));
    let p1: Value<Ptr<i32>> = Rc::new(RefCell::new((x.as_pointer())));
    let c: Value<i8> = Rc::new(RefCell::new(
        (*arr2.borrow())[((*p1.borrow()).read()) as usize],
    ));
    (*c.borrow_mut()) = (*arr2.borrow())[((*p1.borrow()).read()) as usize];
    let p2: Value<Ptr<i32>> = Rc::new(RefCell::new((x.as_pointer())));
    (*p2.borrow()).write(1);
    let r: Ptr<i32> = x.as_pointer();
    r.write(1);
}
pub fn convert_with_rhs_1() {
    let x: Value<i32> = Rc::new(RefCell::new(0));
    (*x.borrow_mut()) = { ((*x.borrow()) + 1) };
    let y: Value<i32> = Rc::new(RefCell::new(0));
    (*y.borrow_mut()) = { ((*y.borrow()) + 1) };
    let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2])));
    (*arr.borrow_mut())[(*y.borrow()) as usize] = { ((*y.borrow()) + 1) };
    (*arr.borrow_mut())[(*x.borrow()) as usize] = { ((*x.borrow()) + 1) };
    (*arr.borrow_mut())[(*x.borrow()) as usize] = { ((*arr.borrow())[(*y.borrow()) as usize] + 1) };
    let z: Ptr<i32> = x.as_pointer();
    (*x.borrow_mut()) += { (z.read()) };
    (*y.borrow_mut()) += { (z.read()) };
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new((x.as_pointer())));
    (*x.borrow_mut()) += { ((*p.borrow()).read()) };
    (*y.borrow_mut()) += { ((*p.borrow()).read()) };
    (*p.borrow_mut()) = ((arr.as_pointer() as Ptr<i32>).offset(0));
    (*arr.borrow_mut())[(0) as usize] = { ((*p.borrow()).read()) };
    {
        let _ptr = z.clone();
        _ptr.write(_ptr.read() + { (*x.borrow()) })
    };
    {
        let _ptr = z.clone();
        _ptr.write(_ptr.read() + { (*y.borrow()) })
    };
    {
        let _ptr = z.clone();
        _ptr.write(_ptr.read() + { ((*p.borrow()).read()) })
    };
    {
        let _ptr = (*p.borrow()).clone();
        _ptr.write(_ptr.read() + { ((*y.borrow()) + (*x.borrow())) })
    };
    {
        let _ptr = (*p.borrow()).clone();
        _ptr.write(_ptr.read() + { ({ (*x.borrow()) } + { (z.read()) }) })
    };
    {
        let _ptr = (*p.borrow()).clone();
        _ptr.write(_ptr.read() + { ({ (*y.borrow()) } + { (z.read()) }) })
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
