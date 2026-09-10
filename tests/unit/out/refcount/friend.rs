extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn get_0(v: Ptr<V>) -> i32 {
    return (*(*v.upgrade().deref()).x.borrow());
}
pub fn operator_eq_1(a: Ptr<V>, b: Ptr<V>) -> bool {
    return {
        let _lhs = (*(*a.upgrade().deref()).x.borrow());
        _lhs == (*(*b.upgrade().deref()).x.borrow())
    };
}
pub fn scaled_2(v: Ptr<V>, k: i32) -> i32 {
    let k: Value<i32> = Rc::new(RefCell::new(k));
    return {
        let _lhs = (*(*v.upgrade().deref()).x.borrow());
        _lhs * (*k.borrow())
    };
}
pub fn scaled_3(v: Ptr<V>, k: f64) -> f64 {
    let k: Value<f64> = Rc::new(RefCell::new(k));
    return {
        let _lhs = ((*(*v.upgrade().deref()).x.borrow()) as f64);
        _lhs * (*k.borrow())
    };
}
#[derive(Default)]
pub struct V {
    pub x: Value<i32>,
}
impl std::cmp::PartialEq for V {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_1(
                Rc::new(RefCell::new(self.clone())).as_pointer(),
                Rc::new(RefCell::new(other.clone())).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for V {}
impl Clone for V {
    fn clone(&self) -> Self {
        let __this: Value<V> = Rc::new(RefCell::new(Self {
            x: Rc::new(RefCell::new((*self.x.borrow()))),
        }));
        let this: Ptr<V> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for V {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.x.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
pub fn get_4(w: Ptr<W_int_>) -> i32 {
    return (*(*w.upgrade().deref()).x.borrow());
}
#[derive(Default)]
pub struct W_int_ {
    pub x: Value<i32>,
}
impl Clone for W_int_ {
    fn clone(&self) -> Self {
        let __this: Value<W_int_> = Rc::new(RefCell::new(Self {
            x: Rc::new(RefCell::new((*self.x.borrow()))),
        }));
        let this: Ptr<W_int_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for W_int_ {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.x.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
pub fn get_5(w: Ptr<W_long_>) -> i64 {
    return (*(*w.upgrade().deref()).x.borrow());
}
#[derive(Default)]
pub struct W_long_ {
    pub x: Value<i64>,
}
impl Clone for W_long_ {
    fn clone(&self) -> Self {
        let __this: Value<W_long_> = Rc::new(RefCell::new(Self {
            x: Rc::new(RefCell::new((*self.x.borrow()))),
        }));
        let this: Ptr<W_long_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for W_long_ {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.x.borrow()).to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x: Rc::new(RefCell::new(<i64>::from_bytes(&buf[0..8]))),
        }
    }
}
#[derive(Default)]
pub struct D {
    pub x: Value<i32>,
}
impl Clone for D {
    fn clone(&self) -> Self {
        let __this: Value<D> = Rc::new(RefCell::new(Self {
            x: Rc::new(RefCell::new((*self.x.borrow()))),
        }));
        let this: Ptr<D> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for D {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.x.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
pub fn declared_then_defined_6(d: Ptr<D>) -> i32 {
    return ((*(*d.upgrade().deref()).x.borrow()) + 1);
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<V> = Rc::new(RefCell::new(V {
        x: Rc::new(RefCell::new(3)),
    }));
    let b: Value<V> = Rc::new(RefCell::new(V {
        x: Rc::new(RefCell::new(3)),
    }));
    let c: Value<V> = Rc::new(RefCell::new(V {
        x: Rc::new(RefCell::new(4)),
    }));
    assert!((({ get_0(a.as_pointer(),) }) == 3));
    assert!(
        ({
            let _a: Ptr<V> = a.as_pointer();
            operator_eq_1(_a, b.as_pointer())
        })
    );
    assert!(
        !({
            let _a: Ptr<V> = a.as_pointer();
            operator_eq_1(_a, c.as_pointer())
        })
    );
    assert!((({ scaled_2(c.as_pointer(), 2,) }) == 8));
    assert!((({ scaled_3(c.as_pointer(), 1.5E+0,) }) == 6.0E+0));
    let wi: Value<W_int_> = Rc::new(RefCell::new(W_int_ {
        x: Rc::new(RefCell::new(5)),
    }));
    let wl: Value<W_long_> = Rc::new(RefCell::new(W_long_ {
        x: Rc::new(RefCell::new(6_i64)),
    }));
    assert!((({ get_4(wi.as_pointer(),) }) == 5));
    assert!((({ get_5(wl.as_pointer(),) }) == 6_i64));
    let d: Value<D> = Rc::new(RefCell::new(D {
        x: Rc::new(RefCell::new(7)),
    }));
    assert!((({ declared_then_defined_6(d.as_pointer(),) }) == 8));
    return 0;
}
