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
    pub prev: Value<Ptr<Node>>,
}
impl Node {
    pub fn SetNext(&self, n: Ptr<Node>) {
        let n: Value<Ptr<Node>> = Rc::new(RefCell::new(n));
        (*self.next.borrow_mut()) = (*n.borrow()).clone();
    }
    pub fn SetPrev(&self, p: Ptr<Node>) {
        let p: Value<Ptr<Node>> = Rc::new(RefCell::new(p));
        (*self.prev.borrow_mut()) = (*p.borrow()).clone();
    }
}
impl Clone for Node {
    fn clone(&self) -> Self {
        let mut this = Self {
            val: Rc::new(RefCell::new((*self.val.borrow()))),
            next: Rc::new(RefCell::new((*self.next.borrow()).clone())),
            prev: Rc::new(RefCell::new((*self.prev.borrow()).clone())),
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
pub fn FindBack_1(tail: Ptr<Node>, idx: i32) -> Ptr<Node> {
    let tail: Value<Ptr<Node>> = Rc::new(RefCell::new(tail));
    let idx: Value<i32> = Rc::new(RefCell::new(idx));
    let curr: Value<Ptr<Node>> = Rc::new(RefCell::new((*tail.borrow()).clone()));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*idx.borrow())) {
        let __rhs = (*(*(*curr.borrow()).upgrade().deref()).prev.borrow()).clone();
        (*curr.borrow_mut()) = __rhs;
        (*i.borrow_mut()).postfix_inc();
    }
    return (*curr.borrow()).clone();
}
pub fn Append_2(head: Ptr<Node>, new_node: Ptr<Node>) {
    let curr: Value<Ptr<Node>> = Rc::new(RefCell::new((head).clone()));
    'loop_: while !((*(*(*curr.borrow()).upgrade().deref()).next.borrow()).is_null()) {
        let __rhs = (*(*(*curr.borrow()).upgrade().deref()).next.borrow()).clone();
        (*curr.borrow_mut()) = __rhs;
    }
    ({
        let _n: Ptr<Node> = (new_node).clone();
        (*(*curr.borrow()).upgrade().deref()).SetNext(_n)
    });
    ({
        let _p: Ptr<Node> = (*curr.borrow()).clone();
        (*new_node.upgrade().deref()).SetPrev(_p)
    });
}
pub fn Delete_3(head: Ptr<Node>, val: i32) -> Ptr<Node> {
    let head: Value<Ptr<Node>> = Rc::new(RefCell::new(head));
    let val: Value<i32> = Rc::new(RefCell::new(val));
    let curr: Value<Ptr<Node>> = Rc::new(RefCell::new((*head.borrow()).clone()));
    'loop_: while !((*curr.borrow()).is_null()) {
        if {
            let _lhs = (*(*(*curr.borrow()).upgrade().deref()).val.borrow());
            _lhs == (*val.borrow())
        } {
            let prev: Value<Ptr<Node>> = Rc::new(RefCell::new(
                (*(*(*curr.borrow()).upgrade().deref()).prev.borrow()).clone(),
            ));
            let next: Value<Ptr<Node>> = Rc::new(RefCell::new(
                (*(*(*curr.borrow()).upgrade().deref()).next.borrow()).clone(),
            ));
            if !((*prev.borrow()).is_null()) {
                (*(*(*prev.borrow()).upgrade().deref()).next.borrow_mut()) =
                    (*next.borrow()).clone();
            }
            if !((*next.borrow()).is_null()) {
                (*(*(*next.borrow()).upgrade().deref()).prev.borrow_mut()) =
                    (*prev.borrow()).clone();
            }
            if !((*prev.borrow()).is_null()) {
                return (*head.borrow()).clone();
            } else {
                return (*next.borrow()).clone();
            }
        }
        let __rhs = (*(*(*curr.borrow()).upgrade().deref()).next.borrow()).clone();
        (*curr.borrow_mut()) = __rhs;
    }
    return (*head.borrow()).clone();
}
pub fn Tail_4(head: Ptr<Node>) -> Ptr<Node> {
    let head: Value<Ptr<Node>> = Rc::new(RefCell::new(head));
    let curr: Value<Ptr<Node>> = Rc::new(RefCell::new((*head.borrow()).clone()));
    'loop_: while !((*(*(*curr.borrow()).upgrade().deref()).next.borrow()).is_null()) {
        let __rhs = (*(*(*curr.borrow()).upgrade().deref()).next.borrow()).clone();
        (*curr.borrow_mut()) = __rhs;
    }
    return (*curr.borrow()).clone();
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let n0: Value<Node> = Rc::new(RefCell::new(Node {
        val: Rc::new(RefCell::new(5)),
        next: Rc::new(RefCell::new(Ptr::<Node>::null())),
        prev: Rc::new(RefCell::new(Ptr::<Node>::null())),
    }));
    let head: Value<Ptr<Node>> = Rc::new(RefCell::new((n0.as_pointer())));
    let n1: Value<Node> = Rc::new(RefCell::new(Node {
        val: Rc::new(RefCell::new(4)),
        next: Rc::new(RefCell::new(Ptr::<Node>::null())),
        prev: Rc::new(RefCell::new(Ptr::<Node>::null())),
    }));
    let n2: Value<Node> = Rc::new(RefCell::new(Node {
        val: Rc::new(RefCell::new(3)),
        next: Rc::new(RefCell::new(Ptr::<Node>::null())),
        prev: Rc::new(RefCell::new(Ptr::<Node>::null())),
    }));
    let n3: Value<Node> = Rc::new(RefCell::new(Node {
        val: Rc::new(RefCell::new(2)),
        next: Rc::new(RefCell::new(Ptr::<Node>::null())),
        prev: Rc::new(RefCell::new(Ptr::<Node>::null())),
    }));
    let n4: Value<Node> = Rc::new(RefCell::new(Node {
        val: Rc::new(RefCell::new(1)),
        next: Rc::new(RefCell::new(Ptr::<Node>::null())),
        prev: Rc::new(RefCell::new(Ptr::<Node>::null())),
    }));
    let n5: Value<Node> = Rc::new(RefCell::new(Node {
        val: Rc::new(RefCell::new(0)),
        next: Rc::new(RefCell::new(Ptr::<Node>::null())),
        prev: Rc::new(RefCell::new(Ptr::<Node>::null())),
    }));
    let n6: Value<Node> = Rc::new(RefCell::new(Node {
        val: Rc::new(RefCell::new(-1_i32)),
        next: Rc::new(RefCell::new(Ptr::<Node>::null())),
        prev: Rc::new(RefCell::new(Ptr::<Node>::null())),
    }));
    let n7: Value<Node> = Rc::new(RefCell::new(Node {
        val: Rc::new(RefCell::new(-2_i32)),
        next: Rc::new(RefCell::new(Ptr::<Node>::null())),
        prev: Rc::new(RefCell::new(Ptr::<Node>::null())),
    }));
    ({
        let _head: Ptr<Node> = (*head.borrow()).clone();
        let _new_node: Ptr<Node> = n1.as_pointer();
        Append_2(_head, _new_node)
    });
    ({
        let _head: Ptr<Node> = (*head.borrow()).clone();
        let _new_node: Ptr<Node> = n2.as_pointer();
        Append_2(_head, _new_node)
    });
    ({
        let _head: Ptr<Node> = (*head.borrow()).clone();
        let _new_node: Ptr<Node> = n3.as_pointer();
        Append_2(_head, _new_node)
    });
    ({
        let _head: Ptr<Node> = (*head.borrow()).clone();
        let _new_node: Ptr<Node> = n4.as_pointer();
        Append_2(_head, _new_node)
    });
    ({
        let _head: Ptr<Node> = (*head.borrow()).clone();
        let _new_node: Ptr<Node> = n5.as_pointer();
        Append_2(_head, _new_node)
    });
    ({
        let _head: Ptr<Node> = (*head.borrow()).clone();
        let _new_node: Ptr<Node> = n6.as_pointer();
        Append_2(_head, _new_node)
    });
    ({
        let _head: Ptr<Node> = (*head.borrow()).clone();
        let _new_node: Ptr<Node> = n7.as_pointer();
        Append_2(_head, _new_node)
    });
    let __rhs = ({
        let _head: Ptr<Node> = (*head.borrow()).clone();
        let _val: i32 = 5;
        Delete_3(_head, _val)
    });
    (*head.borrow_mut()) = __rhs;
    let __rhs = ({
        let _head: Ptr<Node> = (*head.borrow()).clone();
        let _val: i32 = 0;
        Delete_3(_head, _val)
    });
    (*head.borrow_mut()) = __rhs;
    let __rhs = ({
        let _head: Ptr<Node> = (*head.borrow()).clone();
        let _val: i32 = -2_i32;
        Delete_3(_head, _val)
    });
    (*head.borrow_mut()) = __rhs;
    let tail: Value<Ptr<Node>> = Rc::new(RefCell::new(
        ({
            let _head: Ptr<Node> = (*head.borrow()).clone();
            Tail_4(_head)
        }),
    ));
    assert!(
        ((*(*({
            let _head: Ptr<Node> = (*head.borrow()).clone();
            let _idx: i32 = 0;
            Find_0(_head, _idx)
        })
        .upgrade()
        .deref())
        .val
        .borrow())
            == 4)
    );
    assert!(
        ((*(*({
            let _head: Ptr<Node> = (*head.borrow()).clone();
            let _idx: i32 = 1;
            Find_0(_head, _idx)
        })
        .upgrade()
        .deref())
        .val
        .borrow())
            == 3)
    );
    assert!(
        ((*(*({
            let _head: Ptr<Node> = (*head.borrow()).clone();
            let _idx: i32 = 2;
            Find_0(_head, _idx)
        })
        .upgrade()
        .deref())
        .val
        .borrow())
            == 2)
    );
    assert!(
        ((*(*({
            let _head: Ptr<Node> = (*head.borrow()).clone();
            let _idx: i32 = 3;
            Find_0(_head, _idx)
        })
        .upgrade()
        .deref())
        .val
        .borrow())
            == 1)
    );
    assert!(
        ((*(*({
            let _head: Ptr<Node> = (*head.borrow()).clone();
            let _idx: i32 = 4;
            Find_0(_head, _idx)
        })
        .upgrade()
        .deref())
        .val
        .borrow())
            == -1_i32)
    );
    assert!(({
        let _head: Ptr<Node> = (*head.borrow()).clone();
        let _idx: i32 = 5;
        Find_0(_head, _idx)
    })
    .is_null());
    assert!(
        ((*(*({
            let _tail: Ptr<Node> = (*tail.borrow()).clone();
            let _idx: i32 = 0;
            FindBack_1(_tail, _idx)
        })
        .upgrade()
        .deref())
        .val
        .borrow())
            == -1_i32)
    );
    assert!(
        ((*(*({
            let _tail: Ptr<Node> = (*tail.borrow()).clone();
            let _idx: i32 = 1;
            FindBack_1(_tail, _idx)
        })
        .upgrade()
        .deref())
        .val
        .borrow())
            == 1)
    );
    assert!(
        ((*(*({
            let _tail: Ptr<Node> = (*tail.borrow()).clone();
            let _idx: i32 = 2;
            FindBack_1(_tail, _idx)
        })
        .upgrade()
        .deref())
        .val
        .borrow())
            == 2)
    );
    assert!(
        ((*(*({
            let _tail: Ptr<Node> = (*tail.borrow()).clone();
            let _idx: i32 = 3;
            FindBack_1(_tail, _idx)
        })
        .upgrade()
        .deref())
        .val
        .borrow())
            == 3)
    );
    assert!(
        ((*(*({
            let _tail: Ptr<Node> = (*tail.borrow()).clone();
            let _idx: i32 = 4;
            FindBack_1(_tail, _idx)
        })
        .upgrade()
        .deref())
        .val
        .borrow())
            == 4)
    );
    assert!((*(*({
        let _tail: Ptr<Node> = (*tail.borrow()).clone();
        let _idx: i32 = 4;
        FindBack_1(_tail, _idx)
    })
    .upgrade()
    .deref())
    .prev
    .borrow())
    .is_null());
    assert!(
        ((*(*(*(*({
            let _head: Ptr<Node> = (*head.borrow()).clone();
            let _idx: i32 = 0;
            Find_0(_head, _idx)
        })
        .upgrade()
        .deref())
        .next
        .borrow())
        .upgrade()
        .deref())
        .val
        .borrow())
            == 3)
    );
    assert!(
        ((*(*(*(*(*(*({
            let _head: Ptr<Node> = (*head.borrow()).clone();
            let _idx: i32 = 1;
            Find_0(_head, _idx)
        })
        .upgrade()
        .deref())
        .next
        .borrow())
        .upgrade()
        .deref())
        .next
        .borrow())
        .upgrade()
        .deref())
        .val
        .borrow())
            == 1)
    );
    assert!(
        ((*(*(*(*({
            let _head: Ptr<Node> = (*head.borrow()).clone();
            let _idx: i32 = 2;
            Find_0(_head, _idx)
        })
        .upgrade()
        .deref())
        .prev
        .borrow())
        .upgrade()
        .deref())
        .val
        .borrow())
            == 3)
    );
    assert!((*(*({
        let _head: Ptr<Node> = (*head.borrow()).clone();
        let _idx: i32 = 4;
        Find_0(_head, _idx)
    })
    .upgrade()
    .deref())
    .next
    .borrow())
    .is_null());
    assert!(
        ((*(*(*(*(*(*({
            let _tail: Ptr<Node> = (*tail.borrow()).clone();
            let _idx: i32 = 1;
            FindBack_1(_tail, _idx)
        })
        .upgrade()
        .deref())
        .prev
        .borrow())
        .upgrade()
        .deref())
        .prev
        .borrow())
        .upgrade()
        .deref())
        .val
        .borrow())
            == 3)
    );
    (*(*(*(*({
        let _head: Ptr<Node> = (*head.borrow()).clone();
        let _idx: i32 = 0;
        Find_0(_head, _idx)
    })
    .upgrade()
    .deref())
    .next
    .borrow())
    .upgrade()
    .deref())
    .val
    .borrow_mut()) = 30;
    assert!(
        ((*(*({
            let _head: Ptr<Node> = (*head.borrow()).clone();
            let _idx: i32 = 1;
            Find_0(_head, _idx)
        })
        .upgrade()
        .deref())
        .val
        .borrow())
            == 30)
    );
    let __rhs = ((*(*({
        let _head: Ptr<Node> = (*head.borrow()).clone();
        let _idx: i32 = 0;
        Find_0(_head, _idx)
    })
    .upgrade()
    .deref())
    .val
    .borrow())
        + (*(*({
            let _head: Ptr<Node> = (*head.borrow()).clone();
            let _idx: i32 = 3;
            Find_0(_head, _idx)
        })
        .upgrade()
        .deref())
        .val
        .borrow()));
    (*(*(*(*({
        let _head: Ptr<Node> = (*head.borrow()).clone();
        let _idx: i32 = 1;
        Find_0(_head, _idx)
    })
    .upgrade()
    .deref())
    .next
    .borrow())
    .upgrade()
    .deref())
    .val
    .borrow_mut()) = __rhs;
    assert!(
        ((*(*({
            let _head: Ptr<Node> = (*head.borrow()).clone();
            let _idx: i32 = 2;
            Find_0(_head, _idx)
        })
        .upgrade()
        .deref())
        .val
        .borrow())
            == (4 + 1))
    );
    let sum: Value<i32> = Rc::new(RefCell::new(
        (((((*(*({
            let _head: Ptr<Node> = (*head.borrow()).clone();
            let _idx: i32 = 0;
            Find_0(_head, _idx)
        })
        .upgrade()
        .deref())
        .val
        .borrow())
            + (*(*({
                let _head: Ptr<Node> = (*head.borrow()).clone();
                let _idx: i32 = 1;
                Find_0(_head, _idx)
            })
            .upgrade()
            .deref())
            .val
            .borrow()))
            + (*(*({
                let _head: Ptr<Node> = (*head.borrow()).clone();
                let _idx: i32 = 2;
                Find_0(_head, _idx)
            })
            .upgrade()
            .deref())
            .val
            .borrow()))
            + (*(*({
                let _head: Ptr<Node> = (*head.borrow()).clone();
                let _idx: i32 = 3;
                Find_0(_head, _idx)
            })
            .upgrade()
            .deref())
            .val
            .borrow()))
            + (*(*({
                let _head: Ptr<Node> = (*head.borrow()).clone();
                let _idx: i32 = 4;
                Find_0(_head, _idx)
            })
            .upgrade()
            .deref())
            .val
            .borrow())),
    ));
    assert!(((*sum.borrow()) == ((((4 + 30) + 5) + 1) + -1_i32)));
    assert!(
        ({
            let _lhs = (*(*({
                let _head: Ptr<Node> = (*head.borrow()).clone();
                let _idx: i32 = 0;
                Find_0(_head, _idx)
            })
            .upgrade()
            .deref())
            .val
            .borrow());
            _lhs + (*(*({
                let _tail: Ptr<Node> = (*tail.borrow()).clone();
                let _idx: i32 = 0;
                FindBack_1(_tail, _idx)
            })
            .upgrade()
            .deref())
            .val
            .borrow())
        } == (4 + -1_i32))
    );
    assert!({
        let _lhs = (*(*(*(*({
            let _head: Ptr<Node> = (*head.borrow()).clone();
            let _idx: i32 = 2;
            Find_0(_head, _idx)
        })
        .upgrade()
        .deref())
        .next
        .borrow())
        .upgrade()
        .deref())
        .val
        .borrow());
        _lhs == (*(*({
            let _tail: Ptr<Node> = (*tail.borrow()).clone();
            let _idx: i32 = 1;
            FindBack_1(_tail, _idx)
        })
        .upgrade()
        .deref())
        .val
        .borrow())
    });
    assert!({
        let _lhs = (*(*({
            let _head: Ptr<Node> = (*head.borrow()).clone();
            let _idx: i32 = 0;
            Find_0(_head, _idx)
        })
        .upgrade()
        .deref())
        .prev
        .borrow())
        .clone();
        _lhs == (*(*({
            let _tail: Ptr<Node> = (*tail.borrow()).clone();
            let _idx: i32 = 4;
            FindBack_1(_tail, _idx)
        })
        .upgrade()
        .deref())
        .prev
        .borrow())
        .clone()
    });
    return 0;
}
