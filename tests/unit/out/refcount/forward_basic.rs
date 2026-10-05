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
#[derive(Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(12)]
pub struct Tracked {
    #[offset(0)]
    pub v: i32,
    #[offset(4)]
    pub copies: i32,
    #[offset(8)]
    pub moves: i32,
}
impl Tracked {
    pub fn new(mut v: i32) -> Self {
        let __this: Value<Tracked> = Rc::new(RefCell::new(Self {
            v: v,
            copies: 0,
            moves: 0,
        }));
        let this: Ptr<Tracked> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn copy_from(o: Ptr<Tracked>) -> Self {
        let __this: Value<Tracked> = Rc::new(RefCell::new(Self {
            v: o.with(|__s| __s.v),
            copies: (o.with(|__s| __s.copies) + 1),
            moves: o.with(|__s| __s.moves),
        }));
        let this: Ptr<Tracked> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn move_from(o: Ptr<Tracked>) -> Self {
        let __this: Value<Tracked> = Rc::new(RefCell::new(Self {
            v: o.with(|__s| __s.v),
            copies: o.with(|__s| __s.copies),
            moves: (o.with(|__s| __s.moves) + 1),
        }));
        let this: Ptr<Tracked> = __this.as_pointer();
        field!(o, v).write(0);
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
pub fn chosen_overload_0(_a0: Ptr<Tracked>) -> Overload {
    return Overload_kLvalueOverload;
}
pub fn chosen_overload_1(_a0: Ptr<Tracked>) -> Overload {
    return Overload_kRvalueOverload;
}
impl Holder {
    pub fn new_1(x: Ptr<Tracked>) -> Self {
        let __this: Value<Holder> = Rc::new(RefCell::new(Self {
            t: Tracked::copy_from({ (x).clone() }),
        }));
        let this: Ptr<Holder> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Holder {
    pub fn new_2(x: Ptr<Tracked>) -> Self {
        let __this: Value<Holder> = Rc::new(RefCell::new(Self {
            t: Tracked::move_from({ (x).clone() }),
        }));
        let this: Ptr<Holder> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(12)]
pub struct Holder {
    #[offset(0)]
    #[byte_size(12)]
    pub t: Tracked,
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
    assert!(({ (*lvalue.borrow()).v } == 7));
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
    assert!(({ (*relayed.borrow()).v } == 9));
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
    assert!(({ (*from_lvalue.borrow()).t.v } == 11));
    assert!(({ (*from_lvalue.borrow()).t.copies } == 1));
    assert!(({ (*from_lvalue.borrow()).t.moves } == 0));
    assert!(({ (*kept.borrow()).v } == 11));
    let from_rvalue: Value<Holder> = Rc::new(RefCell::new(
        ({
            let _x: Value<Tracked> = Rc::new(RefCell::new(Tracked::new({ 12 })));
            forward_into_ctor_7(_x.as_pointer())
        }),
    ));
    assert!(({ (*from_rvalue.borrow()).t.v } == 12));
    assert!(({ (*from_rvalue.borrow()).t.copies } == 0));
    assert!(({ (*from_rvalue.borrow()).t.moves } == 1));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
