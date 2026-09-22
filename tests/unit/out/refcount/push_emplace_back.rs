extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Chunk {
    pub data: Value<i32>,
}
impl Clone for Chunk {
    fn clone(&self) -> Self {
        let __this: Value<Chunk> = Rc::new(RefCell::new(Self {
            data: Rc::new(RefCell::new((*self.data.borrow()))),
        }));
        let this: Ptr<Chunk> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Chunk {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.data.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            data: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
#[derive(Default)]
pub struct Writer {
    pub output: Value<Ptr<Vec<Chunk>>>,
    pub chunk: Value<Chunk>,
}
impl Clone for Writer {
    fn clone(&self) -> Self {
        let __this: Value<Writer> = Rc::new(RefCell::new(Self {
            output: Rc::new(RefCell::new((*self.output.borrow()).clone())),
            chunk: Rc::new(RefCell::new((*self.chunk.borrow()).clone())),
        }));
        let this: Ptr<Writer> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Writer {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.output.borrow()).to_bytes(&mut buf[0..8]);
        (*self.chunk.borrow()).to_bytes(&mut buf[8..12]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            output: Rc::new(RefCell::new(<Ptr<Vec<Chunk>>>::from_bytes(&buf[0..8]))),
            chunk: Rc::new(RefCell::new(<Chunk>::from_bytes(&buf[8..12]))),
        }
    }
}
#[derive(Default)]
pub struct JPEGData {
    pub com_data: Value<Vec<Value<Vec<u8>>>>,
    pub app_data: Value<Vec<Value<Vec<u8>>>>,
}
impl Clone for JPEGData {
    fn clone(&self) -> Self {
        let __this: Value<JPEGData> = Rc::new(RefCell::new(Self {
            com_data: Rc::new(RefCell::new(
                (*self.com_data.borrow())
                    .iter()
                    .map(|inner_vec| Rc::new(RefCell::new(inner_vec.borrow().clone())))
                    .collect(),
            )),
            app_data: Rc::new(RefCell::new(
                (*self.app_data.borrow())
                    .iter()
                    .map(|inner_vec| Rc::new(RefCell::new(inner_vec.borrow().clone())))
                    .collect(),
            )),
        }));
        let this: Ptr<JPEGData> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for JPEGData {
    fn byte_size() -> usize {
        48
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.com_data.borrow()).to_bytes(&mut buf[0..24]);
        (*self.app_data.borrow()).to_bytes(&mut buf[24..48]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            com_data: Rc::new(RefCell::new(<Vec<Value<Vec<u8>>>>::from_bytes(&buf[0..24]))),
            app_data: Rc::new(RefCell::new(<Vec<Value<Vec<u8>>>>::from_bytes(
                &buf[24..48],
            ))),
        }
    }
}
pub fn push_param_0(dest: Ptr<Vec<Value<Vec<u8>>>>) {
    let dest: Value<Ptr<Vec<Value<Vec<u8>>>>> = Rc::new(RefCell::new(dest));
    ((*dest.borrow()).clone() as Ptr<Vec<Value<Vec<u8>>>>)
        .with_mut(|__v: &mut Vec<Value<Vec<u8>>>| __v.push(Rc::new(RefCell::new(Vec::new()))));
}
pub fn push_local_from_field_1(jpg: Ptr<JPEGData>, cond: bool) {
    let jpg: Value<Ptr<JPEGData>> = Rc::new(RefCell::new(jpg));
    let cond: Value<bool> = Rc::new(RefCell::new(cond));
    let head: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([1_u8, 2_u8, 3_u8])));
    let dest: Value<Ptr<Vec<Value<Vec<u8>>>>> =
        Rc::new(RefCell::new(Ptr::<Vec<Value<Vec<u8>>>>::null()));
    if (*cond.borrow()) {
        (*dest.borrow_mut()) = ((*(*jpg.borrow()).upgrade().deref()).com_data.as_pointer());
    } else {
        (*dest.borrow_mut()) = ((*(*jpg.borrow()).upgrade().deref()).app_data.as_pointer());
    }
    ((*dest.borrow()).clone() as Ptr<Vec<Value<Vec<u8>>>>).with_mut(
        |__v: &mut Vec<Value<Vec<u8>>>| {
            __v.push(Rc::new(RefCell::new({
                let __count = (head.as_pointer() as Ptr<u8>)
                    .offset((3) as isize)
                    .get_offset()
                    - (head.as_pointer() as Ptr<u8>).get_offset();
                PtrValueIter::new(&(head.as_pointer() as Ptr<u8>), __count)
                    .map(|item| u8::try_from(item).ok().unwrap())
                    .collect::<Vec<_>>()
            })))
        },
    );
}
pub fn shrink_through_ptr_2(comps: Ptr<Vec<Chunk>>) {
    let comps: Value<Ptr<Vec<Chunk>>> = Rc::new(RefCell::new(comps));
    (*comps.borrow()).with_mut(|__v: &mut Vec<Chunk>| __v.shrink_to_fit());
}
pub fn nested_push_move_3(bw: Ptr<Writer>) {
    let bw: Value<Ptr<Writer>> = Rc::new(RefCell::new(bw));
    (*(*(*bw.borrow()).upgrade().deref()).output.borrow()).with_mut(|__v: &mut Vec<Chunk>| {
        __v.push((*(*(*bw.borrow()).upgrade().deref()).chunk.borrow()).clone())
    });
}
pub fn emplace_local_from_field_4(jpg: Ptr<JPEGData>, cond: bool) {
    let jpg: Value<Ptr<JPEGData>> = Rc::new(RefCell::new(jpg));
    let cond: Value<bool> = Rc::new(RefCell::new(cond));
    let head: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([1_u8, 2_u8, 3_u8])));
    let dest: Value<Ptr<Vec<Value<Vec<u8>>>>> =
        Rc::new(RefCell::new(Ptr::<Vec<Value<Vec<u8>>>>::null()));
    if (*cond.borrow()) {
        (*dest.borrow_mut()) = ((*(*jpg.borrow()).upgrade().deref()).com_data.as_pointer());
    } else {
        (*dest.borrow_mut()) = ((*(*jpg.borrow()).upgrade().deref()).app_data.as_pointer());
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
        ((*dest.borrow()).clone() as Ptr<Vec<Value<Vec<u8>>>>)
            .with_mut(|__v: &mut Vec<Value<Vec<u8>>>| __v.push(Rc::new(RefCell::new(__init))))
    };
}
pub fn nested_emplace_move_5(bw: Ptr<Writer>) {
    let bw: Value<Ptr<Writer>> = Rc::new(RefCell::new(bw));
    {
        let __init = (*(*(*bw.borrow()).upgrade().deref()).chunk.borrow()).clone();
        (*(*(*bw.borrow()).upgrade().deref()).output.borrow())
            .with_mut(|__v: &mut Vec<Chunk>| __v.push(__init))
    };
}
pub fn self_ref_push_6(comps: Ptr<Vec<Chunk>>) {
    let comps: Value<Ptr<Vec<Chunk>>> = Rc::new(RefCell::new(comps));
    {
        let a0_clone = (*((*comps.borrow()).decay() as Ptr<Chunk>).upgrade().deref()).clone();
        (*comps.borrow()).with_mut(|__v: &mut Vec<Chunk>| __v.push(a0_clone))
    };
}
#[derive()]
pub struct Pair {
    pub first: Value<i32>,
    pub second: Value<i32>,
}
impl Pair {
    pub fn Pair1() -> Self {
        let __this: Value<Pair> = Rc::new(RefCell::new(Self {
            first: Rc::new(RefCell::new(-1_i32)),
            second: Rc::new(RefCell::new(-1_i32)),
        }));
        let this: Ptr<Pair> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn Pair2(a: i32) -> Self {
        let a: Value<i32> = Rc::new(RefCell::new(a));
        let __this: Value<Pair> = Rc::new(RefCell::new(Self {
            first: Rc::new(RefCell::new((*a.borrow()))),
            second: Rc::new(RefCell::new(0)),
        }));
        let this: Ptr<Pair> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn Pair3(a: i32, b: i32) -> Self {
        let a: Value<i32> = Rc::new(RefCell::new(a));
        let b: Value<i32> = Rc::new(RefCell::new(b));
        let __this: Value<Pair> = Rc::new(RefCell::new(Self {
            first: Rc::new(RefCell::new((*a.borrow()))),
            second: Rc::new(RefCell::new(((*b.borrow()) * 2))),
        }));
        let this: Ptr<Pair> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for Pair {
    fn clone(&self) -> Self {
        let __this: Value<Pair> = Rc::new(RefCell::new(Self {
            first: Rc::new(RefCell::new((*self.first.borrow()))),
            second: Rc::new(RefCell::new((*self.second.borrow()))),
        }));
        let this: Ptr<Pair> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Pair {
    fn default() -> Self {
        { Pair::Pair1() }
    }
}
impl ByteRepr for Pair {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.first.borrow()).to_bytes(&mut buf[0..4]);
        (*self.second.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            first: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            second: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
        }
    }
}
pub fn emplace_ctor_args_7(pairs: Ptr<Vec<Pair>>) {
    let pairs: Value<Ptr<Vec<Pair>>> = Rc::new(RefCell::new(pairs));
    {
        let __init = Pair::Pair1();
        (*pairs.borrow()).with_mut(|__v: &mut Vec<Pair>| __v.push(__init))
    };
    {
        let __init = Pair::Pair2({ 3 });
        (*pairs.borrow()).with_mut(|__v: &mut Vec<Pair>| __v.push(__init))
    };
    {
        let __init = Pair::Pair3({ 4 }, { 5 });
        (*pairs.borrow()).with_mut(|__v: &mut Vec<Pair>| __v.push(__init))
    };
}
pub fn emplace_scalar_8(values: Ptr<Vec<i64>>, x: i32) {
    let values: Value<Ptr<Vec<i64>>> = Rc::new(RefCell::new(values));
    let x: Value<i32> = Rc::new(RefCell::new(x));
    {
        let __init = <i64>::default();
        (*values.borrow()).with_mut(|__v: &mut Vec<i64>| __v.push(__init))
    };
    {
        let __init = ((*x.borrow()) as i64);
        (*values.borrow()).with_mut(|__v: &mut Vec<i64>| __v.push(__init))
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
    assert!(((*(*jpg.borrow()).com_data.borrow()).len() == 1_usize));
    assert!(
        ((*(((*jpg.borrow()).com_data.as_pointer() as Ptr<Value<Vec<u8>>>)
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
        ((((((*jpg.borrow()).com_data.as_pointer() as Ptr<Value<Vec<u8>>>)
            .offset(0_usize)
            .upgrade()
            .deref()
            .as_pointer() as Ptr<u8>)
            .offset(0_usize)
            .read()) as i32)
            == 1)
    );
    assert!(
        ((((((*jpg.borrow()).com_data.as_pointer() as Ptr<Value<Vec<u8>>>)
            .offset(0_usize)
            .upgrade()
            .deref()
            .as_pointer() as Ptr<u8>)
            .offset(1_usize)
            .read()) as i32)
            == 2)
    );
    assert!(
        ((((((*jpg.borrow()).com_data.as_pointer() as Ptr<Value<Vec<u8>>>)
            .offset(0_usize)
            .upgrade()
            .deref()
            .as_pointer() as Ptr<u8>)
            .offset(2_usize)
            .read()) as i32)
            == 3)
    );
    assert!((*(*jpg.borrow()).app_data.borrow()).is_empty());
    let chunks: Value<Vec<Chunk>> = Rc::new(RefCell::new(Vec::new()));
    ({ shrink_through_ptr_2((chunks.as_pointer())) });
    assert!((*chunks.borrow()).is_empty());
    let w: Value<Writer> = Rc::new(RefCell::new(<Writer>::default()));
    (*(*(*w.borrow()).chunk.borrow()).data.borrow_mut()) = 42;
    (*(*w.borrow()).output.borrow_mut()) = (chunks.as_pointer());
    ({ nested_push_move_3((w.as_pointer())) });
    assert!(((*chunks.borrow()).len() == 1_usize));
    assert!(
        ((*(*(chunks.as_pointer() as Ptr<Chunk>)
            .offset(0_usize)
            .upgrade()
            .deref())
        .data
        .borrow())
            == 42)
    );
    ({ emplace_local_from_field_4((jpg.as_pointer()), false) });
    assert!(((*(*jpg.borrow()).app_data.borrow()).len() == 1_usize));
    assert!(
        ((*(((*jpg.borrow()).app_data.as_pointer() as Ptr<Value<Vec<u8>>>)
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
        ((((((*jpg.borrow()).app_data.as_pointer() as Ptr<Value<Vec<u8>>>)
            .offset(0_usize)
            .upgrade()
            .deref()
            .as_pointer() as Ptr<u8>)
            .offset(0_usize)
            .read()) as i32)
            == 1)
    );
    assert!(
        ((((((*jpg.borrow()).app_data.as_pointer() as Ptr<Value<Vec<u8>>>)
            .offset(0_usize)
            .upgrade()
            .deref()
            .as_pointer() as Ptr<u8>)
            .offset(2_usize)
            .read()) as i32)
            == 3)
    );
    assert!(((*(*jpg.borrow()).com_data.borrow()).len() == 1_usize));
    (*(*(*w.borrow()).chunk.borrow()).data.borrow_mut()) = 99;
    (*(*w.borrow()).output.borrow_mut()) = (chunks.as_pointer());
    ({ nested_emplace_move_5((w.as_pointer())) });
    assert!(((*chunks.borrow()).len() == 2_usize));
    assert!(
        ((*(*(chunks.as_pointer() as Ptr<Chunk>)
            .offset(1_usize)
            .upgrade()
            .deref())
        .data
        .borrow())
            == 99)
    );
    ({ self_ref_push_6((chunks.as_pointer())) });
    assert!(((*chunks.borrow()).len() == 3_usize));
    assert!(
        ((*(*(chunks.as_pointer() as Ptr<Chunk>)
            .offset(2_usize)
            .upgrade()
            .deref())
        .data
        .borrow())
            == 42)
    );
    let pairs: Value<Vec<Pair>> = Rc::new(RefCell::new(Vec::new()));
    ({ emplace_ctor_args_7((pairs.as_pointer())) });
    assert!(((*pairs.borrow()).len() == 3_usize));
    assert!(
        ((*(*(pairs.as_pointer() as Ptr<Pair>)
            .offset(0_usize)
            .upgrade()
            .deref())
        .first
        .borrow())
            == -1_i32)
            && ((*(*(pairs.as_pointer() as Ptr<Pair>)
                .offset(0_usize)
                .upgrade()
                .deref())
            .second
            .borrow())
                == -1_i32)
    );
    assert!(
        ((*(*(pairs.as_pointer() as Ptr<Pair>)
            .offset(1_usize)
            .upgrade()
            .deref())
        .first
        .borrow())
            == 3)
            && ((*(*(pairs.as_pointer() as Ptr<Pair>)
                .offset(1_usize)
                .upgrade()
                .deref())
            .second
            .borrow())
                == 0)
    );
    assert!(
        ((*(*(pairs.as_pointer() as Ptr<Pair>)
            .offset(2_usize)
            .upgrade()
            .deref())
        .first
        .borrow())
            == 4)
            && ((*(*(pairs.as_pointer() as Ptr<Pair>)
                .offset(2_usize)
                .upgrade()
                .deref())
            .second
            .borrow())
                == 10)
    );
    let values: Value<Vec<i64>> = Rc::new(RefCell::new(Vec::new()));
    ({ emplace_scalar_8((values.as_pointer()), 7) });
    assert!(((*values.borrow()).len() == 2_usize));
    assert!((((values.as_pointer() as Ptr<i64>).offset(0_usize).read()) == 0_i64));
    assert!((((values.as_pointer() as Ptr<i64>).offset(1_usize).read()) == 7_i64));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
