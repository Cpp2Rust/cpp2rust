extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct Counted {
    #[offset(0)]
    pub copies: i32,
    #[offset(4)]
    pub moves: i32,
}
impl Counted {
    pub fn new() -> Self {
        Self {
            copies: 0,
            moves: 0,
        }
    }
    pub fn copy_from(o: Ptr<Counted>) -> Self {
        Self {
            copies: (o.with(|__s| __s.copies) + 1),
            moves: o.with(|__s| __s.moves),
        }
    }
    pub fn move_from(o: Ptr<Counted>) -> Self {
        Self {
            copies: o.with(|__s| __s.copies),
            moves: (o.with(|__s| __s.moves) + 1),
        }
    }
}
impl Clone for Counted {
    fn clone(&self) -> Self {
        let __src: Value<Counted> = Rc::new(RefCell::new(Counted {
            copies: self.copies.clone(),
            moves: self.moves.clone(),
        }));
        Counted::copy_from(__src.as_pointer())
    }
}
impl Default for Counted {
    fn default() -> Self {
        { Counted::new() }
    }
}
thread_local!(
    pub static drops_0: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct Dropped {}
impl Dropped {
    pub fn new() -> Self {
        Self {}
    }
    pub fn copy_from(_a0: Ptr<Dropped>) -> Self {
        Self {}
    }
    pub fn move_from(_a0: Ptr<Dropped>) -> Self {
        Self {}
    }
}
impl Clone for Dropped {
    fn clone(&self) -> Self {
        let __src: Value<Dropped> = Rc::new(RefCell::new(Dropped {}));
        Dropped::copy_from(__src.as_pointer())
    }
}
impl Default for Dropped {
    fn default() -> Self {
        { Dropped::new() }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let c: Value<Counted> = Rc::new(RefCell::new(Counted::new()));
    let f: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let c: Value<Counted> = Rc::new(RefCell::new(Counted::copy_from({ c.as_pointer() })));
        },
        || -> i32 {
            return (({ (*c.borrow()).copies } * 10) + { (*c.borrow()).moves });
        }
    )));
    assert!((({ (*f.borrow()).call() }) == 10));
    let g: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new((*f.borrow()).clone()));
    assert!((({ (*g.borrow()).call() }) == 20));
    assert!((({ (*f.borrow()).call() }) == 10));
    let h: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new((*f.borrow_mut()).clone()));
    assert!((({ (*h.borrow()).call() }) == 11));
    let mut returned: i32 = {
        ({
            lambda!(
                {
                    let c: Value<Counted> =
                        Rc::new(RefCell::new(Counted::copy_from({ c.as_pointer() })));
                },
                || -> Counted {
                    return Counted::copy_from({ c.as_pointer() });
                }
            )
            .call()
        })
        .copies
    };
    assert!((returned == 2));
    let arr: Value<Box<[Counted]>> = Rc::new(RefCell::new(Box::new(
        std::array::from_fn::<_, 2, _>(|_| Counted::new()),
    )));
    let a: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let arr: Value<Box<[Counted]>> = Rc::new(RefCell::new(Box::new(
                std::array::from_fn::<_, 2, _>(|__i: usize| {
                    Counted::copy_from({ (arr.as_pointer() as Ptr<Counted>).offset(__i) })
                }),
            )));
        },
        || -> i32 {
            return ({ (*arr.borrow())[(0) as usize].copies } + {
                (*arr.borrow())[(1) as usize].copies
            });
        }
    )));
    assert!((({ (*a.borrow()).call() }) == 2));
    let a2: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new((*a.borrow()).clone()));
    assert!((({ (*a2.borrow()).call() }) == 4));
    {
        let m: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
            {
                let d: Value<Dropped> = Rc::new(RefCell::new(Dropped::new()));
            },
            || {}
        )));
        let m2: Value<FnPtr<fn()>> = Rc::new(RefCell::new((*m.borrow_mut()).clone()));
    }
    assert!((drops_0.with(|rc| *rc.borrow()) == 2));
    {
        let k: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
            {
                let d: Value<Dropped> = Rc::new(RefCell::new(Dropped::new()));
            },
            || {}
        )));
        let k2: Value<FnPtr<fn()>> = Rc::new(RefCell::new((*k.borrow()).clone()));
    }
    assert!((drops_0.with(|rc| *rc.borrow()) == 4));
    return 0;
}
pub trait DroppedImpl {
    fn destructor(&self);
}
impl DroppedImpl for Ptr<Dropped> {
    fn destructor(&self) {
        (*drops_0.with(Value::clone).borrow_mut()).postfix_inc();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = drops_0.with(|_| ());
}
