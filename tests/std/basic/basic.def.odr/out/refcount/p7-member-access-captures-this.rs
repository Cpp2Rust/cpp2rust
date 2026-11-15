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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor)]
#[byte_size(8)]
pub struct HasMemberUsedInLambda {
    #[offset(0)]
    #[byte_size(4)]
    pub x: LifetimeTracked,
    #[offset(4)]
    pub n: i32,
}
impl Default for HasMemberUsedInLambda {
    fn default() -> Self {
        {
            HasMemberUsedInLambda {
                x: <LifetimeTracked>::default(),
                n: 1,
            }
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let c: Value<HasMemberUsedInLambda> = Rc::new(RefCell::new(<HasMemberUsedInLambda>::default()));
    let _dtor_c = std::mem::DropGuard::new(c.clone(), |__p| __p.as_pointer().destructor());
    let f: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new(
        ({ HasMemberUsedInLambdaImpl::makeLambda(&c.as_pointer()) }),
    ));
    assert!((copyCount_1.with(|rc| *rc.borrow()) == 0));
    assert!(({ (({ (*f.borrow()).call() }) == 1) }));
    assert!(({ ({ (*c.borrow()).x.uses } == 1) }));
    (*c.borrow_mut()).n = 10;
    assert!(({ (({ (*f.borrow()).call() }) == 10) }));
    assert!(({ ({ (*c.borrow()).x.uses } == 2) }));
    assert!((copyCount_1.with(|rc| *rc.borrow()) == 0));
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
pub trait HasMemberUsedInLambdaImpl {
    fn makeLambda(&self) -> FnPtr<fn() -> i32>;
    fn copy_assign(&self, _a0: Ptr<HasMemberUsedInLambda>) -> Ptr<HasMemberUsedInLambda>;
    fn destructor(&self);
}
impl HasMemberUsedInLambdaImpl for Ptr<HasMemberUsedInLambda> {
    fn makeLambda(&self) -> FnPtr<fn() -> i32> {
        return lambda!(
            {
                let this_: Value<Ptr<HasMemberUsedInLambda>> =
                    Rc::new(RefCell::new((*self).clone()));
            },
            || -> i32 {
                ({ LifetimeTrackedImpl::use_(&field_ptr!((*this_.borrow()), x)) });
                return (*this_.borrow()).with(|__s| __s.n);
            }
        );
    }
    fn copy_assign(&self, _a0: Ptr<HasMemberUsedInLambda>) -> Ptr<HasMemberUsedInLambda> {
        ({
            let _arg0: Ptr<LifetimeTracked> = field_ptr!(_a0, x);
            LifetimeTrackedImpl::copy_assign(&field_ptr!((*self), x), _arg0)
        });
        field!((*self), n).write({ _a0.with(|__s| __s.n) });
        return (*self).clone();
    }
    fn destructor(&self) {
        field_ptr!(self, x).destructor();
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
