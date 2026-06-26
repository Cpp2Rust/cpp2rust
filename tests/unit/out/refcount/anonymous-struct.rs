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
impl ByteRepr for Outer_Named {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.a.borrow()).to_bytes(&mut buf[0..4]);
        (*self.b.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            a: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            b: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
        }
    }
}
#[derive(Default)]
pub struct anon_0 {
    pub c: Value<i32>,
    pub d: Value<i32>,
}
impl Clone for anon_0 {
    fn clone(&self) -> Self {
        let mut this = Self {
            c: Rc::new(RefCell::new((*self.c.borrow()))),
            d: Rc::new(RefCell::new((*self.d.borrow()))),
        };
        this
    }
}
impl ByteRepr for anon_0 {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.c.borrow()).to_bytes(&mut buf[0..4]);
        (*self.d.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            c: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            d: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
        }
    }
}
#[derive(Default)]
pub struct anon_1 {
    pub g: Value<i32>,
    pub h: Value<i32>,
}
impl Clone for anon_1 {
    fn clone(&self) -> Self {
        let mut this = Self {
            g: Rc::new(RefCell::new((*self.g.borrow()))),
            h: Rc::new(RefCell::new((*self.h.borrow()))),
        };
        this
    }
}
impl ByteRepr for anon_1 {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.g.borrow()).to_bytes(&mut buf[0..4]);
        (*self.h.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            g: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            h: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
        }
    }
}
#[derive(Default)]
pub struct anon_2 {
    pub e: Value<i32>,
    pub f: Value<i32>,
}
impl Clone for anon_2 {
    fn clone(&self) -> Self {
        let mut this = Self {
            e: Rc::new(RefCell::new((*self.e.borrow()))),
            f: Rc::new(RefCell::new((*self.f.borrow()))),
        };
        this
    }
}
impl ByteRepr for anon_2 {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.e.borrow()).to_bytes(&mut buf[0..4]);
        (*self.f.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            e: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            f: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
        }
    }
}
#[derive(Default)]
pub struct anon_4 {
    pub j: Value<i32>,
}
impl Clone for anon_4 {
    fn clone(&self) -> Self {
        let mut this = Self {
            j: Rc::new(RefCell::new((*self.j.borrow()))),
        };
        this
    }
}
impl ByteRepr for anon_4 {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.j.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            j: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
#[derive(Default)]
pub struct anon_5 {
    pub k: Value<i32>,
}
impl Clone for anon_5 {
    fn clone(&self) -> Self {
        let mut this = Self {
            k: Rc::new(RefCell::new((*self.k.borrow()))),
        };
        this
    }
}
impl ByteRepr for anon_5 {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.k.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            k: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
#[derive(Default)]
pub struct anon_3 {
    pub i: Value<i32>,
    pub inner_named: Value<anon_4>,
    pub anon_5: Value<anon_5>,
}
impl Clone for anon_3 {
    fn clone(&self) -> Self {
        let mut this = Self {
            i: Rc::new(RefCell::new((*self.i.borrow()))),
            inner_named: Rc::new(RefCell::new((*self.inner_named.borrow()).clone())),
            anon_5: Rc::new(RefCell::new((*self.anon_5.borrow()).clone())),
        };
        this
    }
}
impl ByteRepr for anon_3 {
    fn byte_size() -> usize {
        12
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.i.borrow()).to_bytes(&mut buf[0..4]);
        (*self.inner_named.borrow()).to_bytes(&mut buf[4..8]);
        (*self.anon_5.borrow()).to_bytes(&mut buf[8..12]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            i: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            inner_named: Rc::new(RefCell::new(<anon_4>::from_bytes(&buf[4..8]))),
            anon_5: Rc::new(RefCell::new(<anon_5>::from_bytes(&buf[8..12]))),
        }
    }
}
#[derive(Default)]
pub struct Outer {
    pub named: Value<Outer_Named>,
    pub anonymous_named_0: Value<anon_0>,
    pub anonymous_named_1: Value<anon_1>,
    pub anon_2: Value<anon_2>,
    pub anon_3: Value<anon_3>,
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
impl ByteRepr for Outer {
    fn byte_size() -> usize {
        44
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.named.borrow()).to_bytes(&mut buf[0..8]);
        (*self.anonymous_named_0.borrow()).to_bytes(&mut buf[8..16]);
        (*self.anonymous_named_1.borrow()).to_bytes(&mut buf[16..24]);
        (*self.anon_2.borrow()).to_bytes(&mut buf[24..32]);
        (*self.anon_3.borrow()).to_bytes(&mut buf[32..44]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            named: Rc::new(RefCell::new(<Outer_Named>::from_bytes(&buf[0..8]))),
            anonymous_named_0: Rc::new(RefCell::new(<anon_0>::from_bytes(&buf[8..16]))),
            anonymous_named_1: Rc::new(RefCell::new(<anon_1>::from_bytes(&buf[16..24]))),
            anon_2: Rc::new(RefCell::new(<anon_2>::from_bytes(&buf[24..32]))),
            anon_3: Rc::new(RefCell::new(<anon_3>::from_bytes(&buf[32..44]))),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let o: Value<Outer> = Rc::new(RefCell::new(Outer {
        named: Rc::new(RefCell::new(Outer_Named {
            a: Rc::new(RefCell::new(<i32>::default())),
            b: Rc::new(RefCell::new(<i32>::default())),
        })),
        anonymous_named_0: Rc::new(RefCell::new(anon_0 {
            c: Rc::new(RefCell::new(<i32>::default())),
            d: Rc::new(RefCell::new(<i32>::default())),
        })),
        anonymous_named_1: Rc::new(RefCell::new(anon_1 {
            g: Rc::new(RefCell::new(<i32>::default())),
            h: Rc::new(RefCell::new(<i32>::default())),
        })),
        anon_2: Rc::new(RefCell::new(anon_2 {
            e: Rc::new(RefCell::new(<i32>::default())),
            f: Rc::new(RefCell::new(<i32>::default())),
        })),
        anon_3: Rc::new(RefCell::new(anon_3 {
            i: Rc::new(RefCell::new(<i32>::default())),
            inner_named: Rc::new(RefCell::new(anon_4 {
                j: Rc::new(RefCell::new(<i32>::default())),
            })),
            anon_5: Rc::new(RefCell::new(anon_5 {
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
    (*(*(*(*o.borrow()).anon_3.borrow()).anon_5.borrow())
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
        ((*(*(*(*o.borrow()).anon_3.borrow()).anon_5.borrow())
            .k
            .borrow())
            == 11)
    );
    #[derive(Default)]
    pub struct anon_6 {
        pub x: Value<i32>,
        pub z: Value<i32>,
    }
    impl Clone for anon_6 {
        fn clone(&self) -> Self {
            let mut this = Self {
                x: Rc::new(RefCell::new((*self.x.borrow()))),
                z: Rc::new(RefCell::new((*self.z.borrow()))),
            };
            this
        }
    }
    impl ByteRepr for anon_6 {
        fn byte_size() -> usize {
            8
        }
        fn to_bytes(&self, buf: &mut [u8]) {
            (*self.x.borrow()).to_bytes(&mut buf[0..4]);
            (*self.z.borrow()).to_bytes(&mut buf[4..8]);
        }
        fn from_bytes(buf: &[u8]) -> Self {
            Self {
                x: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
                z: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
            }
        }
    };
    let s: Value<anon_6> = Rc::new(RefCell::new(<anon_6>::default()));
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
