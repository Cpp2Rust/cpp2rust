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
pub struct MinHeapNode {
    pub data: u8,
    pub freq: i32,
    pub left: *mut MinHeapNode,
    pub right: *mut MinHeapNode,
}
impl MinHeapNode {
    pub unsafe fn IsLeaf(&self) -> bool {
        return (((self.left).is_null()) && ((self.right).is_null()));
    }
}
pub unsafe fn Swap_0(a: *mut MinHeapNode, b: *mut MinHeapNode) {
    let mut t: MinHeapNode = MinHeapNode {
        data: (*a).data,
        freq: (*a).freq,
        left: (*a).left,
        right: (*a).right,
    };
    (*a) = (MinHeapNode {
        data: (*b).data,
        freq: (*b).freq,
        left: (*b).left,
        right: (*b).right,
    })
    .clone();
    (*b) = (MinHeapNode {
        data: t.data,
        freq: t.freq,
        left: t.left,
        right: t.right,
    })
    .clone();
}
#[repr(C)]
#[derive(Default)]
pub struct MinHeap {
    pub size: i32,
    pub capacity: i32,
    pub arr: Option<Box<[*mut MinHeapNode]>>,
    pub next: i32,
    pub alloc: Option<Box<[MinHeapNode]>>,
}
impl MinHeap {
    pub unsafe fn Alloc(&mut self, mut data: u8, mut freq: i32) -> *mut MinHeapNode {
        self.alloc.as_mut().unwrap()[(self.next as u64) as usize] = MinHeapNode {
            data: data,
            freq: freq,
            left: Default::default(),
            right: Default::default(),
        };
        return (&mut self.alloc.as_mut().unwrap()[(self.next.postfix_inc() as u64) as usize]
            as *mut MinHeapNode);
    }
    pub unsafe fn Heapify(&mut self, mut idx: i32) {
        let mut smallest: i32 = idx;
        let mut left: i32 = (((2) * (idx)) + (1));
        let mut right: i32 = (((2) * (idx)) + (2));
        if (((left) < (self.size))
            && (((*self.arr.as_mut().unwrap()[(left as u64) as usize]).freq)
                < ((*self.arr.as_mut().unwrap()[(smallest as u64) as usize]).freq)))
        {
            smallest = left;
        }
        if (((right) < (self.size))
            && (((*self.arr.as_mut().unwrap()[(right as u64) as usize]).freq)
                < ((*self.arr.as_mut().unwrap()[(smallest as u64) as usize]).freq)))
        {
            smallest = right;
        }
        if ((smallest) != (idx)) {
            (unsafe {
                let _a: *mut MinHeapNode = &mut (*self.arr.as_mut().unwrap()
                    [(smallest as u64) as usize])
                    as *mut MinHeapNode;
                let _b: *mut MinHeapNode =
                    &mut (*self.arr.as_mut().unwrap()[(idx as u64) as usize]) as *mut MinHeapNode;
                Swap_0(_a, _b)
            });
            (unsafe {
                let _idx: i32 = smallest;
                self.Heapify(_idx)
            });
        }
    }
    pub unsafe fn ExtractMin(&mut self) -> *mut MinHeapNode {
        let mut out: *mut MinHeapNode = self.arr.as_mut().unwrap()[(0_u64) as usize];
        self.size.prefix_dec();
        self.arr.as_mut().unwrap()[(0_u64) as usize] =
            self.arr.as_mut().unwrap()[(self.size as u64) as usize];
        (unsafe {
            let _idx: i32 = 0;
            self.Heapify(_idx)
        });
        return out;
    }
    pub unsafe fn Insert(&mut self, mut node: *mut MinHeapNode) {
        self.size.prefix_inc();
        let mut i: i32 = ((self.size) - (1));
        'loop_: while (((i) != (0))
            && (((*node).freq)
                < ((*self.arr.as_mut().unwrap()[(((((i) - (1)) as i32) / (2)) as u64) as usize])
                    .freq)))
        {
            self.arr.as_mut().unwrap()[(i as u64) as usize] =
                self.arr.as_mut().unwrap()[(((((i) - (1)) as i32) / (2)) as u64) as usize];
            i = ((((i) - (1)) as i32) / (2));
        }
        self.arr.as_mut().unwrap()[(i as u64) as usize] = node;
    }
    pub unsafe fn Build(
        &mut self,
        data: *mut Option<Box<[u8]>>,
        freq: *mut Option<Box<[i32]>>,
        mut n: i32,
    ) {
        let mut i: i32 = 0;
        'loop_: while ((i) < (n)) {
            self.arr.as_mut().unwrap()[(self.size.postfix_inc() as u64) as usize] = (unsafe {
                let _data: u8 = (*data).as_mut().unwrap()[(i as u64) as usize];
                let _freq: i32 = (*freq).as_mut().unwrap()[(i as u64) as usize];
                self.Alloc(_data, _freq)
            });
            i.prefix_inc();
        }
        let mut i: i32 = ((((self.size) - (2)) as i32) / (2));
        'loop_: while ((i) >= (0)) {
            (unsafe {
                let _idx: i32 = i;
                self.Heapify(_idx)
            });
            i.prefix_dec();
        }
    }
}
pub unsafe fn AllocMinHeap_1(mut capacity: i32) -> Option<Box<MinHeap>> {
    let mut minHeap: Option<Box<MinHeap>> = Some(Box::new(MinHeap {
        size: 0,
        capacity: capacity,
        arr: Some(
            (0..(capacity as u64))
                .map(|_| <*mut MinHeapNode>::default())
                .collect::<Box<[_]>>(),
        ),
        next: 0,
        alloc: Some(
            (0..10000_u64)
                .map(|_| <MinHeapNode>::default())
                .collect::<Box<[_]>>(),
        ),
    }));
    return minHeap;
}
pub unsafe fn Huffman_2(
    data: *mut Option<Box<[u8]>>,
    freq: *mut Option<Box<[i32]>>,
    mut size: i32,
) -> Option<Box<MinHeap>> {
    let mut minHeap: Option<Box<MinHeap>> = (unsafe {
        let _capacity: i32 = size;
        AllocMinHeap_1(_capacity)
    });
    (unsafe {
        let _data: *mut Option<Box<[u8]>> = data;
        let _freq: *mut Option<Box<[i32]>> = freq;
        let _n: i32 = size;
        (*minHeap.as_deref_mut().unwrap()).Build(_data, _freq, _n)
    });
    'loop_: while (((*minHeap.as_deref_mut().unwrap()).size) != (1)) {
        let mut left: *mut MinHeapNode =
            (unsafe { (*minHeap.as_deref_mut().unwrap()).ExtractMin() });
        let mut right: *mut MinHeapNode =
            (unsafe { (*minHeap.as_deref_mut().unwrap()).ExtractMin() });
        let mut top: *mut MinHeapNode = (unsafe {
            let _data: u8 = ('$' as u8);
            let _freq: i32 = (((*left).freq) + ((*right).freq));
            (*minHeap.as_deref_mut().unwrap()).Alloc(_data, _freq)
        });
        (*top).left = left;
        (*top).right = right;
        (unsafe {
            let _node: *mut MinHeapNode = top;
            (*minHeap.as_deref_mut().unwrap()).Insert(_node)
        });
    }
    return minHeap;
}
pub unsafe fn CollectCode_3(
    arr: *mut Option<Box<[i32]>>,
    mut top: i32,
    out: *mut Option<Box<[i32]>>,
    next: *mut i32,
) {
    (*out).as_mut().unwrap()[((*next) as u64) as usize] = 0;
    let mut i: i32 = 0;
    'loop_: while ((i) < (top)) {
        (*out).as_mut().unwrap()[((*next) as u64) as usize] =
            (((*out).as_mut().unwrap()[((*next) as u64) as usize]) * (10));
        (*out).as_mut().unwrap()[((*next) as u64) as usize] = (((*out).as_mut().unwrap()
            [((*next) as u64) as usize])
            + ((*arr).as_mut().unwrap()[(i as u64) as usize]));
        i.prefix_inc();
    }
    (*next).prefix_inc();
}
pub unsafe fn CollectCodes_4(
    mut root: *mut MinHeapNode,
    arr: *mut Option<Box<[i32]>>,
    mut top: i32,
    out: *mut Option<Box<[i32]>>,
    next: *mut i32,
) {
    if !(((*root).left).is_null()) {
        (*arr).as_mut().unwrap()[(top as u64) as usize] = 0;
        (unsafe {
            let _root: *mut MinHeapNode = (*root).left;
            let _arr: *mut Option<Box<[i32]>> = arr;
            let _top: i32 = ((top) + (1));
            let _out: *mut Option<Box<[i32]>> = out;
            let _next: *mut i32 = next;
            CollectCodes_4(_root, _arr, _top, _out, _next)
        });
    }
    if !(((*root).right).is_null()) {
        (*arr).as_mut().unwrap()[(top as u64) as usize] = 1;
        (unsafe {
            let _root: *mut MinHeapNode = (*root).right;
            let _arr: *mut Option<Box<[i32]>> = arr;
            let _top: i32 = ((top) + (1));
            let _out: *mut Option<Box<[i32]>> = out;
            let _next: *mut i32 = next;
            CollectCodes_4(_root, _arr, _top, _out, _next)
        });
    }
    if (unsafe { (*root.cast_const()).IsLeaf() }) {
        (unsafe {
            let _arr: *mut Option<Box<[i32]>> = arr;
            let _top: i32 = top;
            let _out: *mut Option<Box<[i32]>> = out;
            let _next: *mut i32 = next;
            CollectCode_3(_arr, _top, _out, _next)
        });
    }
}
pub unsafe fn HuffmanCodes_5(
    data: *mut Option<Box<[u8]>>,
    freq: *mut Option<Box<[i32]>>,
    mut size: i32,
) -> Option<Box<[i32]>> {
    let mut minHeap: Option<Box<MinHeap>> = (unsafe {
        let _data: *mut Option<Box<[u8]>> = data;
        let _freq: *mut Option<Box<[i32]>> = freq;
        let _size: i32 = size;
        Huffman_2(_data, _freq, _size)
    });
    let mut root: *mut MinHeapNode = (unsafe { (*minHeap.as_deref_mut().unwrap()).ExtractMin() });
    let mut arr: Option<Box<[i32]>> =
        Some((0..100_u64).map(|_| <i32>::default()).collect::<Box<[_]>>());
    let mut out: Option<Box<[i32]>> =
        Some((0..100_u64).map(|_| <i32>::default()).collect::<Box<[_]>>());
    let mut top: i32 = 0;
    let mut next: i32 = 0;
    (unsafe {
        let _root: *mut MinHeapNode = root;
        let _arr: *mut Option<Box<[i32]>> = &mut arr as *mut Option<Box<[i32]>>;
        let _top: i32 = top;
        let _out: *mut Option<Box<[i32]>> = &mut out as *mut Option<Box<[i32]>>;
        let _next: *mut i32 = &mut next as *mut i32;
        CollectCodes_4(_root, _arr, _top, _out, _next)
    });
    return out;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut size: i32 = 6;
    let mut arr1: [u8; 6] = [
        ('a' as u8),
        ('b' as u8),
        ('c' as u8),
        ('d' as u8),
        ('e' as u8),
        ('f' as u8),
    ];
    let mut arr2: [i32; 6] = [5, 9, 12, 13, 16, 45];
    let mut data: Option<Box<[u8]>> = Some(
        (0..(size as u64))
            .map(|_| <u8>::default())
            .collect::<Box<[_]>>(),
    );
    let mut freq: Option<Box<[i32]>> = Some(
        (0..(size as u64))
            .map(|_| <i32>::default())
            .collect::<Box<[_]>>(),
    );
    let mut i: i32 = 0;
    'loop_: while ((i) < (size)) {
        data.as_mut().unwrap()[(i as u64) as usize] = arr1[(i) as usize];
        freq.as_mut().unwrap()[(i as u64) as usize] = arr2[(i) as usize];
        i.prefix_inc();
    }
    let mut out: Option<Box<[i32]>> = (unsafe {
        let _data: *mut Option<Box<[u8]>> = &mut data as *mut Option<Box<[u8]>>;
        let _freq: *mut Option<Box<[i32]>> = &mut freq as *mut Option<Box<[i32]>>;
        let _size: i32 = size;
        HuffmanCodes_5(_data, _freq, _size)
    });
    return ((((((((out.as_mut().unwrap()[(0_u64) as usize]) == (0))
        && ((out.as_mut().unwrap()[(1_u64) as usize]) == (100)))
        && ((out.as_mut().unwrap()[(2_u64) as usize]) == (101)))
        && ((out.as_mut().unwrap()[(3_u64) as usize]) == (1100)))
        && ((out.as_mut().unwrap()[(4_u64) as usize]) == (1101)))
        && ((out.as_mut().unwrap()[(5_u64) as usize]) == (111))) as i32);
}
