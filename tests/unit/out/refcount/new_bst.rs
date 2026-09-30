extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(24)]
pub struct node_t {
    #[offset(0)]
    #[byte_size(8)]
    pub left: Ptr<node_t>,
    #[offset(8)]
    #[byte_size(8)]
    pub right: Ptr<node_t>,
    #[offset(16)]
    pub value: i32,
}
pub fn find_0(node: Ptr<node_t>, value: i32) -> Ptr<node_t> {
    let node: Value<Ptr<node_t>> = Rc::new(RefCell::new(node));
    let value: Value<i32> = Rc::new(RefCell::new(value));
    if ({
        let _lhs = (*value.borrow());
        _lhs < (*node.borrow()).with(|__s| __s.value)
    }) && (!(((*node.borrow()).with(|__s| __s.left.clone())).is_null()))
    {
        return ({
            find_0(
                (*node.borrow()).with(|__s| __s.left.clone()),
                (*value.borrow()),
            )
        });
    } else if ({
        let _lhs = (*value.borrow());
        _lhs > (*node.borrow()).with(|__s| __s.value)
    }) && (!(((*node.borrow()).with(|__s| __s.right.clone())).is_null()))
    {
        return ({
            find_0(
                (*node.borrow()).with(|__s| __s.right.clone()),
                (*value.borrow()),
            )
        });
    } else if {
        let _lhs = (*value.borrow());
        _lhs == (*node.borrow()).with(|__s| __s.value)
    } {
        return (*node.borrow()).clone();
    }
    return Ptr::<node_t>::null();
}
pub fn insert_1(node: Ptr<node_t>, value: i32) -> Ptr<node_t> {
    let node: Value<Ptr<node_t>> = Rc::new(RefCell::new(node));
    let value: Value<i32> = Rc::new(RefCell::new(value));
    if (*node.borrow()).is_null() {
        return Ptr::alloc(node_t {
            left: Ptr::<node_t>::null(),
            right: Ptr::<node_t>::null(),
            value: (*value.borrow()),
        });
    }
    if {
        let _lhs = (*value.borrow());
        _lhs < (*node.borrow()).with(|__s| __s.value)
    } {
        let __rhs = ({
            insert_1(
                (*node.borrow()).with(|__s| __s.left.clone()),
                (*value.borrow()),
            )
        });
        field!((*node.borrow()), left).write(__rhs);
    } else if {
        let _lhs = (*value.borrow());
        _lhs > (*node.borrow()).with(|__s| __s.value)
    } {
        let __rhs = ({
            insert_1(
                (*node.borrow()).with(|__s| __s.right.clone()),
                (*value.borrow()),
            )
        });
        field!((*node.borrow()), right).write(__rhs);
    }
    return (*node.borrow()).clone();
}
pub fn del_2(node: Ptr<node_t>) {
    let node: Value<Ptr<node_t>> = Rc::new(RefCell::new(node));
    if !(((*node.borrow()).with(|__s| __s.left.clone())).is_null()) {
        ({ del_2((*node.borrow()).with(|__s| __s.left.clone())) });
    }
    if !(((*node.borrow()).with(|__s| __s.right.clone())).is_null()) {
        ({ del_2((*node.borrow()).with(|__s| __s.right.clone())) });
    }
    (*node.borrow()).delete();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let root: Value<Ptr<node_t>> = Rc::new(RefCell::new(Ptr::alloc(node_t {
        left: Ptr::<node_t>::null(),
        right: Ptr::<node_t>::null(),
        value: 0,
    })));
    let __rhs = ({ insert_1((*root.borrow()).clone(), 1) });
    (*root.borrow_mut()) = __rhs;
    let __rhs = ({ insert_1((*root.borrow()).clone(), 2) });
    (*root.borrow_mut()) = __rhs;
    let __rhs = ({ insert_1((*root.borrow()).clone(), 3) });
    (*root.borrow_mut()) = __rhs;
    let __rhs = ({ insert_1((*root.borrow()).clone(), 4) });
    (*root.borrow_mut()) = __rhs;
    let out: Value<bool> = Rc::new(RefCell::new(
        (((((({ find_0((*root.borrow()).clone(), 0) }).with(|__s| __s.value) == 0)
            && (({ find_0((*root.borrow()).clone(), 1) }).with(|__s| __s.value) == 1))
            && (({ find_0((*root.borrow()).clone(), 2) }).with(|__s| __s.value) == 2))
            && (({ find_0((*root.borrow()).clone(), 3) }).with(|__s| __s.value) == 3))
            && (({ find_0((*root.borrow()).clone(), 4) }).with(|__s| __s.value) == 4))
            && (({ find_0((*root.borrow()).clone(), 5) }).is_null()),
    ));
    ({ del_2((*root.borrow()).clone()) });
    assert!((*out.borrow()));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
