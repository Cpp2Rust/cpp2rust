extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Copy, Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(48)]
pub struct JPEGData {
    #[offset(0)]
    #[byte_size(24)]
    pub com_data: Vec<Vec<u8>>,
    #[offset(24)]
    #[byte_size(24)]
    pub app_data: Vec<Vec<u8>>,
}
impl JPEGData {
    pub fn move_from(_a0: Ptr<JPEGData>) -> Self {
        Self {
            com_data: field!(_a0, com_data).with_mut(std::mem::take),
            app_data: field!(_a0, app_data).with_mut(std::mem::take),
        }
    }
}
pub fn push_param_0(mut dest: Ptr<Vec<Vec<u8>>>) {
    {
        let __a1 = Vec::new();
        dest.with_mut(|__v: &mut Vec<Vec<u8>>| __v.push(__a1))
    };
}
pub fn push_local_from_field_1(mut jpg: Ptr<JPEGData>, mut cond: bool) {
    let head: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([1_u8, 2_u8, 3_u8])));
    let mut dest: Ptr<Vec<Vec<u8>>> = Ptr::<Vec<Vec<u8>>>::null();
    if cond {
        dest = (field_ref!(jpg, com_data).as_pointer());
    } else {
        dest = (field_ref!(jpg, app_data).as_pointer());
    }
    {
        let __a1 = {
            let mut __tmp_0 = { head.decay().offset((3) as isize) };
            let mut __v = Vec::new();
            let mut __it = head.decay();
            while __it != __tmp_0 {
                __v.push((__it.iter_deref().read()));
                __it.iter_inc();
            }
            __v
        };
        dest.with_mut(|__v: &mut Vec<Vec<u8>>| __v.push(__a1))
    };
}
pub fn shrink_through_ptr_2(mut comps: Ptr<Vec<Chunk>>) {
    comps.with_mut(|__v: &mut Vec<Chunk>| __v.shrink_to_fit());
}
pub fn nested_push_move_3(mut bw: Ptr<Writer>) {
    {
        let __a1 = bw.with(|__s| __s.chunk);
        bw.with(|__s| __s.output.clone())
            .with_mut(|__v: &mut Vec<Chunk>| __v.push(__a1))
    };
}
pub fn emplace_local_from_field_4(mut jpg: Ptr<JPEGData>, mut cond: bool) {
    let head: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([1_u8, 2_u8, 3_u8])));
    let mut dest: Ptr<Vec<Vec<u8>>> = Ptr::<Vec<Vec<u8>>>::null();
    if cond {
        dest = (field_ref!(jpg, com_data).as_pointer());
    } else {
        dest = (field_ref!(jpg, app_data).as_pointer());
    }
    {
        dest.with_mut(|__v: &mut Vec<Vec<u8>>| {
            __v.push({
                let mut __tmp_1 = { head.decay().offset((3) as isize) };
                let mut __v = Vec::new();
                let mut __it = head.decay();
                while __it != __tmp_1 {
                    __v.push((__it.iter_deref().read()));
                    __it.iter_inc();
                }
                __v
            })
        });
    };
}
pub fn nested_emplace_move_5(mut bw: Ptr<Writer>) {
    {
        let mut __tmp_2 = { bw.with(|__s| __s.output.clone()) };
        let mut __tmp_3 = { bw.with(|__s| __s.chunk) };
        __tmp_2.with_mut(|__v: &mut Vec<Chunk>| __v.push(__tmp_3));
    };
}
pub fn self_ref_push_6(mut comps: Ptr<Vec<Chunk>>) {
    {
        let a0_clone = ({ (*comps.decay().upgrade().deref()) }).clone();
        comps.with_mut(|__v: &mut Vec<Chunk>| __v.push(a0_clone))
    };
}
#[derive(Copy, Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom)]
#[cpp_from(i32, new_1, value)]
#[byte_size(8)]
pub struct Pair {
    #[offset(0)]
    pub first: i32,
    #[offset(4)]
    pub second: i32,
}
impl Pair {
    pub fn new_2() -> Self {
        Self {
            first: -1_i32,
            second: -1_i32,
        }
    }
    pub fn new_1(mut a: i32) -> Self {
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
        { Pair::new_2() }
    }
}
pub fn emplace_ctor_args_7(mut pairs: Ptr<Vec<Pair>>) {
    {
        pairs.with_mut(|__v: &mut Vec<Pair>| __v.push(Pair::new_2()));
    };
    {
        let mut __tmp_4 = { Pair::new_1({ 3 }) };
        pairs.with_mut(|__v: &mut Vec<Pair>| __v.push(__tmp_4));
    };
    {
        let mut __tmp_5 = { Pair::new_3({ 4 }, { 5 }) };
        pairs.with_mut(|__v: &mut Vec<Pair>| __v.push(__tmp_5));
    };
}
pub fn emplace_deque_8(mut queue: Ptr<Vec<Pair>>) {
    {
        let mut __tmp_6 = { Pair::new_3({ 6 }, { 7 }) };
        queue.with_mut(|__v: &mut Vec<Pair>| __v.push(__tmp_6));
    };
    {
        queue.with_mut(|__v: &mut Vec<Pair>| __v.push(Pair::new_2()));
    };
}
pub fn emplace_scalar_9(mut values: Ptr<Vec<i64>>, x: i32) {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    {
        values.with_mut(|__v: &mut Vec<i64>| __v.push(0_i64));
    };
    {
        values.with_mut(|__v: &mut Vec<i64>| __v.push(((*x.borrow()) as i64)));
    };
}
pub fn emplace_ref_through_ptr_10(mut pairs: Ptr<Vec<Pair>>) {
    let p: Ptr<Pair> = {
        let mut __tmp_7 = { Pair::new_3({ 1 }, { 2 }) };
        pairs.with_mut(|__b0| {
            (*__b0).push(__tmp_7);
            let __n = (*__b0).len() - 1;
            pairs.decay().offset((__n) as isize)
        })
    };
    field!(p, second).write(3);
    {
        field!(
            {
                let mut __tmp_8 = { Pair::new_1({ 4 }) };
                pairs.with_mut(|__b0| {
                    (*__b0).push(__tmp_8);
                    let __n = (*__b0).len() - 1;
                    pairs.decay().offset((__n) as isize)
                })
            },
            second
        )
        .with_mut(|__v| *__v = *__v + 1)
    };
}
pub fn emplace_addr_11(values: Ptr<Vec<i32>>) -> Ptr<i32> {
    return (values.with_mut(|__b0| {
        (*__b0).push(9);
        let __n = (*__b0).len() - 1;
        values.decay().offset((__n) as isize)
    }));
}
pub fn emplace_self_reads_12(mut values: Ptr<Vec<i64>>, mut pairs: Ptr<Vec<Pair>>) {
    {
        let mut __tmp_9 = { (((*values.upgrade().deref()).len()) as i64) };
        values.with_mut(|__v: &mut Vec<i64>| __v.push(__tmp_9));
    };
    {
        let mut __tmp_10 = { index!(pairs, 0_usize).read() };
        pairs.with_mut(|__v: &mut Vec<Pair>| __v.push(__tmp_10));
    };
    {
        let mut __tmp_11 = {
            Pair::new_3(
                {
                    pairs.with(|__s| {
                        (*{
                            let __n = __s.len() - 1;
                            pairs.decay().offset((__n) as isize)
                        }
                        .upgrade()
                        .deref())
                        .first
                    })
                },
                { (((*pairs.upgrade().deref()).len()) as i32) },
            )
        };
        pairs.with_mut(|__v: &mut Vec<Pair>| __v.push(__tmp_11));
    };
}
pub fn emplace_ref_self_read_13(values: Ptr<Vec<i64>>) -> Ptr<i64> {
    return {
        let mut __tmp_12 = { (values.decay().read()) };
        values.with_mut(|__b0| {
            (*__b0).push(__tmp_12);
            let __n = (*__b0).len() - 1;
            values.decay().offset((__n) as isize)
        })
    };
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(32)]
pub struct Bytes {
    #[offset(0)]
    #[byte_size(24)]
    pub v: Vec<Vec<u8>>,
    #[offset(24)]
    #[byte_size(3)]
    pub b: Box<[u8]>,
}
impl Bytes {
    pub fn move_from(_a0: Ptr<Bytes>) -> Self {
        Self {
            v: field!(_a0, v).with_mut(std::mem::take),
            b: Box::new(std::array::from_fn::<_, 3, _>(|__i: usize| {
                (*_a0.upgrade().deref()).b[(__i) as usize]
            })),
        }
    }
}
impl Default for Bytes {
    fn default() -> Self {
        {
            Bytes {
                v: Default::default(),
                b: (0..3).map(|_| 0_u8).collect::<Box<[u8]>>(),
            }
        }
    }
}
pub fn emplace_from_element_14(mut v: Ptr<Vec<Vec<u8>>>) {
    let p: Value<Ptr<u8>> = Rc::new(RefCell::new((v).decay().offset(0_usize).decay()));
    {
        let mut __tmp_15 = {
            {
                let mut __tmp_14 = { (*p.borrow()).offset((3) as isize) };
                let mut __v = Vec::new();
                let mut __it = ({ (*p.borrow()).clone() });
                while __it != __tmp_14 {
                    __v.push((__it.iter_deref().read()));
                    __it.iter_inc();
                }
                __v
            }
        };
        v.with_mut(|__v: &mut Vec<Vec<u8>>| __v.push(__tmp_15));
    };
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
pub struct Plain {
    #[offset(0)]
    pub a: i32,
    #[offset(8)]
    #[byte_size(16)]
    pub b: Box<[f64]>,
}
impl Default for Plain {
    fn default() -> Self {
        {
            Plain {
                a: 0_i32,
                b: (0..2).map(|_| 0_f64).collect::<Box<[f64]>>(),
            }
        }
    }
}
pub fn emplace_plain_15(mut v: Ptr<Vec<Plain>>) {
    let p: Value<Plain> = Rc::new(RefCell::new(Plain {
        a: 1,
        b: Box::new([2_f64, 3_f64]),
    }));
    {
        v.with_mut(|__v: &mut Vec<Plain>| __v.push((*p.borrow()).clone()));
    };
}
thread_local!(
    pub static all_counted_16: Value<Ptr<Vec<Counted>>> =
        Rc::new(RefCell::new(Ptr::<Vec<Counted>>::null()));
);
#[derive(Copy, Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Counted {
    #[offset(0)]
    pub seen: usize,
}
impl Counted {
    pub fn new(mut _a0: i32) -> Self {
        Self {
            seen: (*(*all_counted_16.with(Value::clone).borrow())
                .upgrade()
                .deref())
            .len(),
        }
    }
}
impl Counted {}
pub fn emplace_counted_17(mut v: Ptr<Vec<Counted>>) {
    {
        let mut __tmp_16 = { Counted::new({ 1 }) };
        v.with_mut(|__v: &mut Vec<Counted>| __v.push(__tmp_16));
    };
}
pub fn emplace_from_sibling_18(mut s: Ptr<Bytes>) {
    let p: Value<Ptr<u8>> = Rc::new(RefCell::new(array_field_ptr!(s, b)));
    let mut v: Ptr<Vec<Vec<u8>>> = (field_ref!(s, v).as_pointer());
    {
        let mut __tmp_18 = {
            {
                let mut __tmp_17 = { (*p.borrow()).offset((3) as isize) };
                let mut __v = Vec::new();
                let mut __it = ({ (*p.borrow()).clone() });
                while __it != __tmp_17 {
                    __v.push((__it.iter_deref().read()));
                    __it.iter_inc();
                }
                __v
            }
        };
        v.with_mut(|__v: &mut Vec<Vec<u8>>| __v.push(__tmp_18));
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let vecs: Value<Vec<Vec<u8>>> = Rc::new(RefCell::new(Vec::new()));
    ({ push_param_0((vecs.as_pointer())) });
    assert!(({ ((*vecs.borrow()).len() == 1_usize) }));
    assert!(({ (*vecs.decay().offset(0_usize).upgrade().deref()).is_empty() }));
    let jpg: Value<JPEGData> = Rc::new(RefCell::new(<JPEGData>::default()));
    ({ push_local_from_field_1((jpg.as_pointer()), true) });
    assert!(({ ((*jpg.borrow()).com_data.len() == 1_usize) }));
    assert!(
        ({
            ((*field_ref!(jpg.as_pointer(), com_data)
                .decay()
                .offset(0_usize)
                .upgrade()
                .deref())
            .len()
                == 3_usize)
        })
    );
    assert!(
        (((index!(vec_elem!(jpg.as_pointer(), com_data, 0_usize), 0_usize).read()) as i32) == 1)
    );
    assert!(
        (((index!(vec_elem!(jpg.as_pointer(), com_data, 0_usize), 1_usize).read()) as i32) == 2)
    );
    assert!(
        (((index!(vec_elem!(jpg.as_pointer(), com_data, 0_usize), 2_usize).read()) as i32) == 3)
    );
    assert!(({ (*jpg.borrow()).app_data.is_empty() }));
    let chunks: Value<Vec<Chunk>> = Rc::new(RefCell::new(Vec::new()));
    ({ shrink_through_ptr_2((chunks.as_pointer())) });
    assert!(({ (*chunks.borrow()).is_empty() }));
    let w: Value<Writer> = Rc::new(RefCell::new(<Writer>::default()));
    (*w.borrow_mut()).chunk.data = 42;
    (*w.borrow_mut()).output = (chunks.as_pointer());
    ({ nested_push_move_3((w.as_pointer())) });
    assert!(({ ((*chunks.borrow()).len() == 1_usize) }));
    assert!(({ ({ { (*chunks.borrow())[0_usize] }.data } == 42) }));
    ({ emplace_local_from_field_4((jpg.as_pointer()), false) });
    assert!(({ ((*jpg.borrow()).app_data.len() == 1_usize) }));
    assert!(
        ({
            ((*field_ref!(jpg.as_pointer(), app_data)
                .decay()
                .offset(0_usize)
                .upgrade()
                .deref())
            .len()
                == 3_usize)
        })
    );
    assert!(
        (((index!(vec_elem!(jpg.as_pointer(), app_data, 0_usize), 0_usize).read()) as i32) == 1)
    );
    assert!(
        (((index!(vec_elem!(jpg.as_pointer(), app_data, 0_usize), 2_usize).read()) as i32) == 3)
    );
    assert!(({ ((*jpg.borrow()).com_data.len() == 1_usize) }));
    (*w.borrow_mut()).chunk.data = 99;
    (*w.borrow_mut()).output = (chunks.as_pointer());
    ({ nested_emplace_move_5((w.as_pointer())) });
    assert!(({ ((*chunks.borrow()).len() == 2_usize) }));
    assert!(({ ({ { (*chunks.borrow())[1_usize] }.data } == 99) }));
    ({ self_ref_push_6((chunks.as_pointer())) });
    assert!(({ ((*chunks.borrow()).len() == 3_usize) }));
    assert!(({ ({ { (*chunks.borrow())[2_usize] }.data } == 42) }));
    let pairs: Value<Vec<Pair>> = Rc::new(RefCell::new(Vec::new()));
    ({ emplace_ctor_args_7((pairs.as_pointer())) });
    assert!(({ ((*pairs.borrow()).len() == 3_usize) }));
    assert!(
        ({
            ({ { (*pairs.borrow())[0_usize] }.first } == -1_i32)
                && ({ { (*pairs.borrow())[0_usize] }.second } == -1_i32)
        })
    );
    assert!(
        ({
            ({ { (*pairs.borrow())[1_usize] }.first } == 3)
                && ({ { (*pairs.borrow())[1_usize] }.second } == 0)
        })
    );
    assert!(
        ({
            ({ { (*pairs.borrow())[2_usize] }.first } == 4)
                && ({ { (*pairs.borrow())[2_usize] }.second } == 10)
        })
    );
    let queue: Value<Vec<Pair>> = Rc::new(RefCell::new(Vec::new()));
    ({ emplace_deque_8((queue.as_pointer())) });
    assert!(
        (queue.decay().with(|__s| __s.first) == 6) && (queue.decay().with(|__s| __s.second) == 14)
    );
    assert!(
        ({
            ({
                let __n = (*queue.borrow()).len() - 1;
                queue.decay().offset((__n) as isize)
            }
            .with(|__s| __s.first)
                == -1_i32)
                && ({
                    let __n = (*queue.borrow()).len() - 1;
                    queue.decay().offset((__n) as isize)
                }
                .with(|__s| __s.second)
                    == -1_i32)
        })
    );
    let values: Value<Vec<i64>> = Rc::new(RefCell::new(Vec::new()));
    ({ emplace_scalar_9((values.as_pointer()), 7) });
    assert!(({ ((*values.borrow()).len() == 2_usize) }));
    assert!(({ ({ (*values.borrow())[0_usize] } == 0_i64) }));
    assert!(({ ({ (*values.borrow())[1_usize] } == 7_i64) }));
    let ints: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    {
        (*ints.borrow_mut()).push(0);
    };
    let r: Ptr<i32> = ints.with_mut(|__b0| {
        (*__b0).push(1);
        let __n = (*__b0).len() - 1;
        ints.decay().offset((__n) as isize)
    });
    r.write(5);
    let mut q: Ptr<i32> = ({ emplace_addr_11(ints.as_pointer()) });
    {
        q.with_mut(|__v| *__v = *__v + 1)
    };
    assert!(
        ({
            (((*ints.borrow()).len() == 3_usize) && ({ (*ints.borrow())[1_usize] } == 5))
                && ({ (*ints.borrow())[2_usize] } == 10)
        })
    );
    let ints_queue: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    {
        {
            (*ints_queue.borrow_mut()).push(2);
            let __n = (*ints_queue.borrow()).len() - 1;
            ints_queue.decay().offset((__n) as isize)
        }
        .with_mut(|__v| *__v = *__v + 3)
    };
    assert!(((ints_queue.decay().read()) == 5));
    let more_pairs: Value<Vec<Pair>> = Rc::new(RefCell::new(Vec::new()));
    ({ emplace_ref_through_ptr_10((more_pairs.as_pointer())) });
    assert!(
        ({
            ({ { (*more_pairs.borrow())[0_usize] }.first } == 1)
                && ({ { (*more_pairs.borrow())[0_usize] }.second } == 3)
        })
    );
    assert!(
        ({
            ({ { (*more_pairs.borrow())[1_usize] }.first } == 4)
                && ({ { (*more_pairs.borrow())[1_usize] }.second } == 1)
        })
    );
    let self_values: Value<Vec<i64>> = Rc::new(RefCell::new(Vec::from([7_i64])));
    let self_pairs: Value<Vec<Pair>> = Rc::new(RefCell::new(Vec::new()));
    {
        let mut __tmp_19 = { Pair::new_3({ 1 }, { 2 }) };
        (*self_pairs.borrow_mut()).push(__tmp_19);
    };
    ({ emplace_self_reads_12((self_values.as_pointer()), (self_pairs.as_pointer())) });
    {
        ({ emplace_ref_self_read_13(self_values.as_pointer()) }).with_mut(|__v| *__v = *__v + 1_i64)
    };
    assert!(
        ({
            (((*self_values.borrow()).len() == 3_usize)
                && ({ (*self_values.borrow())[1_usize] } == 1_i64))
                && ({ (*self_values.borrow())[2_usize] } == 8_i64)
        })
    );
    assert!(
        ({
            ({ { (*self_pairs.borrow())[1_usize] }.first } == 1)
                && ({ { (*self_pairs.borrow())[1_usize] }.second } == 4)
        })
    );
    assert!(
        ({
            ({ { (*self_pairs.borrow())[2_usize] }.first } == 1)
                && ({ { (*self_pairs.borrow())[2_usize] }.second } == 4)
        })
    );
    let rows: Value<Vec<Vec<u8>>> =
        Rc::new(RefCell::new(Vec::from([Vec::from([1_u8, 2_u8, 3_u8])])));
    ({ emplace_from_element_14((rows.as_pointer())) });
    assert!(
        ({
            ((*rows.borrow()).len() == 2_usize)
                && (((index!(elem!(rows.decay(), 1_usize), 2_usize).read()) as i32) == 3)
        })
    );
    let bytes: Value<Bytes> = Rc::new(RefCell::new(Bytes {
        v: Vec::new(),
        b: Box::new([4_u8, 5_u8, 6_u8]),
    }));
    ({ emplace_from_sibling_18((bytes.as_pointer())) });
    assert!(
        ({
            ((*bytes.borrow()).v.len() == 1_usize)
                && (((index!(vec_elem!(bytes.as_pointer(), v, 0_usize), 2_usize).read()) as i32)
                    == 6)
        })
    );
    let plains: Value<Vec<Plain>> = Rc::new(RefCell::new(Vec::new()));
    ({ emplace_plain_15((plains.as_pointer())) });
    assert!(
        ({
            ({ (*plains.borrow())[0_usize].a } == 1)
                && ((array_elem!(plains.decay().offset(0_usize), b, 1).read()) == 3_f64)
        })
    );
    let counted: Value<Vec<Counted>> = Rc::new(RefCell::new(Vec::new()));
    all_counted_16.with(|rc| *rc.borrow_mut() = (counted.as_pointer()));
    ({ emplace_counted_17((counted.as_pointer())) });
    ({ emplace_counted_17((counted.as_pointer())) });
    assert!(
        ({
            ({ { (*counted.borrow())[0_usize] }.seen } == 0_usize)
                && ({ { (*counted.borrow())[1_usize] }.seen } == 1_usize)
        })
    );
    return 0;
}
pub trait BytesImpl {
    fn move_assign(&self, _a0: Ptr<Bytes>) -> Ptr<Bytes>;
}
impl BytesImpl for Ptr<Bytes> {
    fn move_assign(&self, _a0: Ptr<Bytes>) -> Ptr<Bytes> {
        field!((*self), v).with_mut(|__b0| {
            (*__b0) = field!(_a0, v).with_mut(std::mem::take);
        });
        {
            let mut __tmp_13 = { (array_field_ptr!((*self), b)).to_any() };
            __tmp_13.memcpy(&(array_field_ptr!(_a0, b)).to_any(), 3_usize);
            (__tmp_13).clone()
        };
        return (*self).clone();
    }
}
pub trait JPEGDataImpl {
    fn move_assign(&self, _a0: Ptr<JPEGData>) -> Ptr<JPEGData>;
}
impl JPEGDataImpl for Ptr<JPEGData> {
    fn move_assign(&self, _a0: Ptr<JPEGData>) -> Ptr<JPEGData> {
        field!((*self), com_data).with_mut(|__b0| {
            (*__b0) = field!(_a0, com_data).with_mut(std::mem::take);
        });
        field!((*self), app_data).with_mut(|__b0| {
            (*__b0) = field!(_a0, app_data).with_mut(std::mem::take);
        });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = all_counted_16.with(|_| ());
}
