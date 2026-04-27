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
pub struct Queue {
    pub elems: *mut u32,
    pub front: u64,
    pub back: u64,
    pub capacity: u64,
}
impl Queue {
    pub unsafe fn enqueue(&mut self, mut elem: i32) {
        if ((self.back) == (self.capacity)) {
            return;
        }
        (*self.elems.offset((self.back.postfix_inc()) as isize)) = (elem as u32);
    }
    pub unsafe fn dequeue(&mut self) -> u32 {
        if (unsafe { self.empty() }) {
            return (-1_i32 as u32);
        }
        let mut elem: u32 = (*self.elems.offset((self.front.postfix_inc()) as isize));
        if ((self.front) == (self.back)) {
            self.front = 0_u64;
            self.back = 0_u64;
        }
        return elem;
    }
    pub unsafe fn empty(&self) -> bool {
        return ((self.back) == (0_u64));
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct GraphNode {
    pub vertex: u32,
    pub next: *mut GraphNode,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Graph {
    pub V: u32,
    pub adj: *mut *mut GraphNode,
}
impl Graph {
    pub unsafe fn push(&mut self, mut src: u32, mut dst: u32) {
        (*self.adj.offset((src) as isize)) = (Box::leak(Box::new(GraphNode {
            vertex: dst,
            next: (*self.adj.offset((src) as isize)),
        })) as *mut GraphNode);
        (*self.adj.offset((dst) as isize)) = (Box::leak(Box::new(GraphNode {
            vertex: src,
            next: (*self.adj.offset((dst) as isize)),
        })) as *mut GraphNode);
    }
}
pub unsafe fn BFS_0(graph: *const Graph, mut start_vertex: u32) -> *mut u32 {
    let mut Q: Queue = Queue {
        elems: Box::leak(
            (0..((*graph).V as u64))
                .map(|_| 0_u32)
                .collect::<Box<[u32]>>(),
        )
        .as_mut_ptr(),
        front: 0_u64,
        back: 0_u64,
        capacity: ((*graph).V as u64),
    };
    let mut visited: *mut bool = Box::leak(
        (0..((*graph).V as u64))
            .map(|_| false)
            .collect::<Box<[bool]>>(),
    )
    .as_mut_ptr();
    let mut pred: *mut u32 = Box::leak(
        (0..((*graph).V as u64))
            .map(|_| 0_u32)
            .collect::<Box<[u32]>>(),
    )
    .as_mut_ptr();
    let mut i: u32 = 0_u32;
    'loop_: while ((i) < ((*graph).V)) {
        (*visited.offset((i) as isize)) = false;
        (*pred.offset((i) as isize)) = i;
        i.prefix_inc();
    }
    (*visited.offset((start_vertex) as isize)) = true;
    (unsafe {
        let _elem: i32 = (start_vertex as i32);
        Q.enqueue(_elem)
    });
    'loop_: while !(unsafe { Q.empty() }) {
        let mut current_vertex: i32 = ((unsafe { Q.dequeue() }) as i32);
        let mut head: *mut GraphNode = (*(*graph).adj.offset((current_vertex) as isize));
        'loop_: while !((head).is_null()) {
            let mut adj_vertex: i32 = ((*head).vertex as i32);
            if !(*visited.offset((adj_vertex) as isize)) {
                (*visited.offset((adj_vertex) as isize)) = true;
                (unsafe {
                    let _elem: i32 = adj_vertex;
                    Q.enqueue(_elem)
                });
                (*pred.offset((adj_vertex) as isize)) = (current_vertex as u32);
            }
            head = (*head).next;
        }
    }

    ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
        visited,
        libcc2rs::malloc_usable_size(visited as *mut ::libc::c_void)
            / ::std::mem::size_of::<bool>(),
    )));

    ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
        Q.elems,
        libcc2rs::malloc_usable_size(Q.elems as *mut ::libc::c_void) / ::std::mem::size_of::<u32>(),
    )));
    return pred;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let V: u64 = 5000_u64;
    let mut graph: Graph = Graph {
        V: (V as u32),
        adj: Box::leak(
            (0..V)
                .map(|_| Default::default())
                .collect::<Box<[*mut GraphNode]>>(),
        )
        .as_mut_ptr(),
    };
    let mut i: u32 = 0_u32;
    'loop_: while ((i as u64) < (V)) {
        (*graph.adj.offset((i) as isize)) = Default::default();
        i.prefix_inc();
    }
    let mut i: u32 = 0_u32;
    'loop_: while ((i as u64) < (V)) {
        let mut j: u32 = (i).wrapping_add(1_u32);
        'loop_: while ((j as u64) < (V)) {
            (unsafe {
                let _src: u32 = i;
                let _dst: u32 = j;
                graph.push(_src, _dst)
            });
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    let mut pred: *mut u32 = (unsafe {
        let _graph: *const Graph = &graph as *const Graph;
        let _start_vertex: u32 = 0_u32;
        BFS_0(_graph, _start_vertex)
    });
    let mut i: u32 = 0_u32;
    'loop_: while ((i as u64) < (V)) {
        let mut head: *mut GraphNode = (*graph.adj.offset((i) as isize));
        'loop_: while !((head).is_null()) {
            let mut next: *mut GraphNode = (*head).next;
            ::std::mem::drop(Box::from_raw(head));
            head = next;
        }
        i.prefix_inc();
    }

    ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
        graph.adj,
        libcc2rs::malloc_usable_size(graph.adj as *mut ::libc::c_void)
            / ::std::mem::size_of::<*mut GraphNode>(),
    )));

    ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
        pred,
        libcc2rs::malloc_usable_size(pred as *mut ::libc::c_void) / ::std::mem::size_of::<u32>(),
    )));
    return 0;
}
