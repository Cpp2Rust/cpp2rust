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
    match (Rc::new(RefCell::new(1)).as_pointer(),) {
        (__tmp_0,) => {
            ({ (*m.borrow_mut()).subscript_move(__tmp_0, |__k| cpp_move!(__k), || 0_i32) })
        }
    }
    .write(2);
    {
        (*n.borrow_mut()) = ({ std::mem::take(&mut (*m.borrow_mut())) });
    };
    assert!(
        ((match (Rc::new(RefCell::new(1)).as_pointer(),) {
            (__tmp_1,) => {
                ({ (*n.borrow_mut()).subscript_move(__tmp_1, |__k| cpp_move!(__k), || 0_i32) })
            }
        }
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
    match (Rc::new(RefCell::new(1)).as_pointer(),) {
        (__tmp_2,) => {
            ({ (*m.borrow_mut()).subscript_move(__tmp_2, |__k| cpp_move!(__k), || 0_i32) })
        }
    }
    .write(2);
    assert!(
        ((match (Rc::new(RefCell::new(1)).as_pointer(),) {
            (__tmp_3,) => {
                ({ (*m.borrow_mut()).subscript_move(__tmp_3, |__k| cpp_move!(__k), || 0_i32) })
            }
        }
        .read())
            == 2)
    );
    assert!(({ ((*m.borrow()).len() == 1_usize) }));
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
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
