extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Edge {
    #[offset(0)]
    pub u: i32,
    #[offset(4)]
    pub v: i32,
    #[offset(8)]
    pub weight: f64,
}
pub fn partition_0(arr: Ptr<Option<Value<Box<[Edge]>>>>, mut start: i32, mut end: i32) -> i32 {
    let pivot: Ptr<Edge> = ((*arr.upgrade().deref())
        .as_ref()
        .unwrap()
        .as_pointer()
        .offset((start as usize)))
    .clone();
    let mut count: i32 = 0;
    let mut i: i32 = (start + 1);
    'loop_: while (i <= end) {
        if ({
            { (*arr.upgrade().deref()).as_ref().unwrap().borrow()[(i as usize) as usize].weight }
        } <= { pivot.with(|__s| __s.weight) })
        {
            count.postfix_inc();
        }
        i.prefix_inc();
    }
    let mut pidx: i32 = (start + count);
    let tmp: Value<Edge> = Rc::new(RefCell::new(Edge {
        u: { (*arr.upgrade().deref()).as_ref().unwrap().borrow()[(pidx as usize) as usize].u },
        v: { (*arr.upgrade().deref()).as_ref().unwrap().borrow()[(pidx as usize) as usize].v },
        weight: {
            (*arr.upgrade().deref()).as_ref().unwrap().borrow()[(pidx as usize) as usize].weight
        },
    }));
    let __rhs = Edge {
        u: { (*arr.upgrade().deref()).as_ref().unwrap().borrow()[(start as usize) as usize].u },
        v: { (*arr.upgrade().deref()).as_ref().unwrap().borrow()[(start as usize) as usize].v },
        weight: {
            (*arr.upgrade().deref()).as_ref().unwrap().borrow()[(start as usize) as usize].weight
        },
    };
    (*arr.upgrade().deref()).as_ref().unwrap().borrow_mut()[(pidx as usize) as usize] = __rhs;
    (*arr.upgrade().deref()).as_ref().unwrap().borrow_mut()[(start as usize) as usize] = Edge {
        u: { (*tmp.borrow()).u },
        v: { (*tmp.borrow()).v },
        weight: { (*tmp.borrow()).weight },
    };
    let mut i: i32 = start;
    let mut j: i32 = end;
    'loop_: while (i < pidx) && (j > pidx) {
        'loop_: while ({
            { (*arr.upgrade().deref()).as_ref().unwrap().borrow()[(i as usize) as usize].weight }
        } <= { pivot.with(|__s| __s.weight) })
        {
            i.prefix_inc();
        }
        'loop_: while ({
            { (*arr.upgrade().deref()).as_ref().unwrap().borrow()[(j as usize) as usize].weight }
        } > { pivot.with(|__s| __s.weight) })
        {
            j.prefix_dec();
        }
        if (i < pidx) && (j > pidx) {
            (*tmp.borrow_mut()) = Edge {
                u: { (*arr.upgrade().deref()).as_ref().unwrap().borrow()[(i as usize) as usize].u },
                v: { (*arr.upgrade().deref()).as_ref().unwrap().borrow()[(i as usize) as usize].v },
                weight: {
                    (*arr.upgrade().deref()).as_ref().unwrap().borrow()[(i as usize) as usize]
                        .weight
                },
            };
            let __rhs = Edge {
                u: { (*arr.upgrade().deref()).as_ref().unwrap().borrow()[(j as usize) as usize].u },
                v: { (*arr.upgrade().deref()).as_ref().unwrap().borrow()[(j as usize) as usize].v },
                weight: {
                    (*arr.upgrade().deref()).as_ref().unwrap().borrow()[(j as usize) as usize]
                        .weight
                },
            };
            (*arr.upgrade().deref()).as_ref().unwrap().borrow_mut()[(i as usize) as usize] = __rhs;
            (*arr.upgrade().deref()).as_ref().unwrap().borrow_mut()[(j as usize) as usize] = Edge {
                u: { (*tmp.borrow()).u },
                v: { (*tmp.borrow()).v },
                weight: { (*tmp.borrow()).weight },
            };
            i.postfix_inc();
            j.postfix_dec();
        }
    }
    return pidx;
}
pub fn quicksort_1(arr: Ptr<Option<Value<Box<[Edge]>>>>, mut start: i32, mut end: i32) {
    if (start >= end) {
        return;
    }
    let mut p: i32 = ({
        let _arr: Ptr<Option<Value<Box<[Edge]>>>> = (arr).clone();
        let _start: i32 = start;
        let _end: i32 = end;
        partition_0(_arr, _start, _end)
    });
    ({
        let _arr: Ptr<Option<Value<Box<[Edge]>>>> = (arr).clone();
        let _start: i32 = start;
        let _end: i32 = (p - 1);
        quicksort_1(_arr, _start, _end)
    });
    ({
        let _arr: Ptr<Option<Value<Box<[Edge]>>>> = (arr).clone();
        let _start: i32 = (p + 1);
        let _end: i32 = end;
        quicksort_1(_arr, _start, _end)
    });
}
#[derive(Record, ByteRepr, Default)]
#[byte_size(24)]
pub struct DisjointSet {
    #[offset(0)]
    #[byte_size(8)]
    pub rank: Option<Value<Box<[i32]>>>,
    #[offset(8)]
    #[byte_size(8)]
    pub parent: Option<Value<Box<[i32]>>>,
    #[offset(16)]
    pub n: i32,
}
impl DisjointSet {
    pub fn move_from(_a0: Ptr<DisjointSet>) -> Self {
        Self {
            rank: field!(_a0, rank).with_mut(|__v: &mut Option<Value<Box<[i32]>>>| __v.take()),
            parent: field!(_a0, parent).with_mut(|__v: &mut Option<Value<Box<[i32]>>>| __v.take()),
            n: { (*_a0.upgrade().deref()).n },
        }
    }
}
#[derive(Record, ByteRepr, Default)]
#[byte_size(16)]
pub struct Graph {
    #[offset(0)]
    #[byte_size(8)]
    pub edges: Option<Value<Box<[Edge]>>>,
    #[offset(8)]
    pub V: i32,
    #[offset(12)]
    pub E: i32,
}
impl Graph {
    pub fn move_from(_a0: Ptr<Graph>) -> Self {
        Self {
            edges: field!(_a0, edges).with_mut(|__v: &mut Option<Value<Box<[Edge]>>>| __v.take()),
            V: { (*_a0.upgrade().deref()).V },
            E: { (*_a0.upgrade().deref()).E },
        }
    }
}
pub fn MSTKruskal_2(graph: Ptr<Graph>) -> f64 {
    ({
        let _arr: Ptr<Option<Value<Box<[Edge]>>>> = field_ptr!(graph, edges);
        let _end: i32 = (graph.with(|__s| __s.E) - 1);
        quicksort_1(_arr, 0, _end)
    });
    let set: Value<DisjointSet> = Rc::new(RefCell::new(DisjointSet {
        rank: Some(Rc::new(RefCell::new(
            (0..(graph.with(|__s| __s.V) as usize))
                .map(|_| <i32>::default())
                .collect::<Box<[_]>>(),
        ))),
        parent: Some(Rc::new(RefCell::new(
            (0..(graph.with(|__s| __s.V) as usize))
                .map(|_| <i32>::default())
                .collect::<Box<[_]>>(),
        ))),
        n: graph.with(|__s| __s.V),
    }));
    ({ DisjointSetImpl::makeSet(&set.as_pointer()) });
    let mut total_weight: f64 = 0_f64;
    let mut i: i32 = 0;
    'loop_: while ({ i } < { graph.with(|__s| __s.E) }) {
        let mut x: i32 = {
            graph
                .with(|__s| __s.edges.clone())
                .as_ref()
                .unwrap()
                .borrow()[(i as usize) as usize]
                .u
        };
        let mut y: i32 = {
            graph
                .with(|__s| __s.edges.clone())
                .as_ref()
                .unwrap()
                .borrow()[(i as usize) as usize]
                .v
        };
        let mut w: f64 = {
            graph
                .with(|__s| __s.edges.clone())
                .as_ref()
                .unwrap()
                .borrow()[(i as usize) as usize]
                .weight
        };
        if (({ DisjointSetImpl::find(&set.as_pointer(), x) })
            != ({ DisjointSetImpl::find(&set.as_pointer(), y) }))
        {
            ({ DisjointSetImpl::merge(&set.as_pointer(), x, y) });
            total_weight += w;
        }
        i.prefix_inc();
    }
    return total_weight;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let V: Value<i32> = Rc::new(RefCell::new(4));
    let E: Value<i32> = Rc::new(RefCell::new(5));
    let graph: Value<Graph> = Rc::new(RefCell::new(Graph {
        edges: Some(Rc::new(RefCell::new(
            (0..((*E.borrow()) as usize))
                .map(|_| <Edge>::default())
                .collect::<Box<[_]>>(),
        ))),
        V: (*V.borrow()),
        E: (*E.borrow()),
    }));
    { (*graph.borrow()).edges.clone() }
        .as_ref()
        .unwrap()
        .borrow_mut()[(0_usize) as usize] = Edge {
        u: 0,
        v: 1,
        weight: 10_f64,
    };
    { (*graph.borrow()).edges.clone() }
        .as_ref()
        .unwrap()
        .borrow_mut()[(1_usize) as usize] = Edge {
        u: 1,
        v: 3,
        weight: 15_f64,
    };
    { (*graph.borrow()).edges.clone() }
        .as_ref()
        .unwrap()
        .borrow_mut()[(2_usize) as usize] = Edge {
        u: 2,
        v: 3,
        weight: 4_f64,
    };
    { (*graph.borrow()).edges.clone() }
        .as_ref()
        .unwrap()
        .borrow_mut()[(3_usize) as usize] = Edge {
        u: 2,
        v: 0,
        weight: 6_f64,
    };
    { (*graph.borrow()).edges.clone() }
        .as_ref()
        .unwrap()
        .borrow_mut()[(4_usize) as usize] = Edge {
        u: 0,
        v: 3,
        weight: 5_f64,
    };
    let mut total_weight: f64 = ({ MSTKruskal_2(graph.as_pointer()) });
    assert!((total_weight == 19_f64));
    return 0;
}
pub trait DisjointSetImpl {
    fn makeSet(&self);
    fn find(&self, x: i32) -> i32;
    fn merge(&self, x: i32, y: i32);
    fn move_assign(&self, _a0: Ptr<DisjointSet>) -> Ptr<DisjointSet>;
}
impl DisjointSetImpl for Ptr<DisjointSet> {
    fn makeSet(&self) {
        let mut i: i32 = 0;
        'loop_: while (i < (*self).with(|__s| __s.n)) {
            let __rhs = i;
            (*self)
                .with(|__s| __s.parent.clone())
                .as_ref()
                .unwrap()
                .borrow_mut()[(i as usize) as usize] = __rhs;
            (*self)
                .with(|__s| __s.rank.clone())
                .as_ref()
                .unwrap()
                .borrow_mut()[(i as usize) as usize] = 1;
            i.postfix_inc();
        }
    }
    fn find(&self, mut x: i32) -> i32 {
        if ((*self)
            .with(|__s| __s.parent.clone())
            .as_ref()
            .unwrap()
            .borrow()[(x as usize) as usize]
            != x)
        {
            let __rhs = ({
                let _x: i32 = (*self)
                    .with(|__s| __s.parent.clone())
                    .as_ref()
                    .unwrap()
                    .borrow()[(x as usize) as usize];
                DisjointSetImpl::find(self, _x)
            });
            (*self)
                .with(|__s| __s.parent.clone())
                .as_ref()
                .unwrap()
                .borrow_mut()[(x as usize) as usize] = __rhs;
        }
        return (*self)
            .with(|__s| __s.parent.clone())
            .as_ref()
            .unwrap()
            .borrow()[(x as usize) as usize];
    }
    fn merge(&self, mut x: i32, mut y: i32) {
        let mut xset: i32 = ({ DisjointSetImpl::find(self, x) });
        let mut yset: i32 = ({ DisjointSetImpl::find(self, y) });
        if (xset == yset) {
            return;
        }
        if ((*self)
            .with(|__s| __s.rank.clone())
            .as_ref()
            .unwrap()
            .borrow()[(xset as usize) as usize]
            < (*self)
                .with(|__s| __s.rank.clone())
                .as_ref()
                .unwrap()
                .borrow()[(yset as usize) as usize])
        {
            (*self)
                .with(|__s| __s.parent.clone())
                .as_ref()
                .unwrap()
                .borrow_mut()[(xset as usize) as usize] = yset;
        } else if ((*self)
            .with(|__s| __s.rank.clone())
            .as_ref()
            .unwrap()
            .borrow()[(xset as usize) as usize]
            > (*self)
                .with(|__s| __s.rank.clone())
                .as_ref()
                .unwrap()
                .borrow()[(yset as usize) as usize])
        {
            (*self)
                .with(|__s| __s.parent.clone())
                .as_ref()
                .unwrap()
                .borrow_mut()[(yset as usize) as usize] = xset;
        } else {
            (*self)
                .with(|__s| __s.parent.clone())
                .as_ref()
                .unwrap()
                .borrow_mut()[(yset as usize) as usize] = xset;
            let __rhs = ((*self)
                .with(|__s| __s.rank.clone())
                .as_ref()
                .unwrap()
                .borrow()[(xset as usize) as usize]
                + 1);
            (*self)
                .with(|__s| __s.rank.clone())
                .as_ref()
                .unwrap()
                .borrow_mut()[(xset as usize) as usize] = __rhs;
        }
    }
    fn move_assign(&self, _a0: Ptr<DisjointSet>) -> Ptr<DisjointSet> {
        (field_ptr!((*self), rank) as Ptr<Option<Value<Box<[i32]>>>>)
            .write(field!(_a0, rank).with_mut(|__v: &mut Option<Value<Box<[i32]>>>| __v.take()));
        (field_ptr!((*self), parent) as Ptr<Option<Value<Box<[i32]>>>>)
            .write(field!(_a0, parent).with_mut(|__v: &mut Option<Value<Box<[i32]>>>| __v.take()));
        field!((*self), n).write({ { (*_a0.upgrade().deref()).n } });
        return (*self).clone();
    }
}
pub trait GraphImpl {
    fn move_assign(&self, _a0: Ptr<Graph>) -> Ptr<Graph>;
}
impl GraphImpl for Ptr<Graph> {
    fn move_assign(&self, _a0: Ptr<Graph>) -> Ptr<Graph> {
        (field_ptr!((*self), edges) as Ptr<Option<Value<Box<[Edge]>>>>)
            .write(field!(_a0, edges).with_mut(|__v: &mut Option<Value<Box<[Edge]>>>| __v.take()));
        field!((*self), V).write({ { (*_a0.upgrade().deref()).V } });
        field!((*self), E).write({ { (*_a0.upgrade().deref()).E } });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
