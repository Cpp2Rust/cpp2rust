extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive()]
pub union node_anon_10_3 {
    pub bytes: Value<Box<[u8]>>,
    pub aligner: Value<AnyPtr>,
}
impl Default for node_anon_10_3 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl ByteRepr for node_anon_10_3 {}
#[derive(Default)]
pub struct node {
    pub len: Value<u64>,
    pub x: Value<node_anon_10_3>,
}
impl ByteRepr for node {}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let tail_size: Value<u64> = Rc::new(RefCell::new(32_u64));
    let n: Value<Ptr<node>> = Rc::new(RefCell::new(
        ({
            let ___size: u64 =
                (::std::mem::size_of::<node>() as u64 as u64).wrapping_add((*tail_size.borrow()));
            malloc_0(___size)
        })
        .cast::<node>()
        .expect("ub:wrong type"),
    ));
    (*(*(*n.borrow()).upgrade().deref()).len.borrow_mut()) = (*tail_size.borrow());
    let i: Value<u64> = Rc::new(RefCell::new(0_u64));
    'loop_: while ((*i.borrow()) < (*tail_size.borrow())) {
        let __rhs = ((((*i.borrow()) & 255_u64) as u64) as u8);
        (*(*(*(*n.borrow()).upgrade().deref()).x.borrow())
            .bytes
            .borrow_mut())[(*i.borrow()) as usize] = __rhs;
        (*i.borrow_mut()).postfix_inc();
    }
    let i: Value<u64> = Rc::new(RefCell::new(0_u64));
    'loop_: while ((*i.borrow()) < (*tail_size.borrow())) {
        assert!({
            let _lhs = ((*(*(*(*n.borrow()).upgrade().deref()).x.borrow())
                .bytes
                .borrow())[(*i.borrow()) as usize] as i32);
            _lhs == (((((*i.borrow()) & 255_u64) as u64) as u8) as i32)
        });
        (*i.borrow_mut()).postfix_inc();
    }
    let p: Value<Ptr<u8>> = Rc::new(RefCell::new(
        (((*(*(*n.borrow()).upgrade().deref()).x.borrow())
            .bytes
            .as_pointer() as Ptr<u8>)
            .offset(10 as isize)),
    ));
    assert!(((((*p.borrow()).read()) as i32) == 10));
    (*p.borrow()).write(170_u8);
    assert!(
        (((*(*(*(*n.borrow()).upgrade().deref()).x.borrow())
            .bytes
            .borrow())[(10) as usize] as i32)
            == 170)
    );
    ({
        let ___ptr: AnyPtr = ((*n.borrow()).clone() as Ptr<node>).to_any();
        free_1(___ptr)
    });
    return 0;
}
