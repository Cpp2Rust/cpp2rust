extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type Overload = u32;
pub const Overload_kMutableOverload: Overload = 1;
pub const Overload_kConstOverload: Overload = 2;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct S {
    #[offset(0)]
    pub v: i32,
}
pub fn g_0(_a0: Ptr<S>) -> Overload {
    return Overload_kMutableOverload;
}
pub fn g_1(_a0: Ptr<S>) -> Overload {
    return Overload_kConstOverload;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<S> = Rc::new(RefCell::new(S { v: 7 }));
    assert!(((({ SImpl::f_1(&s.as_pointer(),) }) as i32) == (Overload_kMutableOverload as i32)));
    assert!(((({ SImpl::f_2(&s.as_pointer(),) }) as i32) == (Overload_kConstOverload as i32)));
    assert!(((({ g_0(s.as_pointer(),) }) as i32) == (Overload_kMutableOverload as i32)));
    assert!(((({ g_1(s.as_pointer(),) }) as i32) == (Overload_kConstOverload as i32)));
    ({ SImpl::value_ref_3(&s.as_pointer()) }).write(9);
    assert!(({ (*s.borrow()).v } == 9));
    assert!(((({ SImpl::value_ref_4(&s.as_pointer(),) }).read()) == 9));
    let cs: Ptr<S> = s.as_pointer();
    assert!(((({ SImpl::f_2(&cs,) }) as i32) == (Overload_kConstOverload as i32)));
    assert!((cs.with(|__s| __s.v) == 9));
    let p: Value<Ptr<S>> = Rc::new(RefCell::new((s.as_pointer())));
    field!((*p.borrow()), v).write(11);
    assert!(({ (*s.borrow()).v } == 11));
    assert!(((({ SImpl::f_1(&(*p.borrow()),) }) as i32) == (Overload_kMutableOverload as i32)));
    return 0;
}
pub trait SImpl {
    fn f_1(&self) -> Overload;
    fn f_2(&self) -> Overload;
    fn value_ref_3(&self) -> Ptr<i32>;
    fn value_ref_4(&self) -> Ptr<i32>;
}
impl SImpl for Ptr<S> {
    fn f_1(&self) -> Overload {
        return Overload_kMutableOverload;
    }
    fn f_2(&self) -> Overload {
        return Overload_kConstOverload;
    }
    fn value_ref_3(&self) -> Ptr<i32> {
        return field_ptr!((*self), v);
    }
    fn value_ref_4(&self) -> Ptr<i32> {
        return field_ptr!((*self), v);
    }
}
pub fn __cpp2rust_init_globals() {}
