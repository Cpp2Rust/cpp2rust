extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static copies_0: Value<i32> = Rc::new(RefCell::new(0_i32));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CppFrom, CopyAssign, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(8)]
pub struct HasMutableMember {
    #[offset(0)]
    pub value: i32,
    #[offset(4)]
    pub scratch: i32,
}
impl HasMutableMember {
    pub fn new(mut v: i32) -> Self {
        Self {
            value: v,
            scratch: 0,
        }
    }
    pub fn copy_from(o: Ptr<HasMutableMember>) -> Self {
        let __this: HasMutableMember = Self {
            value: o.with(|__s| __s.value),
            scratch: o.with(|__s| __s.scratch),
        };
        (*copies_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Clone for HasMutableMember {
    fn clone(&self) -> Self {
        let __src: Value<HasMutableMember> = Rc::new(RefCell::new(HasMutableMember {
            value: self.value.clone(),
            scratch: self.scratch.clone(),
        }));
        HasMutableMember::copy_from(__src.as_pointer())
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let k: Value<HasMutableMember> = Rc::new(RefCell::new(HasMutableMember::new({ 5 })));
    let f: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let k: Value<HasMutableMember> = Rc::new(RefCell::new(HasMutableMember::copy_from({
                k.as_pointer()
            })));
        },
        || -> i32 {
            return { (*k.borrow()).value };
        }
    )));
    assert!((copies_0.with(|rc| *rc.borrow()) == 1));
    assert!(({ (({ (*f.borrow()).call() }) == 5) }));
    assert!((copies_0.with(|rc| *rc.borrow()) == 1));
    return 0;
}
pub trait HasMutableMemberImpl {
    fn copy_assign(&self, _a0: Ptr<HasMutableMember>) -> Ptr<HasMutableMember>;
}
impl HasMutableMemberImpl for Ptr<HasMutableMember> {
    fn copy_assign(&self, _a0: Ptr<HasMutableMember>) -> Ptr<HasMutableMember> {
        field!((*self), value).write({ _a0.with(|__s| __s.value) });
        field!((*self), scratch).write({ _a0.with(|__s| __s.scratch) });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = copies_0.with(|_| ());
}
