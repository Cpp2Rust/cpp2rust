extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn identity_0(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return (*x.borrow());
}
pub fn swap_by_ptr_1(a: Ptr<i32>, b: Ptr<i32>) {
    let a: Value<Ptr<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Ptr<i32>> = Rc::new(RefCell::new(b));
    let tmp: Value<i32> = Rc::new(RefCell::new(((*a.borrow()).read())));
    let __rhs = ((*b.borrow()).read());
    (*a.borrow()).write(__rhs);
    let __rhs = (*tmp.borrow());
    (*b.borrow()).write(__rhs);
}
pub fn swap_by_ref_2(a: Ptr<i32>, b: Ptr<i32>) {
    let tmp: Value<i32> = Rc::new(RefCell::new((a.read())));
    let __rhs = (b.read());
    a.write(__rhs);
    let __rhs = (*tmp.borrow());
    b.write(__rhs);
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(0));
    write!(libcc2rs::cout(), "{:}\n", (*x.borrow()),);
    let a: Value<i32> = Rc::new(RefCell::new(1));
    let b: Value<i32> = Rc::new(RefCell::new(
        ({
            let _x: i32 = (*a.borrow());
            identity_0(_x)
        }),
    ));
    write!(libcc2rs::cout(), "{:}\n", (*b.borrow()),);
    let c: Value<i32> = Rc::new(RefCell::new(2));
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new((c.as_pointer())));
    write!(libcc2rs::cout(), "{:}\n", ((*p.borrow()).read()),);
    let d: Value<i32> = Rc::new(RefCell::new(3));
    let e: Value<i32> = Rc::new(RefCell::new(4));
    ({
        let _a: Ptr<i32> = (d.as_pointer());
        let _b: Ptr<i32> = (e.as_pointer());
        swap_by_ptr_1(_a, _b)
    });
    let f: Value<i32> = Rc::new(RefCell::new(4));
    let g: Value<i32> = Rc::new(RefCell::new(5));
    ({
        let _a: Ptr<i32> = f.as_pointer();
        let _b: Ptr<i32> = g.as_pointer();
        swap_by_ref_2(_a, _b)
    });
    let h: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::alloc(6)));
    write!(libcc2rs::cout(), "{:}\n", ((*h.borrow()).read()),);
    (*h.borrow()).delete();
    let i: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::alloc_array(Box::new([
        7,
        8,
        <i32>::default(),
    ]))));
    write!(
        libcc2rs::cout(),
        "{:} {:}\n",
        ((*i.borrow()).offset((0) as isize).read()),
        ((*i.borrow()).offset((1) as isize).read()),
    );
    (*i.borrow()).delete_array();
    ({
        let _a: Ptr<i32> = Ptr::alloc(7);
        let _b: Ptr<i32> = Ptr::alloc(8);
        swap_by_ptr_1(_a, _b)
    });
    ({
        let _a: Ptr<i32> = Ptr::alloc(7).offset((0) as isize);
        let _b: Ptr<i32> = Ptr::alloc(8).offset((0) as isize);
        swap_by_ptr_1(_a, _b)
    });
    ({
        let _a: Ptr<i32> = Ptr::alloc(9);
        let _b: Ptr<i32> = Ptr::alloc(10);
        swap_by_ref_2(_a, _b)
    });
    ({
        let _a: Ptr<i32> = (Ptr::alloc(9)).offset((0) as isize);
        let _b: Ptr<i32> = (Ptr::alloc(10)).offset((0) as isize);
        swap_by_ref_2(_a, _b)
    });
    let j: Value<Option<Value<i32>>> = Rc::new(RefCell::new(Ptr::alloc(11).to_owned_opt()));
    let k: Value<Ptr<i32>> = Rc::new(RefCell::new((*j.borrow()).as_pointer()));
    write!(libcc2rs::cout(), "{:}\n", ((*k.borrow()).read()),);
    let l: Value<Option<Value<i32>>> = Rc::new(RefCell::new(Some(Rc::new(RefCell::new(11)))));
    let m: Value<Ptr<i32>> = Rc::new(RefCell::new((*l.borrow()).as_pointer()));
    write!(libcc2rs::cout(), "{:}\n", ((*m.borrow()).read()),);
    return (*c.borrow());
}
