extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Inner {
    #[offset(0)]
    pub value: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Outer {
    #[offset(0)]
    #[byte_size(8)]
    pub p: Ptr<Inner>,
}
thread_local!(
    pub static alpha_0: Value<Inner> = Rc::new(RefCell::new(Inner { value: 1 }));
);
thread_local!(
    pub static beta_1: Value<Inner> = Rc::new(RefCell::new(Inner { value: 2 }));
);
thread_local!(
    pub static shared_2: Value<Inner> = Rc::new(RefCell::new(Inner { value: 42 }));
);
thread_local!(
    pub static items_3: Value<Box<[Ptr<Inner>]>> = Rc::new(RefCell::new(Box::new([
        (alpha_0.with(|v| v.as_pointer())),
        (beta_1.with(|v| v.as_pointer())),
    ])));
);
thread_local!(
    pub static obj_4: Value<Outer> = Rc::new(RefCell::new(Outer {
        p: (shared_2.with(|v| v.as_pointer())),
    }));
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(
        (({
            let __idx = (0) as usize;
            items_3.with(|rc| rc.borrow()[__idx].clone())
        })
        .with(|__s| __s.value)
            == 1)
    );
    assert!(
        (({
            let __idx = (1) as usize;
            items_3.with(|rc| rc.borrow()[__idx].clone())
        })
        .with(|__s| __s.value)
            == 2)
    );
    assert!(({ (*obj_4.with(Value::clone).borrow()).p.clone() }.with(|__s| __s.value) == 42));
    thread_local!(
        static cache_5: Value<Box<[Ptr<Inner>]>> = Rc::new(RefCell::new(Box::new([
            (alpha_0.with(|v| v.as_pointer())),
            (beta_1.with(|v| v.as_pointer())),
        ])));
    );
    assert!(
        (({
            let __idx = (0) as usize;
            cache_5.with(|rc| rc.borrow()[__idx].clone())
        })
        .with(|__s| __s.value)
            == 1)
    );
    assert!(
        (({
            let __idx = (1) as usize;
            cache_5.with(|rc| rc.borrow()[__idx].clone())
        })
        .with(|__s| __s.value)
            == 2)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = alpha_0.with(|_| ());
    let _ = beta_1.with(|_| ());
    let _ = shared_2.with(|_| ());
    let _ = items_3.with(|_| ());
    let _ = obj_4.with(|_| ());
}
