extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(VaArg, FnPtrArg, ByteRepr, Clone)]
#[byte_size(4)]
pub struct anon_0 {
    #[offset(0)]
    #[byte_size(4)]
    __bytes: Box<[u8]>,
}
impl anon_0 {
    pub fn as_int(this: Ptr<Self>) -> Ptr<i32> {
        this.reinterpret_cast()
    }
    pub fn as_unsigned(this: Ptr<Self>) -> Ptr<u32> {
        this.reinterpret_cast()
    }
}
impl Default for anon_0 {
    fn default() -> Self {
        anon_0 {
            __bytes: Box::from([0u8; 4]),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct HasAnonymousUnion {
    #[offset(0)]
    pub before: i32,
    #[offset(4)]
    #[byte_size(4)]
    pub anon_0: anon_0,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut pm: isize = (<HasAnonymousUnion as Record>::OFFSETS.anon_0 as isize);
    let obj: Value<HasAnonymousUnion> = Rc::new(RefCell::new(<HasAnonymousUnion>::default()));
    (*obj.borrow_mut()).before = 1;
    anon_0::as_int(field_ptr!(obj.as_pointer(), anon_0)).write(2);
    assert!((((obj.as_pointer()).member::<i32>(pm).read()) == 2));
    (obj.as_pointer()).member::<i32>(pm).write(3);
    assert!(((anon_0::as_int(field_ptr!(obj.as_pointer(), anon_0)).read()) == 3));
    assert!(({ ({ (*obj.borrow()).before } == 1) }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
