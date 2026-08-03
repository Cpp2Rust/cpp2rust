extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[repr(C)]
#[derive(Default)]
pub struct Edge {
    pub u: i32,
    pub v: i32,
    pub weight: f64,
}
impl Clone for Edge {
    fn clone(&self) -> Self {
        let mut this = Self {
            u: self.u,
            v: self.v,
            weight: self.weight,
        };
        this
    }
}
impl ByteRepr for Edge {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.u.to_bytes(&mut buf[0..4]);
        self.v.to_bytes(&mut buf[4..8]);
        self.weight.to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            u: <i32>::from_bytes(&buf[0..4]),
            v: <i32>::from_bytes(&buf[4..8]),
            weight: <f64>::from_bytes(&buf[8..16]),
        }
    }
}
pub fn partition_0(arr: Ptr<Option<Value<Box<[Edge]>>>>, start: i32, end: i32) -> i32 {
    let start: Value<i32> = Rc::new(RefCell::new(start));
    let end: Value<i32> = Rc::new(RefCell::new(end));
    let pivot: Ptr<Edge> = (((arr.read()).as_ref().unwrap().as_pointer() as Ptr<Edge>)
        .offset(((*start.borrow()) as usize)))
    .clone();
    let count: Value<i32> = Rc::new(RefCell::new(0));
    let i: Value<i32> = Rc::new(RefCell::new(((*start.borrow()) + 1)));
    'loop_: while ((*i.borrow()) <= (*end.borrow())) {
        if {
            let _lhs = (*(arr.read()).as_ref().unwrap().borrow())
                [((*i.borrow()) as usize) as usize]
                .weight;
            _lhs <= pivot.with(|__v| (*__v).weight)
        } {
            (*count.borrow_mut()).postfix_inc();
        }
        (*i.borrow_mut()).prefix_inc();
    }
    let pidx: Value<i32> = Rc::new(RefCell::new(((*start.borrow()) + (*count.borrow()))));
    let tmp: Value<Edge> = Rc::new(RefCell::new(Edge {
        u: (*(arr.read()).as_ref().unwrap().borrow())[((*pidx.borrow()) as usize) as usize].u,
        v: (*(arr.read()).as_ref().unwrap().borrow())[((*pidx.borrow()) as usize) as usize].v,
        weight: (*(arr.read()).as_ref().unwrap().borrow())[((*pidx.borrow()) as usize) as usize]
            .weight,
    }));
    let __rhs = Edge {
        u: (*(arr.read()).as_ref().unwrap().borrow())[((*start.borrow()) as usize) as usize].u,
        v: (*(arr.read()).as_ref().unwrap().borrow())[((*start.borrow()) as usize) as usize].v,
        weight: (*(arr.read()).as_ref().unwrap().borrow())[((*start.borrow()) as usize) as usize]
            .weight,
    };
    (*(arr.read()).as_ref().unwrap().borrow_mut())[((*pidx.borrow()) as usize) as usize] = __rhs;
    (*(arr.read()).as_ref().unwrap().borrow_mut())[((*start.borrow()) as usize) as usize] = Edge {
        u: (*tmp.borrow()).u,
        v: (*tmp.borrow()).v,
        weight: (*tmp.borrow()).weight,
    };
    let i: Value<i32> = Rc::new(RefCell::new((*start.borrow())));
    let j: Value<i32> = Rc::new(RefCell::new((*end.borrow())));
    'loop_: while ((*i.borrow()) < (*pidx.borrow())) && ((*j.borrow()) > (*pidx.borrow())) {
        'loop_: while {
            let _lhs = (*(arr.read()).as_ref().unwrap().borrow())
                [((*i.borrow()) as usize) as usize]
                .weight;
            _lhs <= pivot.with(|__v| (*__v).weight)
        } {
            (*i.borrow_mut()).prefix_inc();
        }
        'loop_: while {
            let _lhs = (*(arr.read()).as_ref().unwrap().borrow())
                [((*j.borrow()) as usize) as usize]
                .weight;
            _lhs > pivot.with(|__v| (*__v).weight)
        } {
            (*j.borrow_mut()).prefix_dec();
        }
        if ((*i.borrow()) < (*pidx.borrow())) && ((*j.borrow()) > (*pidx.borrow())) {
            (*tmp.borrow_mut()) = Edge {
                u: (*(arr.read()).as_ref().unwrap().borrow())[((*i.borrow()) as usize) as usize].u,
                v: (*(arr.read()).as_ref().unwrap().borrow())[((*i.borrow()) as usize) as usize].v,
                weight: (*(arr.read()).as_ref().unwrap().borrow())
                    [((*i.borrow()) as usize) as usize]
                    .weight,
            };
            let __rhs = Edge {
                u: (*(arr.read()).as_ref().unwrap().borrow())[((*j.borrow()) as usize) as usize].u,
                v: (*(arr.read()).as_ref().unwrap().borrow())[((*j.borrow()) as usize) as usize].v,
                weight: (*(arr.read()).as_ref().unwrap().borrow())
                    [((*j.borrow()) as usize) as usize]
                    .weight,
            };
            (*(arr.read()).as_ref().unwrap().borrow_mut())[((*i.borrow()) as usize) as usize] =
                __rhs;
            (*(arr.read()).as_ref().unwrap().borrow_mut())[((*j.borrow()) as usize) as usize] =
                Edge {
                    u: (*tmp.borrow()).u,
                    v: (*tmp.borrow()).v,
                    weight: (*tmp.borrow()).weight,
                };
            (*i.borrow_mut()).postfix_inc();
            (*j.borrow_mut()).postfix_dec();
        }
    }
    return (*pidx.borrow());
}
pub fn quicksort_1(arr: Ptr<Option<Value<Box<[Edge]>>>>, start: i32, end: i32) {
    let start: Value<i32> = Rc::new(RefCell::new(start));
    let end: Value<i32> = Rc::new(RefCell::new(end));
    if ((*start.borrow()) >= (*end.borrow())) {
        return;
    }
    let p: Value<i32> = Rc::new(RefCell::new(
        ({
            let _arr: Ptr<Option<Value<Box<[Edge]>>>> = (arr).clone();
            let _start: i32 = (*start.borrow());
            let _end: i32 = (*end.borrow());
            partition_0(_arr, _start, _end)
        }),
    ));
    ({
        let _arr: Ptr<Option<Value<Box<[Edge]>>>> = (arr).clone();
        let _start: i32 = (*start.borrow());
        let _end: i32 = ((*p.borrow()) - 1);
        quicksort_1(_arr, _start, _end)
    });
    ({
        let _arr: Ptr<Option<Value<Box<[Edge]>>>> = (arr).clone();
        let _start: i32 = ((*p.borrow()) + 1);
        let _end: i32 = (*end.borrow());
        quicksort_1(_arr, _start, _end)
    });
}
#[repr(C)]
#[derive(Default)]
pub struct DisjointSet {
    pub rank: Option<Value<Box<[i32]>>>,
    pub parent: Option<Value<Box<[i32]>>>,
    pub n: i32,
}
pub trait DisjointSetMethods {
    fn makeSet(&self);
    fn find(&self, x: i32) -> i32;
    fn merge(&self, x: i32, y: i32);
}
impl DisjointSetMethods for Ptr<DisjointSet> {
    fn makeSet(&self) {
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < self.with(|__v| (*__v).n)) {
            let __rhs = (*i.borrow());
            (*self
                .with(|__v| (*__v).parent.clone())
                .as_ref()
                .unwrap()
                .borrow_mut())[((*i.borrow()) as usize) as usize] = __rhs;
            (*self
                .with(|__v| (*__v).rank.clone())
                .as_ref()
                .unwrap()
                .borrow_mut())[((*i.borrow()) as usize) as usize] = 1;
            (*i.borrow_mut()).postfix_inc();
        }
    }
    fn find(&self, x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        if ((*self
            .with(|__v| (*__v).parent.clone())
            .as_ref()
            .unwrap()
            .borrow())[((*x.borrow()) as usize) as usize]
            != (*x.borrow()))
        {
            let __rhs = ({
                let _x: i32 = (*self
                    .with(|__v| (*__v).parent.clone())
                    .as_ref()
                    .unwrap()
                    .borrow())[((*x.borrow()) as usize) as usize];
                self.find(_x)
            });
            (*self
                .with(|__v| (*__v).parent.clone())
                .as_ref()
                .unwrap()
                .borrow_mut())[((*x.borrow()) as usize) as usize] = __rhs;
        }
        return (*self
            .with(|__v| (*__v).parent.clone())
            .as_ref()
            .unwrap()
            .borrow())[((*x.borrow()) as usize) as usize];
    }
    fn merge(&self, x: i32, y: i32) {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        let y: Value<i32> = Rc::new(RefCell::new(y));
        let xset: Value<i32> = Rc::new(RefCell::new(({ self.find((*x.borrow())) })));
        let yset: Value<i32> = Rc::new(RefCell::new(({ self.find((*y.borrow())) })));
        if ((*xset.borrow()) == (*yset.borrow())) {
            return;
        }
        if ((*self
            .with(|__v| (*__v).rank.clone())
            .as_ref()
            .unwrap()
            .borrow())[((*xset.borrow()) as usize) as usize]
            < (*self
                .with(|__v| (*__v).rank.clone())
                .as_ref()
                .unwrap()
                .borrow())[((*yset.borrow()) as usize) as usize])
        {
            (*self
                .with(|__v| (*__v).parent.clone())
                .as_ref()
                .unwrap()
                .borrow_mut())[((*xset.borrow()) as usize) as usize] = (*yset.borrow());
        } else if ((*self
            .with(|__v| (*__v).rank.clone())
            .as_ref()
            .unwrap()
            .borrow())[((*xset.borrow()) as usize) as usize]
            > (*self
                .with(|__v| (*__v).rank.clone())
                .as_ref()
                .unwrap()
                .borrow())[((*yset.borrow()) as usize) as usize])
        {
            (*self
                .with(|__v| (*__v).parent.clone())
                .as_ref()
                .unwrap()
                .borrow_mut())[((*yset.borrow()) as usize) as usize] = (*xset.borrow());
        } else {
            (*self
                .with(|__v| (*__v).parent.clone())
                .as_ref()
                .unwrap()
                .borrow_mut())[((*yset.borrow()) as usize) as usize] = (*xset.borrow());
            let __rhs = ((*self
                .with(|__v| (*__v).rank.clone())
                .as_ref()
                .unwrap()
                .borrow())[((*xset.borrow()) as usize) as usize]
                + 1);
            (*self
                .with(|__v| (*__v).rank.clone())
                .as_ref()
                .unwrap()
                .borrow_mut())[((*xset.borrow()) as usize) as usize] = __rhs;
        }
    }
}
impl ByteRepr for DisjointSet {
    fn byte_size() -> usize {
        24
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.rank.to_bytes(&mut buf[0..8]);
        self.parent.to_bytes(&mut buf[8..16]);
        self.n.to_bytes(&mut buf[16..20]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            rank: <Option<Value<Box<[i32]>>>>::from_bytes(&buf[0..8]),
            parent: <Option<Value<Box<[i32]>>>>::from_bytes(&buf[8..16]),
            n: <i32>::from_bytes(&buf[16..20]),
        }
    }
}
#[repr(C)]
#[derive(Default)]
pub struct Graph {
    pub edges: Option<Value<Box<[Edge]>>>,
    pub V: i32,
    pub E: i32,
}
impl ByteRepr for Graph {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.edges.to_bytes(&mut buf[0..8]);
        self.V.to_bytes(&mut buf[8..12]);
        self.E.to_bytes(&mut buf[12..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            edges: <Option<Value<Box<[Edge]>>>>::from_bytes(&buf[0..8]),
            V: <i32>::from_bytes(&buf[8..12]),
            E: <i32>::from_bytes(&buf[12..16]),
        }
    }
}
pub fn MSTKruskal_2(graph: Ptr<Graph>) -> f64 {
    ({
        let _arr: Ptr<Option<Value<Box<[Edge]>>>> = graph.field_ptr(
            0,
            |__v: &Graph| ::std::slice::from_ref(&__v.edges),
            |__v: &mut Graph| ::std::slice::from_mut(&mut __v.edges),
        );
        let _end: i32 = (graph.with(|__v| (*__v).E) - 1);
        quicksort_1(_arr, 0, _end)
    });
    let set: Value<DisjointSet> = Rc::new(RefCell::new(DisjointSet {
        rank: Some(Rc::new(RefCell::new(
            (0..(graph.with(|__v| (*__v).V) as usize))
                .map(|_| <i32>::default())
                .collect::<Box<[_]>>(),
        ))),
        parent: Some(Rc::new(RefCell::new(
            (0..(graph.with(|__v| (*__v).V) as usize))
                .map(|_| <i32>::default())
                .collect::<Box<[_]>>(),
        ))),
        n: graph.with(|__v| (*__v).V),
    }));
    ({ set.as_pointer().makeSet() });
    let total_weight: Value<f64> = Rc::new(RefCell::new(0_f64));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while {
        let _lhs = (*i.borrow());
        _lhs < graph.with(|__v| (*__v).E)
    } {
        let x: Value<i32> = Rc::new(RefCell::new(
            (*graph
                .with(|__v| (*__v).edges.clone())
                .as_ref()
                .unwrap()
                .borrow())[((*i.borrow()) as usize) as usize]
                .u,
        ));
        let y: Value<i32> = Rc::new(RefCell::new(
            (*graph
                .with(|__v| (*__v).edges.clone())
                .as_ref()
                .unwrap()
                .borrow())[((*i.borrow()) as usize) as usize]
                .v,
        ));
        let w: Value<f64> = Rc::new(RefCell::new(
            (*graph
                .with(|__v| (*__v).edges.clone())
                .as_ref()
                .unwrap()
                .borrow())[((*i.borrow()) as usize) as usize]
                .weight,
        ));
        if (({ set.as_pointer().find((*x.borrow())) })
            != ({ set.as_pointer().find((*y.borrow())) }))
        {
            ({ set.as_pointer().merge((*x.borrow()), (*y.borrow())) });
            (*total_weight.borrow_mut()) += (*w.borrow());
        }
        (*i.borrow_mut()).prefix_inc();
    }
    return (*total_weight.borrow());
}
pub fn main() {
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
    (*(*graph.borrow()).edges.as_ref().unwrap().borrow_mut())[(0_usize) as usize] = Edge {
        u: 0,
        v: 1,
        weight: 10_f64,
    };
    (*(*graph.borrow()).edges.as_ref().unwrap().borrow_mut())[(1_usize) as usize] = Edge {
        u: 1,
        v: 3,
        weight: 15_f64,
    };
    (*(*graph.borrow()).edges.as_ref().unwrap().borrow_mut())[(2_usize) as usize] = Edge {
        u: 2,
        v: 3,
        weight: 4_f64,
    };
    (*(*graph.borrow()).edges.as_ref().unwrap().borrow_mut())[(3_usize) as usize] = Edge {
        u: 2,
        v: 0,
        weight: 6_f64,
    };
    (*(*graph.borrow()).edges.as_ref().unwrap().borrow_mut())[(4_usize) as usize] = Edge {
        u: 0,
        v: 3,
        weight: 5_f64,
    };
    let total_weight: Value<f64> = Rc::new(RefCell::new(({ MSTKruskal_2(graph.as_pointer()) })));
    return ((*total_weight.borrow()) as i32);
}
