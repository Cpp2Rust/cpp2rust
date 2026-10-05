extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(12)]
pub struct S {
    #[offset(0)]
    pub before: i32,
    #[offset(4)]
    #[byte_size(4)]
    pub mask: Value<Box<[u8]>>,
    #[offset(8)]
    pub after: i32,
}
impl Default for S {
    fn default() -> Self {
        S {
            before: 0_i32,
            mask: Rc::new(RefCell::new((0..4).map(|_| 0_u8).collect::<Box<[u8]>>())),
            after: 0_i32,
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut s: Ptr<S> = libcc2rs::malloc_refcount(12usize).reinterpret_cast::<S>();
    assert!((((!((s).is_null())) as i32) != 0));
    field!(s, before).write(1);
    {
        ((array_field_ptr!(s, mask) as Ptr<u8>) as Ptr<u8>)
            .to_any()
            .memset((5) as u8, ::std::mem::size_of::<[u8; 4]>() as usize);
        ((array_field_ptr!(s, mask) as Ptr<u8>) as Ptr<u8>).to_any()
    };
    field!(s, after).write(2);
    ((array_field_ptr!(s, mask)) as Ptr<u8>).write(7_u8);
    let out: Value<Box<[u8]>> = Rc::new(RefCell::new((0..4).map(|_| 0_u8).collect::<Box<[u8]>>()));
    {
        ((out.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any().memcpy(
            &((array_field_ptr!(s, mask)) as Ptr<u8>).to_any(),
            ::std::mem::size_of::<[u8; 4]>() as usize,
        );
        ((out.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any()
    };
    assert!(
        ((((((((*out.borrow())[(0) as usize] as i32) == 7) as i32) != 0)
            && (((((*out.borrow())[(3) as usize] as i32) == 5) as i32) != 0)) as i32)
            != 0)
    );
    assert!(
        ((((((s.with(|__s| __s.before) == 1) as i32) != 0)
            && (((s.with(|__s| __s.after) == 2) as i32) != 0)) as i32)
            != 0)
    );
    libcc2rs::free_refcount((s).to_any());
    return 0;
}
pub fn __cpp2rust_init_globals() {}
