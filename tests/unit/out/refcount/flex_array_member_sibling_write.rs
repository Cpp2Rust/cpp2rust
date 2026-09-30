extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(VaArg, FnPtrArg)]
pub struct S {
    pub n: Value<i32>,
    pub name: Value<Box<[u8]>>,
}
impl Clone for S {
    fn clone(&self) -> Self {
        Self {
            n: Rc::new(RefCell::new((*self.n.borrow()).clone())),
            name: Rc::new(RefCell::new((*self.name.borrow()).clone())),
        }
    }
}
impl Default for S {
    fn default() -> Self {
        S {
            n: Rc::new(RefCell::new(0_i32)),
            name: Rc::new(RefCell::new((0..1).map(|_| 0_u8).collect::<Box<[u8]>>())),
        }
    }
}
impl ByteRepr for S {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.n.borrow()).to_bytes(&mut buf[0..4]);
        (*self.name.borrow()).to_bytes(&mut buf[4..5]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            n: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            name: Rc::new(RefCell::new(<Box<[u8]>>::from_bytes(&buf[4..5]))),
        }
    }
}
#[derive(VaArg, FnPtrArg, Default)]
pub struct E {
    pub id: Value<i32>,
    pub w: Value<i32>,
}
impl Clone for E {
    fn clone(&self) -> Self {
        Self {
            id: Rc::new(RefCell::new((*self.id.borrow()).clone())),
            w: Rc::new(RefCell::new((*self.w.borrow()).clone())),
        }
    }
}
impl ByteRepr for E {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.id.borrow()).to_bytes(&mut buf[0..4]);
        (*self.w.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            id: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            w: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
        }
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct T {
    pub n: Value<i32>,
    pub cap: Value<i32>,
    pub a: Value<Box<[E]>>,
}
impl Clone for T {
    fn clone(&self) -> Self {
        Self {
            n: Rc::new(RefCell::new((*self.n.borrow()).clone())),
            cap: Rc::new(RefCell::new((*self.cap.borrow()).clone())),
            a: Rc::new(RefCell::new((*self.a.borrow()).clone())),
        }
    }
}
impl Default for T {
    fn default() -> Self {
        T {
            n: Rc::new(RefCell::new(0_i32)),
            cap: Rc::new(RefCell::new(0_i32)),
            a: Rc::new(RefCell::new(<Value<E>>::default())),
        }
    }
}
impl ByteRepr for T {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.n.borrow()).to_bytes(&mut buf[0..4]);
        (*self.cap.borrow()).to_bytes(&mut buf[4..8]);
        (*self.a.borrow()).to_bytes(&mut buf[8..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            n: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            cap: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
            a: Rc::new(RefCell::new(<Box<[E]>>::from_bytes(&buf[8..8]))),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<Ptr<S>> = Rc::new(RefCell::new(
        libcc2rs::calloc_refcount(1_usize, (8usize as usize).wrapping_add(8_usize))
            .reinterpret_cast::<S>(),
    ));
    assert!((((!((*s.borrow()).is_null())) as i32) != 0));
    {
        (((*(*s.borrow()).upgrade().deref()).name.as_pointer() as Ptr<u8>) as Ptr<u8>)
            .to_any()
            .memcpy(
                &Ptr::<u8>::from_string_literal(b"abcdefg").to_any(),
                8_usize as usize,
            );
        (((*(*s.borrow()).upgrade().deref()).name.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any()
    };
    (*(*(*s.borrow()).upgrade().deref()).n.borrow_mut()) = 5;
    assert!(((((*(*(*s.borrow()).upgrade().deref()).n.borrow()) == 5) as i32) != 0));
    assert!(
        ((({
            let mut __it1 = ((*(*s.borrow()).upgrade().deref()).name.as_pointer() as Ptr<u8>)
                .to_c_string_iterator();
            let mut __it2 = Ptr::<u8>::from_string_literal(b"abcdefg").to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as i32) - (__c2.unwrap_or(0) as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0) as i32)
            != 0)
    );
    libcc2rs::free_refcount((*s.borrow()).to_any());
    let t: Value<Ptr<T>> = Rc::new(RefCell::new(
        libcc2rs::malloc_refcount(
            (8usize as usize).wrapping_add(((2_usize).wrapping_mul((8usize as usize)) as usize)),
        )
        .reinterpret_cast::<T>(),
    ));
    assert!((((!((*t.borrow()).is_null())) as i32) != 0));
    (*(*(*t.borrow()).upgrade().deref()).n.borrow_mut()) = 2;
    (*(*(*t.borrow()).upgrade().deref()).cap.borrow_mut()) = 2;
    (*(*(*(*t.borrow()).upgrade().deref()).a.borrow())[(0) as usize]
        .id
        .borrow_mut()) = 10;
    (*(*(*(*t.borrow()).upgrade().deref()).a.borrow())[(1) as usize]
        .w
        .borrow_mut()) = 20;
    (*(*(*t.borrow()).upgrade().deref()).n.borrow_mut()) = 3;
    assert!(
        (((((((*(*(*(*t.borrow()).upgrade().deref()).a.borrow())[(0) as usize]
            .id
            .borrow())
            == 10) as i32)
            != 0)
            && ((((*(*(*(*t.borrow()).upgrade().deref()).a.borrow())[(1) as usize]
                .w
                .borrow())
                == 20) as i32)
                != 0)) as i32)
            != 0)
    );
    let tail: Value<Ptr<E>> = Rc::new(RefCell::new(
        ((*t.borrow()).offset((1) as isize)).reinterpret_cast::<E>(),
    ));
    assert!(
        ((({
            let _lhs = (*tail.borrow()).clone();
            _lhs == ((*(*t.borrow()).upgrade().deref()).a.as_pointer() as Ptr<E>)
        }) as i32)
            != 0)
    );
    (*(*(*tail.borrow()).offset((1) as isize).upgrade().deref())
        .id
        .borrow_mut()) = 30;
    (*(*(*t.borrow()).upgrade().deref()).cap.borrow_mut()) = 4;
    assert!(
        (((((((*(*(*(*t.borrow()).upgrade().deref()).a.borrow())[(1) as usize]
            .id
            .borrow())
            == 30) as i32)
            != 0)
            && ((((*(*(*(*t.borrow()).upgrade().deref()).a.borrow())[(1) as usize]
                .w
                .borrow())
                == 20) as i32)
                != 0)) as i32)
            != 0)
    );
    assert!(
        (((((((*(*(*t.borrow()).upgrade().deref()).n.borrow()) == 3) as i32) != 0)
            && ((((*(*(*t.borrow()).upgrade().deref()).cap.borrow()) == 4) as i32) != 0))
            as i32)
            != 0)
    );
    libcc2rs::free_refcount((*t.borrow()).to_any());
    return 0;
}
pub fn __cpp2rust_init_globals() {}
