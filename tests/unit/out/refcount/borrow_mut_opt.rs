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
    let arr2: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        ('a' as u8),
        ('b' as u8),
        ('c' as u8),
    ])));
    let p1: Value<Ptr<i32>> = Rc::new(RefCell::new((x.as_pointer())));
    let c: Value<u8> = Rc::new(RefCell::new(
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
    let __rhs = ((*x.borrow()) + 1);
    (*x.borrow_mut()) = __rhs;
    let y: Value<i32> = Rc::new(RefCell::new(0));
    let __rhs = ((*y.borrow()) + 1);
    (*y.borrow_mut()) = __rhs;
    let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2])));
    let __rhs = ((*y.borrow()) + 1);
    (*arr.borrow_mut())[(*y.borrow()) as usize] = __rhs;
    let __rhs = ((*x.borrow()) + 1);
    (*arr.borrow_mut())[(*x.borrow()) as usize] = __rhs;
    let __rhs = ((*arr.borrow())[(*y.borrow()) as usize] + 1);
    (*arr.borrow_mut())[(*x.borrow()) as usize] = __rhs;
    let z: Ptr<i32> = x.as_pointer();
    let __rhs = (z.read());
    (*x.borrow_mut()) += __rhs;
    let __rhs = (z.read());
    (*y.borrow_mut()) += __rhs;
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new((x.as_pointer())));
    let __rhs = ((*p.borrow()).read());
    (*x.borrow_mut()) += __rhs;
    let __rhs = ((*p.borrow()).read());
    (*y.borrow_mut()) += __rhs;
    (*p.borrow_mut()) = ((arr.as_pointer() as Ptr<i32>).offset(0 as isize));
    let __rhs = ((*p.borrow()).read());
    (*arr.borrow_mut())[(0) as usize] = __rhs;
    let __rhs = (*x.borrow());
    {
        let __ptr = z.clone();
        let __tmp = __ptr.read() + __rhs;
        __ptr.write(__tmp)
    };
    let __rhs = (*y.borrow());
    {
        let __ptr = z.clone();
        let __tmp = __ptr.read() + __rhs;
        __ptr.write(__tmp)
    };
    let __rhs = ((*p.borrow()).read());
    {
        let __ptr = z.clone();
        let __tmp = __ptr.read() + __rhs;
        __ptr.write(__tmp)
    };
    let __rhs = ((*y.borrow()) + (*x.borrow()));
    {
        let __ptr = (*p.borrow()).clone();
        let __tmp = __ptr.read() + __rhs;
        __ptr.write(__tmp)
    };
    let __rhs = {
        let _lhs = (*x.borrow());
        _lhs + (z.read())
    };
    {
        let __ptr = (*p.borrow()).clone();
        let __tmp = __ptr.read() + __rhs;
        __ptr.write(__tmp)
    };
    let __rhs = {
        let _lhs = (*y.borrow());
        _lhs + (z.read())
    };
    {
        let __ptr = (*p.borrow()).clone();
        let __tmp = __ptr.read() + __rhs;
        __ptr.write(__tmp)
    };
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ convert_without_rhs_0() });
    ({ convert_with_rhs_1() });
    return 0;
}
