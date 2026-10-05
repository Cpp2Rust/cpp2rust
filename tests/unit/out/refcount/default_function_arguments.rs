extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn foo_0(mut a: i32, b: Option<i32>) -> i32 {
    let mut b: i32 = b.unwrap_or(10);
    return (a + b);
}
pub fn baz_1(mut a: Ptr<i32>, b: Option<Ptr<i32>>) -> bool {
    let mut b: Ptr<i32> = b.unwrap_or(Ptr::<i32>::null());
    return ({ (a).clone() } == { (b).clone() });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct Bar {
    #[offset(0)]
    pub v: i32,
}
impl Bar {
    pub fn new(v: Option<i32>) -> Self {
        let mut v: i32 = v.unwrap_or(1);
        Self { v: v }
    }
}
impl Default for Bar {
    fn default() -> Self {
        { Bar::new(None) }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((({ foo_0(1, None,) }) == 11));
    assert!((({ foo_0(1, Some(2),) }) == 3));
    let a: Value<i32> = Rc::new(RefCell::new(0));
    assert!(((({ baz_1((a.as_pointer()), None,) }) as i32) == (false as i32)));
    assert!(
        ((({
            let _a: Ptr<i32> = (a.as_pointer());
            let _b: Ptr<i32> = (a.as_pointer());
            baz_1(_a, Some(_b))
        }) as i32)
            == (true as i32))
    );
    let mut b: Bar = Bar::new(None);
    assert!((b.v == 1));
    assert!(({ Bar::new({ Some(2) },).v } == 2));
    let mut arr: [Bar; 3] = [Bar::new(None), Bar::new(None), Bar::new(None)];
    assert!(({ arr[(0) as usize].v } == 1));
    assert!(({ arr[(2) as usize].v } == 1));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
