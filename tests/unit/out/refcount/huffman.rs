extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(24)]
pub struct MinHeapNode {
    #[offset(0)]
    pub data: i8,
    #[offset(4)]
    pub freq: i32,
    #[offset(8)]
    #[byte_size(8)]
    pub left: Ptr<MinHeapNode>,
    #[offset(16)]
    #[byte_size(8)]
    pub right: Ptr<MinHeapNode>,
}
pub fn Swap_0(a: Ptr<MinHeapNode>, b: Ptr<MinHeapNode>) {
    let mut t: MinHeapNode = MinHeapNode {
        data: a.with(|__s| __s.data),
        freq: a.with(|__s| __s.freq),
        left: a.with(|__s| __s.left.clone()),
        right: a.with(|__s| __s.right.clone()),
    };
    a.write({
        MinHeapNode {
            data: b.with(|__s| __s.data),
            freq: b.with(|__s| __s.freq),
            left: b.with(|__s| __s.left.clone()),
            right: b.with(|__s| __s.right.clone()),
        }
    });
    b.write({
        MinHeapNode {
            data: t.data,
            freq: t.freq,
            left: (t.left).clone(),
            right: (t.right).clone(),
        }
    });
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, Default)]
#[byte_size(32)]
pub struct MinHeap {
    #[offset(0)]
    pub size: i32,
    #[offset(4)]
    pub capacity: i32,
    #[offset(8)]
    #[byte_size(8)]
    pub arr: Option<Value<Box<[Ptr<MinHeapNode>]>>>,
    #[offset(16)]
    pub next: i32,
    #[offset(24)]
    #[byte_size(8)]
    pub alloc: Option<Value<Box<[MinHeapNode]>>>,
}
impl MinHeap {
    pub fn move_from(_a0: Ptr<MinHeap>) -> Self {
        Self {
            size: { (*_a0.upgrade().deref()).size },
            capacity: { (*_a0.upgrade().deref()).capacity },
            arr: field!(_a0, arr)
                .with_mut(|__v: &mut Option<Value<Box<[Ptr<MinHeapNode>]>>>| __v.take()),
            next: { (*_a0.upgrade().deref()).next },
            alloc: field!(_a0, alloc)
                .with_mut(|__v: &mut Option<Value<Box<[MinHeapNode]>>>| __v.take()),
        }
    }
}
pub fn AllocMinHeap_1(mut capacity: i32) -> Option<Value<MinHeap>> {
    let mut minHeap: Option<Value<MinHeap>> = Some(Rc::new(RefCell::new({
        let __tmp_0: Value<MinHeap> = Rc::new(RefCell::new(MinHeap {
            size: 0,
            capacity: capacity,
            arr: Some(Rc::new(RefCell::new(
                (0..(capacity as usize))
                    .map(|_| <Ptr<MinHeapNode>>::default())
                    .collect::<Box<[_]>>(),
            ))),
            next: 0,
            alloc: Some(Rc::new(RefCell::new(
                (0..10000_usize)
                    .map(|_| <MinHeapNode>::default())
                    .collect::<Box<[_]>>(),
            ))),
        }));
        MinHeap::move_from({ __tmp_0.as_pointer() })
    })));
    return minHeap.take();
}
pub fn Huffman_2(
    data: Ptr<Option<Value<Box<[i8]>>>>,
    freq: Ptr<Option<Value<Box<[i32]>>>>,
    mut size: i32,
) -> Option<Value<MinHeap>> {
    let mut minHeap: Option<Value<MinHeap>> = ({ AllocMinHeap_1(size) });
    ({
        let _data: Ptr<Option<Value<Box<[i8]>>>> = (data).clone();
        let _freq: Ptr<Option<Value<Box<[i32]>>>> = (freq).clone();
        let _n: i32 = size;
        MinHeapImpl::Build(&(minHeap.as_pointer()), _data, _freq, _n)
    });
    'loop_: while ({ (*minHeap.as_ref().unwrap().borrow()).size } != 1) {
        let mut left: Ptr<MinHeapNode> = ({ MinHeapImpl::ExtractMin(&(minHeap.as_pointer())) });
        let mut right: Ptr<MinHeapNode> = ({ MinHeapImpl::ExtractMin(&(minHeap.as_pointer())) });
        let mut top: Ptr<MinHeapNode> = ({
            MinHeapImpl::Alloc(
                &(minHeap.as_pointer()),
                ('$' as i8),
                ({ left.with(|__s| __s.freq) } + { right.with(|__s| __s.freq) }),
            )
        });
        field!(top, left).write((left).clone());
        field!(top, right).write((right).clone());
        ({ MinHeapImpl::Insert(&(minHeap.as_pointer()), (top).clone()) });
    }
    return minHeap.take();
}
pub fn CollectCode_3(
    arr: Ptr<Option<Value<Box<[i32]>>>>,
    mut top: i32,
    out: Ptr<Option<Value<Box<[i32]>>>>,
    next: Ptr<i32>,
) {
    (*out.upgrade().deref()).as_ref().unwrap().borrow_mut()[((next.read()) as usize) as usize] = 0;
    let mut i: i32 = 0;
    'loop_: while (i < top) {
        let __rhs = ((*out.upgrade().deref()).as_ref().unwrap().borrow()
            [((next.read()) as usize) as usize]
            * 10);
        (*out.upgrade().deref()).as_ref().unwrap().borrow_mut()
            [((next.read()) as usize) as usize] = __rhs;
        let __rhs = ({
            (*out.upgrade().deref()).as_ref().unwrap().borrow()[((next.read()) as usize) as usize]
        } + {
            (*arr.upgrade().deref()).as_ref().unwrap().borrow()[(i as usize) as usize]
        });
        (*out.upgrade().deref()).as_ref().unwrap().borrow_mut()
            [((next.read()) as usize) as usize] = __rhs;
        i.prefix_inc();
    }
    next.with_mut(|__v| __v.prefix_inc());
}
pub fn CollectCodes_4(
    mut root: Ptr<MinHeapNode>,
    arr: Ptr<Option<Value<Box<[i32]>>>>,
    mut top: i32,
    out: Ptr<Option<Value<Box<[i32]>>>>,
    next: Ptr<i32>,
) {
    if !((root.with(|__s| __s.left.clone())).is_null()) {
        (*arr.upgrade().deref()).as_ref().unwrap().borrow_mut()[(top as usize) as usize] = 0;
        ({
            let _root: Ptr<MinHeapNode> = root.with(|__s| __s.left.clone());
            let _arr: Ptr<Option<Value<Box<[i32]>>>> = (arr).clone();
            let _top: i32 = (top + 1);
            let _out: Ptr<Option<Value<Box<[i32]>>>> = (out).clone();
            let _next: Ptr<i32> = (next).clone();
            CollectCodes_4(_root, _arr, _top, _out, _next)
        });
    }
    if !((root.with(|__s| __s.right.clone())).is_null()) {
        (*arr.upgrade().deref()).as_ref().unwrap().borrow_mut()[(top as usize) as usize] = 1;
        ({
            let _root: Ptr<MinHeapNode> = root.with(|__s| __s.right.clone());
            let _arr: Ptr<Option<Value<Box<[i32]>>>> = (arr).clone();
            let _top: i32 = (top + 1);
            let _out: Ptr<Option<Value<Box<[i32]>>>> = (out).clone();
            let _next: Ptr<i32> = (next).clone();
            CollectCodes_4(_root, _arr, _top, _out, _next)
        });
    }
    if ({ MinHeapNodeImpl::IsLeaf(&root) }) {
        ({
            let _arr: Ptr<Option<Value<Box<[i32]>>>> = (arr).clone();
            let _top: i32 = top;
            let _out: Ptr<Option<Value<Box<[i32]>>>> = (out).clone();
            let _next: Ptr<i32> = (next).clone();
            CollectCode_3(_arr, _top, _out, _next)
        });
    }
}
pub fn HuffmanCodes_5(
    data: Ptr<Option<Value<Box<[i8]>>>>,
    freq: Ptr<Option<Value<Box<[i32]>>>>,
    mut size: i32,
) -> Option<Value<Box<[i32]>>> {
    let mut minHeap: Option<Value<MinHeap>> = ({
        let _data: Ptr<Option<Value<Box<[i8]>>>> = (data).clone();
        let _freq: Ptr<Option<Value<Box<[i32]>>>> = (freq).clone();
        let _size: i32 = size;
        Huffman_2(_data, _freq, _size)
    });
    let mut root: Ptr<MinHeapNode> = ({ MinHeapImpl::ExtractMin(&(minHeap.as_pointer())) });
    let arr: Value<Option<Value<Box<[i32]>>>> = Rc::new(RefCell::new(Some(Rc::new(RefCell::new(
        (0..100_usize)
            .map(|_| <i32>::default())
            .collect::<Box<[_]>>(),
    )))));
    let out: Value<Option<Value<Box<[i32]>>>> = Rc::new(RefCell::new(Some(Rc::new(RefCell::new(
        (0..100_usize)
            .map(|_| <i32>::default())
            .collect::<Box<[_]>>(),
    )))));
    let mut top: i32 = 0;
    let next: Value<i32> = Rc::new(RefCell::new(0));
    ({
        CollectCodes_4(
            (root).clone(),
            arr.as_pointer(),
            top,
            out.as_pointer(),
            next.as_pointer(),
        )
    });
    return (*out.borrow_mut()).take();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut size: i32 = 6;
    let mut arr1: [i8; 6] = [
        ('a' as i8),
        ('b' as i8),
        ('c' as i8),
        ('d' as i8),
        ('e' as i8),
        ('f' as i8),
    ];
    let mut arr2: [i32; 6] = [5, 9, 12, 13, 16, 45];
    let data: Value<Option<Value<Box<[i8]>>>> = Rc::new(RefCell::new(Some(Rc::new(RefCell::new(
        (0..(size as usize))
            .map(|_| <i8>::default())
            .collect::<Box<[_]>>(),
    )))));
    let freq: Value<Option<Value<Box<[i32]>>>> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new(
            (0..(size as usize))
                .map(|_| <i32>::default())
                .collect::<Box<[_]>>(),
        )))));
    let mut i: i32 = 0;
    'loop_: while (i < size) {
        let __rhs = arr1[(i) as usize];
        (*data.borrow()).as_ref().unwrap().borrow_mut()[(i as usize) as usize] = __rhs;
        let __rhs = arr2[(i) as usize];
        (*freq.borrow()).as_ref().unwrap().borrow_mut()[(i as usize) as usize] = __rhs;
        i.prefix_inc();
    }
    let mut out: Option<Value<Box<[i32]>>> =
        ({ HuffmanCodes_5(data.as_pointer(), freq.as_pointer(), size) });
    assert!(
        (((((out.as_ref().unwrap().borrow()[(0_usize) as usize] == 0)
            && (out.as_ref().unwrap().borrow()[(1_usize) as usize] == 100))
            && (out.as_ref().unwrap().borrow()[(2_usize) as usize] == 101))
            && (out.as_ref().unwrap().borrow()[(3_usize) as usize] == 1100))
            && (out.as_ref().unwrap().borrow()[(4_usize) as usize] == 1101))
            && (out.as_ref().unwrap().borrow()[(5_usize) as usize] == 111)
    );
    return 0;
}
pub trait MinHeapImpl {
    fn Alloc(&self, data: i8, freq: i32) -> Ptr<MinHeapNode>;
    fn Heapify(&self, idx: i32);
    fn ExtractMin(&self) -> Ptr<MinHeapNode>;
    fn Insert(&self, node: Ptr<MinHeapNode>);
    fn Build(
        &self,
        data: Ptr<Option<Value<Box<[i8]>>>>,
        freq: Ptr<Option<Value<Box<[i32]>>>>,
        n: i32,
    );
    fn move_assign(&self, _a0: Ptr<MinHeap>) -> Ptr<MinHeap>;
}
impl MinHeapImpl for Ptr<MinHeap> {
    fn Alloc(&self, mut data: i8, mut freq: i32) -> Ptr<MinHeapNode> {
        (*self)
            .with(|__s| __s.alloc.clone())
            .as_ref()
            .unwrap()
            .borrow_mut()[((*self).with(|__s| __s.next) as usize) as usize] = MinHeapNode {
            data: data,
            freq: freq,
            left: Ptr::<MinHeapNode>::null(),
            right: Ptr::<MinHeapNode>::null(),
        };
        return ((*self)
            .with(|__s| __s.alloc.clone())
            .as_ref()
            .unwrap()
            .as_pointer()
            .offset((field!((*self), next).with_mut(|__v| __v.postfix_inc()) as usize)))
        .clone();
    }
    fn Heapify(&self, mut idx: i32) {
        let mut smallest: i32 = idx;
        let mut left: i32 = ((2 * idx) + 1);
        let mut right: i32 = ((2 * idx) + 2);
        if (left < (*self).with(|__s| __s.size))
            && ((*self)
                .with(|__s| __s.arr.clone())
                .as_ref()
                .unwrap()
                .borrow()[(left as usize) as usize]
                .with(|__s| __s.freq)
                < (*self)
                    .with(|__s| __s.arr.clone())
                    .as_ref()
                    .unwrap()
                    .borrow()[(smallest as usize) as usize]
                    .with(|__s| __s.freq))
        {
            smallest = left;
        }
        if (right < (*self).with(|__s| __s.size))
            && ((*self)
                .with(|__s| __s.arr.clone())
                .as_ref()
                .unwrap()
                .borrow()[(right as usize) as usize]
                .with(|__s| __s.freq)
                < (*self)
                    .with(|__s| __s.arr.clone())
                    .as_ref()
                    .unwrap()
                    .borrow()[(smallest as usize) as usize]
                    .with(|__s| __s.freq))
        {
            smallest = right;
        }
        if (smallest != idx) {
            ({
                let _a: Ptr<MinHeapNode> = ((*self)
                    .with(|__s| __s.arr.clone())
                    .as_ref()
                    .unwrap()
                    .borrow()[(smallest as usize) as usize])
                    .clone();
                let _b: Ptr<MinHeapNode> = ((*self)
                    .with(|__s| __s.arr.clone())
                    .as_ref()
                    .unwrap()
                    .borrow()[(idx as usize) as usize])
                    .clone();
                Swap_0(_a, _b)
            });
            ({ MinHeapImpl::Heapify(self, smallest) });
        }
    }
    fn ExtractMin(&self) -> Ptr<MinHeapNode> {
        let mut out: Ptr<MinHeapNode> = ((*self)
            .with(|__s| __s.arr.clone())
            .as_ref()
            .unwrap()
            .borrow()[(0_usize) as usize])
            .clone();
        field!((*self), size).with_mut(|__v| __v.prefix_dec());
        let __rhs = ((*self)
            .with(|__s| __s.arr.clone())
            .as_ref()
            .unwrap()
            .borrow()[((*self).with(|__s| __s.size) as usize) as usize])
            .clone();
        (*self)
            .with(|__s| __s.arr.clone())
            .as_ref()
            .unwrap()
            .borrow_mut()[(0_usize) as usize] = __rhs;
        ({ MinHeapImpl::Heapify(self, 0) });
        return out;
    }
    fn Insert(&self, mut node: Ptr<MinHeapNode>) {
        field!((*self), size).with_mut(|__v| __v.prefix_inc());
        let mut i: i32 = ((*self).with(|__s| __s.size) - 1);
        'loop_: while (i != 0)
            && ({ node.with(|__s| __s.freq) } < {
                (*self)
                    .with(|__s| __s.arr.clone())
                    .as_ref()
                    .unwrap()
                    .borrow()[(((i - 1) / 2) as usize) as usize]
                    .with(|__s| __s.freq)
            })
        {
            let __rhs = ((*self)
                .with(|__s| __s.arr.clone())
                .as_ref()
                .unwrap()
                .borrow()[(((i - 1) / 2) as usize) as usize])
                .clone();
            (*self)
                .with(|__s| __s.arr.clone())
                .as_ref()
                .unwrap()
                .borrow_mut()[(i as usize) as usize] = __rhs;
            i = { ((i - 1) / 2) };
        }
        (*self)
            .with(|__s| __s.arr.clone())
            .as_ref()
            .unwrap()
            .borrow_mut()[(i as usize) as usize] = (node).clone();
    }
    fn Build(
        &self,
        data: Ptr<Option<Value<Box<[i8]>>>>,
        freq: Ptr<Option<Value<Box<[i32]>>>>,
        mut n: i32,
    ) {
        let mut i: i32 = 0;
        'loop_: while (i < n) {
            (*self)
                .with(|__s| __s.arr.clone())
                .as_ref()
                .unwrap()
                .borrow_mut()
                [(field!((*self), size).with_mut(|__v| __v.postfix_inc()) as usize) as usize] = ({
                let _data: i8 =
                    (*data.upgrade().deref()).as_ref().unwrap().borrow()[(i as usize) as usize];
                let _freq: i32 =
                    (*freq.upgrade().deref()).as_ref().unwrap().borrow()[(i as usize) as usize];
                MinHeapImpl::Alloc(self, _data, _freq)
            });
            i.prefix_inc();
        }
        let mut i: i32 = (((*self).with(|__s| __s.size) - 2) / 2);
        'loop_: while (i >= 0) {
            ({ MinHeapImpl::Heapify(self, i) });
            i.prefix_dec();
        }
    }
    fn move_assign(&self, _a0: Ptr<MinHeap>) -> Ptr<MinHeap> {
        field!((*self), size).write({ { (*_a0.upgrade().deref()).size } });
        field!((*self), capacity).write({ { (*_a0.upgrade().deref()).capacity } });
        (field_ptr!((*self), arr) as Ptr<Option<Value<Box<[Ptr<MinHeapNode>]>>>>).write(
            field!(_a0, arr)
                .with_mut(|__v: &mut Option<Value<Box<[Ptr<MinHeapNode>]>>>| __v.take()),
        );
        field!((*self), next).write({ { (*_a0.upgrade().deref()).next } });
        (field_ptr!((*self), alloc) as Ptr<Option<Value<Box<[MinHeapNode]>>>>).write(
            field!(_a0, alloc).with_mut(|__v: &mut Option<Value<Box<[MinHeapNode]>>>| __v.take()),
        );
        return (*self).clone();
    }
}
pub trait MinHeapNodeImpl {
    fn IsLeaf(&self) -> bool;
}
impl MinHeapNodeImpl for Ptr<MinHeapNode> {
    fn IsLeaf(&self) -> bool {
        return (((*self).with(|__s| __s.left.clone())).is_null())
            && (((*self).with(|__s| __s.right.clone())).is_null());
    }
}
pub fn __cpp2rust_init_globals() {}
