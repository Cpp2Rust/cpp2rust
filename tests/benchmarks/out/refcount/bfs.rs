extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Queue {
    pub elems: Value<Ptr<u32>>,
    pub front: Value<u64>,
    pub back: Value<u64>,
    pub capacity: Value<u64>,
}
impl Queue {
    pub fn enqueue(&self, elem: i32) {
        let elem: Value<i32> = Rc::new(RefCell::new(elem));
        if ((*self.back.borrow()) == (*self.capacity.borrow())) {
            return;
        }
        let __rhs = ((*elem.borrow()) as u32);
        (*self.elems.borrow())
            .offset(((*self.back.borrow_mut()).postfix_inc()) as isize)
            .write(__rhs);
    }
    pub fn dequeue(&self) -> u32 {
        if ({ self.empty() }) {
            return (-1_i32 as u32);
        }
        let elem: Value<u32> = Rc::new(RefCell::new(
            ((*self.elems.borrow())
                .offset(((*self.front.borrow_mut()).postfix_inc()) as isize)
                .read()),
        ));
        if ((*self.front.borrow()) == (*self.back.borrow())) {
            (*self.front.borrow_mut()) = 0_u64;
            (*self.back.borrow_mut()) = 0_u64;
        }
        return (*elem.borrow());
    }
    pub fn empty(&self) -> bool {
        return ((*self.back.borrow()) == 0_u64);
    }
}
impl Clone for Queue {
    fn clone(&self) -> Self {
        let mut this = Self {
            elems: Rc::new(RefCell::new((*self.elems.borrow()).clone())),
            front: Rc::new(RefCell::new((*self.front.borrow()))),
            back: Rc::new(RefCell::new((*self.back.borrow()))),
            capacity: Rc::new(RefCell::new((*self.capacity.borrow()))),
        };
        this
    }
}
impl ByteRepr for Queue {}
#[derive(Default)]
pub struct GraphNode {
    pub vertex: Value<u32>,
    pub next: Value<Ptr<GraphNode>>,
}
impl Clone for GraphNode {
    fn clone(&self) -> Self {
        let mut this = Self {
            vertex: Rc::new(RefCell::new((*self.vertex.borrow()))),
            next: Rc::new(RefCell::new((*self.next.borrow()).clone())),
        };
        this
    }
}
impl ByteRepr for GraphNode {}
#[derive(Default)]
pub struct Graph {
    pub V: Value<u32>,
    pub adj: Value<Ptr<Ptr<GraphNode>>>,
}
impl Graph {
    pub fn push(&self, src: u32, dst: u32) {
        let src: Value<u32> = Rc::new(RefCell::new(src));
        let dst: Value<u32> = Rc::new(RefCell::new(dst));
        let __rhs = Ptr::alloc(GraphNode {
            vertex: Rc::new(RefCell::new((*dst.borrow()))),
            next: Rc::new(RefCell::new(
                ((*self.adj.borrow()).offset((*src.borrow()) as isize).read()).clone(),
            )),
        });
        (*self.adj.borrow())
            .offset((*src.borrow()) as isize)
            .write(__rhs);
        let __rhs = Ptr::alloc(GraphNode {
            vertex: Rc::new(RefCell::new((*src.borrow()))),
            next: Rc::new(RefCell::new(
                ((*self.adj.borrow()).offset((*dst.borrow()) as isize).read()).clone(),
            )),
        });
        (*self.adj.borrow())
            .offset((*dst.borrow()) as isize)
            .write(__rhs);
    }
}
impl Clone for Graph {
    fn clone(&self) -> Self {
        let mut this = Self {
            V: Rc::new(RefCell::new((*self.V.borrow()))),
            adj: Rc::new(RefCell::new((*self.adj.borrow()).clone())),
        };
        this
    }
}
impl ByteRepr for Graph {}
pub fn BFS_0(graph: Ptr<Graph>, start_vertex: u32) -> Ptr<u32> {
    let start_vertex: Value<u32> = Rc::new(RefCell::new(start_vertex));
    let Q: Value<Queue> = Rc::new(RefCell::new(Queue {
        elems: Rc::new(RefCell::new(Ptr::alloc_array(
            (0..((*(*graph.upgrade().deref()).V.borrow()) as u64))
                .map(|_| <u32>::default())
                .collect::<Box<[u32]>>(),
        ))),
        front: Rc::new(RefCell::new(0_u64)),
        back: Rc::new(RefCell::new(0_u64)),
        capacity: Rc::new(RefCell::new(
            ((*(*graph.upgrade().deref()).V.borrow()) as u64),
        )),
    }));
    let visited: Value<Ptr<bool>> = Rc::new(RefCell::new(Ptr::alloc_array(
        (0..((*(*graph.upgrade().deref()).V.borrow()) as u64))
            .map(|_| <bool>::default())
            .collect::<Box<[bool]>>(),
    )));
    let pred: Value<Ptr<u32>> = Rc::new(RefCell::new(Ptr::alloc_array(
        (0..((*(*graph.upgrade().deref()).V.borrow()) as u64))
            .map(|_| <u32>::default())
            .collect::<Box<[u32]>>(),
    )));
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while {
        let _lhs = (*i.borrow());
        _lhs < (*(*graph.upgrade().deref()).V.borrow())
    } {
        (*visited.borrow())
            .offset((*i.borrow()) as isize)
            .write(false);
        let __rhs = (*i.borrow());
        (*pred.borrow()).offset((*i.borrow()) as isize).write(__rhs);
        (*i.borrow_mut()).prefix_inc();
    }
    (*visited.borrow())
        .offset((*start_vertex.borrow()) as isize)
        .write(true);
    ({
        let _elem: i32 = ((*start_vertex.borrow()) as i32);
        (*Q.borrow()).enqueue(_elem)
    });
    'loop_: while !({ (*Q.borrow()).empty() }) {
        let current_vertex: Value<i32> =
            Rc::new(RefCell::new((({ (*Q.borrow()).dequeue() }) as i32)));
        let head: Value<Ptr<GraphNode>> = Rc::new(RefCell::new(
            ((*(*graph.upgrade().deref()).adj.borrow())
                .offset((*current_vertex.borrow()) as isize)
                .read())
            .clone(),
        ));
        'loop_: while !((*head.borrow()).is_null()) {
            let adj_vertex: Value<i32> = Rc::new(RefCell::new(
                ((*(*(*head.borrow()).upgrade().deref()).vertex.borrow()) as i32),
            ));
            if !((*visited.borrow())
                .offset((*adj_vertex.borrow()) as isize)
                .read())
            {
                (*visited.borrow())
                    .offset((*adj_vertex.borrow()) as isize)
                    .write(true);
                ({
                    let _elem: i32 = (*adj_vertex.borrow());
                    (*Q.borrow()).enqueue(_elem)
                });
                let __rhs = ((*current_vertex.borrow()) as u32);
                (*pred.borrow())
                    .offset((*adj_vertex.borrow()) as isize)
                    .write(__rhs);
            }
            let __rhs = (*(*(*head.borrow()).upgrade().deref()).next.borrow()).clone();
            (*head.borrow_mut()) = __rhs;
        }
    }
    (*visited.borrow()).delete_array();
    (*(*Q.borrow()).elems.borrow()).delete_array();
    return (*pred.borrow()).clone();
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let V: Value<u64> = Rc::new(RefCell::new(5000_u64));
    let graph: Value<Graph> = Rc::new(RefCell::new(Graph {
        V: Rc::new(RefCell::new(((*V.borrow()) as u32))),
        adj: Rc::new(RefCell::new(Ptr::alloc_array(
            (0..(*V.borrow()))
                .map(|_| Ptr::<GraphNode>::null())
                .collect::<Box<[Ptr<GraphNode>]>>(),
        ))),
    }));
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while (((*i.borrow()) as u64) < (*V.borrow())) {
        (*(*graph.borrow()).adj.borrow())
            .offset((*i.borrow()) as isize)
            .write(Ptr::<GraphNode>::null());
        (*i.borrow_mut()).prefix_inc();
    }
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while (((*i.borrow()) as u64) < (*V.borrow())) {
        let j: Value<u32> = Rc::new(RefCell::new((*i.borrow()).wrapping_add(1_u32)));
        'loop_: while (((*j.borrow()) as u64) < (*V.borrow())) {
            ({
                let _src: u32 = (*i.borrow());
                let _dst: u32 = (*j.borrow());
                (*graph.borrow()).push(_src, _dst)
            });
            (*j.borrow_mut()).prefix_inc();
        }
        (*i.borrow_mut()).prefix_inc();
    }
    let pred: Value<Ptr<u32>> = Rc::new(RefCell::new(
        ({
            let _graph: Ptr<Graph> = graph.as_pointer();
            let _start_vertex: u32 = 0_u32;
            BFS_0(_graph, _start_vertex)
        }),
    ));
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while (((*i.borrow()) as u64) < (*V.borrow())) {
        let head: Value<Ptr<GraphNode>> = Rc::new(RefCell::new(
            ((*(*graph.borrow()).adj.borrow())
                .offset((*i.borrow()) as isize)
                .read())
            .clone(),
        ));
        'loop_: while !((*head.borrow()).is_null()) {
            let next: Value<Ptr<GraphNode>> = Rc::new(RefCell::new(
                (*(*(*head.borrow()).upgrade().deref()).next.borrow()).clone(),
            ));
            (*head.borrow()).delete();
            (*head.borrow_mut()) = (*next.borrow()).clone();
        }
        (*i.borrow_mut()).prefix_inc();
    }
    (*(*graph.borrow()).adj.borrow()).delete_array();
    (*pred.borrow()).delete_array();
    return 0;
}
