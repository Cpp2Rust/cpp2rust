extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Named {
    pub a: Value<i32>,
    pub b: Value<i32>,
}
impl ByteRepr for Named {}
#[derive(Default)]
pub struct Outer_anon_0 {
    pub c: Value<i32>,
    pub d: Value<i32>,
}
impl ByteRepr for Outer_anon_0 {}
#[derive(Default)]
pub struct Outer_anon_1 {
    pub g: Value<i32>,
    pub h: Value<i32>,
}
impl ByteRepr for Outer_anon_1 {}
#[derive(Default)]
pub struct Outer_anon_2 {
    pub e: Value<i32>,
    pub f: Value<i32>,
}
impl ByteRepr for Outer_anon_2 {}
#[derive(Default)]
pub struct Outer_anon_3_anon_0 {
    pub j: Value<i32>,
}
impl ByteRepr for Outer_anon_3_anon_0 {}
#[derive(Default)]
pub struct Outer_anon_3_anon_1 {
    pub k: Value<i32>,
}
impl ByteRepr for Outer_anon_3_anon_1 {}
#[derive(Default)]
pub struct Outer_anon_3 {
    pub i: Value<i32>,
    pub inner_named: Value<Outer_anon_3_anon_0>,
    pub anon_1: Value<Outer_anon_3_anon_1>,
}
impl ByteRepr for Outer_anon_3 {}
#[derive(Default)]
pub struct Outer {
    pub named: Value<Named>,
    pub anon0: Value<Outer_anon_0>,
    pub anon1: Value<Outer_anon_1>,
    pub anon_2: Value<Outer_anon_2>,
    pub anon_3: Value<Outer_anon_3>,
}
impl ByteRepr for Outer {}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let o: Value<Outer> = Rc::new(RefCell::new(Outer {
        named: Rc::new(RefCell::new(Named {
            a: Rc::new(RefCell::new(0)),
            b: Rc::new(RefCell::new(<i32>::default())),
        })),
        anon0: Rc::new(RefCell::new(<Outer_anon_0>::default())),
        anon1: Rc::new(RefCell::new(<Outer_anon_1>::default())),
        anon_2: Rc::new(RefCell::new(<Outer_anon_2>::default())),
        anon_3: Rc::new(RefCell::new(<Outer_anon_3>::default())),
    }));
    (*(*(*o.borrow()).named.borrow()).a.borrow_mut()) = 1;
    (*(*(*o.borrow()).named.borrow()).b.borrow_mut()) = 2;
    (*(*(*o.borrow()).anon0.borrow()).c.borrow_mut()) = 3;
    (*(*(*o.borrow()).anon0.borrow()).d.borrow_mut()) = 4;
    (*(*(*o.borrow()).anon1.borrow()).g.borrow_mut()) = 5;
    (*(*(*o.borrow()).anon1.borrow()).h.borrow_mut()) = 6;
    (*(*(*o.borrow()).anon_2.borrow()).e.borrow_mut()) = 7;
    (*(*(*o.borrow()).anon_2.borrow()).f.borrow_mut()) = 8;
    (*(*(*o.borrow()).anon_3.borrow()).i.borrow_mut()) = 9;
    (*(*(*(*o.borrow()).anon_3.borrow()).inner_named.borrow())
        .j
        .borrow_mut()) = 10;
    (*(*(*(*o.borrow()).anon_3.borrow()).anon_1.borrow())
        .k
        .borrow_mut()) = 11;
    assert!(((*(*(*o.borrow()).named.borrow()).a.borrow()) == 1));
    assert!(((*(*(*o.borrow()).named.borrow()).b.borrow()) == 2));
    assert!(((*(*(*o.borrow()).anon0.borrow()).c.borrow()) == 3));
    assert!(((*(*(*o.borrow()).anon0.borrow()).d.borrow()) == 4));
    assert!(((*(*(*o.borrow()).anon1.borrow()).g.borrow()) == 5));
    assert!(((*(*(*o.borrow()).anon1.borrow()).h.borrow()) == 6));
    assert!(((*(*(*o.borrow()).anon_2.borrow()).e.borrow()) == 7));
    assert!(((*(*(*o.borrow()).anon_2.borrow()).f.borrow()) == 8));
    assert!(((*(*(*o.borrow()).anon_3.borrow()).i.borrow()) == 9));
    assert!(
        ((*(*(*(*o.borrow()).anon_3.borrow()).inner_named.borrow())
            .j
            .borrow())
            == 10)
    );
    assert!(
        ((*(*(*(*o.borrow()).anon_3.borrow()).anon_1.borrow())
            .k
            .borrow())
            == 11)
    );
    #[derive(Default)]
    pub struct anon_0 {
        pub x: Value<i32>,
        pub z: Value<i32>,
    }
    impl ByteRepr for anon_0 {};
    let s: Value<anon_0> = <Value<anon_0>>::default();
    (*(*s.borrow()).x.borrow_mut()) = 1;
    (*(*s.borrow()).z.borrow_mut()) = 2;
    assert!(
        ({
            (*(*s.borrow()).x.borrow_mut()) = 1;
            (*(*s.borrow()).x.borrow())
        } != 0)
    );
    assert!(
        ({
            (*(*s.borrow()).z.borrow_mut()) = 2;
            (*(*s.borrow()).z.borrow())
        } != 0)
    );
    return 0;
}
