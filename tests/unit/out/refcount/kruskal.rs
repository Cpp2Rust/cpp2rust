extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Edge {
    pub u: Value<i32>,
    pub v: Value<i32>,
    pub weight: Value<f64>,
}
impl Clone for Edge {
    fn clone(&self) -> Self {
        let mut this = Self {
            u: Rc::new(RefCell::new((*self.u.borrow()))),
            v: Rc::new(RefCell::new((*self.v.borrow()))),
            weight: Rc::new(RefCell::new((*self.weight.borrow()))),
        };
        this
    }
}
impl ByteRepr for Edge {}
pub fn partition_0(arr: Ptr<Option<Value<Box<[Edge]>>>>, start: i32, end: i32) -> i32 {
    let start: Value<i32> = Rc::new(RefCell::new(start));
    let end: Value<i32> = Rc::new(RefCell::new(end));
    let pivot: Ptr<Edge> = ((*arr.upgrade().deref())
        .as_ref()
        .unwrap()
        .as_pointer()
        .offset(((*start.borrow()) as u64) as isize))
    .clone();
    let count: Value<i32> = Rc::new(RefCell::new(0));
    let i: Value<i32> = Rc::new(RefCell::new(((*start.borrow()) + 1)));
    'loop_: while ((*i.borrow()) <= (*end.borrow())) {
        if {
            let _lhs = (*(*arr.upgrade().deref()).as_ref().unwrap().borrow()
                [((*i.borrow()) as u64) as usize]
                .weight
                .borrow());
            _lhs <= (*(*pivot.upgrade().deref()).weight.borrow())
        } {
            (*count.borrow_mut()).postfix_inc();
        }
        (*i.borrow_mut()).prefix_inc();
    }
    let pidx: Value<i32> = Rc::new(RefCell::new(((*start.borrow()) + (*count.borrow()))));
    let tmp: Value<Edge> = Rc::new(RefCell::new(Edge {
        u: Rc::new(RefCell::new(
            (*(*arr.upgrade().deref()).as_ref().unwrap().borrow()
                [((*pidx.borrow()) as u64) as usize]
                .u
                .borrow()),
        )),
        v: Rc::new(RefCell::new(
            (*(*arr.upgrade().deref()).as_ref().unwrap().borrow()
                [((*pidx.borrow()) as u64) as usize]
                .v
                .borrow()),
        )),
        weight: Rc::new(RefCell::new(
            (*(*arr.upgrade().deref()).as_ref().unwrap().borrow()
                [((*pidx.borrow()) as u64) as usize]
                .weight
                .borrow()),
        )),
    }));
    let __rhs = Edge {
        u: Rc::new(RefCell::new(
            (*(*arr.upgrade().deref()).as_ref().unwrap().borrow()
                [((*start.borrow()) as u64) as usize]
                .u
                .borrow()),
        )),
        v: Rc::new(RefCell::new(
            (*(*arr.upgrade().deref()).as_ref().unwrap().borrow()
                [((*start.borrow()) as u64) as usize]
                .v
                .borrow()),
        )),
        weight: Rc::new(RefCell::new(
            (*(*arr.upgrade().deref()).as_ref().unwrap().borrow()
                [((*start.borrow()) as u64) as usize]
                .weight
                .borrow()),
        )),
    };
    (*arr.upgrade().deref()).as_ref().unwrap().borrow_mut()[((*pidx.borrow()) as u64) as usize] =
        __rhs;
    (*arr.upgrade().deref()).as_ref().unwrap().borrow_mut()[((*start.borrow()) as u64) as usize] =
        Edge {
            u: Rc::new(RefCell::new((*(*tmp.borrow()).u.borrow()))),
            v: Rc::new(RefCell::new((*(*tmp.borrow()).v.borrow()))),
            weight: Rc::new(RefCell::new((*(*tmp.borrow()).weight.borrow()))),
        };
    let i: Value<i32> = Rc::new(RefCell::new((*start.borrow())));
    let j: Value<i32> = Rc::new(RefCell::new((*end.borrow())));
    'loop_: while (((*i.borrow()) < (*pidx.borrow())) && ((*j.borrow()) > (*pidx.borrow()))) {
        'loop_: while {
            let _lhs = (*(*arr.upgrade().deref()).as_ref().unwrap().borrow()
                [((*i.borrow()) as u64) as usize]
                .weight
                .borrow());
            _lhs <= (*(*pivot.upgrade().deref()).weight.borrow())
        } {
            (*i.borrow_mut()).prefix_inc();
        }
        'loop_: while {
            let _lhs = (*(*arr.upgrade().deref()).as_ref().unwrap().borrow()
                [((*j.borrow()) as u64) as usize]
                .weight
                .borrow());
            _lhs > (*(*pivot.upgrade().deref()).weight.borrow())
        } {
            (*j.borrow_mut()).prefix_dec();
        }
        if (((*i.borrow()) < (*pidx.borrow())) && ((*j.borrow()) > (*pidx.borrow()))) {
            (*tmp.borrow_mut()) = Edge {
                u: Rc::new(RefCell::new(
                    (*(*arr.upgrade().deref()).as_ref().unwrap().borrow()
                        [((*i.borrow()) as u64) as usize]
                        .u
                        .borrow()),
                )),
                v: Rc::new(RefCell::new(
                    (*(*arr.upgrade().deref()).as_ref().unwrap().borrow()
                        [((*i.borrow()) as u64) as usize]
                        .v
                        .borrow()),
                )),
                weight: Rc::new(RefCell::new(
                    (*(*arr.upgrade().deref()).as_ref().unwrap().borrow()
                        [((*i.borrow()) as u64) as usize]
                        .weight
                        .borrow()),
                )),
            };
            let __rhs = Edge {
                u: Rc::new(RefCell::new(
                    (*(*arr.upgrade().deref()).as_ref().unwrap().borrow()
                        [((*j.borrow()) as u64) as usize]
                        .u
                        .borrow()),
                )),
                v: Rc::new(RefCell::new(
                    (*(*arr.upgrade().deref()).as_ref().unwrap().borrow()
                        [((*j.borrow()) as u64) as usize]
                        .v
                        .borrow()),
                )),
                weight: Rc::new(RefCell::new(
                    (*(*arr.upgrade().deref()).as_ref().unwrap().borrow()
                        [((*j.borrow()) as u64) as usize]
                        .weight
                        .borrow()),
                )),
            };
            (*arr.upgrade().deref()).as_ref().unwrap().borrow_mut()
                [((*i.borrow()) as u64) as usize] = __rhs;
            (*arr.upgrade().deref()).as_ref().unwrap().borrow_mut()
                [((*j.borrow()) as u64) as usize] = Edge {
                u: Rc::new(RefCell::new((*(*tmp.borrow()).u.borrow()))),
                v: Rc::new(RefCell::new((*(*tmp.borrow()).v.borrow()))),
                weight: Rc::new(RefCell::new((*(*tmp.borrow()).weight.borrow()))),
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
#[derive(Default)]
pub struct DisjointSet {
    pub rank: Value<Option<Value<Box<[i32]>>>>,
    pub parent: Value<Option<Value<Box<[i32]>>>>,
    pub n: Value<i32>,
}
impl DisjointSet {
    pub fn makeSet(&self) {
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < (*self.n.borrow())) {
            let __rhs = (*i.borrow());
            (*self.parent.borrow()).as_ref().unwrap().borrow_mut()
                [((*i.borrow()) as u64) as usize] = __rhs;
            (*self.rank.borrow()).as_ref().unwrap().borrow_mut()[((*i.borrow()) as u64) as usize] =
                1;
            (*i.borrow_mut()).postfix_inc();
        }
    }
    pub fn find(&self, x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        if ((*self.parent.borrow()).as_ref().unwrap().borrow()[((*x.borrow()) as u64) as usize]
            != (*x.borrow()))
        {
            let __rhs = ({
                let _x: i32 = (*self.parent.borrow()).as_ref().unwrap().borrow()
                    [((*x.borrow()) as u64) as usize];
                self.find(_x)
            });
            (*self.parent.borrow()).as_ref().unwrap().borrow_mut()
                [((*x.borrow()) as u64) as usize] = __rhs;
        }
        return (*self.parent.borrow()).as_ref().unwrap().borrow()[((*x.borrow()) as u64) as usize];
    }
    pub fn merge(&self, x: i32, y: i32) {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        let y: Value<i32> = Rc::new(RefCell::new(y));
        let xset: Value<i32> = Rc::new(RefCell::new(
            ({
                let _x: i32 = (*x.borrow());
                self.find(_x)
            }),
        ));
        let yset: Value<i32> = Rc::new(RefCell::new(
            ({
                let _x: i32 = (*y.borrow());
                self.find(_x)
            }),
        ));
        if ((*xset.borrow()) == (*yset.borrow())) {
            return;
        }
        if ((*self.rank.borrow()).as_ref().unwrap().borrow()[((*xset.borrow()) as u64) as usize]
            < (*self.rank.borrow()).as_ref().unwrap().borrow()[((*yset.borrow()) as u64) as usize])
        {
            (*self.parent.borrow()).as_ref().unwrap().borrow_mut()
                [((*xset.borrow()) as u64) as usize] = (*yset.borrow());
        } else if ((*self.rank.borrow()).as_ref().unwrap().borrow()
            [((*xset.borrow()) as u64) as usize]
            > (*self.rank.borrow()).as_ref().unwrap().borrow()[((*yset.borrow()) as u64) as usize])
        {
            (*self.parent.borrow()).as_ref().unwrap().borrow_mut()
                [((*yset.borrow()) as u64) as usize] = (*xset.borrow());
        } else {
            (*self.parent.borrow()).as_ref().unwrap().borrow_mut()
                [((*yset.borrow()) as u64) as usize] = (*xset.borrow());
            let __rhs = ((*self.rank.borrow()).as_ref().unwrap().borrow()
                [((*xset.borrow()) as u64) as usize]
                + 1);
            (*self.rank.borrow()).as_ref().unwrap().borrow_mut()
                [((*xset.borrow()) as u64) as usize] = __rhs;
        }
    }
}
impl ByteRepr for DisjointSet {}
#[derive(Default)]
pub struct Graph {
    pub edges: Value<Option<Value<Box<[Edge]>>>>,
    pub V: Value<i32>,
    pub E: Value<i32>,
}
impl ByteRepr for Graph {}
pub fn MSTKruskal_2(graph: Ptr<Graph>) -> f64 {
    ({
        let _arr: Ptr<Option<Value<Box<[Edge]>>>> = (*graph.upgrade().deref()).edges.as_pointer();
        let _start: i32 = 0;
        let _end: i32 = ((*(*graph.upgrade().deref()).E.borrow()) - 1);
        quicksort_1(_arr, _start, _end)
    });
    let set: Value<DisjointSet> = Rc::new(RefCell::new(DisjointSet {
        rank: Rc::new(RefCell::new(Some(Rc::new(RefCell::new(
            (0..((*(*graph.upgrade().deref()).V.borrow()) as u64))
                .map(|_| <i32>::default())
                .collect::<Box<[_]>>(),
        ))))),
        parent: Rc::new(RefCell::new(Some(Rc::new(RefCell::new(
            (0..((*(*graph.upgrade().deref()).V.borrow()) as u64))
                .map(|_| <i32>::default())
                .collect::<Box<[_]>>(),
        ))))),
        n: Rc::new(RefCell::new((*(*graph.upgrade().deref()).V.borrow()))),
    }));
    ({ (*set.borrow()).makeSet() });
    let total_weight: Value<f64> = Rc::new(RefCell::new(0_f64));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while {
        let _lhs = (*i.borrow());
        _lhs < (*(*graph.upgrade().deref()).E.borrow())
    } {
        let x: Value<i32> = Rc::new(RefCell::new(
            (*(*(*graph.upgrade().deref()).edges.borrow())
                .as_ref()
                .unwrap()
                .borrow()[((*i.borrow()) as u64) as usize]
                .u
                .borrow()),
        ));
        let y: Value<i32> = Rc::new(RefCell::new(
            (*(*(*graph.upgrade().deref()).edges.borrow())
                .as_ref()
                .unwrap()
                .borrow()[((*i.borrow()) as u64) as usize]
                .v
                .borrow()),
        ));
        let w: Value<f64> = Rc::new(RefCell::new(
            (*(*(*graph.upgrade().deref()).edges.borrow())
                .as_ref()
                .unwrap()
                .borrow()[((*i.borrow()) as u64) as usize]
                .weight
                .borrow()),
        ));
        if (({
            let _x: i32 = (*x.borrow());
            (*set.borrow()).find(_x)
        }) != ({
            let _x: i32 = (*y.borrow());
            (*set.borrow()).find(_x)
        })) {
            ({
                let _x: i32 = (*x.borrow());
                let _y: i32 = (*y.borrow());
                (*set.borrow()).merge(_x, _y)
            });
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
        edges: Rc::new(RefCell::new(Some(Rc::new(RefCell::new(
            (0..((*E.borrow()) as u64))
                .map(|_| <Edge>::default())
                .collect::<Box<[_]>>(),
        ))))),
        V: Rc::new(RefCell::new((*V.borrow()))),
        E: Rc::new(RefCell::new((*E.borrow()))),
    }));
    (*(*graph.borrow()).edges.borrow())
        .as_ref()
        .unwrap()
        .borrow_mut()[(0_u64) as usize] = Edge {
        u: Rc::new(RefCell::new(0)),
        v: Rc::new(RefCell::new(1)),
        weight: Rc::new(RefCell::new(10_f64)),
    };
    (*(*graph.borrow()).edges.borrow())
        .as_ref()
        .unwrap()
        .borrow_mut()[(1_u64) as usize] = Edge {
        u: Rc::new(RefCell::new(1)),
        v: Rc::new(RefCell::new(3)),
        weight: Rc::new(RefCell::new(15_f64)),
    };
    (*(*graph.borrow()).edges.borrow())
        .as_ref()
        .unwrap()
        .borrow_mut()[(2_u64) as usize] = Edge {
        u: Rc::new(RefCell::new(2)),
        v: Rc::new(RefCell::new(3)),
        weight: Rc::new(RefCell::new(4_f64)),
    };
    (*(*graph.borrow()).edges.borrow())
        .as_ref()
        .unwrap()
        .borrow_mut()[(3_u64) as usize] = Edge {
        u: Rc::new(RefCell::new(2)),
        v: Rc::new(RefCell::new(0)),
        weight: Rc::new(RefCell::new(6_f64)),
    };
    (*(*graph.borrow()).edges.borrow())
        .as_ref()
        .unwrap()
        .borrow_mut()[(4_u64) as usize] = Edge {
        u: Rc::new(RefCell::new(0)),
        v: Rc::new(RefCell::new(3)),
        weight: Rc::new(RefCell::new(5_f64)),
    };
    let total_weight: Value<f64> = Rc::new(RefCell::new(
        ({
            let _graph: Ptr<Graph> = graph.as_pointer();
            MSTKruskal_2(_graph)
        }),
    ));
    return ((*total_weight.borrow()) as i32);
}
