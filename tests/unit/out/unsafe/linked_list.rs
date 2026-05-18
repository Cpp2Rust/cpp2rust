extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Node {
    pub val: i32,
    pub next: *mut Node,
}
impl Node {
    pub unsafe fn SetNext(&mut self, mut next: *mut Node) {
        self.next = next;
    }
}
pub unsafe fn Find_0(mut head: *mut Node, mut idx: i32) -> *mut Node {
    let mut curr: *mut Node = head;
    let mut i: i32 = 0;
    'loop_: while ((i) < (idx)) {
        curr = (*curr).next;
        i.postfix_inc();
    }
    return curr;
}
pub unsafe fn Append_1(head: *mut Node, new_node: *mut Node) {
    let mut curr: *mut Node = (head);
    'loop_: while !(((*curr).next).is_null()) {
        curr = (*curr).next;
    }
    (unsafe {
        let _next: *mut Node = (new_node);
        (*curr).SetNext(_next)
    });
}
pub unsafe fn Delete_2(mut head: *mut Node, mut val: i32) -> *mut Node {
    let mut curr: *mut Node = head;
    let mut prev: *mut Node = std::ptr::null_mut();
    'loop_: while !((curr).is_null()) {
        if (((*curr).val) == (val)) {
            if !((prev).is_null()) {
                (*prev).next = (*curr).next;
                return head;
            } else {
                return (*curr).next;
            }
        }
        prev = curr;
        curr = (*curr).next;
    }
    return head;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut n0: Node = Node {
        val: 5,
        next: std::ptr::null_mut(),
    };
    let mut head: *mut Node = (&mut n0 as *mut Node);
    let mut n1: Node = Node {
        val: 4,
        next: std::ptr::null_mut(),
    };
    let mut n2: Node = Node {
        val: 3,
        next: std::ptr::null_mut(),
    };
    let mut n3: Node = Node {
        val: 2,
        next: std::ptr::null_mut(),
    };
    let mut n4: Node = Node {
        val: 1,
        next: std::ptr::null_mut(),
    };
    let mut n5: Node = Node {
        val: 0,
        next: std::ptr::null_mut(),
    };
    let mut n6: Node = Node {
        val: -1_i32,
        next: std::ptr::null_mut(),
    };
    let mut n7: Node = Node {
        val: -2_i32,
        next: std::ptr::null_mut(),
    };
    (unsafe {
        let _head: *mut Node = &mut (*head) as *mut Node;
        let _new_node: *mut Node = &mut n1 as *mut Node;
        Append_1(_head, _new_node)
    });
    (unsafe {
        let _head: *mut Node = &mut (*head) as *mut Node;
        let _new_node: *mut Node = &mut n2 as *mut Node;
        Append_1(_head, _new_node)
    });
    (unsafe {
        let _head: *mut Node = &mut (*head) as *mut Node;
        let _new_node: *mut Node = &mut n3 as *mut Node;
        Append_1(_head, _new_node)
    });
    (unsafe {
        let _head: *mut Node = &mut (*head) as *mut Node;
        let _new_node: *mut Node = &mut n4 as *mut Node;
        Append_1(_head, _new_node)
    });
    (unsafe {
        let _head: *mut Node = &mut (*head) as *mut Node;
        let _new_node: *mut Node = &mut n5 as *mut Node;
        Append_1(_head, _new_node)
    });
    (unsafe {
        let _head: *mut Node = &mut (*head) as *mut Node;
        let _new_node: *mut Node = &mut n6 as *mut Node;
        Append_1(_head, _new_node)
    });
    (unsafe {
        let _head: *mut Node = &mut (*head) as *mut Node;
        let _new_node: *mut Node = &mut n7 as *mut Node;
        Append_1(_head, _new_node)
    });
    head = (unsafe {
        let _head: *mut Node = head;
        let _val: i32 = 5;
        Delete_2(_head, _val)
    });
    head = (unsafe {
        let _head: *mut Node = head;
        let _val: i32 = 0;
        Delete_2(_head, _val)
    });
    head = (unsafe {
        let _head: *mut Node = head;
        let _val: i32 = -2_i32;
        Delete_2(_head, _val)
    });
    return ((((((((*(unsafe {
        let _head: *mut Node = head;
        let _idx: i32 = 0;
        Find_0(_head, _idx)
    }))
    .val)
        == (4))
        && (((*(unsafe {
            let _head: *mut Node = head;
            let _idx: i32 = 1;
            Find_0(_head, _idx)
        }))
        .val)
            == (3)))
        && (((*(unsafe {
            let _head: *mut Node = head;
            let _idx: i32 = 2;
            Find_0(_head, _idx)
        }))
        .val)
            == (2)))
        && (((*(unsafe {
            let _head: *mut Node = head;
            let _idx: i32 = 3;
            Find_0(_head, _idx)
        }))
        .val)
            == (1)))
        && ((((*(unsafe {
            let _head: *mut Node = head;
            let _idx: i32 = 4;
            Find_0(_head, _idx)
        }))
        .val)
            == (-1_i32))
            && ((unsafe {
                let _head: *mut Node = head;
                let _idx: i32 = 5;
                Find_0(_head, _idx)
            })
            .is_null()))) as i32);
}
