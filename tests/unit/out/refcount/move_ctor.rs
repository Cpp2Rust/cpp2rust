extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct MoveOnly {
    pub v: Value<i32>,
}
impl MoveOnly {
    pub fn MoveOnly(v: i32) -> Self {
        let v: Value<i32> = Rc::new(RefCell::new(v));
        let __this: Value<MoveOnly> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*v.borrow()))),
        }));
        let this: Ptr<MoveOnly> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn MoveOnly_pmutMoveOnly(o: Ptr<MoveOnly>) -> Self {
        let __this: Value<MoveOnly> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*(*o.upgrade().deref()).v.borrow()))),
        }));
        let this: Ptr<MoveOnly> = __this.as_pointer();
        (*(*o.upgrade().deref()).v.borrow_mut()) = 0;
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for MoveOnly {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.v.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
#[derive()]
pub struct ConstMove {
    pub mark: Value<i32>,
}
impl ConstMove {
    pub fn ConstMove() -> Self {
        let __this: Value<ConstMove> = Rc::new(RefCell::new(Self {
            mark: Rc::new(RefCell::new(0)),
        }));
        let this: Ptr<ConstMove> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn ConstMove_pmutConstMove(o: Ptr<ConstMove>) -> Self {
        let __this: Value<ConstMove> = Rc::new(RefCell::new(Self {
            mark: Rc::new(RefCell::new(((*(*o.upgrade().deref()).mark.borrow()) + 1))),
        }));
        let this: Ptr<ConstMove> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn ConstMove_pconstConstMove(o: Ptr<ConstMove>) -> Self {
        let __this: Value<ConstMove> = Rc::new(RefCell::new(Self {
            mark: Rc::new(RefCell::new(((*(*o.upgrade().deref()).mark.borrow()) + 10))),
        }));
        let this: Ptr<ConstMove> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for ConstMove {
    fn default() -> Self {
        { ConstMove::ConstMove() }
    }
}
impl ByteRepr for ConstMove {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.mark.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            mark: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
pub fn by_value_0(m: MoveOnly) -> i32 {
    let m: Value<MoveOnly> = Rc::new(RefCell::new(m));
    return (*(*m.borrow()).v.borrow());
}
pub fn make_1(v: i32) -> MoveOnly {
    let v: Value<i32> = Rc::new(RefCell::new(v));
    let m: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::MoveOnly({ (*v.borrow()) })));
    return MoveOnly::MoveOnly_pmutMoveOnly({ m.as_pointer() });
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::MoveOnly({ 1 })));
    let b: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::MoveOnly_pmutMoveOnly({
        a.as_pointer()
    })));
    assert!(((*(*b.borrow()).v.borrow()) == 1));
    assert!(((*(*a.borrow()).v.borrow()) == 0));
    let c: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::MoveOnly_pmutMoveOnly({
        b.as_pointer()
    })));
    assert!(((*(*c.borrow()).v.borrow()) == 1));
    assert!(((*(*b.borrow()).v.borrow()) == 0));
    let d: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::MoveOnly_pmutMoveOnly({
        c.as_pointer()
    })));
    assert!(((*(*d.borrow()).v.borrow()) == 1));
    assert!(((*(*c.borrow()).v.borrow()) == 0));
    let e: Value<MoveOnly> = Rc::new(RefCell::new(({ make_1(5) })));
    assert!(((*(*e.borrow()).v.borrow()) == 5));
    assert!((({ by_value_0(MoveOnly::MoveOnly({ 6 },),) }) == 6));
    assert!((({ by_value_0(MoveOnly::MoveOnly_pmutMoveOnly({ e.as_pointer() },),) }) == 5));
    assert!(((*(*e.borrow()).v.borrow()) == 0));
    let vec_: Value<Vec<MoveOnly>> = Rc::new(RefCell::new(Vec::new()));
    (*vec_.borrow_mut()).push(MoveOnly::MoveOnly({ 7 }));
    let f: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::MoveOnly({ 8 })));
    (*vec_.borrow_mut()).push(std::mem::take(&mut (*f.borrow_mut())));
    assert!(
        ((*(*(vec_.as_pointer() as Ptr<MoveOnly>)
            .offset(0_usize)
            .upgrade()
            .deref())
        .v
        .borrow())
            == 7)
            && ((*(*(vec_.as_pointer() as Ptr<MoveOnly>)
                .offset(1_usize)
                .upgrade()
                .deref())
            .v
            .borrow())
                == 8)
    );
    assert!(((*(*f.borrow()).v.borrow()) == 0));
    let m: Value<ConstMove> = Rc::new(RefCell::new(ConstMove::ConstMove()));
    let m1: Value<ConstMove> = Rc::new(RefCell::new(ConstMove::ConstMove_pmutConstMove({
        m.as_pointer()
    })));
    let cm: Value<ConstMove> = Rc::new(RefCell::new(ConstMove::ConstMove()));
    let m2: Value<ConstMove> = Rc::new(RefCell::new(ConstMove::ConstMove_pconstConstMove({
        cm.as_pointer()
    })));
    assert!(((*(*m1.borrow()).mark.borrow()) == 1));
    assert!(((*(*m2.borrow()).mark.borrow()) == 10));
    return 0;
}
