extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(4)]
pub struct Tracker {
    #[offset(0)]
    pub tag: i32,
}
impl Tracker {
    pub fn new(mut tag: i32) -> Self {
        let __this: Tracker = Self { tag: tag };
        println!("construct {}", tag);
        __this
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(4)]
pub struct HoldsTracker {
    #[offset(0)]
    #[byte_size(4)]
    pub member: Tracker,
}
#[derive(Copy, Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct TriviallyDestructible {
    #[offset(0)]
    pub value: i32,
}
thread_local!(
    pub static global_0: Value<Tracker> = {
        let __static = Rc::new(RefCell::new(Tracker::new({ 1 })));
        libcc2rs::register_static_destructor(|| {
            global_0.with(Value::clone).as_pointer().destructor()
        });
        __static
    };
);
pub fn never_called_1() {
    ({ TrackerImpl::touch(&never_constructed_2.with(|v| v.as_pointer())) });
}
pub fn test_3() {
    ({ TrackerImpl::touch(&global_0.with(|v| v.as_pointer())) });
}
thread_local!(
    static never_constructed_2: Value<Tracker> = {
        let __static = Rc::new(RefCell::new(Tracker::new({ 100 })));
        libcc2rs::register_static_destructor(|| {
            never_constructed_2
                .with(Value::clone)
                .as_pointer()
                .destructor()
        });
        __static
    };
);
pub fn local_static_during_global_init_4() -> Ptr<Tracker> {
    return during_init_5.with(|v| v.as_pointer());
}
thread_local!(
    pub static before_6: Value<Tracker> = {
        let __static = Rc::new(RefCell::new(Tracker::new({ 2 })));
        libcc2rs::register_static_destructor(|| {
            before_6.with(Value::clone).as_pointer().destructor()
        });
        __static
    };
);
thread_local!(
    pub static after_7: Value<Tracker> = {
        let __static = Rc::new(RefCell::new(Tracker::new({
            (({ local_static_during_global_init_4() }).with(|__s| __s.tag) + 1)
        })));
        libcc2rs::register_static_destructor(|| {
            after_7.with(Value::clone).as_pointer().destructor()
        });
        __static
    };
);
thread_local!(
    pub static holder_8: Value<HoldsTracker> = {
        let __static = Rc::new(RefCell::new(HoldsTracker {
            member: Tracker::new({ 5 }),
        }));
        libcc2rs::register_static_destructor(|| {
            holder_8.with(Value::clone).as_pointer().destructor()
        });
        __static
    };
);
thread_local!(
    pub static trivial_9: Value<TriviallyDestructible> =
        Rc::new(RefCell::new(TriviallyDestructible { value: 6 }));
);
#[derive(Copy, Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct p3_reverse_completion_order_WithStaticMember {}
thread_local!(
    pub static member_10: Value<Tracker> = {
        let __static = Rc::new(RefCell::new(Tracker::new({ 7 })));
        libcc2rs::register_static_destructor(|| {
            member_10.with(Value::clone).as_pointer().destructor()
        });
        __static
    };
);
thread_local!(
    pub static capturing_11: Value<FnPtr<fn() -> i32>> = {
        let __static = Rc::new(RefCell::new(lambda!(
            {
                let captured: Value<Tracker> = Rc::new(RefCell::new(Tracker::new({ 8 })));
            },
            || -> i32 {
                return { (*captured.borrow()).tag };
            }
        )));
        libcc2rs::register_static_destructor(|| {
            capturing_11
                .with(Value::clone)
                .as_pointer()
                .with(|__f| __f.destroy())
        });
        __static
    };
);
pub fn run_local_static_12() {
    ({ TrackerImpl::touch(&during_main_13.with(|v| v.as_pointer())) });
}
pub fn test_14() {
    ({ run_local_static_12() });
    ({ run_local_static_12() });
    ({ TrackerImpl::touch(&({ local_static_during_global_init_4() })) });
    println!("trivial {}", { trivial_9.with(|rc| *rc.borrow()).value });
    ({ TrackerImpl::touch(&member_10.with(|v| v.as_pointer())) });
    println!(
        "captured {}",
        ({ (*capturing_11.with(Value::clone).borrow()).call() })
    );
}
thread_local!(
    static during_init_5: Value<Tracker> = {
        let __static = Rc::new(RefCell::new(Tracker::new({ 3 })));
        libcc2rs::register_static_destructor(|| {
            during_init_5.with(Value::clone).as_pointer().destructor()
        });
        __static
    };
);
thread_local!(
    static during_main_13: Value<Tracker> = {
        let __static = Rc::new(RefCell::new(Tracker::new({ 9 })));
        libcc2rs::register_static_destructor(|| {
            during_main_13.with(Value::clone).as_pointer().destructor()
        });
        __static
    };
);
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    ({ test_3() });
    ({ test_14() });
    println!("return from main");
    return 0;
}
pub trait HoldsTrackerImpl {
    fn destructor(&self);
}
impl HoldsTrackerImpl for Ptr<HoldsTracker> {
    fn destructor(&self) {
        field_ptr!(self, member).destructor();
    }
}
pub trait TrackerImpl {
    fn destructor(&self);
    fn touch(&self);
}
impl TrackerImpl for Ptr<Tracker> {
    fn destructor(&self) {
        println!("destroy {}", (*self).with(|__s| __s.tag));
    }
    fn touch(&self) {
        println!("touch {}", (*self).with(|__s| __s.tag));
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = global_0.with(|_| ());
    let _ = before_6.with(|_| ());
    let _ = after_7.with(|_| ());
    let _ = holder_8.with(|_| ());
    let _ = trivial_9.with(|_| ());
    let _ = member_10.with(|_| ());
    let _ = capturing_11.with(|_| ());
}
