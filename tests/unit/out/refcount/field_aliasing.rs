extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct In {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(32)]
pub struct S {
    #[offset(0)]
    #[byte_size(8)]
    pub in_: In,
    #[offset(8)]
    pub total: i32,
    #[offset(12)]
    pub n: i32,
    #[offset(16)]
    #[byte_size(16)]
    pub arr: Value<Box<[i32]>>,
}
impl Default for S {
    fn default() -> Self {
        S {
            in_: <In>::default(),
            total: 0_i32,
            n: 0_i32,
            arr: Rc::new(RefCell::new((0..4).map(|_| 0_i32).collect::<Box<[i32]>>())),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Node {
    #[offset(0)]
    pub x: i32,
    #[offset(8)]
    #[byte_size(8)]
    pub self_: Ptr<Node>,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut p: Ptr<S> = libcc2rs::calloc_refcount(1_usize, 32usize).reinterpret_cast::<S>();
    assert!((((!((p).is_null())) as i32) != 0));
    let mut q: Ptr<S> = (p).clone();
    field!(field!(p, in_), x).write(1);
    field!(field!(p, in_), y).write(2);
    field!(p, total).write(({ q.with(|__s| __s.in_.x) } + { q.with(|__s| __s.in_.y) }));
    assert!((((q.with(|__s| __s.total) == 3) as i32) != 0));
    let mut ip: Ptr<In> = (field_ptr!(p, in_));
    field!(ip, x).write((p.with(|__s| __s.total) + 1));
    assert!(
        ((((((q.with(|__s| __s.in_.x) == 4) as i32) != 0)
            && (((q.with(|__s| __s.in_.y) == 2) as i32) != 0)) as i32)
            != 0)
    );
    elem!(
        (array_field_ptr!(p, arr) as Ptr::<i32>),
        p.with(|__s| __s.n)
    )
    .write({ p.with(|__s| __s.total) });
    {
        let _ptr = field!(p, n);
        _ptr.write(_ptr.read() + 1)
    };
    elem!(
        (array_field_ptr!(p, arr) as Ptr::<i32>),
        p.with(|__s| __s.n)
    )
    .write(q.with(|__s| __s.in_.x));
    assert!(
        ((((((((((elem!((array_field_ptr!(q, arr) as Ptr::<i32>), 0).read()) == 3) as i32) != 0)
            && ((((elem!((array_field_ptr!(q, arr) as Ptr::<i32>), 1).read()) == 4) as i32) != 0))
            as i32)
            != 0)
            && (((q.with(|__s| __s.n) == 1) as i32) != 0)) as i32)
            != 0)
    );
    libcc2rs::free_refcount((p).to_any());
    let s: Value<Node> = <Value<Node>>::default();
    (*s.borrow_mut()).x = 1;
    (*s.borrow_mut()).self_ = { (s.as_pointer()) };
    field!({ (*s.borrow()).self_.clone() }, x).write({ ({ (*s.borrow()).x } + 1) });
    assert!(((({ (*s.borrow()).x } == 2) as i32) != 0));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
