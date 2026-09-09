extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static copies_0: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Default)]
pub struct Counted {
    pub v: Value<i32>,
}
impl Counted {
    pub fn Counted(v: i32) -> Self {
        let v: Value<i32> = Rc::new(RefCell::new(v));
        let __this: Value<Counted> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*v.borrow()))),
        }));
        let this: Ptr<Counted> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn Counted_pconstCounted(o: Ptr<Counted>) -> Self {
        let __this: Value<Counted> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*(*o.upgrade().deref()).v.borrow()))),
        }));
        let this: Ptr<Counted> = __this.as_pointer();
        (*copies_0.with(Value::clone).borrow_mut()).prefix_inc();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for Counted {
    fn clone(&self) -> Self {
        let __src: Value<Counted> = Rc::new(RefCell::new(Counted { v: self.v.clone() }));
        Counted::Counted_pconstCounted(__src.as_pointer())
    }
}
impl ByteRepr for Counted {
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
pub struct NonConst {
    pub mark: Value<i32>,
}
impl NonConst {
    pub fn NonConst() -> Self {
        let __this: Value<NonConst> = Rc::new(RefCell::new(Self {
            mark: Rc::new(RefCell::new(0)),
        }));
        let this: Ptr<NonConst> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn NonConst_pmutNonConst(o: Ptr<NonConst>) -> Self {
        let __this: Value<NonConst> = Rc::new(RefCell::new(Self {
            mark: Rc::new(RefCell::new(((*(*o.upgrade().deref()).mark.borrow()) + 1))),
        }));
        let this: Ptr<NonConst> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn NonConst_pconstNonConst(o: Ptr<NonConst>) -> Self {
        let __this: Value<NonConst> = Rc::new(RefCell::new(Self {
            mark: Rc::new(RefCell::new(((*(*o.upgrade().deref()).mark.borrow()) + 10))),
        }));
        let this: Ptr<NonConst> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for NonConst {
    fn clone(&self) -> Self {
        let __src: Value<NonConst> = Rc::new(RefCell::new(NonConst {
            mark: self.mark.clone(),
        }));
        NonConst::NonConst_pmutNonConst(__src.as_pointer())
    }
}
impl Default for NonConst {
    fn default() -> Self {
        { NonConst::NonConst() }
    }
}
impl ByteRepr for NonConst {
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
#[derive()]
pub struct Holder {
    pub c: Value<Counted>,
    pub arr: Value<Box<[Counted]>>,
}
impl Clone for Holder {
    fn clone(&self) -> Self {
        let __this: Value<Holder> = Rc::new(RefCell::new(Self {
            c: Rc::new(RefCell::new(Counted::Counted_pconstCounted({
                self.c.as_pointer()
            }))),
            arr: Rc::new(RefCell::new((*self.arr.borrow()).clone())),
        }));
        let this: Ptr<Holder> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Holder {
    fn default() -> Self {
        Holder {
            c: <Value<Counted>>::default(),
            arr: Rc::new(RefCell::new(
                (0..2)
                    .map(|_| <Counted>::default())
                    .collect::<Box<[Counted]>>(),
            )),
        }
    }
}
impl ByteRepr for Holder {
    fn byte_size() -> usize {
        12
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.c.borrow()).to_bytes(&mut buf[0..4]);
        (*self.arr.borrow()).to_bytes(&mut buf[4..12]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            c: Rc::new(RefCell::new(<Counted>::from_bytes(&buf[0..4]))),
            arr: Rc::new(RefCell::new(<Box<[Counted]>>::from_bytes(&buf[4..12]))),
        }
    }
}
pub fn by_value_1(c: Counted) -> i32 {
    let c: Value<Counted> = Rc::new(RefCell::new(c));
    return (*(*c.borrow()).v.borrow());
}
pub fn make_2(v: i32) -> Counted {
    let v: Value<i32> = Rc::new(RefCell::new(v));
    let c: Value<Counted> = Rc::new(RefCell::new(Counted::Counted({ (*v.borrow()) })));
    return Counted::Counted_pconstCounted({ c.as_pointer() });
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<Counted> = Rc::new(RefCell::new(Counted::Counted({ 1 })));
    let b: Value<Counted> = Rc::new(RefCell::new(Counted::Counted_pconstCounted({
        a.as_pointer()
    })));
    let c: Value<Counted> = Rc::new(RefCell::new(Counted::Counted_pconstCounted({
        a.as_pointer()
    })));
    let d: Value<Counted> = Rc::new(RefCell::new(Counted::Counted_pconstCounted({
        a.as_pointer()
    })));
    assert!(((*copies_0.with(Value::clone).borrow()) == 3));
    assert!(
        (((*(*b.borrow()).v.borrow()) == 1) && ((*(*c.borrow()).v.borrow()) == 1))
            && ((*(*d.borrow()).v.borrow()) == 1)
    );
    assert!((({ by_value_1(Counted::Counted_pconstCounted({ a.as_pointer() },),) }) == 1));
    assert!(((*copies_0.with(Value::clone).borrow()) == 4));
    let e: Value<Counted> = Rc::new(RefCell::new(({ make_2(5) })));
    assert!(((*(*e.borrow()).v.borrow()) == 5));
    assert!(((*copies_0.with(Value::clone).borrow()) == 5));
    let f: Value<Counted> = Rc::new(RefCell::new(Counted::Counted({ 6 })));
    assert!(((*(*f.borrow()).v.borrow()) == 6));
    assert!(((*copies_0.with(Value::clone).borrow()) == 5));
    let g: Value<Counted> = Rc::new(RefCell::new(Counted::Counted({ 7 })));
    let h: Value<Counted> = Rc::new(RefCell::new(Counted::Counted_pconstCounted({
        g.as_pointer()
    })));
    assert!(((*(*h.borrow()).v.borrow()) == 7));
    assert!(((*copies_0.with(Value::clone).borrow()) == 6));
    let hold: Value<Holder> = Rc::new(RefCell::new(Holder {
        c: Rc::new(RefCell::new(Counted::Counted({ 8 }))),
        arr: Rc::new(RefCell::new(Box::new([
            Counted::Counted({ 9 }),
            Counted::Counted({ 10 }),
        ]))),
    }));
    let hold2: Value<Holder> = Rc::new(RefCell::new((*hold.borrow()).clone()));
    assert!(
        (((*(*(*hold2.borrow()).c.borrow()).v.borrow()) == 8)
            && ((*(*(*hold2.borrow()).arr.borrow())[(0) as usize].v.borrow()) == 9))
            && ((*(*(*hold2.borrow()).arr.borrow())[(1) as usize].v.borrow()) == 10)
    );
    assert!(((*copies_0.with(Value::clone).borrow()) == 9));
    let vec_: Value<Vec<Counted>> = Rc::new(RefCell::new(Vec::new()));
    {
        let a0_clone = (*a.borrow()).clone();
        (*vec_.borrow_mut()).push(a0_clone)
    };
    assert!(
        ((*(*(vec_.as_pointer() as Ptr<Counted>)
            .offset(0_usize)
            .upgrade()
            .deref())
        .v
        .borrow())
            == 1)
    );
    assert!(((*copies_0.with(Value::clone).borrow()) == 10));
    let n: Value<NonConst> = Rc::new(RefCell::new(NonConst::NonConst()));
    let n1: Value<NonConst> = Rc::new(RefCell::new(NonConst::NonConst_pmutNonConst({
        n.as_pointer()
    })));
    let cn: Value<NonConst> = Rc::new(RefCell::new(NonConst::NonConst()));
    let n2: Value<NonConst> = Rc::new(RefCell::new(NonConst::NonConst_pconstNonConst({
        cn.as_pointer()
    })));
    assert!(((*(*n1.borrow()).mark.borrow()) == 1));
    assert!(((*(*n2.borrow()).mark.borrow()) == 10));
    return 0;
}
