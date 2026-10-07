extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(VaArg, FnPtrArg, ByteRepr, DeepClone)]
#[byte_size(8)]
pub struct anon_0 {
    #[offset(0)]
    #[byte_size(8)]
    __bytes: Value<Box<[u8]>>,
}
impl anon_0 {
    pub fn bytes(this: Ptr<Self>) -> Ptr<u8> {
        this.reinterpret_cast()
    }
    pub fn aligner(this: Ptr<Self>) -> Ptr<AnyPtr> {
        this.reinterpret_cast()
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
    let mut tail_size: usize = 32_usize;
    let mut n: Ptr<node> =
        libcc2rs::malloc_refcount(((24usize as u64).wrapping_add((tail_size as u64)) as usize))
            .reinterpret_cast::<node>();
    field!(n, len).write(tail_size);
    let mut i: usize = 0_usize;
    'loop_: while (((i < tail_size) as i32) != 0) {
        elem!(
            (anon_0::bytes(field_ptr!(n, x)).reinterpret_cast::<u8>() as Ptr::<u8>),
            i
        )
        .write({ ((i & 255_usize) as u8) });
        i.postfix_inc();
    }
    let mut i: usize = 0_usize;
    'loop_: while (((i < tail_size) as i32) != 0) {
        assert!(
            ((({
                ((elem!(
                    (anon_0::bytes(field_ptr!(n, x)).reinterpret_cast::<u8>() as Ptr::<u8>),
                    i
                )
                .read()) as i32)
            } == { (((i & 255_usize) as u8) as i32) }) as i32)
                != 0)
        );
        i.postfix_inc();
    }
    let mut p: Ptr<u8> = ((anon_0::bytes(field_ptr!(n, x)).reinterpret_cast::<u8>() as Ptr<u8>)
        .offset((10) as isize));
    assert!((((((p.read()) as i32) == 10) as i32) != 0));
    p.write(170_u8);
    assert!(
        (((((elem!(
            (anon_0::bytes(field_ptr!(n, x)).reinterpret_cast::<u8>() as Ptr::<u8>),
            10
        )
        .read()) as i32)
            == 170) as i32)
            != 0)
    );
    field!(n, pos).write(20_usize);
    let mut q: Ptr<u8> = ((anon_0::bytes(field_ptr!(n, x)).reinterpret_cast::<u8>() as Ptr<u8>)
        .offset((n.with(|__s| __s.pos)) as isize));
    assert!((((((q.read()) as i32) == 20) as i32) != 0));
    q.write(187_u8);
    assert!((((((q.read()) as i32) == 187) as i32) != 0));
    libcc2rs::free_refcount((n).to_any());
    return 0;
}
pub fn __cpp2rust_init_globals() {}
