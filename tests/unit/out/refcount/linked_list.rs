extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Node {
    pub val: Value<i32>,
    pub next: Value<Ptr<Node>>,
}
impl Node {
    pub fn SetNext(&self, next: Ptr<Node>) {
        let next: Value<Ptr<Node>> = Rc::new(RefCell::new(next));
        (*self.next.borrow_mut()) = (*next.borrow()).clone();
    }
}
impl Clone for Node {
    fn clone(&self) -> Self {
        let mut this = Self {
            val: Rc::new(RefCell::new((*self.val.borrow()))),
            next: Rc::new(RefCell::new((*self.next.borrow()).clone())),
        };
        this
    }
}
impl ByteRepr for Node {}
pub fn Find_0(head: Ptr<Node>, idx: i32) -> Ptr<Node> {
    let head: Value<Ptr<Node>> = Rc::new(RefCell::new(head));
    let idx: Value<i32> = Rc::new(RefCell::new(idx));
    let curr: Value<Ptr<Node>> = Rc::new(RefCell::new((*head.borrow()).clone()));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*idx.borrow())) {
        let __rhs = (*(*(*curr.borrow()).upgrade().deref()).next.borrow()).clone();
        (*curr.borrow_mut()) = __rhs;
        (*i.borrow_mut()).postfix_inc();
    }
    return (*curr.borrow()).clone();
}
pub fn Append_1(head: Ptr<Node>, new_node: Ptr<Node>) {
    let curr: Value<Ptr<Node>> = Rc::new(RefCell::new((head).clone()));
    'loop_: while !((*(*(*curr.borrow()).upgrade().deref()).next.borrow()).is_null()) {
        let __rhs = (*(*(*curr.borrow()).upgrade().deref()).next.borrow()).clone();
        (*curr.borrow_mut()) = __rhs;
    }
    ({
        let _next: Ptr<Node> = (new_node).clone();
        (*(*curr.borrow()).upgrade().deref()).SetNext(_next)
    });
}
pub fn Delete_2(head: Ptr<Node>, val: i32) -> Ptr<Node> {
    let head: Value<Ptr<Node>> = Rc::new(RefCell::new(head));
    let val: Value<i32> = Rc::new(RefCell::new(val));
    let curr: Value<Ptr<Node>> = Rc::new(RefCell::new((*head.borrow()).clone()));
    let prev: Value<Ptr<Node>> = Rc::new(RefCell::new(Default::default()));
    'loop_: while !((*curr.borrow()).is_null()) {
        if {
            let _lhs = (*(*(*curr.borrow()).upgrade().deref()).val.borrow());
            _lhs == (*val.borrow())
        } {
            if !((*prev.borrow()).is_null()) {
                let __rhs = (*(*(*curr.borrow()).upgrade().deref()).next.borrow()).clone();
                (*(*(*prev.borrow()).upgrade().deref()).next.borrow_mut()) = __rhs;
                return (*head.borrow()).clone();
            } else {
                return (*(*(*curr.borrow()).upgrade().deref()).next.borrow()).clone();
            }
        }
        (*prev.borrow_mut()) = (*curr.borrow()).clone();
        let __rhs = (*(*(*curr.borrow()).upgrade().deref()).next.borrow()).clone();
        (*curr.borrow_mut()) = __rhs;
    }
    return (*head.borrow()).clone();
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let n0: Value<Node> = Rc::new(RefCell::new(Node {
        val: Rc::new(RefCell::new(5)),
        next: Rc::new(RefCell::new(Default::default())),
    }));
    let head: Value<Ptr<Node>> = Rc::new(RefCell::new((n0.as_pointer())));
    let n1: Value<Node> = Rc::new(RefCell::new(Node {
        val: Rc::new(RefCell::new(4)),
        next: Rc::new(RefCell::new(Default::default())),
    }));
    let n2: Value<Node> = Rc::new(RefCell::new(Node {
        val: Rc::new(RefCell::new(3)),
        next: Rc::new(RefCell::new(Default::default())),
    }));
    let n3: Value<Node> = Rc::new(RefCell::new(Node {
        val: Rc::new(RefCell::new(2)),
        next: Rc::new(RefCell::new(Default::default())),
    }));
    let n4: Value<Node> = Rc::new(RefCell::new(Node {
        val: Rc::new(RefCell::new(1)),
        next: Rc::new(RefCell::new(Default::default())),
    }));
    let n5: Value<Node> = Rc::new(RefCell::new(Node {
        val: Rc::new(RefCell::new(0)),
        next: Rc::new(RefCell::new(Default::default())),
    }));
    let n6: Value<Node> = Rc::new(RefCell::new(Node {
        val: Rc::new(RefCell::new(-1_i32)),
        next: Rc::new(RefCell::new(Default::default())),
    }));
    let n7: Value<Node> = Rc::new(RefCell::new(Node {
        val: Rc::new(RefCell::new(-2_i32)),
        next: Rc::new(RefCell::new(Default::default())),
    }));
    ({
        let _head: Ptr<Node> = (*head.borrow()).clone();
        let _new_node: Ptr<Node> = n1.as_pointer();
        Append_1(_head, _new_node)
    });
    ({
        let _head: Ptr<Node> = (*head.borrow()).clone();
        let _new_node: Ptr<Node> = n2.as_pointer();
        Append_1(_head, _new_node)
    });
    ({
        let _head: Ptr<Node> = (*head.borrow()).clone();
        let _new_node: Ptr<Node> = n3.as_pointer();
        Append_1(_head, _new_node)
    });
    ({
        let _head: Ptr<Node> = (*head.borrow()).clone();
        let _new_node: Ptr<Node> = n4.as_pointer();
        Append_1(_head, _new_node)
    });
    ({
        let _head: Ptr<Node> = (*head.borrow()).clone();
        let _new_node: Ptr<Node> = n5.as_pointer();
        Append_1(_head, _new_node)
    });
    ({
        let _head: Ptr<Node> = (*head.borrow()).clone();
        let _new_node: Ptr<Node> = n6.as_pointer();
        Append_1(_head, _new_node)
    });
    ({
        let _head: Ptr<Node> = (*head.borrow()).clone();
        let _new_node: Ptr<Node> = n7.as_pointer();
        Append_1(_head, _new_node)
    });
    let __rhs = ({
        let _head: Ptr<Node> = (*head.borrow()).clone();
        let _val: i32 = 5;
        Delete_2(_head, _val)
    });
    (*head.borrow_mut()) = __rhs;
    let __rhs = ({
        let _head: Ptr<Node> = (*head.borrow()).clone();
        let _val: i32 = 0;
        Delete_2(_head, _val)
    });
    (*head.borrow_mut()) = __rhs;
    let __rhs = ({
        let _head: Ptr<Node> = (*head.borrow()).clone();
        let _val: i32 = -2_i32;
        Delete_2(_head, _val)
    });
    (*head.borrow_mut()) = __rhs;
    return ((((((((*(*({
        let _head: Ptr<Node> = (*head.borrow()).clone();
        let _idx: i32 = 0;
        Find_0(_head, _idx)
    })
    .upgrade()
    .deref())
    .val
    .borrow())
        == 4) as bool)
        && (((*(*({
            let _head: Ptr<Node> = (*head.borrow()).clone();
            let _idx: i32 = 1;
            Find_0(_head, _idx)
        })
        .upgrade()
        .deref())
        .val
        .borrow())
            == 3) as bool))
        && (((*(*({
            let _head: Ptr<Node> = (*head.borrow()).clone();
            let _idx: i32 = 2;
            Find_0(_head, _idx)
        })
        .upgrade()
        .deref())
        .val
        .borrow())
            == 2) as bool))
        && (((*(*({
            let _head: Ptr<Node> = (*head.borrow()).clone();
            let _idx: i32 = 3;
            Find_0(_head, _idx)
        })
        .upgrade()
        .deref())
        .val
        .borrow())
            == 1) as bool))
        && (({
            let _lhs = ((*(*({
                let _head: Ptr<Node> = (*head.borrow()).clone();
                let _idx: i32 = 4;
                Find_0(_head, _idx)
            })
            .upgrade()
            .deref())
            .val
            .borrow())
                == -1_i32);
            _lhs && ((({
                let _head: Ptr<Node> = (*head.borrow()).clone();
                let _idx: i32 = 5;
                Find_0(_head, _idx)
            })
            .is_null()) as bool)
        }) as bool)) as i32);
}
