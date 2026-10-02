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
pub struct context {
    #[offset(0)]
    pub verbose: i32,
    #[offset(4)]
    pub last_error: i32,
}
pub fn set_error_0(ctx: Ptr<context>, fmt: Ptr<u8>, __args: &[VaArg]) {
    let ctx: Value<Ptr<context>> = Rc::new(RefCell::new(ctx));
    let fmt: Value<Ptr<u8>> = Rc::new(RefCell::new(fmt));
    if ((*ctx.borrow()).with(|__s| __s.verbose) != 0) {
        let ap: Value<VaList> = Rc::new(RefCell::new(VaList::default()));
        (*ap.borrow_mut()) = VaList::new(__args);
        let __rhs = (*ap.borrow_mut()).arg::<i32>();
        field!((*ctx.borrow()), last_error).write(__rhs);
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let ctx: Value<context> = <Value<context>>::default();
    (*ctx.borrow_mut()).verbose = 1;
    (*ctx.borrow_mut()).last_error = 0;
    ({
        set_error_0(
            (ctx.as_pointer()),
            Ptr::<u8>::from_string_literal(b"error %d"),
            &[(42).into()],
        )
    });
    assert!(((({ (*ctx.borrow()).last_error } == 42) as i32) != 0));
    (*ctx.borrow_mut()).verbose = 0;
    ({
        set_error_0(
            (ctx.as_pointer()),
            Ptr::<u8>::from_string_literal(b"error %d"),
            &[(99).into()],
        )
    });
    assert!(((({ (*ctx.borrow()).last_error } == 42) as i32) != 0));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
