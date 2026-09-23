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
pub fn forward_by_decltype_2(x: Ptr<Tracked>) -> Overload {
    return ({ chosen_overload_0((x).clone()) });
}
pub fn forward_by_decltype_3(x: Ptr<Tracked>) -> Overload {
    return ({ chosen_overload_1((x).clone()) });
}
pub fn forward_abbreviated_4(x: Ptr<Tracked>) -> Overload {
    return ({ chosen_overload_0((x).clone()) });
}
pub fn forward_abbreviated_5(x: Ptr<Tracked>) -> Overload {
    return ({ chosen_overload_1((x).clone()) });
}
pub fn forward_abbreviated_pack_6(args_0: Ptr<Tracked>, args_1: Ptr<Tracked>) -> i32 {
    return {
        let _lhs = (({ chosen_overload_0((args_0).clone()) }) as i32);
        _lhs + (({ chosen_overload_1((args_1).clone()) }) as i32)
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<Tracked> = Rc::new(RefCell::new(Tracked::new({ 3 })));
    assert!(
        ((({ forward_by_decltype_2(a.as_pointer(),) }) as i32)
            == (Overload_kLvalueOverload as i32))
    );
    assert!(((*(*a.borrow()).v.borrow()) == 3));
    assert!(
        ((({
            let _x: Value<Tracked> = Rc::new(RefCell::new(Tracked::new({ 4 })));
            forward_by_decltype_3(_x.as_pointer())
        }) as i32)
            == (Overload_kRvalueOverload as i32))
    );
    let b: Value<Tracked> = Rc::new(RefCell::new(Tracked::new({ 5 })));
    assert!(
        ((({ forward_abbreviated_4(b.as_pointer(),) }) as i32)
            == (Overload_kLvalueOverload as i32))
    );
    assert!(((*(*b.borrow()).v.borrow()) == 5));
    assert!(
        ((({
            let _x: Value<Tracked> = Rc::new(RefCell::new(Tracked::new({ 6 })));
            forward_abbreviated_5(_x.as_pointer())
        }) as i32)
            == (Overload_kRvalueOverload as i32))
    );
    let c: Value<Tracked> = Rc::new(RefCell::new(Tracked::new({ 7 })));
    assert!(
        (({
            let _args_1: Value<Tracked> = Rc::new(RefCell::new(Tracked::new({ 8 })));
            forward_abbreviated_pack_6(c.as_pointer(), _args_1.as_pointer())
        }) == ((Overload_kLvalueOverload as i32) + (Overload_kRvalueOverload as i32)))
    );
    assert!(((*(*c.borrow()).v.borrow()) == 7));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
