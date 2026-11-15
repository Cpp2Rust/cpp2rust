extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static copyCount_0: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CppFrom, CopyAssign, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct HasConstexprCtor {
    #[offset(0)]
    pub m: i32,
}
impl HasConstexprCtor {
    pub fn new(mut v: i32) -> Self {
        Self { m: v }
    }
    pub fn copy_from(o: Ptr<HasConstexprCtor>) -> Self {
        let __this: HasConstexprCtor = Self {
            m: o.with(|__s| __s.m),
        };
        (*copyCount_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Clone for HasConstexprCtor {
    fn clone(&self) -> Self {
        let __src: Value<HasConstexprCtor> =
            Rc::new(RefCell::new(HasConstexprCtor { m: self.m.clone() }));
        HasConstexprCtor::copy_from(__src.as_pointer())
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let k: Value<HasConstexprCtor> = Rc::new(RefCell::new(HasConstexprCtor::new({ 5 })));
    let f: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let k: Value<HasConstexprCtor> = Rc::new(RefCell::new(HasConstexprCtor::copy_from({
                k.as_pointer()
            })));
        },
        || -> i32 {
            let mut c: HasConstexprCtor = HasConstexprCtor::copy_from({ k.as_pointer() });
            return c.m;
        }
    )));
    assert!((copyCount_0.with(|rc| *rc.borrow()) == 1));
    assert!(({ (({ (*f.borrow()).call() }) == 5) }));
    assert!((copyCount_0.with(|rc| *rc.borrow()) == 2));
    return 0;
}
pub trait HasConstexprCtorImpl {
    fn copy_assign(&self, _a0: Ptr<HasConstexprCtor>) -> Ptr<HasConstexprCtor>;
}
impl HasConstexprCtorImpl for Ptr<HasConstexprCtor> {
    fn copy_assign(&self, _a0: Ptr<HasConstexprCtor>) -> Ptr<HasConstexprCtor> {
        field!((*self), m).write({ _a0.with(|__s| __s.m) });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = copyCount_0.with(|_| ());
}
