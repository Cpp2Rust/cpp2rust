extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct header {
    pub tag: Value<i32>,
    pub size: Value<i32>,
}
impl Clone for header {
    fn clone(&self) -> Self {
        Self {
            tag: Rc::new(RefCell::new((*self.tag.borrow()).clone())),
            size: Rc::new(RefCell::new((*self.size.borrow()).clone())),
        }
    }
}
impl ByteRepr for header {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.tag.borrow()).to_bytes(&mut buf[0..4]);
        (*self.size.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            tag: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            size: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
        }
    }
}
#[derive(Default)]
pub struct view {
    pub tag: Value<i32>,
}
impl Clone for view {
    fn clone(&self) -> Self {
        Self {
            tag: Rc::new(RefCell::new((*self.tag.borrow()).clone())),
        }
    }
}
impl ByteRepr for view {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.tag.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            tag: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
#[derive(Default)]
pub struct entry {
    pub id: Value<i32>,
}
impl Clone for entry {
    fn clone(&self) -> Self {
        Self {
            id: Rc::new(RefCell::new((*self.id.borrow()).clone())),
        }
    }
}
impl ByteRepr for entry {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.id.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            id: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
thread_local!(
    pub static e0_0: Value<entry> = Rc::new(RefCell::new(entry {
        id: Rc::new(RefCell::new(1)),
    }));
);
thread_local!(
    pub static e1_1: Value<entry> = Rc::new(RefCell::new(entry {
        id: Rc::new(RefCell::new(2)),
    }));
);
thread_local!(
    pub static registry_2: Value<Box<[Ptr<entry>]>> = Rc::new(RefCell::new(Box::new([
        (e0_0.with(Value::clone).as_pointer()),
        (e1_1.with(Value::clone).as_pointer()),
        Ptr::<entry>::null(),
    ])));
);
pub fn get_registry_3(out: Ptr<Ptr<Ptr<entry>>>) {
    let out: Value<Ptr<Ptr<Ptr<entry>>>> = Rc::new(RefCell::new(out));
    let __rhs = (registry_2.with(Value::clone).as_pointer() as Ptr<Ptr<entry>>)
        .reinterpret_cast::<Ptr<entry>>();
    (*out.borrow()).write(__rhs);
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let text: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::from(*b"hi\0")));
    let cp: Value<Ptr<u8>> = Rc::new(RefCell::new((text.as_pointer() as Ptr<u8>)));
    let u: Value<Ptr<u8>> = Rc::new(RefCell::new(
        ((*cp.borrow()).reinterpret_cast::<u8>()).clone(),
    ));
    assert!(((((((*u.borrow()).offset((0) as isize).read()) as i32) == ('h' as i32)) as i32) != 0));
    assert!(((((((*u.borrow()).offset((1) as isize).read()) as i32) == ('i' as i32)) as i32) != 0));
    let h: Value<header> = Rc::new(RefCell::new(header {
        tag: Rc::new(RefCell::new(7)),
        size: Rc::new(RefCell::new(32)),
    }));
    let hp: Value<Ptr<header>> = Rc::new(RefCell::new((h.as_pointer())));
    let v: Value<Ptr<view>> = Rc::new(RefCell::new(
        ((*hp.borrow()).reinterpret_cast::<view>()).clone(),
    ));
    assert!(((((*(*(*v.borrow()).upgrade().deref()).tag.borrow()) == 7) as i32) != 0));
    let data: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::from(*b"hi\0")));
    let vp: Value<AnyPtr> = Rc::new(RefCell::new(
        ((data.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any(),
    ));
    let n: Value<i32> = Rc::new(RefCell::new(2));
    let sel: Value<Ptr<u8>> = Rc::new(RefCell::new(
        if ((((*n.borrow()) < 100) as i32) != 0) {
            (*vp.borrow()).clone()
        } else {
            (AnyPtr::default())
        }
        .reinterpret_cast::<u8>(),
    ));
    assert!((((!((*sel.borrow()).is_null())) as i32) != 0));
    assert!(
        ((((((*sel.borrow()).offset((0) as isize).read()) as i32) == ('h' as i32)) as i32) != 0)
    );
    (*n.borrow_mut()) = 200;
    (*sel.borrow_mut()) = if ((((*n.borrow()) < 100) as i32) != 0) {
        (*vp.borrow()).clone()
    } else {
        (AnyPtr::default())
    }
    .reinterpret_cast::<u8>();
    assert!(((((*sel.borrow()).is_null()) as i32) != 0));
    let avail: Value<Ptr<Ptr<entry>>> = Rc::new(RefCell::new(Ptr::<Ptr<entry>>::null()));
    ({ get_registry_3((avail.as_pointer())) });
    assert!((((!((*avail.borrow()).is_null())) as i32) != 0));
    assert!(
        ((((*(*((*avail.borrow()).offset((0) as isize).read())
            .upgrade()
            .deref())
        .id
        .borrow())
            == 1) as i32)
            != 0)
    );
    assert!(
        ((((*(*((*avail.borrow()).offset((1) as isize).read())
            .upgrade()
            .deref())
        .id
        .borrow())
            == 2) as i32)
            != 0)
    );
    assert!((((((*avail.borrow()).offset((2) as isize).read()).is_null()) as i32) != 0));
    return 0;
}
