extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive()]
pub struct label {
    pub name: Value<Ptr<u8>>,
    pub probe: Value<FnPtr<fn() -> i32>>,
    pub mask: Value<i32>,
}
impl Clone for label {
    fn clone(&self) -> Self {
        Self {
            name: Rc::new(RefCell::new((*self.name.borrow()).clone())),
            probe: Rc::new(RefCell::new((*self.probe.borrow()).clone())),
            mask: Rc::new(RefCell::new((*self.mask.borrow()).clone())),
        }
    }
}
impl Default for label {
    fn default() -> Self {
        label {
            name: Rc::new(RefCell::new(Ptr::<u8>::null())),
            probe: Rc::new(RefCell::new(FnPtr::null())),
            mask: <Value<i32>>::default(),
        }
    }
}
impl ByteRepr for label {
    fn byte_size() -> usize {
        24
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.name.borrow()).to_bytes(&mut buf[0..8]);
        (*self.probe.borrow()).to_bytes(&mut buf[8..16]);
        (*self.mask.borrow()).to_bytes(&mut buf[16..20]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            name: Rc::new(RefCell::new(<Ptr<u8>>::from_bytes(&buf[0..8]))),
            probe: Rc::new(RefCell::new(<FnPtr<fn() -> i32>>::from_bytes(&buf[8..16]))),
            mask: Rc::new(RefCell::new(<i32>::from_bytes(&buf[16..20]))),
        }
    }
}
pub fn probe_two_0() -> i32 {
    return 1;
}
thread_local!(
    pub static table_1: Value<Box<[label]>> = Rc::new(RefCell::new(Box::new([
        label {
            name: Rc::new(RefCell::new(Ptr::from_string_literal(b"first"))),
            probe: Rc::new(RefCell::new(FnPtr::null())),
            mask: Rc::new(RefCell::new((1 << 4))),
        },
        label {
            name: Rc::new(RefCell::new(Ptr::from_string_literal(b"second"))),
            probe: Rc::new(RefCell::new((FnPtr::<fn() -> i32>::new(probe_two_0)))),
            mask: Rc::new(RefCell::new((1 << 5))),
        },
    ])));
);
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(
        ((((((*(*table_1.with(Value::clone).borrow())[(0) as usize]
            .name
            .borrow())
        .offset((0) as isize)
        .read()) as i32)
            == ('f' as i32)) as i32)
            != 0)
    );
    assert!(
        ((((((*(*table_1.with(Value::clone).borrow())[(0) as usize]
            .name
            .borrow())
        .offset((4) as isize)
        .read()) as i32)
            == ('t' as i32)) as i32)
            != 0)
    );
    assert!(
        ((((*(*table_1.with(Value::clone).borrow())[(0) as usize]
            .probe
            .borrow())
        .is_null()) as i32)
            != 0)
    );
    assert!(
        ((((*(*table_1.with(Value::clone).borrow())[(0) as usize]
            .mask
            .borrow())
            == 16) as i32)
            != 0)
    );
    assert!(
        ((((((*(*table_1.with(Value::clone).borrow())[(1) as usize]
            .name
            .borrow())
        .offset((0) as isize)
        .read()) as i32)
            == ('s' as i32)) as i32)
            != 0)
    );
    assert!(
        (((({
            (*(*(*table_1.with(Value::clone).borrow())[(1) as usize]
                .probe
                .borrow()))()
        }) == 1) as i32)
            != 0)
    );
    assert!(
        ((((*(*table_1.with(Value::clone).borrow())[(1) as usize]
            .mask
            .borrow())
            == 32) as i32)
            != 0)
    );
    let tail: Value<Ptr<u8>> =
        Rc::new(RefCell::new((Ptr::from_string_literal(b"ab.cd").offset(2))));
    assert!(
        ((((((*tail.borrow()).offset((0) as isize).read()) as i32) == ('.' as i32)) as i32) != 0)
    );
    assert!(
        ((((((*tail.borrow()).offset((1) as isize).read()) as i32) == ('c' as i32)) as i32) != 0)
    );
    assert!(
        ((((((*tail.borrow()).offset((2) as isize).read()) as i32) == ('d' as i32)) as i32) != 0)
    );
    let have: Value<i32> = Rc::new(RefCell::new(0));
    let p: Value<AnyPtr> = Rc::new(RefCell::new(if ((*have.borrow()) != 0) {
        (*(*table_1.with(Value::clone).borrow())[(0) as usize]
            .name
            .borrow())
        .clone()
        .to_any()
    } else {
        Ptr::from_string_literal(b"").to_any()
    }));
    assert!(
        (((((((*p.borrow()).reinterpret_cast::<u8>())
            .offset((0) as isize)
            .read()) as i32)
            == ('\0' as i32)) as i32)
            != 0)
    );
    (*have.borrow_mut()) = 1;
    (*p.borrow_mut()) = if ((*have.borrow()) != 0) {
        (*(*table_1.with(Value::clone).borrow())[(0) as usize]
            .name
            .borrow())
        .clone()
        .to_any()
    } else {
        Ptr::from_string_literal(b"").to_any()
    };
    assert!(
        (((((((*p.borrow()).reinterpret_cast::<u8>())
            .offset((0) as isize)
            .read()) as i32)
            == ('f' as i32)) as i32)
            != 0)
    );
    return 0;
}
