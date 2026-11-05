extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[inline]
pub fn inlined_free_0(mut a: i32) -> i32 {
    return (a + 1);
}
#[inline]
pub fn inlined_constexpr_free_1(mut a: i32) -> i32 {
    return (a + 2);
}
#[inline]
pub fn inlined_declared_free_2(mut a: i32) -> i32 {
    return (a + 3);
}
pub fn not_inlined_free_3(mut a: i32) -> i32 {
    return (a + 4);
}
pub fn not_inlined_static_free_4(mut a: i32) -> i32 {
    return (a + 5);
}
pub trait Base {
    fn inlined_override(&self, a: i32) -> i32;
    fn not_inlined_override(&self, a: i32) -> i32;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Derived {
    #[offset(8)]
    v_: i32,
}
impl Derived {
    #[inline]
    pub fn new(mut v: i32) -> Self {
        Self { v_: v }
    }
    #[inline]
    pub fn inlined_static_in_class(mut a: i32) -> i32 {
        return (a * 3);
    }
}
impl Derived {
    pub fn not_inlined_static_out_of_line(mut a: i32) -> i32 {
        return (a * 7);
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let d: Value<Derived> = Rc::new(RefCell::new(Derived::new({ 3 })));
    let b: Ptr<Base> = d.as_pointer();
    assert!((({ inlined_free_0(1,) }) == 2));
    assert!((({ inlined_constexpr_free_1(1,) }) == 3));
    assert!((({ inlined_declared_free_2(1,) }) == 4));
    assert!((({ not_inlined_free_3(1,) }) == 5));
    assert!((({ not_inlined_static_free_4(1,) }) == 6));
    assert!(({ (({ (*b.upgrade().deref()).inlined_override(1,) }) == 4) }));
    assert!(({ (({ (*b.upgrade().deref()).not_inlined_override(1,) }) == 2) }));
    assert!((({ DerivedImpl::inlined_in_class(&d.as_pointer(),) }) == 6));
    assert!((({ Derived::inlined_static_in_class(2,) }) == 6));
    assert!((({ DerivedImpl::inlined_declared_in_class(&d.as_pointer(),) }) == 12));
    assert!((({ DerivedImpl::inlined_defined_out_of_line(&d.as_pointer(),) }) == 15));
    assert!((({ DerivedImpl::not_inlined_out_of_line(&d.as_pointer(),) }) == 18));
    assert!((({ Derived::not_inlined_static_out_of_line(2,) }) == 14));
    return 0;
}
impl Base for Derived {
    #[inline]
    fn inlined_override(&self, mut a: i32) -> i32 {
        return ({ self.v_ } + a);
    }
    fn not_inlined_override(&self, mut a: i32) -> i32 {
        return ({ self.v_ } - a);
    }
    fn not_inlined_override(&self, mut a: i32) -> i32 {
        return ({ self.v_ } - a);
    }
}
pub trait DerivedImpl {
    fn inlined_in_class(&self) -> i32;
    fn inlined_declared_in_class(&self) -> i32;
    fn inlined_defined_out_of_line(&self) -> i32;
    fn not_inlined_out_of_line(&self) -> i32;
}
impl DerivedImpl for Ptr<Derived> {
    #[inline]
    fn inlined_in_class(&self) -> i32 {
        return ((*self).with(|__s| __s.v_) * 2);
    }
    #[inline]
    fn inlined_declared_in_class(&self) -> i32 {
        return ((*self).with(|__s| __s.v_) * 4);
    }
    #[inline]
    fn inlined_defined_out_of_line(&self) -> i32 {
        return ((*self).with(|__s| __s.v_) * 5);
    }
    fn not_inlined_out_of_line(&self) -> i32 {
        return ((*self).with(|__s| __s.v_) * 6);
    }
}
pub fn __cpp2rust_init_globals() {}
