extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(Copy, Clone, Default)]
pub struct Edge {
    pub u: i32,
    pub v: i32,
    pub weight: f64,
}
pub unsafe fn partition_0(arr: *mut Option<Box<[Edge]>>, mut start: i32, mut end: i32) -> i32 {
    let pivot: *mut Edge = &mut (*arr).as_mut().unwrap()[(start as u64) as usize] as *mut Edge;
    let mut count: i32 = 0;
    let mut i: i32 = ((start) + (1));
    'loop_: while ((i) <= (end)) {
        if (((*arr).as_mut().unwrap()[(i as u64) as usize].weight) <= ((*pivot).weight)) {
            count.postfix_inc();
        }
        i.prefix_inc();
    }
    let mut pidx: i32 = ((start) + (count));
    let mut tmp: Edge = Edge {
        u: (*arr).as_mut().unwrap()[(pidx as u64) as usize].u,
        v: (*arr).as_mut().unwrap()[(pidx as u64) as usize].v,
        weight: (*arr).as_mut().unwrap()[(pidx as u64) as usize].weight,
    };
    (*arr).as_mut().unwrap()[(pidx as u64) as usize] = Edge {
        u: (*arr).as_mut().unwrap()[(start as u64) as usize].u,
        v: (*arr).as_mut().unwrap()[(start as u64) as usize].v,
        weight: (*arr).as_mut().unwrap()[(start as u64) as usize].weight,
    };
    (*arr).as_mut().unwrap()[(start as u64) as usize] = Edge {
        u: tmp.u,
        v: tmp.v,
        weight: tmp.weight,
    };
    let mut i: i32 = start;
    let mut j: i32 = end;
    'loop_: while (((i) < (pidx)) && ((j) > (pidx))) {
        'loop_: while (((*arr).as_mut().unwrap()[(i as u64) as usize].weight) <= ((*pivot).weight))
        {
            i.prefix_inc();
        }
        'loop_: while (((*arr).as_mut().unwrap()[(j as u64) as usize].weight) > ((*pivot).weight)) {
            j.prefix_dec();
        }
        if (((i) < (pidx)) && ((j) > (pidx))) {
            tmp = Edge {
                u: (*arr).as_mut().unwrap()[(i as u64) as usize].u,
                v: (*arr).as_mut().unwrap()[(i as u64) as usize].v,
                weight: (*arr).as_mut().unwrap()[(i as u64) as usize].weight,
            };
            (*arr).as_mut().unwrap()[(i as u64) as usize] = Edge {
                u: (*arr).as_mut().unwrap()[(j as u64) as usize].u,
                v: (*arr).as_mut().unwrap()[(j as u64) as usize].v,
                weight: (*arr).as_mut().unwrap()[(j as u64) as usize].weight,
            };
            (*arr).as_mut().unwrap()[(j as u64) as usize] = Edge {
                u: tmp.u,
                v: tmp.v,
                weight: tmp.weight,
            };
            i.postfix_inc();
            j.postfix_dec();
        }
    }
    return pidx;
}
pub unsafe fn quicksort_1(arr: *mut Option<Box<[Edge]>>, mut start: i32, mut end: i32) {
    if ((start) >= (end)) {
        return;
    }
    let mut p: i32 = (unsafe {
        let _arr: *mut Option<Box<[Edge]>> = arr;
        let _start: i32 = start;
        let _end: i32 = end;
        partition_0(_arr, _start, _end)
    });
    (unsafe {
        let _arr: *mut Option<Box<[Edge]>> = arr;
        let _start: i32 = start;
        let _end: i32 = ((p) - (1));
        quicksort_1(_arr, _start, _end)
    });
    (unsafe {
        let _arr: *mut Option<Box<[Edge]>> = arr;
        let _start: i32 = ((p) + (1));
        let _end: i32 = end;
        quicksort_1(_arr, _start, _end)
    });
}
#[derive(Default)]
pub struct DisjointSet {
    pub rank: Option<Box<[i32]>>,
    pub parent: Option<Box<[i32]>>,
    pub n: i32,
}
impl DisjointSet {
    pub unsafe fn makeSet(&mut self) {
        let mut i: i32 = 0;
        'loop_: while ((i) < (self.n)) {
            self.parent.as_mut().unwrap()[(i as u64) as usize] = i;
            self.rank.as_mut().unwrap()[(i as u64) as usize] = 1;
            i.postfix_inc();
        }
    }
    pub unsafe fn find(&mut self, mut x: i32) -> i32 {
        if ((self.parent.as_mut().unwrap()[(x as u64) as usize]) != (x)) {
            self.parent.as_mut().unwrap()[(x as u64) as usize] = (unsafe {
                let _x: i32 = self.parent.as_mut().unwrap()[(x as u64) as usize];
                self.find(_x)
            });
        }
        return self.parent.as_mut().unwrap()[(x as u64) as usize];
    }
    pub unsafe fn merge(&mut self, mut x: i32, mut y: i32) {
        let mut xset: i32 = (unsafe {
            let _x: i32 = x;
            self.find(_x)
        });
        let mut yset: i32 = (unsafe {
            let _x: i32 = y;
            self.find(_x)
        });
        if ((xset) == (yset)) {
            return;
        }
        if ((self.rank.as_mut().unwrap()[(xset as u64) as usize])
            < (self.rank.as_mut().unwrap()[(yset as u64) as usize]))
        {
            self.parent.as_mut().unwrap()[(xset as u64) as usize] = yset;
        } else if ((self.rank.as_mut().unwrap()[(xset as u64) as usize])
            > (self.rank.as_mut().unwrap()[(yset as u64) as usize]))
        {
            self.parent.as_mut().unwrap()[(yset as u64) as usize] = xset;
        } else {
            self.parent.as_mut().unwrap()[(yset as u64) as usize] = xset;
            self.rank.as_mut().unwrap()[(xset as u64) as usize] =
                ((self.rank.as_mut().unwrap()[(xset as u64) as usize]) + (1));
        }
    }
}
#[derive(Default)]
pub struct Graph {
    pub edges: Option<Box<[Edge]>>,
    pub V: i32,
    pub E: i32,
}
pub unsafe fn MSTKruskal_2(graph: *mut Graph) -> f64 {
    (unsafe {
        let _arr: *mut Option<Box<[Edge]>> = &mut (*graph).edges as *mut Option<Box<[Edge]>>;
        let _start: i32 = 0;
        let _end: i32 = (((*graph).E) - (1));
        quicksort_1(_arr, _start, _end)
    });
    let mut set: DisjointSet = DisjointSet {
        rank: Some(
            (0..((*graph).V as u64))
                .map(|_| <i32>::default())
                .collect::<Box<[_]>>(),
        ),
        parent: Some(
            (0..((*graph).V as u64))
                .map(|_| <i32>::default())
                .collect::<Box<[_]>>(),
        ),
        n: (*graph).V,
    };
    (unsafe { set.makeSet() });
    let mut total_weight: f64 = 0_f64;
    let mut i: i32 = 0;
    'loop_: while ((i) < ((*graph).E)) {
        let mut x: i32 = (*graph).edges.as_mut().unwrap()[(i as u64) as usize].u;
        let mut y: i32 = (*graph).edges.as_mut().unwrap()[(i as u64) as usize].v;
        let mut w: f64 = (*graph).edges.as_mut().unwrap()[(i as u64) as usize].weight;
        if ((unsafe {
            let _x: i32 = x;
            set.find(_x)
        }) != (unsafe {
            let _x: i32 = y;
            set.find(_x)
        })) {
            (unsafe {
                let _x: i32 = x;
                let _y: i32 = y;
                set.merge(_x, _y)
            });
            total_weight += w;
        }
        i.prefix_inc();
    }
    return total_weight;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut V: i32 = 4;
    let mut E: i32 = 5;
    let mut graph: Graph = Graph {
        edges: Some(
            (0..(E as u64))
                .map(|_| <Edge>::default())
                .collect::<Box<[_]>>(),
        ),
        V: V,
        E: E,
    };
    graph.edges.as_mut().unwrap()[(0_u64) as usize] = Edge {
        u: 0,
        v: 1,
        weight: 10_f64,
    };
    graph.edges.as_mut().unwrap()[(1_u64) as usize] = Edge {
        u: 1,
        v: 3,
        weight: 15_f64,
    };
    graph.edges.as_mut().unwrap()[(2_u64) as usize] = Edge {
        u: 2,
        v: 3,
        weight: 4_f64,
    };
    graph.edges.as_mut().unwrap()[(3_u64) as usize] = Edge {
        u: 2,
        v: 0,
        weight: 6_f64,
    };
    graph.edges.as_mut().unwrap()[(4_u64) as usize] = Edge {
        u: 0,
        v: 3,
        weight: 5_f64,
    };
    let mut total_weight: f64 = (unsafe {
        let _graph: *mut Graph = &mut graph as *mut Graph;
        MSTKruskal_2(_graph)
    });
    return (total_weight as i32);
}
