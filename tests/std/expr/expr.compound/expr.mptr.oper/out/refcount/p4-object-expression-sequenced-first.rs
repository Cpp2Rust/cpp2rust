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
pub struct HasValue {
    #[offset(0)]
    pub value: i32,
}
thread_local!(
    pub static sequence_0: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static object_stamp_1: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static member_stamp_2: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static object_3: Value<HasValue> = Rc::new(RefCell::new(HasValue { value: 7 }));
);
pub fn select_object_4() -> Ptr<HasValue> {
    object_stamp_1
        .with(|rc| *rc.borrow_mut() = (*sequence_0.with(Value::clone).borrow_mut()).prefix_inc());
    return object_3.with(|v| v.as_pointer());
}
pub fn select_member_5() -> isize {
    member_stamp_2
        .with(|rc| *rc.borrow_mut() = (*sequence_0.with(Value::clone).borrow_mut()).prefix_inc());
    return (<HasValue as Record>::OFFSETS.value as isize);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(
        ((({ select_object_4() })
            .member::<i32>(({ select_member_5() }))
            .read())
            == 7)
    );
    assert!((object_stamp_1.with(|rc| *rc.borrow()) == 1));
    assert!((member_stamp_2.with(|rc| *rc.borrow()) == 2));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = sequence_0.with(|_| ());
    let _ = object_stamp_1.with(|_| ());
    let _ = member_stamp_2.with(|_| ());
    let _ = object_3.with(|_| ());
}
