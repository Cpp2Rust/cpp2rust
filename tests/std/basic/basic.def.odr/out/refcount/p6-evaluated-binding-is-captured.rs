extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static ctorCount_0: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static copyCount_1: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static dtorCount_2: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static useCount_3: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor)]
#[byte_size(4)]
pub struct LifetimeTracked {
    #[offset(0)]
    pub uses: i32,
}
impl LifetimeTracked {
    pub fn new() -> Self {
        let __this: LifetimeTracked = Self { uses: 0 };
        (*ctorCount_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn copy_from(_a0: Ptr<LifetimeTracked>) -> Self {
        let __this: LifetimeTracked = Self { uses: 0 };
        (*copyCount_1.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Clone for LifetimeTracked {
    fn clone(&self) -> Self {
        let __src: Value<LifetimeTracked> = Rc::new(RefCell::new(LifetimeTracked {
            uses: self.uses.clone(),
        }));
        LifetimeTracked::copy_from(__src.as_pointer())
    }
}
impl Default for LifetimeTracked {
    fn default() -> Self {
        { LifetimeTracked::new() }
    }
}
thread_local!(
    pub static copyCount_4: Value<i32> = Rc::new(RefCell::new(0));
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
        (*copyCount_4.with(Value::clone).borrow_mut()).prefix_inc();
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(4)]
pub struct AssertsDtorCountAtExit {
    #[offset(0)]
    pub expected: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor, Default)]
#[byte_size(8)]
pub struct HasTwoMembers {
    #[offset(0)]
    #[byte_size(4)]
    pub a: LifetimeTracked,
    #[offset(4)]
    #[byte_size(4)]
    pub b: LifetimeTracked,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let p: Value<HasTwoMembers> = Rc::new(RefCell::new(HasTwoMembers {
        a: LifetimeTracked::new(),
        b: LifetimeTracked::new(),
    }));
    let _dtor_p = std::mem::DropGuard::new(p.clone(), |__p| __p.as_pointer().destructor());
    assert!((ctorCount_0.with(|rc| *rc.borrow()) == 2));
    let __decomp_5: Ptr<HasTwoMembers> = p.as_pointer();
    assert!((copyCount_1.with(|rc| *rc.borrow()) == 0));
    let f: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
        {
            let a: Value<LifetimeTracked> = Rc::new(RefCell::new(LifetimeTracked::copy_from({
                field_ptr!(__decomp_5, a)
            })));
        },
        || {
            ({ LifetimeTrackedImpl::use_(&field_ptr!(__decomp_5, a)) });
        }
    )));
    let _dtor_f =
        std::mem::DropGuard::new(f.clone(), |__p| __p.as_pointer().with(|__f| __f.destroy()));
    assert!((copyCount_1.with(|rc| *rc.borrow()) == 1));
    ({ (*f.borrow()).call() });
    assert!((copyCount_1.with(|rc| *rc.borrow()) == 1));
    assert!((useCount_3.with(|rc| *rc.borrow()) == 1));
    assert!(({ ({ (*p.borrow()).a.uses } == 0) }));
    return 0;
}
pub trait AssertsDtorCountAtExitImpl {
    fn destructor(&self);
}
impl AssertsDtorCountAtExitImpl for Ptr<AssertsDtorCountAtExit> {
    fn destructor(&self) {
        assert!((dtorCount_2.with(|rc| *rc.borrow()) == (*self).with(|__s| __s.expected)));
    }
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
pub trait HasTwoMembersImpl {
    fn copy_assign(&self, _a0: Ptr<HasTwoMembers>) -> Ptr<HasTwoMembers>;
    fn destructor(&self);
}
impl HasTwoMembersImpl for Ptr<HasTwoMembers> {
    fn copy_assign(&self, _a0: Ptr<HasTwoMembers>) -> Ptr<HasTwoMembers> {
        ({
            let _arg0: Ptr<LifetimeTracked> = field_ptr!(_a0, a);
            LifetimeTrackedImpl::copy_assign(&field_ptr!((*self), a), _arg0)
        });
        ({
            let _arg0: Ptr<LifetimeTracked> = field_ptr!(_a0, b);
            LifetimeTrackedImpl::copy_assign(&field_ptr!((*self), b), _arg0)
        });
        return (*self).clone();
    }
    fn destructor(&self) {
        field_ptr!(self, b).destructor();
        field_ptr!(self, a).destructor();
    }
}
pub trait LifetimeTrackedImpl {
    fn destructor(&self);
    fn use_(&self);
    fn copy_assign(&self, _a0: Ptr<LifetimeTracked>) -> Ptr<LifetimeTracked>;
}
impl LifetimeTrackedImpl for Ptr<LifetimeTracked> {
    fn destructor(&self) {
        (*dtorCount_2.with(Value::clone).borrow_mut()).prefix_inc();
    }
    fn use_(&self) {
        field!((*self), uses).with_mut(|__v| __v.prefix_inc());
        (*useCount_3.with(Value::clone).borrow_mut()).prefix_inc();
    }
    fn copy_assign(&self, _a0: Ptr<LifetimeTracked>) -> Ptr<LifetimeTracked> {
        field!((*self), uses).write({ _a0.with(|__s| __s.uses) });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = ctorCount_0.with(|_| ());
    let _ = copyCount_1.with(|_| ());
    let _ = dtorCount_2.with(|_| ());
    let _ = useCount_3.with(|_| ());
    let _ = copyCount_4.with(|_| ());
}
