extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Outer_Named {
    pub a: Value<i32>,
    pub b: Value<i32>,
}
impl Clone for Outer_Named {
    fn clone(&self) -> Self {
        let mut this = Self {
            a: Rc::new(RefCell::new((*self.a.borrow()))),
            b: Rc::new(RefCell::new((*self.b.borrow()))),
        };
        this
    }
}
impl ByteRepr for Outer_Named {}
#[derive(Default)]
pub struct Outer_anon_0 {
    pub c: Value<i32>,
    pub d: Value<i32>,
}
impl Clone for Outer_anon_0 {
    fn clone(&self) -> Self {
        let mut this = Self {
            c: Rc::new(RefCell::new((*self.c.borrow()))),
            d: Rc::new(RefCell::new((*self.d.borrow()))),
        };
        this
    }
}
impl ByteRepr for Outer_anon_0 {}
#[derive(Default)]
pub struct Outer_anon_1 {
    pub g: Value<i32>,
    pub h: Value<i32>,
}
impl Clone for Outer_anon_1 {
    fn clone(&self) -> Self {
        let mut this = Self {
            g: Rc::new(RefCell::new((*self.g.borrow()))),
            h: Rc::new(RefCell::new((*self.h.borrow()))),
        };
        this
    }
}
impl ByteRepr for Outer_anon_1 {}
#[derive(Default)]
pub struct Outer_anon_2 {
    pub e: Value<i32>,
    pub f: Value<i32>,
}
impl Clone for Outer_anon_2 {
    fn clone(&self) -> Self {
        let mut this = Self {
            e: Rc::new(RefCell::new((*self.e.borrow()))),
            f: Rc::new(RefCell::new((*self.f.borrow()))),
        };
        this
    }
}
impl ByteRepr for Outer_anon_2 {}
#[derive(Default)]
pub struct Outer_anon_3_anon_0 {
    pub j: Value<i32>,
}
impl Clone for Outer_anon_3_anon_0 {
    fn clone(&self) -> Self {
        let mut this = Self {
            j: Rc::new(RefCell::new((*self.j.borrow()))),
        };
        this
    }
}
impl ByteRepr for Outer_anon_3_anon_0 {}
#[derive(Default)]
pub struct Outer_anon_3_anon_1 {
    pub k: Value<i32>,
}
impl Clone for Outer_anon_3_anon_1 {
    fn clone(&self) -> Self {
        let mut this = Self {
            k: Rc::new(RefCell::new((*self.k.borrow()))),
        };
        this
    }
}
impl ByteRepr for Outer_anon_3_anon_1 {}
#[derive(Default)]
pub struct Outer_anon_3 {
    pub i: Value<i32>,
    pub inner_named: Value<Outer_anon_3_anon_0>,
    pub anon_1: Value<Outer_anon_3_anon_1>,
}
impl Clone for Outer_anon_3 {
    fn clone(&self) -> Self {
        let mut this = Self {
            i: Rc::new(RefCell::new((*self.i.borrow()))),
            inner_named: Rc::new(RefCell::new((*self.inner_named.borrow()).clone())),
            anon_1: Rc::new(RefCell::new((*self.anon_1.borrow()).clone())),
        };
        this
    }
}
impl ByteRepr for Outer_anon_3 {}
#[derive(Default)]
pub struct Outer {
    pub named: Value<Outer_Named>,
    pub anonymous_named_0: Value<Outer_anon_0>,
    pub anonymous_named_1: Value<Outer_anon_1>,
    pub anon_2: Value<Outer_anon_2>,
    pub anon_3: Value<Outer_anon_3>,
}
impl Clone for Outer {
    fn clone(&self) -> Self {
        let mut this = Self {
            named: Rc::new(RefCell::new((*self.named.borrow()).clone())),
            anonymous_named_0: Rc::new(RefCell::new((*self.anonymous_named_0.borrow()).clone())),
            anonymous_named_1: Rc::new(RefCell::new((*self.anonymous_named_1.borrow()).clone())),
            anon_2: Rc::new(RefCell::new((*self.anon_2.borrow()).clone())),
            anon_3: Rc::new(RefCell::new((*self.anon_3.borrow()).clone())),
        };
        this
    }
}
impl ByteRepr for Outer {}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let o: Value<Outer> = Rc::new(RefCell::new(Outer {
        named: Rc::new(RefCell::new(Outer_Named {
            a: Rc::new(RefCell::new(<i32>::default())),
            b: Rc::new(RefCell::new(<i32>::default())),
        })),
        anonymous_named_0: Rc::new(RefCell::new(Outer_anon_0 {
            c: Rc::new(RefCell::new(<i32>::default())),
            d: Rc::new(RefCell::new(<i32>::default())),
        })),
        anonymous_named_1: Rc::new(RefCell::new(Outer_anon_1 {
            g: Rc::new(RefCell::new(<i32>::default())),
            h: Rc::new(RefCell::new(<i32>::default())),
        })),
        anon_2: Rc::new(RefCell::new(Outer_anon_2 {
            e: Rc::new(RefCell::new(<i32>::default())),
            f: Rc::new(RefCell::new(<i32>::default())),
        })),
        anon_3: Rc::new(RefCell::new(Outer_anon_3 {
            i: Rc::new(RefCell::new(<i32>::default())),
            inner_named: Rc::new(RefCell::new(Outer_anon_3_anon_0 {
                j: Rc::new(RefCell::new(<i32>::default())),
            })),
            anon_1: Rc::new(RefCell::new(Outer_anon_3_anon_1 {
                k: Rc::new(RefCell::new(<i32>::default())),
            })),
        })),
    }));
    (*(*(*o.borrow()).named.borrow()).a.borrow_mut()) = 1;
    (*(*(*o.borrow()).named.borrow()).b.borrow_mut()) = 2;
    (*(*(*o.borrow()).anonymous_named_0.borrow()).c.borrow_mut()) = 3;
    (*(*(*o.borrow()).anonymous_named_0.borrow()).d.borrow_mut()) = 4;
    (*(*(*o.borrow()).anonymous_named_1.borrow()).g.borrow_mut()) = 5;
    (*(*(*o.borrow()).anonymous_named_1.borrow()).h.borrow_mut()) = 6;
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
    assert!(((*(*(*o.borrow()).anonymous_named_0.borrow()).c.borrow()) == 3));
    assert!(((*(*(*o.borrow()).anonymous_named_0.borrow()).d.borrow()) == 4));
    assert!(((*(*(*o.borrow()).anonymous_named_1.borrow()).g.borrow()) == 5));
    assert!(((*(*(*o.borrow()).anonymous_named_1.borrow()).h.borrow()) == 6));
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
    impl Clone for anon_0 {
        fn clone(&self) -> Self {
            let mut this = Self {
                x: Rc::new(RefCell::new((*self.x.borrow()))),
                z: Rc::new(RefCell::new((*self.z.borrow()))),
            };
            this
        }
    }
    impl ByteRepr for anon_0 {};
    let s: Value<anon_0> = Rc::new(RefCell::new(<anon_0>::default()));
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
