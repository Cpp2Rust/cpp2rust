extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn pick_0(x: Option<i32>) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x.unwrap_or(
        ({
            (|| {
                return Rc::new(RefCell::new(237));
            })()
        }),
    )));
    return (*x.borrow());
}
#[derive()]
pub struct S {
    pub j: Value<i32>,
    pub i: Value<i32>,
    pub k: Value<i32>,
}
impl Clone for S {
    fn clone(&self) -> Self {
        let __this: Value<S> = Rc::new(RefCell::new(Self {
            j: Rc::new(RefCell::new((*self.j.borrow()))),
            i: Rc::new(RefCell::new((*self.i.borrow()))),
            k: Rc::new(RefCell::new((*self.k.borrow()))),
        }));
        let this: Ptr<S> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for S {
    fn default() -> Self {
        S {
            j: Rc::new(RefCell::new(10)),
            i: Rc::new(RefCell::new(
                ({
                    (|| {
                        return Rc::new(RefCell::new(((*self.j.borrow()) * 2)));
                    })()
                }),
            )),
            k: Rc::new(RefCell::new(
                (({
                    (|| {
                        return Rc::new(RefCell::new(3));
                    })()
                }) + 1),
            )),
        }
    }
}
impl ByteRepr for S {
    fn byte_size() -> usize {
        12
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.j.borrow()).to_bytes(&mut buf[0..4]);
        (*self.i.borrow()).to_bytes(&mut buf[4..8]);
        (*self.k.borrow()).to_bytes(&mut buf[8..12]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            j: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            i: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
            k: Rc::new(RefCell::new(<i32>::from_bytes(&buf[8..12]))),
        }
    }
}
thread_local!(
    pub static g_1: Value<i32> = Rc::new(RefCell::new(
        ({
            (|| {
                let s: Value<i32> = Rc::new(RefCell::new(0));
                let i: Value<i32> = Rc::new(RefCell::new(1));
                'loop_: while ((*i.borrow()) <= 4) {
                    (*s.borrow_mut()) += (*i.borrow());
                    (*i.borrow_mut()).postfix_inc();
                }
                return Rc::new(RefCell::new((*s.borrow())));
            })()
        }),
    ));
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((({ pick_0(None,) }) == 237));
    assert!((({ pick_0(Some(1),) }) == 1));
    let s: Value<S> = Rc::new(RefCell::new(<S>::default()));
    assert!(((*(*s.borrow()).i.borrow()) == 20));
    assert!(((*(*s.borrow()).k.borrow()) == 4));
    let t: Value<S> = Rc::new(RefCell::new(S {
        j: Rc::new(RefCell::new(5)),
        i: Rc::new(RefCell::new(
            ({
                (|| {
                    return Rc::new(RefCell::new(((*self.j.borrow()) * 2)));
                })()
            }),
        )),
        k: Rc::new(RefCell::new(
            (({
                (|| {
                    return Rc::new(RefCell::new(3));
                })()
            }) + 1),
        )),
    }));
    assert!(((*(*t.borrow()).i.borrow()) == 10));
    assert!((g_1.with(|rc| *rc.borrow()) == 10));
    let a: Value<i32> = Rc::new(RefCell::new(2));
    let c: Value<i32> = Rc::new(RefCell::new(
        ({
            (|| {
                (*a.borrow_mut()).postfix_inc();
                return Rc::new(RefCell::new(((*a.borrow()) * 10)));
            })()
        }),
    ));
    assert!(((*c.borrow()) == 30));
    assert!(((*a.borrow()) == 3));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = g_1.with(|_| ());
}
