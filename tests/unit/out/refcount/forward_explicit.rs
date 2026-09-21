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
pub const Overload_kIntLvalueOverload: Overload = 3;
pub const Overload_kIntRvalueOverload: Overload = 4;
#[derive(Default)]
pub struct Tracked {
    pub v: Value<i32>,
    pub copies: Value<i32>,
    pub moves: Value<i32>,
}
impl Tracked {
    pub fn Tracked(v: i32) -> Self {
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
pub fn chosen_overload_2(_a0: Ptr<i32>) -> Overload {
    return Overload_kIntLvalueOverload;
}
pub fn chosen_overload_3(_a0: Ptr<i32>) -> Overload {
    return Overload_kIntRvalueOverload;
}
pub fn copy_or_move_into_param_4(t: Tracked) -> Tracked {
    let t: Value<Tracked> = Rc::new(RefCell::new(t));
    return Tracked::Tracked_pmutTracked_rv({ (t.as_pointer()).clone() });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<Tracked> = Rc::new(RefCell::new(Tracked::Tracked({ 5 })));
    assert!(
        ((({ chosen_overload_0(({ forward_5(a.as_pointer(),) }),) }) as i32)
            == (Overload_kLvalueOverload as i32))
    );
    assert!(((*(*a.borrow()).v.borrow()) == 5));
    assert!(
        ((({ chosen_overload_1(({ forward_6(a.as_pointer(),) }),) }) as i32)
            == (Overload_kRvalueOverload as i32))
    );
    assert!(((*(*a.borrow()).v.borrow()) == 5));
    let b: Value<Tracked> = Rc::new(RefCell::new(Tracked::Tracked({ 6 })));
    let moved: Value<Tracked> = Rc::new(RefCell::new(
        ({
            copy_or_move_into_param_4(Tracked::Tracked_pmutTracked_rv({
                ({ forward_6(b.as_pointer()) })
            }))
        }),
    ));
    assert!(((*(*moved.borrow()).v.borrow()) == 6));
    assert!(((*(*moved.borrow()).copies.borrow()) == 0));
    assert!(((*(*moved.borrow()).moves.borrow()) == 2));
    assert!(((*(*b.borrow()).v.borrow()) == 0));
    let c: Value<Tracked> = Rc::new(RefCell::new(Tracked::Tracked({ 7 })));
    let copied: Value<Tracked> = Rc::new(RefCell::new(
        ({
            copy_or_move_into_param_4(Tracked::Tracked_pconstTracked({
                ({ forward_5(c.as_pointer()) })
            }))
        }),
    ));
    assert!(((*(*copied.borrow()).v.borrow()) == 7));
    assert!(((*(*copied.borrow()).copies.borrow()) == 1));
    assert!(((*(*copied.borrow()).moves.borrow()) == 1));
    assert!(((*(*c.borrow()).v.borrow()) == 7));
    let i: Value<i32> = Rc::new(RefCell::new(8));
    assert!(
        ((({ chosen_overload_2(({ forward_7(i.as_pointer(),) }),) }) as i32)
            == (Overload_kIntLvalueOverload as i32))
    );
    assert!(
        ((({ chosen_overload_3(({ forward_8(i.as_pointer(),) }),) }) as i32)
            == (Overload_kIntRvalueOverload as i32))
    );
    assert!(
        ((({ chosen_overload_3(({ forward_9(i.as_pointer(),) }),) }) as i32)
            == (Overload_kIntRvalueOverload as i32))
    );
    assert!(((*i.borrow()) == 8));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
