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
    pub data: u8,
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
    let t: Value<MinHeapNode> = Rc::new(RefCell::new(MinHeapNode {
        data: a.with(|__s| __s.data),
        freq: a.with(|__s| __s.freq),
        left: a.with(|__s| __s.left.clone()),
        right: a.with(|__s| __s.right.clone()),
    }));
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
            data: { (*t.borrow()).data },
            freq: { (*t.borrow()).freq },
            left: { (*t.borrow()).left.clone() },
            right: { (*t.borrow()).right.clone() },
        }
    });
}
#[derive(Record, ByteRepr, Default)]
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
        let __this: Value<MinHeap> = Rc::new(RefCell::new(Self {
            size: { (*_a0.upgrade().deref()).size },
            capacity: { (*_a0.upgrade().deref()).capacity },
            arr: field!(_a0, arr)
                .with_mut(|__v: &mut Option<Value<Box<[Ptr<MinHeapNode>]>>>| __v.take()),
            next: { (*_a0.upgrade().deref()).next },
            alloc: field!(_a0, alloc)
                .with_mut(|__v: &mut Option<Value<Box<[MinHeapNode]>>>| __v.take()),
        }));
        let this: Ptr<MinHeap> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
pub fn AllocMinHeap_1(capacity: i32) -> Option<Value<MinHeap>> {
    let capacity: Value<i32> = Rc::new(RefCell::new(capacity));
    let minHeap: Value<Option<Value<MinHeap>>> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new({
            let __tmp_0: Value<MinHeap> = Rc::new(RefCell::new(MinHeap {
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
            }));
            MinHeap::move_from({ __tmp_0.as_pointer() })
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
        MinHeapImpl::Build(&((*minHeap.borrow()).as_pointer()), _data, _freq, _n)
    });
    'loop_: while ({ (*(*minHeap.borrow()).as_ref().unwrap().borrow()).size } != 1) {
        let left: Value<Ptr<MinHeapNode>> = Rc::new(RefCell::new(
            ({ MinHeapImpl::ExtractMin(&((*minHeap.borrow()).as_pointer())) }),
        ));
        let right: Value<Ptr<MinHeapNode>> = Rc::new(RefCell::new(
            ({ MinHeapImpl::ExtractMin(&((*minHeap.borrow()).as_pointer())) }),
        ));
        let top: Value<Ptr<MinHeapNode>> = Rc::new(RefCell::new(
            ({
                MinHeapImpl::Alloc(
                    &((*minHeap.borrow()).as_pointer()),
                    ('$' as u8),
                    ({ (*left.borrow()).with(|__s| __s.freq) } + {
                        (*right.borrow()).with(|__s| __s.freq)
                    }),
                )
            }),
        ));
        field!((*top.borrow()), left).write((*left.borrow()).clone());
        field!((*top.borrow()), right).write((*right.borrow()).clone());
        ({ MinHeapImpl::Insert(&((*minHeap.borrow()).as_pointer()), (*top.borrow()).clone()) });
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
    (*out.upgrade().deref()).as_ref().unwrap().borrow_mut()[((next.read()) as usize) as usize] = 0;
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*top.borrow())) {
        let __rhs = ((*out.upgrade().deref()).as_ref().unwrap().borrow()
            [((next.read()) as usize) as usize]
            * 10);
        (*out.upgrade().deref()).as_ref().unwrap().borrow_mut()
            [((next.read()) as usize) as usize] = __rhs;
        let __rhs = ({
            (*out.upgrade().deref()).as_ref().unwrap().borrow()[((next.read()) as usize) as usize]
        } + {
            (*arr.upgrade().deref()).as_ref().unwrap().borrow()[((*i.borrow()) as usize) as usize]
        });
        (*out.upgrade().deref()).as_ref().unwrap().borrow_mut()
            [((next.read()) as usize) as usize] = __rhs;
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
    if !(((*root.borrow()).with(|__s| __s.left.clone())).is_null()) {
        (*arr.upgrade().deref()).as_ref().unwrap().borrow_mut()
            [((*top.borrow()) as usize) as usize] = 0;
        ({
            let _root: Ptr<MinHeapNode> = (*root.borrow()).with(|__s| __s.left.clone());
            let _arr: Ptr<Option<Value<Box<[i32]>>>> = (arr).clone();
            let _top: i32 = ((*top.borrow()) + 1);
            let _out: Ptr<Option<Value<Box<[i32]>>>> = (out).clone();
            let _next: Ptr<i32> = (next).clone();
            CollectCodes_4(_root, _arr, _top, _out, _next)
        });
    }
    if !(((*root.borrow()).with(|__s| __s.right.clone())).is_null()) {
        (*arr.upgrade().deref()).as_ref().unwrap().borrow_mut()
            [((*top.borrow()) as usize) as usize] = 1;
        ({
            let _root: Ptr<MinHeapNode> = (*root.borrow()).with(|__s| __s.right.clone());
            let _arr: Ptr<Option<Value<Box<[i32]>>>> = (arr).clone();
            let _top: i32 = ((*top.borrow()) + 1);
            let _out: Ptr<Option<Value<Box<[i32]>>>> = (out).clone();
            let _next: Ptr<i32> = (next).clone();
            CollectCodes_4(_root, _arr, _top, _out, _next)
        });
    }
    if ({ MinHeapNodeImpl::IsLeaf(&(*root.borrow())) }) {
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
        ({ MinHeapImpl::ExtractMin(&((*minHeap.borrow()).as_pointer())) }),
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
    __cpp2rust_init_globals();
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
        (*data.borrow()).as_ref().unwrap().borrow_mut()[((*i.borrow()) as usize) as usize] = __rhs;
        let __rhs = (*arr2.borrow())[(*i.borrow()) as usize];
        (*freq.borrow()).as_ref().unwrap().borrow_mut()[((*i.borrow()) as usize) as usize] = __rhs;
        (*i.borrow_mut()).prefix_inc();
    }
    let out: Value<Option<Value<Box<[i32]>>>> = Rc::new(RefCell::new(
        ({ HuffmanCodes_5(data.as_pointer(), freq.as_pointer(), (*size.borrow())) }),
    ));
    assert!(
        ((((((*out.borrow()).as_ref().unwrap().borrow()[(0_usize) as usize] == 0)
            && ((*out.borrow()).as_ref().unwrap().borrow()[(1_usize) as usize] == 100))
            && ((*out.borrow()).as_ref().unwrap().borrow()[(2_usize) as usize] == 101))
            && ((*out.borrow()).as_ref().unwrap().borrow()[(3_usize) as usize] == 1100))
            && ((*out.borrow()).as_ref().unwrap().borrow()[(4_usize) as usize] == 1101))
            && ((*out.borrow()).as_ref().unwrap().borrow()[(5_usize) as usize] == 111)
    );
    return 0;
}
pub trait MinHeapImpl {
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
    fn move_assign(&self, _a0: Ptr<MinHeap>) -> Ptr<MinHeap>;
}
impl MinHeapImpl for Ptr<MinHeap> {
    fn Alloc(&self, data: u8, freq: i32) -> Ptr<MinHeapNode> {
        let data: Value<u8> = Rc::new(RefCell::new(data));
        let freq: Value<i32> = Rc::new(RefCell::new(freq));
        (*self)
            .with(|__s| __s.alloc.clone())
            .as_ref()
            .unwrap()
            .borrow_mut()[((*self).with(|__s| __s.next) as usize) as usize] = MinHeapNode {
            data: (*data.borrow()),
            freq: (*freq.borrow()),
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
    fn Heapify(&self, idx: i32) {
        let idx: Value<i32> = Rc::new(RefCell::new(idx));
        let smallest: Value<i32> = Rc::new(RefCell::new((*idx.borrow())));
        let left: Value<i32> = Rc::new(RefCell::new(((2 * (*idx.borrow())) + 1)));
        let right: Value<i32> = Rc::new(RefCell::new(((2 * (*idx.borrow())) + 2)));
        if ((*left.borrow()) < (*self).with(|__s| __s.size))
            && ((*self)
                .with(|__s| __s.arr.clone())
                .as_ref()
                .unwrap()
                .borrow()[((*left.borrow()) as usize) as usize]
                .with(|__s| __s.freq)
                < (*self)
                    .with(|__s| __s.arr.clone())
                    .as_ref()
                    .unwrap()
                    .borrow()[((*smallest.borrow()) as usize) as usize]
                    .with(|__s| __s.freq))
        {
            (*smallest.borrow_mut()) = (*left.borrow());
        }
        if ((*right.borrow()) < (*self).with(|__s| __s.size))
            && ((*self)
                .with(|__s| __s.arr.clone())
                .as_ref()
                .unwrap()
                .borrow()[((*right.borrow()) as usize) as usize]
                .with(|__s| __s.freq)
                < (*self)
                    .with(|__s| __s.arr.clone())
                    .as_ref()
                    .unwrap()
                    .borrow()[((*smallest.borrow()) as usize) as usize]
                    .with(|__s| __s.freq))
        {
            (*smallest.borrow_mut()) = (*right.borrow());
        }
        if ((*smallest.borrow()) != (*idx.borrow())) {
            ({
                let _a: Ptr<MinHeapNode> = ((*self)
                    .with(|__s| __s.arr.clone())
                    .as_ref()
                    .unwrap()
                    .borrow()[((*smallest.borrow()) as usize) as usize])
                    .clone();
                let _b: Ptr<MinHeapNode> = ((*self)
                    .with(|__s| __s.arr.clone())
                    .as_ref()
                    .unwrap()
                    .borrow()[((*idx.borrow()) as usize) as usize])
                    .clone();
                Swap_0(_a, _b)
            });
            ({ MinHeapImpl::Heapify(self, (*smallest.borrow())) });
        }
    }
    fn ExtractMin(&self) -> Ptr<MinHeapNode> {
        let out: Value<Ptr<MinHeapNode>> = Rc::new(RefCell::new(
            ((*self)
                .with(|__s| __s.arr.clone())
                .as_ref()
                .unwrap()
                .borrow()[(0_usize) as usize])
                .clone(),
        ));
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
        return (*out.borrow()).clone();
    }
    fn Insert(&self, node: Ptr<MinHeapNode>) {
        let node: Value<Ptr<MinHeapNode>> = Rc::new(RefCell::new(node));
        field!((*self), size).with_mut(|__v| __v.prefix_inc());
        let i: Value<i32> = Rc::new(RefCell::new(((*self).with(|__s| __s.size) - 1)));
        'loop_: while ((*i.borrow()) != 0)
            && ({ (*node.borrow()).with(|__s| __s.freq) } < {
                (*self)
                    .with(|__s| __s.arr.clone())
                    .as_ref()
                    .unwrap()
                    .borrow()[((((*i.borrow()) - 1) / 2) as usize) as usize]
                    .with(|__s| __s.freq)
            })
        {
            let __rhs = ((*self)
                .with(|__s| __s.arr.clone())
                .as_ref()
                .unwrap()
                .borrow()[((((*i.borrow()) - 1) / 2) as usize) as usize])
                .clone();
            (*self)
                .with(|__s| __s.arr.clone())
                .as_ref()
                .unwrap()
                .borrow_mut()[((*i.borrow()) as usize) as usize] = __rhs;
            (*i.borrow_mut()) = { (((*i.borrow()) - 1) / 2) };
        }
        (*self)
            .with(|__s| __s.arr.clone())
            .as_ref()
            .unwrap()
            .borrow_mut()[((*i.borrow()) as usize) as usize] = (*node.borrow()).clone();
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
            (*self)
                .with(|__s| __s.arr.clone())
                .as_ref()
                .unwrap()
                .borrow_mut()
                [(field!((*self), size).with_mut(|__v| __v.postfix_inc()) as usize) as usize] = ({
                let _data: u8 = (*data.upgrade().deref()).as_ref().unwrap().borrow()
                    [((*i.borrow()) as usize) as usize];
                let _freq: i32 = (*freq.upgrade().deref()).as_ref().unwrap().borrow()
                    [((*i.borrow()) as usize) as usize];
                MinHeapImpl::Alloc(self, _data, _freq)
            });
            (*i.borrow_mut()).prefix_inc();
        }
        let i: Value<i32> = Rc::new(RefCell::new((((*self).with(|__s| __s.size) - 2) / 2)));
        'loop_: while ((*i.borrow()) >= 0) {
            ({ MinHeapImpl::Heapify(self, (*i.borrow())) });
            (*i.borrow_mut()).prefix_dec();
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
