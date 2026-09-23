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
    pub fn move_from(o: Ptr<MoveOnly>) -> Self {
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
    pub fn ConstMove_pmutConstMove_rv(o: Ptr<ConstMove>) -> Self {
        let __this: Value<ConstMove> = Rc::new(RefCell::new(Self {
            mark: Rc::new(RefCell::new(((*(*o.upgrade().deref()).mark.borrow()) + 1))),
        }));
        let this: Ptr<ConstMove> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn ConstMove_pconstConstMove_rv(o: Ptr<ConstMove>) -> Self {
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
#[derive(Default)]
pub struct ThrowingMove {
    pub v: Value<i32>,
    pub copies: Value<i32>,
    pub moves: Value<i32>,
}
impl ThrowingMove {
    pub fn ThrowingMove(v: i32) -> Self {
        let v: Value<i32> = Rc::new(RefCell::new(v));
        let __this: Value<ThrowingMove> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*v.borrow()))),
            copies: Rc::new(RefCell::new(0)),
            moves: Rc::new(RefCell::new(0)),
        }));
        let this: Ptr<ThrowingMove> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn copy_from(o: Ptr<ThrowingMove>) -> Self {
        let __this: Value<ThrowingMove> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*(*o.upgrade().deref()).v.borrow()))),
            copies: Rc::new(RefCell::new(
                ((*(*o.upgrade().deref()).copies.borrow()) + 1),
            )),
            moves: Rc::new(RefCell::new((*(*o.upgrade().deref()).moves.borrow()))),
        }));
        let this: Ptr<ThrowingMove> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn move_from(o: Ptr<ThrowingMove>) -> Self {
        let __this: Value<ThrowingMove> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*(*o.upgrade().deref()).v.borrow()))),
            copies: Rc::new(RefCell::new((*(*o.upgrade().deref()).copies.borrow()))),
            moves: Rc::new(RefCell::new(((*(*o.upgrade().deref()).moves.borrow()) + 1))),
        }));
        let this: Ptr<ThrowingMove> = __this.as_pointer();
        (*(*o.upgrade().deref()).v.borrow_mut()) = 0;
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for ThrowingMove {
    fn clone(&self) -> Self {
        let __src: Value<ThrowingMove> = Rc::new(RefCell::new(ThrowingMove {
            v: self.v.clone(),
            copies: self.copies.clone(),
            moves: self.moves.clone(),
        }));
        ThrowingMove::copy_from(__src.as_pointer())
    }
}
impl ByteRepr for ThrowingMove {
    fn byte_size() -> usize {
        12
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.v.borrow()).to_bytes(&mut buf[0..4]);
        (*self.copies.borrow()).to_bytes(&mut buf[4..8]);
        (*self.moves.borrow()).to_bytes(&mut buf[8..12]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            copies: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
            moves: Rc::new(RefCell::new(<i32>::from_bytes(&buf[8..12]))),
        }
    }
}
#[derive(Default)]
pub struct NoexceptMove {
    pub v: Value<i32>,
    pub copies: Value<i32>,
    pub moves: Value<i32>,
}
impl NoexceptMove {
    pub fn NoexceptMove(v: i32) -> Self {
        let v: Value<i32> = Rc::new(RefCell::new(v));
        let __this: Value<NoexceptMove> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*v.borrow()))),
            copies: Rc::new(RefCell::new(0)),
            moves: Rc::new(RefCell::new(0)),
        }));
        let this: Ptr<NoexceptMove> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn copy_from(o: Ptr<NoexceptMove>) -> Self {
        let __this: Value<NoexceptMove> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*(*o.upgrade().deref()).v.borrow()))),
            copies: Rc::new(RefCell::new(
                ((*(*o.upgrade().deref()).copies.borrow()) + 1),
            )),
            moves: Rc::new(RefCell::new((*(*o.upgrade().deref()).moves.borrow()))),
        }));
        let this: Ptr<NoexceptMove> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn move_from(o: Ptr<NoexceptMove>) -> Self {
        let __this: Value<NoexceptMove> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*(*o.upgrade().deref()).v.borrow()))),
            copies: Rc::new(RefCell::new((*(*o.upgrade().deref()).copies.borrow()))),
            moves: Rc::new(RefCell::new(((*(*o.upgrade().deref()).moves.borrow()) + 1))),
        }));
        let this: Ptr<NoexceptMove> = __this.as_pointer();
        (*(*o.upgrade().deref()).v.borrow_mut()) = 0;
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for NoexceptMove {
    fn clone(&self) -> Self {
        let __src: Value<NoexceptMove> = Rc::new(RefCell::new(NoexceptMove {
            v: self.v.clone(),
            copies: self.copies.clone(),
            moves: self.moves.clone(),
        }));
        NoexceptMove::copy_from(__src.as_pointer())
    }
}
impl ByteRepr for NoexceptMove {
    fn byte_size() -> usize {
        12
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.v.borrow()).to_bytes(&mut buf[0..4]);
        (*self.copies.borrow()).to_bytes(&mut buf[4..8]);
        (*self.moves.borrow()).to_bytes(&mut buf[8..12]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            copies: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
            moves: Rc::new(RefCell::new(<i32>::from_bytes(&buf[8..12]))),
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
    return MoveOnly::move_from({ m.as_pointer() });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::MoveOnly({ 1 })));
    let b: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::move_from({ a.as_pointer() })));
    assert!(((*(*b.borrow()).v.borrow()) == 1));
    assert!(((*(*a.borrow()).v.borrow()) == 0));
    let c: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::move_from({ b.as_pointer() })));
    assert!(((*(*c.borrow()).v.borrow()) == 1));
    assert!(((*(*b.borrow()).v.borrow()) == 0));
    let d: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::move_from({ c.as_pointer() })));
    assert!(((*(*d.borrow()).v.borrow()) == 1));
    assert!(((*(*c.borrow()).v.borrow()) == 0));
    let e: Value<MoveOnly> = Rc::new(RefCell::new(({ make_1(5) })));
    assert!(((*(*e.borrow()).v.borrow()) == 5));
    assert!((({ by_value_0(MoveOnly::MoveOnly({ 6 },),) }) == 6));
    assert!((({ by_value_0(MoveOnly::move_from({ e.as_pointer() },),) }) == 5));
    assert!(((*(*e.borrow()).v.borrow()) == 0));
    let vec_: Value<Vec<MoveOnly>> = Rc::new(RefCell::new(Vec::new()));
    (*vec_.borrow_mut()).push(MoveOnly::MoveOnly({ 7 }));
    let f: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::MoveOnly({ 8 })));
    (*vec_.borrow_mut()).push(MoveOnly::move_from({ f.as_pointer() }));
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
    let m1: Value<ConstMove> = Rc::new(RefCell::new(ConstMove::ConstMove_pmutConstMove_rv({
        m.as_pointer()
    })));
    let cm: Value<ConstMove> = Rc::new(RefCell::new(ConstMove::ConstMove()));
    let m2: Value<ConstMove> = Rc::new(RefCell::new(ConstMove::ConstMove_pconstConstMove_rv({
        cm.as_pointer()
    })));
    assert!(((*(*m1.borrow()).mark.borrow()) == 1));
    assert!(((*(*m2.borrow()).mark.borrow()) == 10));
    let t: Value<ThrowingMove> = Rc::new(RefCell::new(ThrowingMove::ThrowingMove({ 1 })));
    let t1: Value<ThrowingMove> =
        Rc::new(RefCell::new(ThrowingMove::copy_from({ t.as_pointer() })));
    assert!(((*(*t1.borrow()).v.borrow()) == 1));
    assert!(((*(*t1.borrow()).copies.borrow()) == 1));
    assert!(((*(*t1.borrow()).moves.borrow()) == 0));
    assert!(((*(*t.borrow()).v.borrow()) == 1));
    let n: Value<NoexceptMove> = Rc::new(RefCell::new(NoexceptMove::NoexceptMove({ 2 })));
    let n1: Value<NoexceptMove> =
        Rc::new(RefCell::new(NoexceptMove::move_from({ n.as_pointer() })));
    assert!(((*(*n1.borrow()).v.borrow()) == 2));
    assert!(((*(*n1.borrow()).copies.borrow()) == 0));
    assert!(((*(*n1.borrow()).moves.borrow()) == 1));
    assert!(((*(*n.borrow()).v.borrow()) == 0));
    let g: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::MoveOnly({ 3 })));
    let g1: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::move_from({ g.as_pointer() })));
    assert!(((*(*g1.borrow()).v.borrow()) == 3));
    assert!(((*(*g.borrow()).v.borrow()) == 0));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
