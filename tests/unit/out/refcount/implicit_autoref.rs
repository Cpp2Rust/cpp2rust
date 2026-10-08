extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(24)]
pub struct Holder {
    #[offset(0)]
    #[byte_size(24)]
    pub v: Value<Vec<i32>>,
}
pub fn write_through_0(mut p: Ptr<i32>) {
    p.write(42);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let v: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    {
        let __a1 = 10;
        (*v.borrow_mut()).push(__a1)
    };
    {
        let __a1 = 20;
        (*v.borrow_mut()).push(__a1)
    };
    let mut p: Ptr<Vec<i32>> = (v.as_pointer());
    let mut a: i32 = (elem!(((Ptr::<Vec<i32>>::decay(&(p))) as Ptr<i32>), 0_usize).read());
    elem!(((Ptr::<Vec<i32>>::decay(&(p))) as Ptr<i32>), 1_usize).write(30);
    let h: Value<Holder> = Rc::new(RefCell::new(<Holder>::default()));
    {
        let __a1 = 40;
        (*{ (*h.borrow()).v.clone() }.borrow_mut()).push(__a1)
    };
    {
        let __a1 = 50;
        (*{ (*h.borrow()).v.clone() }.borrow_mut()).push(__a1)
    };
    let mut hp: Ptr<Holder> = (h.as_pointer());
    let mut b: i32 = (elem!((hp.with(|__s| (__s).v.as_pointer()) as Ptr<i32>), 0_usize).read());
    elem!((hp.with(|__s| (__s).v.as_pointer()) as Ptr<i32>), 1_usize).write(60);
    assert!((a == 10));
    assert!(((elem!(((Ptr::<Vec<i32>>::decay(&(p))) as Ptr<i32>), 1_usize).read()) == 30));
    assert!((b == 40));
    assert!(((elem!((hp.with(|__s| (__s).v.as_pointer()) as Ptr<i32>), 1_usize).read()) == 60));
    ({ write_through_0(((Ptr::<Vec<i32>>::decay(&(p)) as Ptr<i32>).offset(0_usize as isize))) });
    assert!(((elem!(((Ptr::<Vec<i32>>::decay(&(p))) as Ptr<i32>), 0_usize).read()) == 42));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
