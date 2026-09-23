extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type Overload = u32;
pub const Overload_kLvalueOverload: Overload = 1;
pub const Overload_kRvalueOverload: Overload = 2;
#[derive(Default)]
pub struct Tracked {
    pub v: Value<i32>,
    pub copies: Value<i32>,
    pub moves: Value<i32>,
}
impl Tracked {
    pub fn new(v: i32) -> Self {
        let v: Value<i32> = Rc::new(RefCell::new(v));
        let __this: Value<Tracked> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*v.borrow()))),
            copies: Rc::new(RefCell::new(0)),
            moves: Rc::new(RefCell::new(0)),
        }));
        let this: Ptr<Tracked> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn Tracked_pconstTracked(o: Ptr<Tracked>) -> Self {
        let __this: Value<Tracked> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*(*o.upgrade().deref()).v.borrow()))),
            copies: Rc::new(RefCell::new(
                ((*(*o.upgrade().deref()).copies.borrow()) + 1),
            )),
            moves: Rc::new(RefCell::new((*(*o.upgrade().deref()).moves.borrow()))),
        }));
        let this: Ptr<Tracked> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn Tracked_pmutTracked_rv(o: Ptr<Tracked>) -> Self {
        let __this: Value<Tracked> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*(*o.upgrade().deref()).v.borrow()))),
            copies: Rc::new(RefCell::new((*(*o.upgrade().deref()).copies.borrow()))),
            moves: Rc::new(RefCell::new(((*(*o.upgrade().deref()).moves.borrow()) + 1))),
        }));
        let this: Ptr<Tracked> = __this.as_pointer();
        (*(*o.upgrade().deref()).v.borrow_mut()) = 0;
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for Tracked {
    fn clone(&self) -> Self {
        let __src: Value<Tracked> = Rc::new(RefCell::new(Tracked {
            v: self.v.clone(),
            copies: self.copies.clone(),
            moves: self.moves.clone(),
        }));
        Tracked::Tracked_pconstTracked(__src.as_pointer())
    }
}
impl ByteRepr for Tracked {
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
pub fn chosen_overload_0(_a0: Ptr<Tracked>) -> Overload {
    return Overload_kLvalueOverload;
}
pub fn chosen_overload_1(_a0: Ptr<Tracked>) -> Overload {
    return Overload_kRvalueOverload;
}
impl Holder {
    pub fn new_1(x: Ptr<Tracked>) -> Self {
        let __this: Value<Holder> = Rc::new(RefCell::new(Self {
            t: Rc::new(RefCell::new(Tracked::Tracked_pconstTracked({
                (x).clone()
            }))),
        }));
        let this: Ptr<Holder> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Holder {
    pub fn new_2(x: Ptr<Tracked>) -> Self {
        let __this: Value<Holder> = Rc::new(RefCell::new(Self {
            t: Rc::new(RefCell::new(Tracked::Tracked_pmutTracked_rv({
                (x).clone()
            }))),
        }));
        let this: Ptr<Holder> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
#[derive(Default)]
pub struct Holder {
    pub t: Value<Tracked>,
}
impl Clone for Holder {
    fn clone(&self) -> Self {
        let __this: Value<Holder> = Rc::new(RefCell::new(Self {
            t: Rc::new(RefCell::new(Tracked::Tracked_pconstTracked({
                self.t.as_pointer()
            }))),
        }));
        let this: Ptr<Holder> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Holder {
    fn byte_size() -> usize {
        12
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.t.borrow()).to_bytes(&mut buf[0..12]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            t: Rc::new(RefCell::new(<Tracked>::from_bytes(&buf[0..12]))),
        }
    }
}
pub fn forward_once_2(x: Ptr<Tracked>) -> Overload {
    return ({ chosen_overload_0((x).clone()) });
}
pub fn forward_once_3(x: Ptr<Tracked>) -> Overload {
    return ({ chosen_overload_1((x).clone()) });
}
pub fn forward_twice_4(x: Ptr<Tracked>) -> Overload {
    return ({ forward_once_2((x).clone()) });
}
pub fn forward_twice_5(x: Ptr<Tracked>) -> Overload {
    return ({ forward_once_3((x).clone()) });
}
pub fn forward_into_ctor_6(x: Ptr<Tracked>) -> Holder {
    return Holder::new_1({ (x).clone() });
}
pub fn forward_into_ctor_7(x: Ptr<Tracked>) -> Holder {
    return Holder::new_2({ (x).clone() });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let lvalue: Value<Tracked> = Rc::new(RefCell::new(Tracked::new({ 7 })));
    assert!(
        ((({ forward_once_2(lvalue.as_pointer(),) }) as i32) == (Overload_kLvalueOverload as i32))
    );
    assert!(((*(*lvalue.borrow()).v.borrow()) == 7));
    assert!(
        ((({
            let _x: Value<Tracked> = Rc::new(RefCell::new(Tracked::new({ 8 })));
            forward_once_3(_x.as_pointer())
        }) as i32)
            == (Overload_kRvalueOverload as i32))
    );
    let relayed: Value<Tracked> = Rc::new(RefCell::new(Tracked::new({ 9 })));
    assert!(
        ((({ forward_twice_4(relayed.as_pointer(),) }) as i32)
            == (Overload_kLvalueOverload as i32))
    );
    assert!(((*(*relayed.borrow()).v.borrow()) == 9));
    assert!(
        ((({
            let _x: Value<Tracked> = Rc::new(RefCell::new(Tracked::new({ 10 })));
            forward_twice_5(_x.as_pointer())
        }) as i32)
            == (Overload_kRvalueOverload as i32))
    );
    let kept: Value<Tracked> = Rc::new(RefCell::new(Tracked::new({ 11 })));
    let from_lvalue: Value<Holder> =
        Rc::new(RefCell::new(({ forward_into_ctor_6(kept.as_pointer()) })));
    assert!(((*(*(*from_lvalue.borrow()).t.borrow()).v.borrow()) == 11));
    assert!(((*(*(*from_lvalue.borrow()).t.borrow()).copies.borrow()) == 1));
    assert!(((*(*(*from_lvalue.borrow()).t.borrow()).moves.borrow()) == 0));
    assert!(((*(*kept.borrow()).v.borrow()) == 11));
    let from_rvalue: Value<Holder> = Rc::new(RefCell::new(
        ({
            let _x: Value<Tracked> = Rc::new(RefCell::new(Tracked::new({ 12 })));
            forward_into_ctor_7(_x.as_pointer())
        }),
    ));
    assert!(((*(*(*from_rvalue.borrow()).t.borrow()).v.borrow()) == 12));
    assert!(((*(*(*from_rvalue.borrow()).t.borrow()).copies.borrow()) == 0));
    assert!(((*(*(*from_rvalue.borrow()).t.borrow()).moves.borrow()) == 1));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
