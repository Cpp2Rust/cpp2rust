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
pub fn forward_pack_4() -> i32 {
    let digits: Value<i32> = Rc::new(RefCell::new(0));
    ();
    return (*digits.borrow());
}
pub fn forward_pack_5(args: Ptr<Tracked>) -> i32 {
    let digits: Value<i32> = Rc::new(RefCell::new(0));
    let __rhs =
        ({ ((*digits.borrow()) * 10) } + { (({ chosen_overload_0((args).clone()) }) as i32) });
    (*digits.borrow_mut()) = __rhs;
    return (*digits.borrow());
}
pub fn forward_pack_6(args: Ptr<Tracked>) -> i32 {
    let digits: Value<i32> = Rc::new(RefCell::new(0));
    let __rhs =
        ({ ((*digits.borrow()) * 10) } + { (({ chosen_overload_1((args).clone()) }) as i32) });
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
        let __rhs = ({ ((*digits.borrow()) * 10) } + {
            (({ chosen_overload_0((args_0).clone()) }) as i32)
        });
        (*digits.borrow_mut()) = __rhs;
        {
            let __rhs = ({ ((*digits.borrow()) * 10) } + {
                (({ chosen_overload_1((args_1).clone()) }) as i32)
            });
            (*digits.borrow_mut()) = __rhs;
            {
                let __rhs = ({ ((*digits.borrow()) * 10) } + {
                    (({ chosen_overload_2((args_2).clone()) }) as i32)
                });
                (*digits.borrow_mut()) = __rhs;
                let __rhs = ({ ((*digits.borrow()) * 10) } + {
                    (({ chosen_overload_3((args_3).clone()) }) as i32)
                });
                (*digits.borrow_mut()) = __rhs
            }
        }
    };
    return (*digits.borrow());
}
impl Pair {
    pub fn new(x: Ptr<Tracked>, y: Ptr<Tracked>) -> Self {
        let __this: Value<Pair> = Rc::new(RefCell::new(Self {
            a: Tracked::copy_from({ (x).clone() }),
            b: Tracked::move_from({ (y).clone() }),
        }));
        let this: Ptr<Pair> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(24)]
pub struct Pair {
    #[offset(0)]
    #[byte_size(12)]
    pub a: Tracked,
    #[offset(12)]
    #[byte_size(12)]
    pub b: Tracked,
}
pub fn forward_pack_into_ctor_8(args_0: Ptr<Tracked>, args_1: Ptr<Tracked>) -> Pair {
    return Pair::new({ (args_0).clone() }, { (args_1).clone() });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((({ forward_pack_4() }) == 0));
    let a: Value<Tracked> = Rc::new(RefCell::new(Tracked::new({ 1 })));
    assert!((({ forward_pack_5(a.as_pointer(),) }) == (Overload_kLvalueOverload as i32)));
    assert!(({ (*a.borrow()).v } == 1));
    assert!(
        (({
            let _args: Value<Tracked> = Rc::new(RefCell::new(Tracked::new({ 2 })));
            forward_pack_6(_args.as_pointer())
        }) == (Overload_kRvalueOverload as i32))
    );
    let i: Value<i32> = Rc::new(RefCell::new(3));
    assert!(
        (({
            let _args_1: Value<Tracked> = Rc::new(RefCell::new(Tracked::new({ 4 })));
            let _args_3: Value<i32> = Rc::new(RefCell::new(5));
            forward_pack_7(
                a.as_pointer(),
                _args_1.as_pointer(),
                i.as_pointer(),
                _args_3.as_pointer(),
            )
        }) == 1234)
    );
    assert!(({ (*a.borrow()).v } == 1));
    assert!(((*i.borrow()) == 3));
    let lhs: Value<Tracked> = Rc::new(RefCell::new(Tracked::new({ 6 })));
    let p: Value<Pair> = Rc::new(RefCell::new(
        ({
            let _args_1: Value<Tracked> = Rc::new(RefCell::new(Tracked::new({ 7 })));
            forward_pack_into_ctor_8(lhs.as_pointer(), _args_1.as_pointer())
        }),
    ));
    assert!(({ (*p.borrow()).a.v } == 6));
    assert!(({ (*p.borrow()).a.copies } == 1));
    assert!(({ (*p.borrow()).b.v } == 7));
    assert!(({ (*p.borrow()).b.copies } == 0));
    assert!(({ (*p.borrow()).b.moves } == 1));
    assert!(({ (*lhs.borrow()).v } == 6));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
