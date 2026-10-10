extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct In {
    #[offset(0)]
    pub a: i16,
    #[offset(2)]
    pub b: i16,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(12)]
pub struct S {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    #[byte_size(4)]
    pub in_: In,
    #[offset(8)]
    pub z: i32,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let src: Value<S> = Rc::new(RefCell::new(S {
        x: 1,
        in_: In { a: 2_i16, b: 3_i16 },
        z: 4,
    }));
    let mut p: Ptr<S> = libcc2rs::malloc_refcount(12usize).reinterpret_cast::<S>();
    assert!((((!((p).is_null())) as i32) != 0));
    {
        ((field_ptr!(p, x)) as Ptr<i32>)
            .to_any()
            .memcpy(&((src.as_pointer()) as Ptr<S>).to_any(), 12usize as usize);
        ((field_ptr!(p, x)) as Ptr<i32>).to_any()
    };
    assert!(
        ((((((((((((p.with(|__s| __s.x) == 1) as i32) != 0)
            && ((((p.with(|__s| __s.in_.a) as i32) == 2) as i32) != 0)) as i32)
            != 0)
            && ((((p.with(|__s| __s.in_.b) as i32) == 3) as i32) != 0)) as i32)
            != 0)
            && (((p.with(|__s| __s.z) == 4) as i32) != 0)) as i32)
            != 0)
    );
    let n: Value<In> = Rc::new(RefCell::new(In { a: 5_i16, b: 6_i16 }));
    {
        ((field_ptr!(p, in_)) as Ptr<In>)
            .to_any()
            .memcpy(&((n.as_pointer()) as Ptr<In>).to_any(), 4usize as usize);
        ((field_ptr!(p, in_)) as Ptr<In>).to_any()
    };
    assert!(
        ((((((((((((p.with(|__s| __s.x) == 1) as i32) != 0)
            && ((((p.with(|__s| __s.in_.a) as i32) == 5) as i32) != 0)) as i32)
            != 0)
            && ((((p.with(|__s| __s.in_.b) as i32) == 6) as i32) != 0)) as i32)
            != 0)
            && (((p.with(|__s| __s.z) == 4) as i32) != 0)) as i32)
            != 0)
    );
    let mut bz: Ptr<u8> = (field_ptr!(p, z)).reinterpret_cast::<u8>();
    let mut i: i32 = 0;
    'loop_: while (((i < 4) as i32) != 0) {
        elem!(bz, i).write(1_u8);
        i.postfix_inc();
    }
    assert!(
        ((((((p.with(|__s| __s.z) == 16843009) as i32) != 0)
            && ((((p.with(|__s| __s.in_.b) as i32) == 6) as i32) != 0)) as i32)
            != 0)
    );
    libcc2rs::free_refcount((p).to_any());
    return 0;
}
pub fn __cpp2rust_init_globals() {}
