extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Chunk {
    #[offset(0)]
    pub data: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Writer {
    #[offset(0)]
    #[byte_size(8)]
    pub output: Ptr<Vec<Chunk>>,
    #[offset(8)]
    #[byte_size(4)]
    pub chunk: Chunk,
}
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(48)]
pub struct JPEGData {
    #[offset(0)]
    #[byte_size(24)]
    pub com_data: Value<Vec<Value<Vec<u8>>>>,
    #[offset(24)]
    #[byte_size(24)]
    pub app_data: Value<Vec<Value<Vec<u8>>>>,
}
pub fn push_param_0(mut dest: Ptr<Vec<Value<Vec<u8>>>>) {
    ((dest).clone() as Ptr<Vec<Value<Vec<u8>>>>)
        .with_mut(|__v: &mut Vec<Value<Vec<u8>>>| __v.push(Rc::new(RefCell::new(Vec::new()))));
}
pub fn push_local_from_field_1(mut jpg: Ptr<JPEGData>, mut cond: bool) {
    let head: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([1_u8, 2_u8, 3_u8])));
    let mut dest: Ptr<Vec<Value<Vec<u8>>>> = Ptr::<Vec<Value<Vec<u8>>>>::null();
    if cond {
        dest = (jpg.with(|__s| __s.com_data.as_pointer()));
    } else {
        dest = (jpg.with(|__s| __s.app_data.as_pointer()));
    }
    ((dest).clone() as Ptr<Vec<Value<Vec<u8>>>>).with_mut(|__v: &mut Vec<Value<Vec<u8>>>| {
        __v.push(Rc::new(RefCell::new({
            let __count = (head.as_pointer() as Ptr<u8>)
                .offset((3) as isize)
                .get_offset()
                - (head.as_pointer() as Ptr<u8>).get_offset();
            PtrValueIter::new(&(head.as_pointer() as Ptr<u8>), __count)
                .map(|item| u8::try_from(item).ok().unwrap())
                .collect::<Vec<_>>()
        })))
    });
}
pub fn shrink_through_ptr_2(mut comps: Ptr<Vec<Chunk>>) {
    comps.with_mut(|__v: &mut Vec<Chunk>| __v.shrink_to_fit());
}
pub fn nested_push_move_3(mut bw: Ptr<Writer>) {
    {
        let __a1 = ((*bw.upgrade().deref()).chunk).clone();
        bw.with(|__s| __s.output.clone())
            .with_mut(|__v: &mut Vec<Chunk>| __v.push(__a1))
    };
}
pub fn emplace_local_from_field_4(mut jpg: Ptr<JPEGData>, mut cond: bool) {
    let head: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([1_u8, 2_u8, 3_u8])));
    let mut dest: Ptr<Vec<Value<Vec<u8>>>> = Ptr::<Vec<Value<Vec<u8>>>>::null();
    if cond {
        dest = (jpg.with(|__s| __s.com_data.as_pointer()));
    } else {
        dest = (jpg.with(|__s| __s.app_data.as_pointer()));
    }
    {
        let __init = {
            let __count = (head.as_pointer() as Ptr<u8>)
                .offset((3) as isize)
                .get_offset()
                - (head.as_pointer() as Ptr<u8>).get_offset();
            PtrValueIter::new(&(head.as_pointer() as Ptr<u8>), __count)
                .map(|item| u8::try_from(item).ok().unwrap())
                .collect::<Vec<_>>()
        };
        ((dest).clone() as Ptr<Vec<Value<Vec<u8>>>>)
            .with_mut(|__v: &mut Vec<Value<Vec<u8>>>| __v.push(Rc::new(RefCell::new(__init))))
    };
}
pub fn nested_emplace_move_5(mut bw: Ptr<Writer>) {
    {
        let __init = ((*bw.upgrade().deref()).chunk).clone();
        bw.with(|__s| __s.output.clone())
            .with_mut(|__v: &mut Vec<Chunk>| __v.push(__init))
    };
}
pub fn self_ref_push_6(mut comps: Ptr<Vec<Chunk>>) {
    {
        let a0_clone = (*(Ptr::<Vec<Chunk>>::decay(&(comps)) as Ptr<Chunk>)
            .upgrade()
            .deref())
        .clone();
        comps.with_mut(|__v: &mut Vec<Chunk>| __v.push(a0_clone))
    };
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct Pair {
    #[offset(0)]
    pub first: i32,
    #[offset(4)]
    pub second: i32,
}
impl Pair {
    pub fn new_1() -> Self {
        Self {
            first: -1_i32,
            second: -1_i32,
        }
    }
    pub fn new_2(mut a: i32) -> Self {
        Self {
            first: a,
            second: 0,
        }
    }
    pub fn new_3(mut a: i32, mut b: i32) -> Self {
        Self {
            first: a,
            second: (b * 2),
        }
    }
}
impl Default for Pair {
    fn default() -> Self {
        { Pair::new_1() }
    }
}
pub fn emplace_ctor_args_7(mut pairs: Ptr<Vec<Pair>>) {
    {
        let __init = Pair::new_1();
        pairs.with_mut(|__v: &mut Vec<Pair>| __v.push(__init))
    };
    {
        let __init = Pair::new_2({ 3 });
        pairs.with_mut(|__v: &mut Vec<Pair>| __v.push(__init))
    };
    {
        let __init = Pair::new_3({ 4 }, { 5 });
        pairs.with_mut(|__v: &mut Vec<Pair>| __v.push(__init))
    };
}
pub fn emplace_deque_8(mut queue: Ptr<Vec<Pair>>) {
    {
        let __init = Pair::new_3({ 6 }, { 7 });
        queue.with_mut(|__v: &mut Vec<Pair>| __v.push(__init))
    };
    {
        let __init = Pair::new_1();
        queue.with_mut(|__v: &mut Vec<Pair>| __v.push(__init))
    };
}
pub fn emplace_scalar_9(mut values: Ptr<Vec<i64>>, x: i32) {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    {
        let __init = 0_i64;
        values.with_mut(|__v: &mut Vec<i64>| __v.push(__init))
    };
    {
        let __init = ((*x.borrow()) as i64);
        values.with_mut(|__v: &mut Vec<i64>| __v.push(__init))
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let vecs: Value<Vec<Value<Vec<u8>>>> = Rc::new(RefCell::new(Vec::new()));
    ({ push_param_0((vecs.as_pointer())) });
    assert!(((*vecs.borrow()).len() == 1_usize));
    assert!(
        (*((vecs.as_pointer() as Ptr<Value<Vec<u8>>>)
            .offset(0_usize)
            .upgrade()
            .deref()
            .as_pointer() as Ptr<Vec<u8>>)
            .upgrade()
            .deref())
        .is_empty()
    );
    let jpg: Value<JPEGData> = Rc::new(RefCell::new(<JPEGData>::default()));
    ({ push_local_from_field_1((jpg.as_pointer()), true) });
    assert!(((*{ (*jpg.borrow()).com_data.clone() }.borrow()).len() == 1_usize));
    assert!(
        ((*(({ (*jpg.borrow()).com_data.as_pointer() } as Ptr<Value<Vec<u8>>>)
            .offset(0_usize)
            .upgrade()
            .deref()
            .as_pointer() as Ptr<Vec<u8>>)
            .upgrade()
            .deref())
        .len()
            == 3_usize)
    );
    assert!(
        (((elem!(
            (({ (*jpg.borrow()).com_data.as_pointer() } as Ptr<Value<Vec<u8>>>)
                .offset(0_usize)
                .upgrade()
                .deref()
                .as_pointer() as Ptr<u8>),
            0_usize
        )
        .read()) as i32)
            == 1)
    );
    assert!(
        (((elem!(
            (({ (*jpg.borrow()).com_data.as_pointer() } as Ptr<Value<Vec<u8>>>)
                .offset(0_usize)
                .upgrade()
                .deref()
                .as_pointer() as Ptr<u8>),
            1_usize
        )
        .read()) as i32)
            == 2)
    );
    assert!(
        (((elem!(
            (({ (*jpg.borrow()).com_data.as_pointer() } as Ptr<Value<Vec<u8>>>)
                .offset(0_usize)
                .upgrade()
                .deref()
                .as_pointer() as Ptr<u8>),
            2_usize
        )
        .read()) as i32)
            == 3)
    );
    assert!((*{ (*jpg.borrow()).app_data.clone() }.borrow()).is_empty());
    let chunks: Value<Vec<Chunk>> = Rc::new(RefCell::new(Vec::new()));
    ({ shrink_through_ptr_2((chunks.as_pointer())) });
    assert!((*chunks.borrow()).is_empty());
    let w: Value<Writer> = Rc::new(RefCell::new(<Writer>::default()));
    (*w.borrow_mut()).chunk.data = 42;
    (*w.borrow_mut()).output = (chunks.as_pointer());
    ({ nested_push_move_3((w.as_pointer())) });
    assert!(((*chunks.borrow()).len() == 1_usize));
    assert!(({ (*chunks.borrow())[0_usize].data } == 42));
    ({ emplace_local_from_field_4((jpg.as_pointer()), false) });
    assert!(((*{ (*jpg.borrow()).app_data.clone() }.borrow()).len() == 1_usize));
    assert!(
        ((*(({ (*jpg.borrow()).app_data.as_pointer() } as Ptr<Value<Vec<u8>>>)
            .offset(0_usize)
            .upgrade()
            .deref()
            .as_pointer() as Ptr<Vec<u8>>)
            .upgrade()
            .deref())
        .len()
            == 3_usize)
    );
    assert!(
        (((elem!(
            (({ (*jpg.borrow()).app_data.as_pointer() } as Ptr<Value<Vec<u8>>>)
                .offset(0_usize)
                .upgrade()
                .deref()
                .as_pointer() as Ptr<u8>),
            0_usize
        )
        .read()) as i32)
            == 1)
    );
    assert!(
        (((elem!(
            (({ (*jpg.borrow()).app_data.as_pointer() } as Ptr<Value<Vec<u8>>>)
                .offset(0_usize)
                .upgrade()
                .deref()
                .as_pointer() as Ptr<u8>),
            2_usize
        )
        .read()) as i32)
            == 3)
    );
    assert!(((*{ (*jpg.borrow()).com_data.clone() }.borrow()).len() == 1_usize));
    (*w.borrow_mut()).chunk.data = 99;
    (*w.borrow_mut()).output = (chunks.as_pointer());
    ({ nested_emplace_move_5((w.as_pointer())) });
    assert!(((*chunks.borrow()).len() == 2_usize));
    assert!(({ (*chunks.borrow())[1_usize].data } == 99));
    ({ self_ref_push_6((chunks.as_pointer())) });
    assert!(((*chunks.borrow()).len() == 3_usize));
    assert!(({ (*chunks.borrow())[2_usize].data } == 42));
    let pairs: Value<Vec<Pair>> = Rc::new(RefCell::new(Vec::new()));
    ({ emplace_ctor_args_7((pairs.as_pointer())) });
    assert!(((*pairs.borrow()).len() == 3_usize));
    assert!(
        ({ (*pairs.borrow())[0_usize].first } == -1_i32)
            && ({ (*pairs.borrow())[0_usize].second } == -1_i32)
    );
    assert!(
        ({ (*pairs.borrow())[1_usize].first } == 3) && ({ (*pairs.borrow())[1_usize].second } == 0)
    );
    assert!(
        ({ (*pairs.borrow())[2_usize].first } == 4)
            && ({ (*pairs.borrow())[2_usize].second } == 10)
    );
    let queue: Value<Vec<Pair>> = Rc::new(RefCell::new(Vec::new()));
    ({ emplace_deque_8((queue.as_pointer())) });
    assert!(
        ((queue.as_pointer() as Ptr<Pair>).with(|__s| __s.first) == 6)
            && ((queue.as_pointer() as Ptr<Pair>).with(|__s| __s.second) == 14)
    );
    assert!(
        ((queue.as_pointer() as Ptr<Pair>)
            .to_last()
            .with(|__s| __s.first)
            == -1_i32)
            && ((queue.as_pointer() as Ptr<Pair>)
                .to_last()
                .with(|__s| __s.second)
                == -1_i32)
    );
    let values: Value<Vec<i64>> = Rc::new(RefCell::new(Vec::new()));
    ({ emplace_scalar_9((values.as_pointer()), 7) });
    assert!(((*values.borrow()).len() == 2_usize));
    assert!(({ (*values.borrow())[0_usize] } == 0_i64));
    assert!(({ (*values.borrow())[1_usize] } == 7_i64));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
