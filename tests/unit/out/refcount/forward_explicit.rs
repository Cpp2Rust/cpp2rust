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
    pub fn new(v: i32) -> Self {
        let v: Value<i32> = Rc::new(RefCell::new(v));
        let __this: Value<Tracked> = Rc::new(RefCell::new(Self {
            v: (*v.borrow()),
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
pub fn chosen_overload_2(_a0: Ptr<i32>) -> Overload {
    return Overload_kIntLvalueOverload;
}
pub fn chosen_overload_3(_a0: Ptr<i32>) -> Overload {
    return Overload_kIntRvalueOverload;
}
pub fn copy_or_move_into_param_4(t: Tracked) -> Tracked {
    let t: Value<Tracked> = Rc::new(RefCell::new(t));
    return Tracked::move_from({ (t.as_pointer()).clone() });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<Tracked> = Rc::new(RefCell::new(Tracked::new({ 5 })));
    assert!(
        ((({ chosen_overload_0(a.as_pointer(),) }) as i32) == (Overload_kLvalueOverload as i32))
    );
    assert!(({ (*a.borrow()).v } == 5));
    assert!(
        ((({ chosen_overload_1(a.as_pointer(),) }) as i32) == (Overload_kRvalueOverload as i32))
    );
    assert!(({ (*a.borrow()).v } == 5));
    let b: Value<Tracked> = Rc::new(RefCell::new(Tracked::new({ 6 })));
    let moved: Value<Tracked> = Rc::new(RefCell::new(
        ({ copy_or_move_into_param_4(Tracked::move_from({ b.as_pointer() })) }),
    ));
    assert!(({ (*moved.borrow()).v } == 6));
    assert!(({ (*moved.borrow()).copies } == 0));
    assert!(({ (*moved.borrow()).moves } == 2));
    assert!(({ (*b.borrow()).v } == 0));
    let c: Value<Tracked> = Rc::new(RefCell::new(Tracked::new({ 7 })));
    let copied: Value<Tracked> = Rc::new(RefCell::new(
        ({ copy_or_move_into_param_4(Tracked::copy_from({ c.as_pointer() })) }),
    ));
    assert!(({ (*copied.borrow()).v } == 7));
    assert!(({ (*copied.borrow()).copies } == 1));
    assert!(({ (*copied.borrow()).moves } == 1));
    assert!(({ (*c.borrow()).v } == 7));
    let i: Value<i32> = Rc::new(RefCell::new(8));
    assert!(
        ((({ chosen_overload_2(i.as_pointer(),) }) as i32) == (Overload_kIntLvalueOverload as i32))
    );
    assert!(
        ((({ chosen_overload_3(i.as_pointer(),) }) as i32) == (Overload_kIntRvalueOverload as i32))
    );
    assert!(
        ((({ chosen_overload_3(i.as_pointer(),) }) as i32) == (Overload_kIntRvalueOverload as i32))
    );
    assert!(((*i.borrow()) == 8));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
