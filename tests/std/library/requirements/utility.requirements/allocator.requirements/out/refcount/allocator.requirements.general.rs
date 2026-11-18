extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Copy, Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct p16_rebind_member_Alloc_std_pair_const_int__int___rebind_std_pair_const_int__int__ {}
#[derive(Copy, Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct p16_rebind_member_Alloc_std_pair_const_int__int__ {
    #[offset(0)]
    pub id: i32,
}
impl Default for p16_rebind_member_Alloc_std_pair_const_int__int__ {
    fn default() -> Self {
        { p16_rebind_member_Alloc_std_pair_const_int__int__ { id: 0 } }
    }
}
pub fn test_0() {
    let m: Value<Map<i32, (Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(Map::new()));
    let n: Value<Map<i32, (Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(Map::new()));
    Map::subscript_move(
        m.as_pointer(),
        Rc::new(RefCell::new(1)).as_pointer(),
        |__k| cpp_move!(__k),
    )
    .write(2);
    {
        let __src = m
            .as_pointer()
            .with_mut(|__m: &mut Map<i32, (Value<i32>, Value<i32>)>| std::mem::take(__m));
        n.as_pointer().write(__src)
    };
    assert!(
        ((Map::subscript_move(
            n.as_pointer(),
            Rc::new(RefCell::new(1)).as_pointer(),
            |__k| cpp_move!(__k)
        )
        .read())
            == 2)
    );
    assert!(({ ((*n.borrow()).len() == 1_usize) }));
}
#[derive(Copy, Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct p18_rebind_to_container_node_Alloc_std_pair_const_int__int__ {}
pub fn test_1() {
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
    assert!(({ ((*m.borrow()).len() == 1_usize) }));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test_0() });
    ({ test_1() });
    return 0;
}
pub trait p16_rebind_member_Alloc_std_pair_const_int__int__Impl {
    fn allocate(&self, mut n: usize) -> Ptr<(Value<i32>, Value<i32>)> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<(Value<i32>, Value<i32>)>, mut n: usize) {
        unimplemented!()
    }
}
impl p16_rebind_member_Alloc_std_pair_const_int__int__Impl
    for Ptr<p16_rebind_member_Alloc_std_pair_const_int__int__>
{
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
