extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct p18_rebind_to_container_node_Alloc_std_pair_const_int__int__ {}
pub fn test_0() {
    let m: Value<Map<i32, (Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(Map::new()));
    Map::subscript_move(
        m.as_pointer(),
        Rc::new(RefCell::new(1)).as_pointer(),
        |__k| cpp_move!(__k),
    )
    .write(2);
    assert!(
        ((Map::subscript_move(
            m.as_pointer(),
            Rc::new(RefCell::new(1)).as_pointer(),
            |__k| cpp_move!(__k)
        )
        .read())
            == 2)
    );
    assert!(({ ((*m.borrow()).len() == 1_u64) }));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test_0() });
    return 0;
}
pub trait p18_rebind_to_container_node_Alloc_std_pair_const_int__int__Impl {
    fn allocate(&self, mut n: usize) -> Ptr<(Value<i32>, Value<i32>)> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<(Value<i32>, Value<i32>)>, mut n: usize) {
        unimplemented!()
    }
}
impl p18_rebind_to_container_node_Alloc_std_pair_const_int__int__Impl
    for Ptr<p18_rebind_to_container_node_Alloc_std_pair_const_int__int__>
{
}
pub fn __cpp2rust_init_globals() {}
