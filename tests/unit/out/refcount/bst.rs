extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct node_t {
    pub left: Value<Ptr<node_t>>,
    pub right: Value<Ptr<node_t>>,
    pub value: Value<i32>,
}
impl Clone for node_t {
    fn clone(&self) -> Self {
        let mut this = Self {
            left: Rc::new(RefCell::new((*self.left.borrow()).clone())),
            right: Rc::new(RefCell::new((*self.right.borrow()).clone())),
            value: Rc::new(RefCell::new((*self.value.borrow()))),
        };
        this
    }
}
impl ByteRepr for node_t {}
pub fn find_0(node: Ptr<node_t>, value: i32) -> Ptr<node_t> {
    let node: Value<Ptr<node_t>> = Rc::new(RefCell::new(node));
    let value: Value<i32> = Rc::new(RefCell::new(value));
    if {
        let _lhs = {
            let _lhs = (*value.borrow());
            _lhs < (*(*(*node.borrow()).upgrade().deref()).value.borrow())
        };
        _lhs && (!((*(*(*node.borrow()).upgrade().deref()).left.borrow()).is_null())).clone()
    } {
        return ({
            let _node: Ptr<node_t> = (*(*(*node.borrow()).upgrade().deref()).left.borrow()).clone();
            let _value: i32 = (*value.borrow());
            find_0(_node, _value)
        });
    } else if {
        let _lhs = {
            let _lhs = (*value.borrow());
            _lhs > (*(*(*node.borrow()).upgrade().deref()).value.borrow())
        };
        _lhs && (!((*(*(*node.borrow()).upgrade().deref()).right.borrow()).is_null())).clone()
    } {
        return ({
            let _node: Ptr<node_t> =
                (*(*(*node.borrow()).upgrade().deref()).right.borrow()).clone();
            let _value: i32 = (*value.borrow());
            find_0(_node, _value)
        });
    } else if {
        let _lhs = (*value.borrow());
        _lhs == (*(*(*node.borrow()).upgrade().deref()).value.borrow())
    } {
        return (*node.borrow()).clone();
    }
    return Default::default();
}
pub fn insert_1(node: Ptr<node_t>, new_node: Ptr<node_t>) -> Ptr<node_t> {
    let node: Value<Ptr<node_t>> = Rc::new(RefCell::new(node));
    let new_node: Value<Ptr<node_t>> = Rc::new(RefCell::new(new_node));
    if (*node.borrow()).is_null() {
        return (*new_node.borrow()).clone();
    }
    if {
        let _lhs = (*(*(*new_node.borrow()).upgrade().deref()).value.borrow());
        _lhs < (*(*(*node.borrow()).upgrade().deref()).value.borrow())
    } {
        let __rhs = ({
            let _node: Ptr<node_t> = (*(*(*node.borrow()).upgrade().deref()).left.borrow()).clone();
            let _new_node: Ptr<node_t> = (*new_node.borrow()).clone();
            insert_1(_node, _new_node)
        });
        (*(*(*node.borrow()).upgrade().deref()).left.borrow_mut()) = __rhs;
    } else if {
        let _lhs = (*(*(*new_node.borrow()).upgrade().deref()).value.borrow());
        _lhs > (*(*(*node.borrow()).upgrade().deref()).value.borrow())
    } {
        let __rhs = ({
            let _node: Ptr<node_t> =
                (*(*(*node.borrow()).upgrade().deref()).right.borrow()).clone();
            let _new_node: Ptr<node_t> = (*new_node.borrow()).clone();
            insert_1(_node, _new_node)
        });
        (*(*(*node.borrow()).upgrade().deref()).right.borrow_mut()) = __rhs;
    }
    return (*node.borrow()).clone();
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let tree: Value<Option<Value<node_t>>> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new(node_t {
            left: Rc::new(RefCell::new(Default::default())),
            right: Rc::new(RefCell::new(Default::default())),
            value: Rc::new(RefCell::new(0)),
        })))));
    let n1: Value<Option<Value<node_t>>> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new(node_t {
            left: Rc::new(RefCell::new(Default::default())),
            right: Rc::new(RefCell::new(Default::default())),
            value: Rc::new(RefCell::new(1)),
        })))));
    let n2: Value<Option<Value<node_t>>> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new(node_t {
            left: Rc::new(RefCell::new(Default::default())),
            right: Rc::new(RefCell::new(Default::default())),
            value: Rc::new(RefCell::new(2)),
        })))));
    let n3: Value<Option<Value<node_t>>> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new(node_t {
            left: Rc::new(RefCell::new(Default::default())),
            right: Rc::new(RefCell::new(Default::default())),
            value: Rc::new(RefCell::new(3)),
        })))));
    let n4: Value<Option<Value<node_t>>> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new(node_t {
            left: Rc::new(RefCell::new(Default::default())),
            right: Rc::new(RefCell::new(Default::default())),
            value: Rc::new(RefCell::new(4)),
        })))));
    let ptr1: Value<Ptr<node_t>> = Rc::new(RefCell::new(((*tree.borrow()).as_pointer())));
    let __rhs = ({
        let _node: Ptr<node_t> = (*ptr1.borrow()).clone();
        let _new_node: Ptr<node_t> = ((*n1.borrow()).as_pointer());
        insert_1(_node, _new_node)
    });
    (*ptr1.borrow_mut()) = __rhs;
    let __rhs = ({
        let _node: Ptr<node_t> = (*ptr1.borrow()).clone();
        let _new_node: Ptr<node_t> = ((*n2.borrow()).as_pointer());
        insert_1(_node, _new_node)
    });
    (*ptr1.borrow_mut()) = __rhs;
    let __rhs = ({
        let _node: Ptr<node_t> = (*ptr1.borrow()).clone();
        let _new_node: Ptr<node_t> = ((*n3.borrow()).as_pointer());
        insert_1(_node, _new_node)
    });
    (*ptr1.borrow_mut()) = __rhs;
    let __rhs = ({
        let _node: Ptr<node_t> = (*ptr1.borrow()).clone();
        let _new_node: Ptr<node_t> = ((*n4.borrow()).as_pointer());
        insert_1(_node, _new_node)
    });
    (*ptr1.borrow_mut()) = __rhs;
    return (({
        let _lhs = ((((((*(*({
            let _node: Ptr<node_t> = (*ptr1.borrow()).clone();
            let _value: i32 = 0;
            find_0(_node, _value)
        })
        .upgrade()
        .deref())
        .value
        .borrow())
            == 0)
            && ((*(*({
                let _node: Ptr<node_t> = (*ptr1.borrow()).clone();
                let _value: i32 = 1;
                find_0(_node, _value)
            })
            .upgrade()
            .deref())
            .value
            .borrow())
                == 1))
            && ((*(*({
                let _node: Ptr<node_t> = (*ptr1.borrow()).clone();
                let _value: i32 = 2;
                find_0(_node, _value)
            })
            .upgrade()
            .deref())
            .value
            .borrow())
                == 2))
            && ((*(*({
                let _node: Ptr<node_t> = (*ptr1.borrow()).clone();
                let _value: i32 = 3;
                find_0(_node, _value)
            })
            .upgrade()
            .deref())
            .value
            .borrow())
                == 3))
            && ((*(*({
                let _node: Ptr<node_t> = (*ptr1.borrow()).clone();
                let _value: i32 = 4;
                find_0(_node, _value)
            })
            .upgrade()
            .deref())
            .value
            .borrow())
                == 4));
        _lhs && ({
            let _node: Ptr<node_t> = (*ptr1.borrow()).clone();
            let _value: i32 = 5;
            find_0(_node, _value)
        })
        .is_null()
    }) as i32);
}
