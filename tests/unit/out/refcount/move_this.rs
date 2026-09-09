extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Chain {
    pub v: Value<i32>,
}
impl Chain {
    pub fn Chain(v: i32) -> Self {
        let v: Value<i32> = Rc::new(RefCell::new(v));
        let __this: Value<Chain> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*v.borrow()))),
        }));
        let this: Ptr<Chain> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn Chain_pconstChain(o: Ptr<Chain>) -> Self {
        let __this: Value<Chain> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new(((*(*o.upgrade().deref()).v.borrow()) + 100))),
        }));
        let this: Ptr<Chain> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn Chain_pmutChain(o: Ptr<Chain>) -> Self {
        let __this: Value<Chain> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new(((*(*o.upgrade().deref()).v.borrow()) + 1))),
        }));
        let this: Ptr<Chain> = __this.as_pointer();
        (*(*o.upgrade().deref()).v.borrow_mut()) = 0;
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for Chain {
    fn clone(&self) -> Self {
        let __src: Value<Chain> = Rc::new(RefCell::new(Chain { v: self.v.clone() }));
        Chain::Chain_pconstChain(__src.as_pointer())
    }
}
impl ByteRepr for Chain {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.v.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
pub fn consume_0(c: Chain) -> i32 {
    let c: Value<Chain> = Rc::new(RefCell::new(c));
    return (*(*c.borrow()).v.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<Chain> = Rc::new(RefCell::new(Chain::Chain({ 1 })));
    ({ ChainImpl::add_i32_lref(&({ ChainImpl::add_i32_lref(&a.as_pointer(), 1) }), 1) });
    assert!(((*(*a.borrow()).v.borrow()) == 3));
    let b0: Value<Chain> = Rc::new(RefCell::new(Chain::Chain({ 5 })));
    let b: Value<Chain> = Rc::new(RefCell::new(Chain::Chain_pmutChain({
        ({
            ChainImpl::add_i32_rref(
                &Rc::new(RefCell::new(
                    (*({
                        ChainImpl::add_i32_rref(
                            &Rc::new(RefCell::new((*b0.borrow()))).as_pointer(),
                            1,
                        )
                    })
                    .upgrade()
                    .deref()),
                ))
                .as_pointer(),
                1,
            )
        })
    })));
    assert!(((*(*b.borrow()).v.borrow()) == 8) && ((*(*b0.borrow()).v.borrow()) == 0));
    let c: Value<Chain> = Rc::new(RefCell::new(
        ({ ChainImpl::take(&Rc::new(RefCell::new(Chain::Chain({ 10 }))).as_pointer()) }),
    ));
    assert!(((*(*c.borrow()).v.borrow()) == 11));
    let d: Value<Chain> = Rc::new(RefCell::new(({ ChainImpl::copy(&c.as_pointer()) })));
    assert!(((*(*d.borrow()).v.borrow()) == 111) && ((*(*c.borrow()).v.borrow()) == 11));
    let g: Value<Chain> = Rc::new(RefCell::new(Chain::Chain({ 20 })));
    assert!(
        (({
            consume_0(Chain::Chain_pmutChain({
                ({ ChainImpl::self_(&Rc::new(RefCell::new((*g.borrow()))).as_pointer()) })
            }))
        }) == 21)
    );
    let e: Value<Chain> = Rc::new(RefCell::new(Chain::Chain({ 30 })));
    let f: Value<Chain> = Rc::new(RefCell::new(
        ({ ChainImpl::take(&Rc::new(RefCell::new((*e.borrow()))).as_pointer()) }),
    ));
    assert!(((*(*f.borrow()).v.borrow()) == 31) && ((*(*e.borrow()).v.borrow()) == 0));
    return 0;
}
pub trait ChainImpl {
    fn add_i32_lref(&self, n: i32) -> Ptr<Chain>;
    fn add_i32_rref(&self, n: i32) -> Ptr<Chain>;
    fn take(&self) -> Chain;
    fn copy(&self) -> Chain;
    fn self_(&self) -> Ptr<Chain>;
}
impl ChainImpl for Ptr<Chain> {
    fn add_i32_lref(&self, n: i32) -> Ptr<Chain> {
        let n: Value<i32> = Rc::new(RefCell::new(n));
        (*(*(*self).upgrade().deref()).v.borrow_mut()) += (*n.borrow());
        return (*self).clone();
    }
    fn add_i32_rref(&self, n: i32) -> Ptr<Chain> {
        let n: Value<i32> = Rc::new(RefCell::new(n));
        (*(*(*self).upgrade().deref()).v.borrow_mut()) += (*n.borrow());
        return (*self).clone();
    }
    fn take(&self) -> Chain {
        return Chain::Chain_pmutChain({ (*self).clone() });
    }
    fn copy(&self) -> Chain {
        return Chain::Chain_pconstChain({ (*self).clone() });
    }
    fn self_(&self) -> Ptr<Chain> {
        return (*self).clone();
    }
}
