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
    pub fn reset() {
        ctorCount_0.with(|rc| {
            *rc.borrow_mut() = {
                copyCount_1.with(|rc| {
                    *rc.borrow_mut() = {
                        dtorCount_2.with(|rc| {
                            *rc.borrow_mut() = {
                                useCount_3.with(|rc| *rc.borrow_mut() = 0);
                                useCount_3.with(|rc| *rc.borrow())
                            }
                        });
                        dtorCount_2.with(|rc| *rc.borrow())
                    }
                });
                copyCount_1.with(|rc| *rc.borrow())
            }
        });
    }
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(4)]
pub struct AssertsDtorCountAtExit {
    #[offset(0)]
    pub expected: i32,
}
thread_local!(
    pub static check_4: Value<AssertsDtorCountAtExit> = {
        let __static = Rc::new(RefCell::new(AssertsDtorCountAtExit { expected: 2 }));
        libcc2rs::register_static_destructor(|| {
            check_4.with(Value::clone).as_pointer().destructor()
        });
        __static
    };
);
thread_local!(
    pub static shared_5: Value<LifetimeTracked> = {
        let __static = Rc::new(RefCell::new(LifetimeTracked::new()));
        libcc2rs::register_static_destructor(|| {
            shared_5.with(Value::clone).as_pointer().destructor()
        });
        __static
    };
);
#[derive(Copy, Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct HasInlineMembers {}
impl HasInlineMembers {
    pub fn instance() -> Ptr<LifetimeTracked> {
        return local_6.with(|v| v.as_pointer());
    }
}
thread_local!(
    static local_6: Value<LifetimeTracked> = {
        let __static = Rc::new(RefCell::new(LifetimeTracked::new()));
        libcc2rs::register_static_destructor(|| {
            local_6.with(Value::clone).as_pointer().destructor()
        });
        __static
    };
);
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    ({ LifetimeTrackedImpl::use_(&shared_5.with(|v| v.as_pointer())) });
    ({ LifetimeTrackedImpl::use_(&({ HasInlineMembers::instance() })) });
    ({ b_use_7() });
    assert!((ctorCount_0.with(|rc| *rc.borrow()) == 2));
    assert!(({ ({ (*shared_5.with(Value::clone).borrow()).uses } == 2) }));
    assert!((({ HasInlineMembers::instance() }).with(|__s| __s.uses) == 2));
    assert!(((shared_5.with(|v| v.as_pointer())) == ({ b_shared_address_8() })));
    assert!((({ HasInlineMembers::instance() }) == ({ b_instance_address_9() })));
    assert!((copyCount_1.with(|rc| *rc.borrow()) == 0));
    return 0;
}
pub fn b_use_7() {
    ({ LifetimeTrackedImpl::use_(&shared_5.with(|v| v.as_pointer())) });
    ({ LifetimeTrackedImpl::use_(&({ HasInlineMembers::instance() })) });
}
pub fn b_shared_address_8() -> Ptr<LifetimeTracked> {
    return (shared_5.with(|v| v.as_pointer()));
}
pub fn b_instance_address_9() -> Ptr<LifetimeTracked> {
    return ({ HasInlineMembers::instance() });
}
pub trait AssertsDtorCountAtExitImpl {
    fn destructor(&self);
}
impl AssertsDtorCountAtExitImpl for Ptr<AssertsDtorCountAtExit> {
    fn destructor(&self) {
        assert!((dtorCount_2.with(|rc| *rc.borrow()) == (*self).with(|__s| __s.expected)));
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
        field!((*self), uses).with_mut(|__v| {
            __v.prefix_inc();
        });
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
    let _ = check_4.with(|_| ());
    let _ = shared_5.with(|_| ());
}
