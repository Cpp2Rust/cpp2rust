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
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Default)]
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
        Self {
            v: v,
            copies: 0,
            moves: 0,
        }
    }
    pub fn copy_from(o: Ptr<Tracked>) -> Self {
        Self {
            v: o.with(|__s| __s.v),
            copies: (o.with(|__s| __s.copies) + 1),
            moves: o.with(|__s| __s.moves),
        }
    }
    pub fn move_from(o: Ptr<Tracked>) -> Self {
        let __this: Tracked = Self {
            v: o.with(|__s| __s.v),
            copies: o.with(|__s| __s.copies),
            moves: (o.with(|__s| __s.moves) + 1),
        };
        field!(o, v).write(0);
        __this
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
    return ({ (({ chosen_overload_0((args_0).clone()) }) as i32) } + {
        (({ chosen_overload_1((args_1).clone()) }) as i32)
    });
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
    assert!(({ (*a.borrow()).v } == 3));
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
    assert!(({ (*b.borrow()).v } == 5));
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
    assert!(({ (*c.borrow()).v } == 7));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
