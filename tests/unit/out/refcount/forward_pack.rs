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
    pub fn copy_from(o: Ptr<Tracked>) -> Self {
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
    pub fn move_from(o: Ptr<Tracked>) -> Self {
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
        Tracked::copy_from(__src.as_pointer())
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
pub fn forward_pack_4() -> i32 {
    let digits: Value<i32> = Rc::new(RefCell::new(0));
    ();
    return (*digits.borrow());
}
pub fn forward_pack_5(args: Ptr<Tracked>) -> i32 {
    let digits: Value<i32> = Rc::new(RefCell::new(0));
    let __rhs = {
        let _lhs = ((*digits.borrow()) * 10);
        _lhs + (({ chosen_overload_0((args).clone()) }) as i32)
    };
    (*digits.borrow_mut()) = __rhs;
    return (*digits.borrow());
}
pub fn forward_pack_6(args: Ptr<Tracked>) -> i32 {
    let digits: Value<i32> = Rc::new(RefCell::new(0));
    let __rhs = {
        let _lhs = ((*digits.borrow()) * 10);
        _lhs + (({ chosen_overload_1((args).clone()) }) as i32)
    };
    (*digits.borrow_mut()) = __rhs;
    return (*digits.borrow());
}
pub fn forward_pack_7(
    args_0: Ptr<Tracked>,
    args_1: Ptr<Tracked>,
    args_2: Ptr<i32>,
    args_3: Ptr<i32>,
) -> i32 {
    let digits: Value<i32> = Rc::new(RefCell::new(0));
    {
        let __rhs = {
            let _lhs = ((*digits.borrow()) * 10);
            _lhs + (({ chosen_overload_0((args_0).clone()) }) as i32)
        };
        (*digits.borrow_mut()) = __rhs;
        {
            let __rhs = {
                let _lhs = ((*digits.borrow()) * 10);
                _lhs + (({ chosen_overload_1((args_1).clone()) }) as i32)
            };
            (*digits.borrow_mut()) = __rhs;
            {
                let __rhs = {
                    let _lhs = ((*digits.borrow()) * 10);
                    _lhs + (({ chosen_overload_2((args_2).clone()) }) as i32)
                };
                (*digits.borrow_mut()) = __rhs;
                let __rhs = {
                    let _lhs = ((*digits.borrow()) * 10);
                    _lhs + (({ chosen_overload_3((args_3).clone()) }) as i32)
                };
                (*digits.borrow_mut()) = __rhs
            }
        }
    };
    return (*digits.borrow());
}
impl Pair {
    pub fn Pair(x: Ptr<Tracked>, y: Ptr<Tracked>) -> Self {
        let __this: Value<Pair> = Rc::new(RefCell::new(Self {
            a: Rc::new(RefCell::new(Tracked::copy_from({ (x).clone() }))),
            b: Rc::new(RefCell::new(Tracked::move_from({ (y).clone() }))),
        }));
        let this: Ptr<Pair> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
#[derive(Default)]
pub struct Pair {
    pub a: Value<Tracked>,
    pub b: Value<Tracked>,
}
impl Clone for Pair {
    fn clone(&self) -> Self {
        let __this: Value<Pair> = Rc::new(RefCell::new(Self {
            a: Rc::new(RefCell::new(Tracked::copy_from({ self.a.as_pointer() }))),
            b: Rc::new(RefCell::new(Tracked::copy_from({ self.b.as_pointer() }))),
        }));
        let this: Ptr<Pair> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Pair {
    fn byte_size() -> usize {
        24
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.a.borrow()).to_bytes(&mut buf[0..12]);
        (*self.b.borrow()).to_bytes(&mut buf[12..24]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            a: Rc::new(RefCell::new(<Tracked>::from_bytes(&buf[0..12]))),
            b: Rc::new(RefCell::new(<Tracked>::from_bytes(&buf[12..24]))),
        }
    }
}
pub fn forward_pack_into_ctor_8(args_0: Ptr<Tracked>, args_1: Ptr<Tracked>) -> Pair {
    return Pair::Pair({ (args_0).clone() }, { (args_1).clone() });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((({ forward_pack_4() }) == 0));
    let a: Value<Tracked> = Rc::new(RefCell::new(Tracked::Tracked({ 1 })));
    assert!((({ forward_pack_5(a.as_pointer(),) }) == (Overload_kLvalueOverload as i32)));
    assert!(((*(*a.borrow()).v.borrow()) == 1));
    assert!(
        (({
            let _args: Value<Tracked> = Rc::new(RefCell::new(Tracked::Tracked({ 2 })));
            forward_pack_6(_args.as_pointer())
        }) == (Overload_kRvalueOverload as i32))
    );
    let i: Value<i32> = Rc::new(RefCell::new(3));
    assert!(
        (({
            let _args_1: Value<Tracked> = Rc::new(RefCell::new(Tracked::Tracked({ 4 })));
            let _args_3: Value<i32> = Rc::new(RefCell::new(5));
            forward_pack_7(
                a.as_pointer(),
                _args_1.as_pointer(),
                i.as_pointer(),
                _args_3.as_pointer(),
            )
        }) == 1234)
    );
    assert!(((*(*a.borrow()).v.borrow()) == 1));
    assert!(((*i.borrow()) == 3));
    let lhs: Value<Tracked> = Rc::new(RefCell::new(Tracked::Tracked({ 6 })));
    let p: Value<Pair> = Rc::new(RefCell::new(
        ({
            let _args_1: Value<Tracked> = Rc::new(RefCell::new(Tracked::Tracked({ 7 })));
            forward_pack_into_ctor_8(lhs.as_pointer(), _args_1.as_pointer())
        }),
    ));
    assert!(((*(*(*p.borrow()).a.borrow()).v.borrow()) == 6));
    assert!(((*(*(*p.borrow()).a.borrow()).copies.borrow()) == 1));
    assert!(((*(*(*p.borrow()).b.borrow()).v.borrow()) == 7));
    assert!(((*(*(*p.borrow()).b.borrow()).copies.borrow()) == 0));
    assert!(((*(*(*p.borrow()).b.borrow()).moves.borrow()) == 1));
    assert!(((*(*lhs.borrow()).v.borrow()) == 6));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
