extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn get_0(t: Local_1) -> i32 {
    let t: Value<Local_1> = Rc::new(RefCell::new(t));
    return ((*(*t.borrow()).x.borrow()) as i32);
}
pub fn get_2(t: Local_3) -> i32 {
    let t: Value<Local_3> = Rc::new(RefCell::new(t));
    return (*(*t.borrow()).x.borrow());
}
pub fn get_4(t: Local_5) -> i32 {
    let t: Value<Local_5> = Rc::new(RefCell::new(t));
    return (*(*t.borrow()).x.borrow());
}
pub fn get_6(t: Local_7) -> i32 {
    let t: Value<Local_7> = Rc::new(RefCell::new(t));
    return ((*(*t.borrow()).x.borrow()) as i32);
}
pub fn twice_8(t: Local_3) -> i32 {
    let t: Value<Local_3> = Rc::new(RefCell::new(t));
    return ((*(*t.borrow()).x.borrow()) * 2);
}
pub fn wrap_9(v: i32) -> i32 {
    let v: Value<i32> = Rc::new(RefCell::new(v));
    let l: Value<Local_5> = Rc::new(RefCell::new(Local_5 {
        x: Rc::new(RefCell::new((*v.borrow()))),
    }));
    return ({ get_4((*l.borrow()).clone()) });
}
pub fn wrap_10(v: i64) -> i32 {
    let v: Value<i64> = Rc::new(RefCell::new(v));
    let l: Value<Local_7> = Rc::new(RefCell::new(Local_7 {
        x: Rc::new(RefCell::new((*v.borrow()))),
    }));
    return ({ get_6((*l.borrow()).clone()) });
}
#[derive(Default)]
pub struct Local_5 {
    pub x: Value<i32>,
}
impl Clone for Local_5 {
    fn clone(&self) -> Self {
        let __this: Value<Local_5> = Rc::new(RefCell::new(Self {
            x: Rc::new(RefCell::new((*self.x.borrow()))),
        }));
        let this: Ptr<Local_5> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Local_5 {
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
#[derive(Default)]
pub struct Local_7 {
    pub x: Value<i64>,
}
impl Clone for Local_7 {
    fn clone(&self) -> Self {
        let __this: Value<Local_7> = Rc::new(RefCell::new(Self {
            x: Rc::new(RefCell::new((*self.x.borrow()))),
        }));
        let this: Ptr<Local_7> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Local_7 {
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
pub fn other_11() -> i32 {
    let l: Value<Local_1> = Rc::new(RefCell::new(Local_1 {
        x: Rc::new(RefCell::new(3_i64)),
        y: Rc::new(RefCell::new(4_i64)),
    }));
    return (({ get_0((*l.borrow()).clone()) }) + ((*(*l.borrow()).y.borrow()) as i32));
}
#[derive(Default)]
pub struct Local_1 {
    pub x: Value<i64>,
    pub y: Value<i64>,
}
impl Clone for Local_1 {
    fn clone(&self) -> Self {
        let __this: Value<Local_1> = Rc::new(RefCell::new(Self {
            x: Rc::new(RefCell::new((*self.x.borrow()))),
            y: Rc::new(RefCell::new((*self.y.borrow()))),
        }));
        let this: Ptr<Local_1> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Local_1 {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.x.borrow()).to_bytes(&mut buf[0..8]);
        (*self.y.borrow()).to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x: Rc::new(RefCell::new(<i64>::from_bytes(&buf[0..8]))),
            y: Rc::new(RefCell::new(<i64>::from_bytes(&buf[8..16]))),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let l: Value<Local_3> = Rc::new(RefCell::new(Local_3 {
        x: Rc::new(RefCell::new(7)),
    }));
    assert!((({ get_2((*l.borrow()).clone(),) }) == 7));
    assert!((({ twice_8((*l.borrow()).clone(),) }) == 14));
    assert!((({ other_11() }) == 7));
    assert!((({ wrap_9(5,) }) == 5));
    assert!((({ wrap_10(6_i64,) }) == 6));
    return 0;
}
#[derive(Default)]
pub struct Local_3 {
    pub x: Value<i32>,
}
impl Clone for Local_3 {
    fn clone(&self) -> Self {
        let __this: Value<Local_3> = Rc::new(RefCell::new(Self {
            x: Rc::new(RefCell::new((*self.x.borrow()))),
        }));
        let this: Ptr<Local_3> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Local_3 {
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
