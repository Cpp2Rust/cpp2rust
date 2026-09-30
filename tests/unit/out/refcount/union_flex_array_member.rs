extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(ByteRepr)]
#[byte_size(8)]
pub struct anon_0 {
    #[offset(0)]
    #[byte_size(8)]
    __bytes: Value<Box<[u8]>>,
}
impl anon_0 {
    pub fn bytes(&self) -> Ptr<u8> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
    pub fn aligner(&self) -> Ptr<AnyPtr> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
}
impl Clone for anon_0 {
    fn clone(&self) -> Self {
        anon_0 {
            __bytes: Rc::new(RefCell::new(self.__bytes.borrow().clone())),
        }
    }
}
impl Default for anon_0 {
    fn default() -> Self {
        anon_0 {
            __bytes: Rc::new(RefCell::new(Box::from([0u8; 8]))),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(24)]
pub struct node {
    #[offset(0)]
    pub len: usize,
    #[offset(8)]
    pub pos: usize,
    #[offset(16)]
    #[byte_size(8)]
    pub x: anon_0,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let tail_size: Value<usize> = Rc::new(RefCell::new(32_usize));
    let n: Value<Ptr<node>> = Rc::new(RefCell::new(
        libcc2rs::malloc_refcount(
            ((24usize as u64).wrapping_add(((*tail_size.borrow()) as u64)) as usize),
        )
        .reinterpret_cast::<node>(),
    ));
    field!((*n.borrow()), len).write((*tail_size.borrow()));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((((*i.borrow()) < (*tail_size.borrow())) as i32) != 0) {
        let __rhs = (((*i.borrow()) & 255_usize) as u8);
        ((*(*n.borrow()).upgrade().deref())
            .x
            .bytes()
            .reinterpret_cast::<u8>() as Ptr<u8>)
            .offset((*i.borrow()) as isize)
            .write(__rhs);
        (*i.borrow_mut()).postfix_inc();
    }
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((((*i.borrow()) < (*tail_size.borrow())) as i32) != 0) {
        assert!(
            ((({
                let _lhs = ((((*(*n.borrow()).upgrade().deref())
                    .x
                    .bytes()
                    .reinterpret_cast::<u8>() as Ptr<u8>)
                    .offset((*i.borrow()) as isize)
                    .read()) as i32);
                _lhs == ((((*i.borrow()) & 255_usize) as u8) as i32)
            }) as i32)
                != 0)
        );
        (*i.borrow_mut()).postfix_inc();
    }
    let p: Value<Ptr<u8>> = Rc::new(RefCell::new(
        (((*(*n.borrow()).upgrade().deref())
            .x
            .bytes()
            .reinterpret_cast::<u8>() as Ptr<u8>)
            .offset((10) as isize)),
    ));
    assert!(((((((*p.borrow()).read()) as i32) == 10) as i32) != 0));
    (*p.borrow()).write(170_u8);
    assert!(
        (((((((*(*n.borrow()).upgrade().deref())
            .x
            .bytes()
            .reinterpret_cast::<u8>() as Ptr::<u8>)
            .offset((10) as isize)
            .read()) as i32)
            == 170) as i32)
            != 0)
    );
    field!((*n.borrow()), pos).write(20_usize);
    let q: Value<Ptr<u8>> = Rc::new(RefCell::new(
        (((*(*n.borrow()).upgrade().deref())
            .x
            .bytes()
            .reinterpret_cast::<u8>() as Ptr<u8>)
            .offset(((*n.borrow()).with(|__s| __s.pos)) as isize)),
    ));
    assert!(((((((*q.borrow()).read()) as i32) == 20) as i32) != 0));
    (*q.borrow()).write(187_u8);
    assert!(((((((*q.borrow()).read()) as i32) == 187) as i32) != 0));
    libcc2rs::free_refcount((*n.borrow()).to_any());
    return 0;
}
pub fn __cpp2rust_init_globals() {}
