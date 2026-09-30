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
pub fn partition_0(arr: Ptr<Option<Value<Box<[Edge]>>>>, start: i32, end: i32) -> i32 {
    let start: Value<i32> = Rc::new(RefCell::new(start));
    let end: Value<i32> = Rc::new(RefCell::new(end));
    let pivot: Ptr<Edge> = ((*arr.upgrade().deref())
        .as_ref()
        .unwrap()
        .as_pointer()
        .offset(((*start.borrow()) as usize)))
    .clone();
    let count: Value<i32> = Rc::new(RefCell::new(0));
    let i: Value<i32> = Rc::new(RefCell::new(((*start.borrow()) + 1)));
    'loop_: while ((*i.borrow()) <= (*end.borrow())) {
        if {
            let _lhs = {
                (*arr.upgrade().deref()).as_ref().unwrap().borrow()
                    [((*i.borrow()) as usize) as usize]
                    .weight
            };
            _lhs <= pivot.with(|__s| __s.weight)
        } {
            (*count.borrow_mut()).postfix_inc();
        }
        (*i.borrow_mut()).prefix_inc();
    }
    let pidx: Value<i32> = Rc::new(RefCell::new(((*start.borrow()) + (*count.borrow()))));
    let tmp: Value<Edge> = Rc::new(RefCell::new(Edge {
        u: {
            (*arr.upgrade().deref()).as_ref().unwrap().borrow()
                [((*pidx.borrow()) as usize) as usize]
                .u
        },
        v: {
            (*arr.upgrade().deref()).as_ref().unwrap().borrow()
                [((*pidx.borrow()) as usize) as usize]
                .v
        },
        weight: {
            (*arr.upgrade().deref()).as_ref().unwrap().borrow()
                [((*pidx.borrow()) as usize) as usize]
                .weight
        },
    }));
    let __rhs = Edge {
        u: {
            (*arr.upgrade().deref()).as_ref().unwrap().borrow()
                [((*start.borrow()) as usize) as usize]
                .u
        },
        v: {
            (*arr.upgrade().deref()).as_ref().unwrap().borrow()
                [((*start.borrow()) as usize) as usize]
                .v
        },
        weight: {
            (*arr.upgrade().deref()).as_ref().unwrap().borrow()
                [((*start.borrow()) as usize) as usize]
                .weight
        },
    };
    (*arr.upgrade().deref()).as_ref().unwrap().borrow_mut()[((*pidx.borrow()) as usize) as usize] =
        __rhs;
    (*arr.upgrade().deref()).as_ref().unwrap().borrow_mut()
        [((*start.borrow()) as usize) as usize] = Edge {
        u: { (*tmp.borrow()).u },
        v: { (*tmp.borrow()).v },
        weight: { (*tmp.borrow()).weight },
    };
    let i: Value<i32> = Rc::new(RefCell::new((*start.borrow())));
    let j: Value<i32> = Rc::new(RefCell::new((*end.borrow())));
    'loop_: while ((*i.borrow()) < (*pidx.borrow())) && ((*j.borrow()) > (*pidx.borrow())) {
        'loop_: while {
            let _lhs = {
                (*arr.upgrade().deref()).as_ref().unwrap().borrow()
                    [((*i.borrow()) as usize) as usize]
                    .weight
            };
            _lhs <= pivot.with(|__s| __s.weight)
        } {
            (*i.borrow_mut()).prefix_inc();
        }
        'loop_: while {
            let _lhs = {
                (*arr.upgrade().deref()).as_ref().unwrap().borrow()
                    [((*j.borrow()) as usize) as usize]
                    .weight
            };
            _lhs > pivot.with(|__s| __s.weight)
        } {
            (*j.borrow_mut()).prefix_dec();
        }
        if ((*i.borrow()) < (*pidx.borrow())) && ((*j.borrow()) > (*pidx.borrow())) {
            (*tmp.borrow_mut()) = Edge {
                u: {
                    (*arr.upgrade().deref()).as_ref().unwrap().borrow()
                        [((*i.borrow()) as usize) as usize]
                        .u
                },
                v: {
                    (*arr.upgrade().deref()).as_ref().unwrap().borrow()
                        [((*i.borrow()) as usize) as usize]
                        .v
                },
                weight: {
                    (*arr.upgrade().deref()).as_ref().unwrap().borrow()
                        [((*i.borrow()) as usize) as usize]
                        .weight
                },
            };
            let __rhs = Edge {
                u: {
                    (*arr.upgrade().deref()).as_ref().unwrap().borrow()
                        [((*j.borrow()) as usize) as usize]
                        .u
                },
                v: {
                    (*arr.upgrade().deref()).as_ref().unwrap().borrow()
                        [((*j.borrow()) as usize) as usize]
                        .v
                },
                weight: {
                    (*arr.upgrade().deref()).as_ref().unwrap().borrow()
                        [((*j.borrow()) as usize) as usize]
                        .weight
                },
            };
            (*arr.upgrade().deref()).as_ref().unwrap().borrow_mut()
                [((*i.borrow()) as usize) as usize] = __rhs;
            (*arr.upgrade().deref()).as_ref().unwrap().borrow_mut()
                [((*j.borrow()) as usize) as usize] = Edge {
                u: { (*tmp.borrow()).u },
                v: { (*tmp.borrow()).v },
                weight: { (*tmp.borrow()).weight },
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
        let __this: Value<DisjointSet> = Rc::new(RefCell::new(Self {
            rank: field!(_a0, rank).with_mut(|__v: &mut Option<Value<Box<[i32]>>>| __v.take()),
            parent: field!(_a0, parent).with_mut(|__v: &mut Option<Value<Box<[i32]>>>| __v.take()),
            n: { (*_a0.upgrade().deref()).n },
        }));
        let this: Ptr<DisjointSet> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
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
        let __this: Value<Graph> = Rc::new(RefCell::new(Self {
            edges: field!(_a0, edges).with_mut(|__v: &mut Option<Value<Box<[Edge]>>>| __v.take()),
            V: { (*_a0.upgrade().deref()).V },
            E: { (*_a0.upgrade().deref()).E },
        }));
        let this: Ptr<Graph> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
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
    let total_weight: Value<f64> = Rc::new(RefCell::new(0_f64));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while {
        let _lhs = (*i.borrow());
        _lhs < graph.with(|__s| __s.E)
    } {
        let x: Value<i32> = Rc::new(RefCell::new({
            graph
                .with(|__s| __s.edges.clone())
                .as_ref()
                .unwrap()
                .borrow()[((*i.borrow()) as usize) as usize]
                .u
        }));
        let y: Value<i32> = Rc::new(RefCell::new({
            graph
                .with(|__s| __s.edges.clone())
                .as_ref()
                .unwrap()
                .borrow()[((*i.borrow()) as usize) as usize]
                .v
        }));
        let w: Value<f64> = Rc::new(RefCell::new({
            graph
                .with(|__s| __s.edges.clone())
                .as_ref()
                .unwrap()
                .borrow()[((*i.borrow()) as usize) as usize]
                .weight
        }));
        if (({ DisjointSetImpl::find(&set.as_pointer(), (*x.borrow())) })
            != ({ DisjointSetImpl::find(&set.as_pointer(), (*y.borrow())) }))
        {
            ({ DisjointSetImpl::merge(&set.as_pointer(), (*x.borrow()), (*y.borrow())) });
            (*total_weight.borrow_mut()) += (*w.borrow());
        }
        (*i.borrow_mut()).prefix_inc();
    }
    return (*total_weight.borrow());
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
    let total_weight: Value<f64> = Rc::new(RefCell::new(({ MSTKruskal_2(graph.as_pointer()) })));
    assert!(((*total_weight.borrow()) == 19_f64));
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
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < (*self).with(|__s| __s.n)) {
            let __rhs = (*i.borrow());
            (*self)
                .with(|__s| __s.parent.clone())
                .as_ref()
                .unwrap()
                .borrow_mut()[((*i.borrow()) as usize) as usize] = __rhs;
            (*self)
                .with(|__s| __s.rank.clone())
                .as_ref()
                .unwrap()
                .borrow_mut()[((*i.borrow()) as usize) as usize] = 1;
            (*i.borrow_mut()).postfix_inc();
        }
    }
    fn find(&self, x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        if ((*self)
            .with(|__s| __s.parent.clone())
            .as_ref()
            .unwrap()
            .borrow()[((*x.borrow()) as usize) as usize]
            != (*x.borrow()))
        {
            let __rhs = ({
                let _x: i32 = (*self)
                    .with(|__s| __s.parent.clone())
                    .as_ref()
                    .unwrap()
                    .borrow()[((*x.borrow()) as usize) as usize];
                DisjointSetImpl::find(self, _x)
            });
            (*self)
                .with(|__s| __s.parent.clone())
                .as_ref()
                .unwrap()
                .borrow_mut()[((*x.borrow()) as usize) as usize] = __rhs;
        }
        return (*self)
            .with(|__s| __s.parent.clone())
            .as_ref()
            .unwrap()
            .borrow()[((*x.borrow()) as usize) as usize];
    }
    fn merge(&self, x: i32, y: i32) {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        let y: Value<i32> = Rc::new(RefCell::new(y));
        let xset: Value<i32> = Rc::new(RefCell::new(
            ({ DisjointSetImpl::find(self, (*x.borrow())) }),
        ));
        let yset: Value<i32> = Rc::new(RefCell::new(
            ({ DisjointSetImpl::find(self, (*y.borrow())) }),
        ));
        if ((*xset.borrow()) == (*yset.borrow())) {
            return;
        }
        if ((*self)
            .with(|__s| __s.rank.clone())
            .as_ref()
            .unwrap()
            .borrow()[((*xset.borrow()) as usize) as usize]
            < (*self)
                .with(|__s| __s.rank.clone())
                .as_ref()
                .unwrap()
                .borrow()[((*yset.borrow()) as usize) as usize])
        {
            (*self)
                .with(|__s| __s.parent.clone())
                .as_ref()
                .unwrap()
                .borrow_mut()[((*xset.borrow()) as usize) as usize] = (*yset.borrow());
        } else if ((*self)
            .with(|__s| __s.rank.clone())
            .as_ref()
            .unwrap()
            .borrow()[((*xset.borrow()) as usize) as usize]
            > (*self)
                .with(|__s| __s.rank.clone())
                .as_ref()
                .unwrap()
                .borrow()[((*yset.borrow()) as usize) as usize])
        {
            (*self)
                .with(|__s| __s.parent.clone())
                .as_ref()
                .unwrap()
                .borrow_mut()[((*yset.borrow()) as usize) as usize] = (*xset.borrow());
        } else {
            (*self)
                .with(|__s| __s.parent.clone())
                .as_ref()
                .unwrap()
                .borrow_mut()[((*yset.borrow()) as usize) as usize] = (*xset.borrow());
            let __rhs = ((*self)
                .with(|__s| __s.rank.clone())
                .as_ref()
                .unwrap()
                .borrow()[((*xset.borrow()) as usize) as usize]
                + 1);
            (*self)
                .with(|__s| __s.rank.clone())
                .as_ref()
                .unwrap()
                .borrow_mut()[((*xset.borrow()) as usize) as usize] = __rhs;
        }
    }
    fn move_assign(&self, _a0: Ptr<DisjointSet>) -> Ptr<DisjointSet> {
        (field_ptr!((*self), rank) as Ptr<Option<Value<Box<[i32]>>>>)
            .write(field!(_a0, rank).with_mut(|__v: &mut Option<Value<Box<[i32]>>>| __v.take()));
        (field_ptr!((*self), parent) as Ptr<Option<Value<Box<[i32]>>>>)
            .write(field!(_a0, parent).with_mut(|__v: &mut Option<Value<Box<[i32]>>>| __v.take()));
        let __rhs = { (*_a0.upgrade().deref()).n };
        field!((*self), n).write(__rhs);
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
        let __rhs = { (*_a0.upgrade().deref()).V };
        field!((*self), V).write(__rhs);
        let __rhs = { (*_a0.upgrade().deref()).E };
        field!((*self), E).write(__rhs);
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
