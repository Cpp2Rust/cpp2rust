extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct node {
    #[offset(0)]
    pub data: i32,
    #[offset(8)]
    #[byte_size(8)]
    pub next: Ptr<node>,
}
pub type opt = u32;
pub const opt_OPT_STRING_OUT: opt = 0;
pub const opt_OPT_FILE: opt = 1;
pub const opt_OPT_NODE: opt = 2;
pub const opt_OPT_NODE_OUT: opt = 3;
pub fn dispatch_0(option: i32, __args: &[VaArg]) -> i32 {
    let __args: Value<Box<[VaArg]>> = Rc::new(RefCell::new(__args.into()));
    let option: Value<i32> = Rc::new(RefCell::new(option));
    let ap: Value<Ptr<VaArg>> = Rc::new(RefCell::new(Ptr::<VaArg>::default()));
    (*ap.borrow_mut()) = __args.as_pointer();
    let mut result: i32 = 0;
    'switch: {
        match { (*option.borrow()) } {
            __v if __v == (opt_OPT_STRING_OUT as i32) => {
                let mut out: Ptr<Ptr<i8>> = (*ap.borrow_mut()).arg::<Ptr<Ptr<i8>>>();
                out.write(Ptr::<i8>::from_string_literal(b"hello"));
                result = 1;
                break 'switch;
            }
            __v if __v == (opt_OPT_FILE as i32) => {
                let mut f: Ptr<CFile> = (*ap.borrow_mut()).arg::<Ptr<CFile>>();
                result = ((!((f).is_null())) as i32);
                break 'switch;
            }
            __v if __v == (opt_OPT_NODE as i32) => {
                let mut n: Ptr<node> = (*ap.borrow_mut()).arg::<Ptr<node>>();
                result = n.with(|__s| __s.data);
                break 'switch;
            }
            __v if __v == (opt_OPT_NODE_OUT as i32) => {
                let mut out: Ptr<Ptr<node>> = (*ap.borrow_mut()).arg::<Ptr<Ptr<node>>>();
                out.write(Ptr::<node>::null());
                result = 2;
                break 'switch;
            }
            _ => {}
        }
    };
    return result;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<Ptr<i8>> = Rc::new(RefCell::new(Ptr::<i8>::null()));
    assert!(
        (((({ dispatch_0((opt_OPT_STRING_OUT as i32), &[(s.as_pointer()).into(),]) }) == 1)
            as i32)
            != 0)
    );
    assert!((((!((*s.borrow()).is_null())) as i32) != 0));
    assert!(
        (((({ dispatch_0((opt_OPT_FILE as i32), &[(libcc2rs::c_stdout()).into(),]) }) == 1)
            as i32)
            != 0)
    );
    assert!(
        (((({
            dispatch_0(
                (opt_OPT_FILE as i32),
                &[((AnyPtr::default()).reinterpret_cast::<CFile>()).into()],
            )
        }) == 0) as i32)
            != 0)
    );
    let head: Value<node> = Rc::new(RefCell::new(node {
        data: 42,
        next: Ptr::<node>::null(),
    }));
    assert!(
        (((({ dispatch_0((opt_OPT_NODE as i32), &[(head.as_pointer()).into(),]) }) == 42) as i32)
            != 0)
    );
    let outp: Value<Ptr<node>> = Rc::new(RefCell::new((head.as_pointer())));
    assert!(
        (((({ dispatch_0((opt_OPT_NODE_OUT as i32), &[(outp.as_pointer()).into(),]) }) == 2)
            as i32)
            != 0)
    );
    assert!(((((*outp.borrow()).is_null()) as i32) != 0));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
