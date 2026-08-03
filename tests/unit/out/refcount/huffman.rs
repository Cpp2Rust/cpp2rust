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
pub struct MinHeapNode {
    pub data: u8,
    pub freq: i32,
    pub left: Ptr<MinHeapNode>,
    pub right: Ptr<MinHeapNode>,
}
pub trait MinHeapNodeMethods {
    fn IsLeaf(&self) -> bool;
}
impl MinHeapNodeMethods for Ptr<MinHeapNode> {
    fn IsLeaf(&self) -> bool {
        return ((self.with(|__v| (*__v).left.clone())).is_null())
            && ((self.with(|__v| (*__v).right.clone())).is_null());
    }
}
impl Clone for MinHeapNode {
    fn clone(&self) -> Self {
        let mut this = Self {
            data: self.data,
            freq: self.freq,
            left: (self.left).clone(),
            right: (self.right).clone(),
        };
        this
    }
}
impl ByteRepr for MinHeapNode {
    fn byte_size() -> usize {
        24
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.data.to_bytes(&mut buf[0..1]);
        self.freq.to_bytes(&mut buf[4..8]);
        self.left.to_bytes(&mut buf[8..16]);
        self.right.to_bytes(&mut buf[16..24]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            data: <u8>::from_bytes(&buf[0..1]),
            freq: <i32>::from_bytes(&buf[4..8]),
            left: <Ptr<MinHeapNode>>::from_bytes(&buf[8..16]),
            right: <Ptr<MinHeapNode>>::from_bytes(&buf[16..24]),
        }
    }
}
pub fn Swap_0(a: Ptr<MinHeapNode>, b: Ptr<MinHeapNode>) {
    let t: Value<MinHeapNode> = Rc::new(RefCell::new(MinHeapNode {
        data: a.with(|__v| (*__v).data),
        freq: a.with(|__v| (*__v).freq),
        left: (a.with(|__v| (*__v).left.clone())).clone(),
        right: (a.with(|__v| (*__v).right.clone())).clone(),
    }));
    let __rhs = MinHeapNode {
        data: b.with(|__v| (*__v).data),
        freq: b.with(|__v| (*__v).freq),
        left: (b.with(|__v| (*__v).left.clone())).clone(),
        right: (b.with(|__v| (*__v).right.clone())).clone(),
    };
    a.write(__rhs);
    let __rhs = MinHeapNode {
        data: (*t.borrow()).data,
        freq: (*t.borrow()).freq,
        left: ((*t.borrow()).left).clone(),
        right: ((*t.borrow()).right).clone(),
    };
    b.write(__rhs);
}
#[repr(C)]
#[derive(Default)]
pub struct MinHeap {
    pub size: i32,
    pub capacity: i32,
    pub arr: Option<Value<Box<[Ptr<MinHeapNode>]>>>,
    pub next: i32,
    pub alloc: Option<Value<Box<[MinHeapNode]>>>,
}
pub trait MinHeapMethods {
    fn Alloc(&self, data: u8, freq: i32) -> Ptr<MinHeapNode>;
    fn Heapify(&self, idx: i32);
    fn ExtractMin(&self) -> Ptr<MinHeapNode>;
    fn Insert(&self, node: Ptr<MinHeapNode>);
    fn Build(
        &self,
        data: Ptr<Option<Value<Box<[u8]>>>>,
        freq: Ptr<Option<Value<Box<[i32]>>>>,
        n: i32,
    );
}
impl MinHeapMethods for Ptr<MinHeap> {
    fn Alloc(&self, data: u8, freq: i32) -> Ptr<MinHeapNode> {
        let data: Value<u8> = Rc::new(RefCell::new(data));
        let freq: Value<i32> = Rc::new(RefCell::new(freq));
        (*self
            .with(|__v| (*__v).alloc.clone())
            .as_ref()
            .unwrap()
            .borrow_mut())[(self.with(|__v| (*__v).next) as usize) as usize] = MinHeapNode {
            data: (*data.borrow()),
            freq: (*freq.borrow()),
            left: Ptr::<MinHeapNode>::null(),
            right: Ptr::<MinHeapNode>::null(),
        };
        return ((self
            .with(|__v| (*__v).alloc.clone())
            .as_ref()
            .unwrap()
            .as_pointer() as Ptr<MinHeapNode>)
            .offset((self.with_mut(|__v| __v.next.postfix_inc()) as usize)))
        .clone();
    }
    fn Heapify(&self, idx: i32) {
        let idx: Value<i32> = Rc::new(RefCell::new(idx));
        let smallest: Value<i32> = Rc::new(RefCell::new((*idx.borrow())));
        let left: Value<i32> = Rc::new(RefCell::new(((2 * (*idx.borrow())) + 1)));
        let right: Value<i32> = Rc::new(RefCell::new(((2 * (*idx.borrow())) + 2)));
        if ((*left.borrow()) < self.with(|__v| (*__v).size))
            && ((*self
                .with(|__v| (*__v).arr.clone())
                .as_ref()
                .unwrap()
                .borrow())[((*left.borrow()) as usize) as usize]
                .with(|__v| (*__v).freq)
                < (*self
                    .with(|__v| (*__v).arr.clone())
                    .as_ref()
                    .unwrap()
                    .borrow())[((*smallest.borrow()) as usize) as usize]
                    .with(|__v| (*__v).freq))
        {
            (*smallest.borrow_mut()) = (*left.borrow());
        }
        if ((*right.borrow()) < self.with(|__v| (*__v).size))
            && ((*self
                .with(|__v| (*__v).arr.clone())
                .as_ref()
                .unwrap()
                .borrow())[((*right.borrow()) as usize) as usize]
                .with(|__v| (*__v).freq)
                < (*self
                    .with(|__v| (*__v).arr.clone())
                    .as_ref()
                    .unwrap()
                    .borrow())[((*smallest.borrow()) as usize) as usize]
                    .with(|__v| (*__v).freq))
        {
            (*smallest.borrow_mut()) = (*right.borrow());
        }
        if ((*smallest.borrow()) != (*idx.borrow())) {
            ({
                let _a: Ptr<MinHeapNode> = ((*self
                    .with(|__v| (*__v).arr.clone())
                    .as_ref()
                    .unwrap()
                    .borrow())[((*smallest.borrow()) as usize) as usize])
                    .clone();
                let _b: Ptr<MinHeapNode> = ((*self
                    .with(|__v| (*__v).arr.clone())
                    .as_ref()
                    .unwrap()
                    .borrow())[((*idx.borrow()) as usize) as usize])
                    .clone();
                Swap_0(_a, _b)
            });
            ({ self.Heapify((*smallest.borrow())) });
        }
    }
    fn ExtractMin(&self) -> Ptr<MinHeapNode> {
        let out: Value<Ptr<MinHeapNode>> = Rc::new(RefCell::new(
            ((*self
                .with(|__v| (*__v).arr.clone())
                .as_ref()
                .unwrap()
                .borrow())[(0_usize) as usize])
                .clone(),
        ));
        self.with_mut(|__v| __v.size.prefix_dec());
        let __rhs = ((*self
            .with(|__v| (*__v).arr.clone())
            .as_ref()
            .unwrap()
            .borrow())[(self.with(|__v| (*__v).size) as usize) as usize])
            .clone();
        (*self
            .with(|__v| (*__v).arr.clone())
            .as_ref()
            .unwrap()
            .borrow_mut())[(0_usize) as usize] = __rhs;
        ({ self.Heapify(0) });
        return (*out.borrow()).clone();
    }
    fn Insert(&self, node: Ptr<MinHeapNode>) {
        let node: Value<Ptr<MinHeapNode>> = Rc::new(RefCell::new(node));
        self.with_mut(|__v| __v.size.prefix_inc());
        let i: Value<i32> = Rc::new(RefCell::new((self.with(|__v| (*__v).size) - 1)));
        'loop_: while ((*i.borrow()) != 0)
            && ({
                let _lhs = (*node.borrow()).with(|__v| (*__v).freq);
                _lhs < (*self
                    .with(|__v| (*__v).arr.clone())
                    .as_ref()
                    .unwrap()
                    .borrow())[((((*i.borrow()) - 1) / 2) as usize) as usize]
                    .with(|__v| (*__v).freq)
            })
        {
            let __rhs = ((*self
                .with(|__v| (*__v).arr.clone())
                .as_ref()
                .unwrap()
                .borrow())[((((*i.borrow()) - 1) / 2) as usize) as usize])
                .clone();
            (*self
                .with(|__v| (*__v).arr.clone())
                .as_ref()
                .unwrap()
                .borrow_mut())[((*i.borrow()) as usize) as usize] = __rhs;
            let __rhs = (((*i.borrow()) - 1) / 2);
            (*i.borrow_mut()) = __rhs;
        }
        (*self
            .with(|__v| (*__v).arr.clone())
            .as_ref()
            .unwrap()
            .borrow_mut())[((*i.borrow()) as usize) as usize] = (*node.borrow()).clone();
    }
    fn Build(
        &self,
        data: Ptr<Option<Value<Box<[u8]>>>>,
        freq: Ptr<Option<Value<Box<[i32]>>>>,
        n: i32,
    ) {
        let n: Value<i32> = Rc::new(RefCell::new(n));
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < (*n.borrow())) {
            (*self
                .with(|__v| (*__v).arr.clone())
                .as_ref()
                .unwrap()
                .borrow_mut())[(self.with_mut(|__v| __v.size.postfix_inc()) as usize) as usize] =
                ({
                    let _data: u8 = (*(data.read()).as_ref().unwrap().borrow())
                        [((*i.borrow()) as usize) as usize];
                    let _freq: i32 = (*(freq.read()).as_ref().unwrap().borrow())
                        [((*i.borrow()) as usize) as usize];
                    self.Alloc(_data, _freq)
                });
            (*i.borrow_mut()).prefix_inc();
        }
        let i: Value<i32> = Rc::new(RefCell::new(((self.with(|__v| (*__v).size) - 2) / 2)));
        'loop_: while ((*i.borrow()) >= 0) {
            ({ self.Heapify((*i.borrow())) });
            (*i.borrow_mut()).prefix_dec();
        }
    }
}
impl ByteRepr for MinHeap {
    fn byte_size() -> usize {
        32
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.size.to_bytes(&mut buf[0..4]);
        self.capacity.to_bytes(&mut buf[4..8]);
        self.arr.to_bytes(&mut buf[8..16]);
        self.next.to_bytes(&mut buf[16..20]);
        self.alloc.to_bytes(&mut buf[24..32]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            size: <i32>::from_bytes(&buf[0..4]),
            capacity: <i32>::from_bytes(&buf[4..8]),
            arr: <Option<Value<Box<[Ptr<MinHeapNode>]>>>>::from_bytes(&buf[8..16]),
            next: <i32>::from_bytes(&buf[16..20]),
            alloc: <Option<Value<Box<[MinHeapNode]>>>>::from_bytes(&buf[24..32]),
        }
    }
}
pub fn AllocMinHeap_1(capacity: i32) -> Option<Value<MinHeap>> {
    let capacity: Value<i32> = Rc::new(RefCell::new(capacity));
    let minHeap: Value<Option<Value<MinHeap>>> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new(MinHeap {
            size: 0,
            capacity: (*capacity.borrow()),
            arr: Some(Rc::new(RefCell::new(
                (0..((*capacity.borrow()) as usize))
                    .map(|_| <Ptr<MinHeapNode>>::default())
                    .collect::<Box<[_]>>(),
            ))),
            next: 0,
            alloc: Some(Rc::new(RefCell::new(
                (0..10000_usize)
                    .map(|_| <MinHeapNode>::default())
                    .collect::<Box<[_]>>(),
            ))),
        })))));
    return (*minHeap.borrow_mut()).take();
}
pub fn Huffman_2(
    data: Ptr<Option<Value<Box<[u8]>>>>,
    freq: Ptr<Option<Value<Box<[i32]>>>>,
    size: i32,
) -> Option<Value<MinHeap>> {
    let size: Value<i32> = Rc::new(RefCell::new(size));
    let minHeap: Value<Option<Value<MinHeap>>> =
        Rc::new(RefCell::new(({ AllocMinHeap_1((*size.borrow())) })));
    ({
        let _data: Ptr<Option<Value<Box<[u8]>>>> = (data).clone();
        let _freq: Ptr<Option<Value<Box<[i32]>>>> = (freq).clone();
        let _n: i32 = (*size.borrow());
        ((*minHeap.borrow()).as_pointer()).Build(_data, _freq, _n)
    });
    'loop_: while ((*(*minHeap.borrow()).as_ref().unwrap().borrow()).size != 1) {
        let left: Value<Ptr<MinHeapNode>> = Rc::new(RefCell::new(
            ({ ((*minHeap.borrow()).as_pointer()).ExtractMin() }),
        ));
        let right: Value<Ptr<MinHeapNode>> = Rc::new(RefCell::new(
            ({ ((*minHeap.borrow()).as_pointer()).ExtractMin() }),
        ));
        let top: Value<Ptr<MinHeapNode>> = Rc::new(RefCell::new(
            ({
                ((*minHeap.borrow()).as_pointer()).Alloc(('$' as u8), {
                    let _lhs = (*left.borrow()).with(|__v| (*__v).freq);
                    _lhs + (*right.borrow()).with(|__v| (*__v).freq)
                })
            }),
        ));
        (*top.borrow()).with_mut(|__v| __v.left = (*left.borrow()).clone());
        (*top.borrow()).with_mut(|__v| __v.right = (*right.borrow()).clone());
        ({ ((*minHeap.borrow()).as_pointer()).Insert((*top.borrow()).clone()) });
    }
    return (*minHeap.borrow_mut()).take();
}
pub fn CollectCode_3(
    arr: Ptr<Option<Value<Box<[i32]>>>>,
    top: i32,
    out: Ptr<Option<Value<Box<[i32]>>>>,
    next: Ptr<i32>,
) {
    let top: Value<i32> = Rc::new(RefCell::new(top));
    (*(out.read()).as_ref().unwrap().borrow_mut())[((next.read()) as usize) as usize] = 0;
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*top.borrow())) {
        let __rhs =
            ((*(out.read()).as_ref().unwrap().borrow())[((next.read()) as usize) as usize] * 10);
        (*(out.read()).as_ref().unwrap().borrow_mut())[((next.read()) as usize) as usize] = __rhs;
        let __rhs = {
            let _lhs =
                (*(out.read()).as_ref().unwrap().borrow())[((next.read()) as usize) as usize];
            _lhs + (*(arr.read()).as_ref().unwrap().borrow())[((*i.borrow()) as usize) as usize]
        };
        (*(out.read()).as_ref().unwrap().borrow_mut())[((next.read()) as usize) as usize] = __rhs;
        (*i.borrow_mut()).prefix_inc();
    }
    next.with_mut(|__v| __v.prefix_inc());
}
pub fn CollectCodes_4(
    root: Ptr<MinHeapNode>,
    arr: Ptr<Option<Value<Box<[i32]>>>>,
    top: i32,
    out: Ptr<Option<Value<Box<[i32]>>>>,
    next: Ptr<i32>,
) {
    let root: Value<Ptr<MinHeapNode>> = Rc::new(RefCell::new(root));
    let top: Value<i32> = Rc::new(RefCell::new(top));
    if !(((*root.borrow()).with(|__v| (*__v).left.clone())).is_null()) {
        (*(arr.read()).as_ref().unwrap().borrow_mut())[((*top.borrow()) as usize) as usize] = 0;
        ({
            let _root: Ptr<MinHeapNode> =
                ((*root.borrow()).with(|__v| (*__v).left.clone())).clone();
            let _arr: Ptr<Option<Value<Box<[i32]>>>> = (arr).clone();
            let _top: i32 = ((*top.borrow()) + 1);
            let _out: Ptr<Option<Value<Box<[i32]>>>> = (out).clone();
            let _next: Ptr<i32> = (next).clone();
            CollectCodes_4(_root, _arr, _top, _out, _next)
        });
    }
    if !(((*root.borrow()).with(|__v| (*__v).right.clone())).is_null()) {
        (*(arr.read()).as_ref().unwrap().borrow_mut())[((*top.borrow()) as usize) as usize] = 1;
        ({
            let _root: Ptr<MinHeapNode> =
                ((*root.borrow()).with(|__v| (*__v).right.clone())).clone();
            let _arr: Ptr<Option<Value<Box<[i32]>>>> = (arr).clone();
            let _top: i32 = ((*top.borrow()) + 1);
            let _out: Ptr<Option<Value<Box<[i32]>>>> = (out).clone();
            let _next: Ptr<i32> = (next).clone();
            CollectCodes_4(_root, _arr, _top, _out, _next)
        });
    }
    if ({ (*root.borrow()).IsLeaf() }) {
        ({
            let _arr: Ptr<Option<Value<Box<[i32]>>>> = (arr).clone();
            let _top: i32 = (*top.borrow());
            let _out: Ptr<Option<Value<Box<[i32]>>>> = (out).clone();
            let _next: Ptr<i32> = (next).clone();
            CollectCode_3(_arr, _top, _out, _next)
        });
    }
}
pub fn HuffmanCodes_5(
    data: Ptr<Option<Value<Box<[u8]>>>>,
    freq: Ptr<Option<Value<Box<[i32]>>>>,
    size: i32,
) -> Option<Value<Box<[i32]>>> {
    let size: Value<i32> = Rc::new(RefCell::new(size));
    let minHeap: Value<Option<Value<MinHeap>>> = Rc::new(RefCell::new(
        ({
            let _data: Ptr<Option<Value<Box<[u8]>>>> = (data).clone();
            let _freq: Ptr<Option<Value<Box<[i32]>>>> = (freq).clone();
            let _size: i32 = (*size.borrow());
            Huffman_2(_data, _freq, _size)
        }),
    ));
    let root: Value<Ptr<MinHeapNode>> = Rc::new(RefCell::new(
        ({ ((*minHeap.borrow()).as_pointer()).ExtractMin() }),
    ));
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
    let top: Value<i32> = Rc::new(RefCell::new(0));
    let next: Value<i32> = Rc::new(RefCell::new(0));
    ({
        CollectCodes_4(
            (*root.borrow()).clone(),
            arr.as_pointer(),
            (*top.borrow()),
            out.as_pointer(),
            next.as_pointer(),
        )
    });
    return (*out.borrow_mut()).take();
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let size: Value<i32> = Rc::new(RefCell::new(6));
    let arr1: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        ('a' as u8),
        ('b' as u8),
        ('c' as u8),
        ('d' as u8),
        ('e' as u8),
        ('f' as u8),
    ])));
    let arr2: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([5, 9, 12, 13, 16, 45])));
    let data: Value<Option<Value<Box<[u8]>>>> = Rc::new(RefCell::new(Some(Rc::new(RefCell::new(
        (0..((*size.borrow()) as usize))
            .map(|_| <u8>::default())
            .collect::<Box<[_]>>(),
    )))));
    let freq: Value<Option<Value<Box<[i32]>>>> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new(
            (0..((*size.borrow()) as usize))
                .map(|_| <i32>::default())
                .collect::<Box<[_]>>(),
        )))));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*size.borrow())) {
        let __rhs = (*arr1.borrow())[(*i.borrow()) as usize];
        (*(*data.borrow()).as_ref().unwrap().borrow_mut())[((*i.borrow()) as usize) as usize] =
            __rhs;
        let __rhs = (*arr2.borrow())[(*i.borrow()) as usize];
        (*(*freq.borrow()).as_ref().unwrap().borrow_mut())[((*i.borrow()) as usize) as usize] =
            __rhs;
        (*i.borrow_mut()).prefix_inc();
    }
    let out: Value<Option<Value<Box<[i32]>>>> = Rc::new(RefCell::new(
        ({ HuffmanCodes_5(data.as_pointer(), freq.as_pointer(), (*size.borrow())) }),
    ));
    return ((((((((*(*out.borrow()).as_ref().unwrap().borrow())[(0_usize) as usize] == 0)
        && ((*(*out.borrow()).as_ref().unwrap().borrow())[(1_usize) as usize] == 100))
        && ((*(*out.borrow()).as_ref().unwrap().borrow())[(2_usize) as usize] == 101))
        && ((*(*out.borrow()).as_ref().unwrap().borrow())[(3_usize) as usize] == 1100))
        && ((*(*out.borrow()).as_ref().unwrap().borrow())[(4_usize) as usize] == 1101))
        && ((*(*out.borrow()).as_ref().unwrap().borrow())[(5_usize) as usize] == 111))
        as i32);
}
