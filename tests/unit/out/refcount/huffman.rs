extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct MinHeapNode {
    pub data: Value<u8>,
    pub freq: Value<i32>,
    pub left: Value<Ptr<MinHeapNode>>,
    pub right: Value<Ptr<MinHeapNode>>,
}
impl MinHeapNode {
    pub fn IsLeaf(&self) -> bool {
        return {
            let _lhs = ((*self.left.borrow()).is_null()).clone();
            _lhs && ((*self.right.borrow()).is_null()).clone()
        };
    }
}
impl Clone for MinHeapNode {
    fn clone(&self) -> Self {
        let mut this = Self {
            data: Rc::new(RefCell::new((*self.data.borrow()))),
            freq: Rc::new(RefCell::new((*self.freq.borrow()))),
            left: Rc::new(RefCell::new((*self.left.borrow()).clone())),
            right: Rc::new(RefCell::new((*self.right.borrow()).clone())),
        };
        this
    }
}
impl ByteRepr for MinHeapNode {}
pub fn Swap_0(a: Ptr<MinHeapNode>, b: Ptr<MinHeapNode>) {
    let t: Value<MinHeapNode> = Rc::new(RefCell::new(MinHeapNode {
        data: Rc::new(RefCell::new((*(*a.upgrade().deref()).data.borrow()))),
        freq: Rc::new(RefCell::new((*(*a.upgrade().deref()).freq.borrow()))),
        left: Rc::new(RefCell::new(
            (*(*a.upgrade().deref()).left.borrow()).clone(),
        )),
        right: Rc::new(RefCell::new(
            (*(*a.upgrade().deref()).right.borrow()).clone(),
        )),
    }));
    let __rhs = MinHeapNode {
        data: Rc::new(RefCell::new((*(*b.upgrade().deref()).data.borrow()))),
        freq: Rc::new(RefCell::new((*(*b.upgrade().deref()).freq.borrow()))),
        left: Rc::new(RefCell::new(
            (*(*b.upgrade().deref()).left.borrow()).clone(),
        )),
        right: Rc::new(RefCell::new(
            (*(*b.upgrade().deref()).right.borrow()).clone(),
        )),
    };
    a.write(__rhs);
    let __rhs = MinHeapNode {
        data: Rc::new(RefCell::new((*(*t.borrow()).data.borrow()))),
        freq: Rc::new(RefCell::new((*(*t.borrow()).freq.borrow()))),
        left: Rc::new(RefCell::new((*(*t.borrow()).left.borrow()).clone())),
        right: Rc::new(RefCell::new((*(*t.borrow()).right.borrow()).clone())),
    };
    b.write(__rhs);
}
#[derive(Default)]
pub struct MinHeap {
    pub size: Value<i32>,
    pub capacity: Value<i32>,
    pub arr: Value<Option<Value<Box<[Ptr<MinHeapNode>]>>>>,
    pub next: Value<i32>,
    pub alloc: Value<Option<Value<Box<[MinHeapNode]>>>>,
}
impl MinHeap {
    pub fn Alloc(&self, data: u8, freq: i32) -> Ptr<MinHeapNode> {
        let data: Value<u8> = Rc::new(RefCell::new(data));
        let freq: Value<i32> = Rc::new(RefCell::new(freq));
        (*self.alloc.borrow()).as_ref().unwrap().borrow_mut()
            [((*self.next.borrow()) as u64) as usize] = MinHeapNode {
            data: Rc::new(RefCell::new((*data.borrow()))),
            freq: Rc::new(RefCell::new((*freq.borrow()))),
            left: Rc::new(RefCell::new(Default::default())),
            right: Rc::new(RefCell::new(Default::default())),
        };
        return ((*self.alloc.borrow())
            .as_ref()
            .unwrap()
            .as_pointer()
            .offset(((*self.next.borrow_mut()).postfix_inc() as u64) as isize))
        .clone();
    }
    pub fn Heapify(&self, idx: i32) {
        let idx: Value<i32> = Rc::new(RefCell::new(idx));
        let smallest: Value<i32> = Rc::new(RefCell::new((*idx.borrow())));
        let left: Value<i32> = Rc::new(RefCell::new(((2 * (*idx.borrow())) + 1)));
        let right: Value<i32> = Rc::new(RefCell::new(((2 * (*idx.borrow())) + 2)));
        if (((*left.borrow()) < (*self.size.borrow()))
            && ((*(*(*self.arr.borrow()).as_ref().unwrap().borrow()
                [((*left.borrow()) as u64) as usize]
                .upgrade()
                .deref())
            .freq
            .borrow())
                < (*(*(*self.arr.borrow()).as_ref().unwrap().borrow()
                    [((*smallest.borrow()) as u64) as usize]
                    .upgrade()
                    .deref())
                .freq
                .borrow())))
        {
            (*smallest.borrow_mut()) = (*left.borrow());
        }
        if (((*right.borrow()) < (*self.size.borrow()))
            && ((*(*(*self.arr.borrow()).as_ref().unwrap().borrow()
                [((*right.borrow()) as u64) as usize]
                .upgrade()
                .deref())
            .freq
            .borrow())
                < (*(*(*self.arr.borrow()).as_ref().unwrap().borrow()
                    [((*smallest.borrow()) as u64) as usize]
                    .upgrade()
                    .deref())
                .freq
                .borrow())))
        {
            (*smallest.borrow_mut()) = (*right.borrow());
        }
        if ((*smallest.borrow()) != (*idx.borrow())) {
            ({
                let _a: Ptr<MinHeapNode> = ((*self.arr.borrow()).as_ref().unwrap().borrow()
                    [((*smallest.borrow()) as u64) as usize])
                    .clone();
                let _b: Ptr<MinHeapNode> = ((*self.arr.borrow()).as_ref().unwrap().borrow()
                    [((*idx.borrow()) as u64) as usize])
                    .clone();
                Swap_0(_a, _b)
            });
            ({
                let _idx: i32 = (*smallest.borrow());
                self.Heapify(_idx)
            });
        }
    }
    pub fn ExtractMin(&self) -> Ptr<MinHeapNode> {
        let out: Value<Ptr<MinHeapNode>> = Rc::new(RefCell::new(
            ((*self.arr.borrow()).as_ref().unwrap().borrow()[(0_u64) as usize]).clone(),
        ));
        (*self.size.borrow_mut()).prefix_dec();
        let __rhs = ((*self.arr.borrow()).as_ref().unwrap().borrow()
            [((*self.size.borrow()) as u64) as usize])
            .clone();
        (*self.arr.borrow()).as_ref().unwrap().borrow_mut()[(0_u64) as usize] = __rhs;
        ({
            let _idx: i32 = 0;
            self.Heapify(_idx)
        });
        return (*out.borrow()).clone();
    }
    pub fn Insert(&self, node: Ptr<MinHeapNode>) {
        let node: Value<Ptr<MinHeapNode>> = Rc::new(RefCell::new(node));
        (*self.size.borrow_mut()).prefix_inc();
        let i: Value<i32> = Rc::new(RefCell::new(((*self.size.borrow()) - 1)));
        'loop_: while {
            let _lhs = ((*i.borrow()) != 0);
            _lhs && {
                let _lhs = (*(*(*node.borrow()).upgrade().deref()).freq.borrow());
                _lhs < (*(*(*self.arr.borrow()).as_ref().unwrap().borrow()
                    [((((*i.borrow()) - 1) / 2) as u64) as usize]
                    .upgrade()
                    .deref())
                .freq
                .borrow())
            }
        } {
            let __rhs = ((*self.arr.borrow()).as_ref().unwrap().borrow()
                [((((*i.borrow()) - 1) / 2) as u64) as usize])
                .clone();
            (*self.arr.borrow()).as_ref().unwrap().borrow_mut()[((*i.borrow()) as u64) as usize] =
                __rhs;
            let __rhs = (((*i.borrow()) - 1) / 2);
            (*i.borrow_mut()) = __rhs;
        }
        (*self.arr.borrow()).as_ref().unwrap().borrow_mut()[((*i.borrow()) as u64) as usize] =
            (*node.borrow()).clone();
    }
    pub fn Build(
        &self,
        data: Ptr<Option<Value<Box<[u8]>>>>,
        freq: Ptr<Option<Value<Box<[i32]>>>>,
        n: i32,
    ) {
        let n: Value<i32> = Rc::new(RefCell::new(n));
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < (*n.borrow())) {
            (*self.arr.borrow()).as_ref().unwrap().borrow_mut()
                [((*self.size.borrow_mut()).postfix_inc() as u64) as usize] = ({
                let _data: u8 = (*data.upgrade().deref()).as_ref().unwrap().borrow()
                    [((*i.borrow()) as u64) as usize];
                let _freq: i32 = (*freq.upgrade().deref()).as_ref().unwrap().borrow()
                    [((*i.borrow()) as u64) as usize];
                self.Alloc(_data, _freq)
            });
            (*i.borrow_mut()).prefix_inc();
        }
        let i: Value<i32> = Rc::new(RefCell::new((((*self.size.borrow()) - 2) / 2)));
        'loop_: while ((*i.borrow()) >= 0) {
            ({
                let _idx: i32 = (*i.borrow());
                self.Heapify(_idx)
            });
            (*i.borrow_mut()).prefix_dec();
        }
    }
}
impl ByteRepr for MinHeap {}
pub fn AllocMinHeap_1(capacity: i32) -> Option<Value<MinHeap>> {
    let capacity: Value<i32> = Rc::new(RefCell::new(capacity));
    let minHeap: Value<Option<Value<MinHeap>>> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new(MinHeap {
            size: Rc::new(RefCell::new(0)),
            capacity: Rc::new(RefCell::new((*capacity.borrow()))),
            arr: Rc::new(RefCell::new(Some(Rc::new(RefCell::new(
                (0..((*capacity.borrow()) as u64))
                    .map(|_| <Ptr<MinHeapNode>>::default())
                    .collect::<Box<[_]>>(),
            ))))),
            next: Rc::new(RefCell::new(0)),
            alloc: Rc::new(RefCell::new(Some(Rc::new(RefCell::new(
                (0..10000_u64)
                    .map(|_| <MinHeapNode>::default())
                    .collect::<Box<[_]>>(),
            ))))),
        })))));
    return (*minHeap.borrow_mut()).take();
}
pub fn Huffman_2(
    data: Ptr<Option<Value<Box<[u8]>>>>,
    freq: Ptr<Option<Value<Box<[i32]>>>>,
    size: i32,
) -> Option<Value<MinHeap>> {
    let size: Value<i32> = Rc::new(RefCell::new(size));
    let minHeap: Value<Option<Value<MinHeap>>> = Rc::new(RefCell::new(
        ({
            let _capacity: i32 = (*size.borrow());
            AllocMinHeap_1(_capacity)
        }),
    ));
    ({
        let _data: Ptr<Option<Value<Box<[u8]>>>> = (data).clone();
        let _freq: Ptr<Option<Value<Box<[i32]>>>> = (freq).clone();
        let _n: i32 = (*size.borrow());
        (*(*minHeap.borrow()).as_ref().unwrap().borrow()).Build(_data, _freq, _n)
    });
    'loop_: while ((*(*(*minHeap.borrow()).as_ref().unwrap().borrow())
        .size
        .borrow())
        != 1)
    {
        let left: Value<Ptr<MinHeapNode>> = Rc::new(RefCell::new(
            ({ (*(*minHeap.borrow()).as_ref().unwrap().borrow()).ExtractMin() }),
        ));
        let right: Value<Ptr<MinHeapNode>> = Rc::new(RefCell::new(
            ({ (*(*minHeap.borrow()).as_ref().unwrap().borrow()).ExtractMin() }),
        ));
        let top: Value<Ptr<MinHeapNode>> = Rc::new(RefCell::new(
            ({
                let _data: u8 = ('$' as u8);
                let _freq: i32 = {
                    let _lhs = (*(*(*left.borrow()).upgrade().deref()).freq.borrow());
                    _lhs + (*(*(*right.borrow()).upgrade().deref()).freq.borrow())
                };
                (*(*minHeap.borrow()).as_ref().unwrap().borrow()).Alloc(_data, _freq)
            }),
        ));
        (*(*(*top.borrow()).upgrade().deref()).left.borrow_mut()) = (*left.borrow()).clone();
        (*(*(*top.borrow()).upgrade().deref()).right.borrow_mut()) = (*right.borrow()).clone();
        ({
            let _node: Ptr<MinHeapNode> = (*top.borrow()).clone();
            (*(*minHeap.borrow()).as_ref().unwrap().borrow()).Insert(_node)
        });
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
    (*out.upgrade().deref()).as_ref().unwrap().borrow_mut()[((next.read()) as u64) as usize] = 0;
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*top.borrow())) {
        let __rhs = ((*out.upgrade().deref()).as_ref().unwrap().borrow()
            [((next.read()) as u64) as usize]
            * 10);
        (*out.upgrade().deref()).as_ref().unwrap().borrow_mut()[((next.read()) as u64) as usize] =
            __rhs;
        let __rhs = {
            let _lhs = (*out.upgrade().deref()).as_ref().unwrap().borrow()
                [((next.read()) as u64) as usize];
            _lhs + (*arr.upgrade().deref()).as_ref().unwrap().borrow()
                [((*i.borrow()) as u64) as usize]
        };
        (*out.upgrade().deref()).as_ref().unwrap().borrow_mut()[((next.read()) as u64) as usize] =
            __rhs;
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
    if !((*(*(*root.borrow()).upgrade().deref()).left.borrow()).is_null()) {
        (*arr.upgrade().deref()).as_ref().unwrap().borrow_mut()
            [((*top.borrow()) as u64) as usize] = 0;
        ({
            let _root: Ptr<MinHeapNode> =
                (*(*(*root.borrow()).upgrade().deref()).left.borrow()).clone();
            let _arr: Ptr<Option<Value<Box<[i32]>>>> = (arr).clone();
            let _top: i32 = ((*top.borrow()) + 1);
            let _out: Ptr<Option<Value<Box<[i32]>>>> = (out).clone();
            let _next: Ptr<i32> = (next).clone();
            CollectCodes_4(_root, _arr, _top, _out, _next)
        });
    }
    if !((*(*(*root.borrow()).upgrade().deref()).right.borrow()).is_null()) {
        (*arr.upgrade().deref()).as_ref().unwrap().borrow_mut()
            [((*top.borrow()) as u64) as usize] = 1;
        ({
            let _root: Ptr<MinHeapNode> =
                (*(*(*root.borrow()).upgrade().deref()).right.borrow()).clone();
            let _arr: Ptr<Option<Value<Box<[i32]>>>> = (arr).clone();
            let _top: i32 = ((*top.borrow()) + 1);
            let _out: Ptr<Option<Value<Box<[i32]>>>> = (out).clone();
            let _next: Ptr<i32> = (next).clone();
            CollectCodes_4(_root, _arr, _top, _out, _next)
        });
    }
    if ({ (*(*root.borrow()).upgrade().deref()).IsLeaf() }) {
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
        ({ (*(*minHeap.borrow()).as_ref().unwrap().borrow()).ExtractMin() }),
    ));
    let arr: Value<Option<Value<Box<[i32]>>>> = Rc::new(RefCell::new(Some(Rc::new(RefCell::new(
        (0..100_u64).map(|_| <i32>::default()).collect::<Box<[_]>>(),
    )))));
    let out: Value<Option<Value<Box<[i32]>>>> = Rc::new(RefCell::new(Some(Rc::new(RefCell::new(
        (0..100_u64).map(|_| <i32>::default()).collect::<Box<[_]>>(),
    )))));
    let top: Value<i32> = Rc::new(RefCell::new(0));
    let next: Value<i32> = Rc::new(RefCell::new(0));
    ({
        let _root: Ptr<MinHeapNode> = (*root.borrow()).clone();
        let _arr: Ptr<Option<Value<Box<[i32]>>>> = arr.as_pointer();
        let _top: i32 = (*top.borrow());
        let _out: Ptr<Option<Value<Box<[i32]>>>> = out.as_pointer();
        let _next: Ptr<i32> = next.as_pointer();
        CollectCodes_4(_root, _arr, _top, _out, _next)
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
        (0..((*size.borrow()) as u64))
            .map(|_| <u8>::default())
            .collect::<Box<[_]>>(),
    )))));
    let freq: Value<Option<Value<Box<[i32]>>>> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new(
            (0..((*size.borrow()) as u64))
                .map(|_| <i32>::default())
                .collect::<Box<[_]>>(),
        )))));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*size.borrow())) {
        let __rhs = (*arr1.borrow())[(*i.borrow()) as usize];
        (*data.borrow()).as_ref().unwrap().borrow_mut()[((*i.borrow()) as u64) as usize] = __rhs;
        let __rhs = (*arr2.borrow())[(*i.borrow()) as usize];
        (*freq.borrow()).as_ref().unwrap().borrow_mut()[((*i.borrow()) as u64) as usize] = __rhs;
        (*i.borrow_mut()).prefix_inc();
    }
    let out: Value<Option<Value<Box<[i32]>>>> = Rc::new(RefCell::new(
        ({
            let _data: Ptr<Option<Value<Box<[u8]>>>> = data.as_pointer();
            let _freq: Ptr<Option<Value<Box<[i32]>>>> = freq.as_pointer();
            let _size: i32 = (*size.borrow());
            HuffmanCodes_5(_data, _freq, _size)
        }),
    ));
    return ((((((((*out.borrow()).as_ref().unwrap().borrow()[(0_u64) as usize] == 0)
        && ((*out.borrow()).as_ref().unwrap().borrow()[(1_u64) as usize] == 100))
        && ((*out.borrow()).as_ref().unwrap().borrow()[(2_u64) as usize] == 101))
        && ((*out.borrow()).as_ref().unwrap().borrow()[(3_u64) as usize] == 1100))
        && ((*out.borrow()).as_ref().unwrap().borrow()[(4_u64) as usize] == 1101))
        && ((*out.borrow()).as_ref().unwrap().borrow()[(5_u64) as usize] == 111))
        as i32);
}
