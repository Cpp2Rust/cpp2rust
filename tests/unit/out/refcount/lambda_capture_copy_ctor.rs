extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive()]
pub struct Counted {
    pub copies: Value<i32>,
    pub moves: Value<i32>,
}
impl Counted {
    pub fn new() -> Self {
        let __this: Value<Counted> = Rc::new(RefCell::new(Self {
            copies: Rc::new(RefCell::new(0)),
            moves: Rc::new(RefCell::new(0)),
        }));
        let this: Ptr<Counted> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn copy_from(o: Ptr<Counted>) -> Self {
        let __this: Value<Counted> = Rc::new(RefCell::new(Self {
            copies: Rc::new(RefCell::new(
                ((*(*o.upgrade().deref()).copies.borrow()) + 1),
            )),
            moves: Rc::new(RefCell::new((*(*o.upgrade().deref()).moves.borrow()))),
        }));
        let this: Ptr<Counted> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn move_from(o: Ptr<Counted>) -> Self {
        let __this: Value<Counted> = Rc::new(RefCell::new(Self {
            copies: Rc::new(RefCell::new((*(*o.upgrade().deref()).copies.borrow()))),
            moves: Rc::new(RefCell::new(((*(*o.upgrade().deref()).moves.borrow()) + 1))),
        }));
        let this: Ptr<Counted> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
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
impl ByteRepr for Counted {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.copies.borrow()).to_bytes(&mut buf[0..4]);
        (*self.moves.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            copies: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            moves: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let c: Value<Counted> = Rc::new(RefCell::new(Counted::new()));
    let f: Value<_> = Rc::new(RefCell::new(
        (|| {
            return (((*(*c.borrow()).copies.borrow()) * 10) + (*(*c.borrow()).moves.borrow()));
        }),
    ));
    assert!((({ (*f.borrow_mut())() }) == 10));
    let g: Value<_> = Rc::new(RefCell::new((*f.borrow()).clone()));
    assert!((({ (*g.borrow_mut())() }) == 20));
    assert!((({ (*f.borrow_mut())() }) == 10));
    let h: Value<_> = Rc::new(RefCell::new((*f.borrow_mut()).clone()));
    assert!((({ (*h.borrow_mut())() }) == 11));
    let returned: Value<i32> = Rc::new(RefCell::new(
        (*({
            (|| {
                return Rc::new(RefCell::new(Counted::copy_from({ c.as_pointer() })));
            })()
        })
        .copies
        .borrow()),
    ));
    assert!(((*returned.borrow()) == 2));
    let arr: Value<Box<[Counted]>> = Rc::new(RefCell::new(Box::new(
        std::array::from_fn::<_, 2, _>(|_| Counted::new()),
    )));
    let a: Value<_> = Rc::new(RefCell::new(
        (|| {
            return ((*(*arr.borrow())[(0) as usize].copies.borrow())
                + (*(*arr.borrow())[(1) as usize].copies.borrow()));
        }),
    ));
    assert!((({ (*a.borrow_mut())() }) == 2));
    let a2: Value<_> = Rc::new(RefCell::new((*a.borrow()).clone()));
    assert!((({ (*a2.borrow_mut())() }) == 4));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
