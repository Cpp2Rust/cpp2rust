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
    if ({
        let _lhs = (*value.borrow());
        _lhs < (*(*(*node.borrow()).upgrade().deref()).value.borrow())
    }) && (!((*(*(*node.borrow()).upgrade().deref()).left.borrow()).is_null()))
    {
        return ({
            let _node: Ptr<node_t> = (*(*(*node.borrow()).upgrade().deref()).left.borrow()).clone();
            let _value: i32 = (*value.borrow());
            find_0(_node, _value)
        });
    } else if ({
        let _lhs = (*value.borrow());
        _lhs > (*(*(*node.borrow()).upgrade().deref()).value.borrow())
    }) && (!((*(*(*node.borrow()).upgrade().deref()).right.borrow()).is_null()))
    {
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
    return Ptr::<node_t>::null();
}
pub fn insert_1(node: Ptr<node_t>, value: i32) -> Ptr<node_t> {
    let node: Value<Ptr<node_t>> = Rc::new(RefCell::new(node));
    let value: Value<i32> = Rc::new(RefCell::new(value));
    if (*node.borrow()).is_null() {
        return Ptr::alloc(node_t {
            left: Rc::new(RefCell::new(Ptr::<node_t>::null())),
            right: Rc::new(RefCell::new(Ptr::<node_t>::null())),
            value: Rc::new(RefCell::new((*value.borrow()))),
        });
    }
    if {
        let _lhs = (*value.borrow());
        _lhs < (*(*(*node.borrow()).upgrade().deref()).value.borrow())
    } {
        let __rhs = ({
            let _node: Ptr<node_t> = (*(*(*node.borrow()).upgrade().deref()).left.borrow()).clone();
            let _value: i32 = (*value.borrow());
            insert_1(_node, _value)
        });
        (*(*(*node.borrow()).upgrade().deref()).left.borrow_mut()) = __rhs;
    } else if {
        let _lhs = (*value.borrow());
        _lhs > (*(*(*node.borrow()).upgrade().deref()).value.borrow())
    } {
        let __rhs = ({
            let _node: Ptr<node_t> =
                (*(*(*node.borrow()).upgrade().deref()).right.borrow()).clone();
            let _value: i32 = (*value.borrow());
            insert_1(_node, _value)
        });
        (*(*(*node.borrow()).upgrade().deref()).right.borrow_mut()) = __rhs;
    }
    return (*node.borrow()).clone();
}
pub fn del_2(node: Ptr<node_t>) {
    let node: Value<Ptr<node_t>> = Rc::new(RefCell::new(node));
    if !((*(*(*node.borrow()).upgrade().deref()).left.borrow()).is_null()) {
        ({
            let _node: Ptr<node_t> = (*(*(*node.borrow()).upgrade().deref()).left.borrow()).clone();
            del_2(_node)
        });
    }
    if !((*(*(*node.borrow()).upgrade().deref()).right.borrow()).is_null()) {
        ({
            let _node: Ptr<node_t> =
                (*(*(*node.borrow()).upgrade().deref()).right.borrow()).clone();
            del_2(_node)
        });
    }
    (*node.borrow()).delete();
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let root: Value<Ptr<node_t>> = Rc::new(RefCell::new(Ptr::alloc(node_t {
        left: Rc::new(RefCell::new(Ptr::<node_t>::null())),
        right: Rc::new(RefCell::new(Ptr::<node_t>::null())),
        value: Rc::new(RefCell::new(0)),
    })));
    let __rhs = ({
        let _node: Ptr<node_t> = (*root.borrow()).clone();
        let _value: i32 = 1;
        insert_1(_node, _value)
    });
    (*root.borrow_mut()) = __rhs;
    let __rhs = ({
        let _node: Ptr<node_t> = (*root.borrow()).clone();
        let _value: i32 = 2;
        insert_1(_node, _value)
    });
    (*root.borrow_mut()) = __rhs;
    let __rhs = ({
        let _node: Ptr<node_t> = (*root.borrow()).clone();
        let _value: i32 = 3;
        insert_1(_node, _value)
    });
    (*root.borrow_mut()) = __rhs;
    let __rhs = ({
        let _node: Ptr<node_t> = (*root.borrow()).clone();
        let _value: i32 = 4;
        insert_1(_node, _value)
    });
    (*root.borrow_mut()) = __rhs;
    let out: Value<bool> = Rc::new(RefCell::new(
        ((((((*(*({
            let _node: Ptr<node_t> = (*root.borrow()).clone();
            let _value: i32 = 0;
            find_0(_node, _value)
        })
        .upgrade()
        .deref())
        .value
        .borrow())
            == 0)
            && ((*(*({
                let _node: Ptr<node_t> = (*root.borrow()).clone();
                let _value: i32 = 1;
                find_0(_node, _value)
            })
            .upgrade()
            .deref())
            .value
            .borrow())
                == 1))
            && ((*(*({
                let _node: Ptr<node_t> = (*root.borrow()).clone();
                let _value: i32 = 2;
                find_0(_node, _value)
            })
            .upgrade()
            .deref())
            .value
            .borrow())
                == 2))
            && ((*(*({
                let _node: Ptr<node_t> = (*root.borrow()).clone();
                let _value: i32 = 3;
                find_0(_node, _value)
            })
            .upgrade()
            .deref())
            .value
            .borrow())
                == 3))
            && ((*(*({
                let _node: Ptr<node_t> = (*root.borrow()).clone();
                let _value: i32 = 4;
                find_0(_node, _value)
            })
            .upgrade()
            .deref())
            .value
            .borrow())
                == 4))
            && (({
                let _node: Ptr<node_t> = (*root.borrow()).clone();
                let _value: i32 = 5;
                find_0(_node, _value)
            })
            .is_null()),
    ));
    ({
        let _node: Ptr<node_t> = (*root.borrow()).clone();
        del_2(_node)
    });
    return ((*out.borrow()) as i32);
}
