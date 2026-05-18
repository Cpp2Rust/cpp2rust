extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct context {
    pub verbose: Value<i32>,
    pub last_error: Value<i32>,
}
impl ByteRepr for context {}
pub fn set_error_0(ctx: Ptr<context>, fmt: Ptr<u8>, __args: &[VaArg]) {
    let ctx: Value<Ptr<context>> = Rc::new(RefCell::new(ctx));
    let fmt: Value<Ptr<u8>> = Rc::new(RefCell::new(fmt));
    if ((*(*(*ctx.borrow()).upgrade().deref()).verbose.borrow()) != 0) {
        let ap: Value<VaList> = Rc::new(RefCell::new(VaList::default()));
        (*ap.borrow_mut()) = VaList::new(__args);
        (*(*(*ctx.borrow()).upgrade().deref()).last_error.borrow_mut()) =
            ((*ap.borrow_mut()).arg::<i32>()).clone();
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let ctx: Value<context> = <Value<context>>::default();
    (*(*ctx.borrow()).verbose.borrow_mut()) = 1;
    (*(*ctx.borrow()).last_error.borrow_mut()) = 0;
    ({
        let _ctx: Ptr<context> = (ctx.as_pointer());
        let _fmt: Ptr<u8> = Ptr::from_string_literal("error %d");
        set_error_0(_ctx, _fmt, &[42.into()])
    });
    assert!(((((*(*ctx.borrow()).last_error.borrow()) == 42) as i32) != 0));
    (*(*ctx.borrow()).verbose.borrow_mut()) = 0;
    ({
        let _ctx: Ptr<context> = (ctx.as_pointer());
        let _fmt: Ptr<u8> = Ptr::from_string_literal("error %d");
        set_error_0(_ctx, _fmt, &[99.into()])
    });
    assert!(((((*(*ctx.borrow()).last_error.borrow()) == 42) as i32) != 0));
    return 0;
}
