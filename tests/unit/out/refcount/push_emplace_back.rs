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
        let mut this = Self {
            data: Rc::new(RefCell::new((*self.data.borrow()))),
        };
        this
    }
}
impl ByteRepr for Chunk {}
#[derive(Default)]
pub struct Writer {
    pub output: Value<Ptr<Vec<Chunk>>>,
    pub chunk: Value<Chunk>,
}
impl Clone for Writer {
    fn clone(&self) -> Self {
        let mut this = Self {
            output: Rc::new(RefCell::new((*self.output.borrow()).clone())),
            chunk: Rc::new(RefCell::new((*self.chunk.borrow()).clone())),
        };
        this
    }
}
impl ByteRepr for Writer {}
#[derive(Default)]
pub struct JPEGData {
    pub com_data: Value<Vec<Value<Vec<u8>>>>,
    pub app_data: Value<Vec<Value<Vec<u8>>>>,
}
impl Clone for JPEGData {
    fn clone(&self) -> Self {
        let mut this = Self {
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
        };
        this
    }
}
impl ByteRepr for JPEGData {}
pub fn push_param_0(dest: Ptr<Vec<Value<Vec<u8>>>>) {
    let dest: Value<Ptr<Vec<Value<Vec<u8>>>>> = Rc::new(RefCell::new(dest));
    ((*dest.borrow()).to_strong().as_pointer() as Ptr<Vec<Value<Vec<u8>>>>).with_mut(
        |__v: &mut Vec<Value<Vec<u8>>>| __v.push(Rc::new(RefCell::new(Vec::new().clone()))),
    );
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
    ((*dest.borrow()).to_strong().as_pointer() as Ptr<Vec<Value<Vec<u8>>>>).with_mut(
        |__v: &mut Vec<Value<Vec<u8>>>| {
            __v.push(Rc::new(RefCell::new(
                {
                    let mut __a0 = (head.as_pointer() as Ptr<u8>).clone();
                    let mut __out = Vec::with_capacity(
                        (head.as_pointer() as Ptr<u8>)
                            .offset((3) as isize)
                            .get_offset()
                            - __a0.get_offset(),
                    );
                    while __a0 != (head.as_pointer() as Ptr<u8>).offset((3) as isize) {
                        __out.push(__a0.read());
                        __a0 += 1;
                    }
                    __out
                }
                .clone(),
            )))
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
        __v.push(std::mem::take(
            &mut (*(*(*bw.borrow()).upgrade().deref()).chunk.borrow_mut()),
        ))
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
    (*dest.borrow())
        .to_strong()
        .as_pointer()
        .with_mut(|__v: &mut Vec<Value<Vec<u8>>>| {
            __v.push(Rc::new(RefCell::new({
                let mut __a0 = (head.as_pointer() as Ptr<u8>).clone();
                let mut __out = Vec::with_capacity(
                    (head.as_pointer() as Ptr<u8>)
                        .offset((3) as isize)
                        .get_offset()
                        - __a0.get_offset(),
                );
                while __a0 != (head.as_pointer() as Ptr<u8>).offset((3) as isize) {
                    __out.push(__a0.read());
                    __a0 += 1;
                }
                __out
            })))
        });
}
pub fn nested_emplace_move_5(bw: Ptr<Writer>) {
    let bw: Value<Ptr<Writer>> = Rc::new(RefCell::new(bw));
    (*(*(*bw.borrow()).upgrade().deref()).output.borrow())
        .to_strong()
        .as_pointer()
        .with_mut(|__v: &mut Vec<Chunk>| {
            __v.push(std::mem::take(
                &mut (*(*(*bw.borrow()).upgrade().deref()).chunk.borrow_mut()),
            ))
        });
}
pub fn self_ref_push_6(comps: Ptr<Vec<Chunk>>) {
    let comps: Value<Ptr<Vec<Chunk>>> = Rc::new(RefCell::new(comps));
    {
        let a0_clone = (*((*comps.borrow()).to_strong().as_pointer() as Ptr<Chunk>)
            .upgrade()
            .deref())
        .clone();
        (*comps.borrow()).with_mut(|__v: &mut Vec<Chunk>| __v.push(a0_clone))
    };
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let vecs: Value<Vec<Value<Vec<u8>>>> = Rc::new(RefCell::new(Vec::new()));
    ({
        let _dest: Ptr<Vec<Value<Vec<u8>>>> = (vecs.as_pointer());
        push_param_0(_dest)
    });
    assert!(((*vecs.borrow()).len() as u64 == 1_u64));
    assert!((*((vecs.as_pointer() as Ptr<Value<Vec<u8>>>)
        .offset(0_u64 as isize)
        .upgrade()
        .deref()
        .as_pointer() as Ptr<Vec<u8>>)
        .upgrade()
        .deref())
    .is_empty());
    let jpg: Value<JPEGData> = Rc::new(RefCell::new(<JPEGData>::default()));
    ({
        let _jpg: Ptr<JPEGData> = (jpg.as_pointer());
        let _cond: bool = true;
        push_local_from_field_1(_jpg, _cond)
    });
    assert!(((*(*jpg.borrow()).com_data.borrow()).len() as u64 == 1_u64));
    assert!(
        ((*(((*jpg.borrow()).com_data.as_pointer() as Ptr<Value<Vec<u8>>>)
            .offset(0_u64 as isize)
            .upgrade()
            .deref()
            .as_pointer() as Ptr<Vec<u8>>)
            .upgrade()
            .deref())
        .len() as u64
            == 3_u64)
    );
    assert!(
        ((((((*jpg.borrow()).com_data.as_pointer() as Ptr<Value<Vec<u8>>>)
            .offset(0_u64 as isize)
            .upgrade()
            .deref()
            .as_pointer() as Ptr<u8>)
            .offset(0_u64 as isize)
            .read()) as i32)
            == 1)
    );
    assert!(
        ((((((*jpg.borrow()).com_data.as_pointer() as Ptr<Value<Vec<u8>>>)
            .offset(0_u64 as isize)
            .upgrade()
            .deref()
            .as_pointer() as Ptr<u8>)
            .offset(1_u64 as isize)
            .read()) as i32)
            == 2)
    );
    assert!(
        ((((((*jpg.borrow()).com_data.as_pointer() as Ptr<Value<Vec<u8>>>)
            .offset(0_u64 as isize)
            .upgrade()
            .deref()
            .as_pointer() as Ptr<u8>)
            .offset(2_u64 as isize)
            .read()) as i32)
            == 3)
    );
    assert!((*(*jpg.borrow()).app_data.borrow()).is_empty());
    let chunks: Value<Vec<Chunk>> = Rc::new(RefCell::new(Vec::new()));
    ({
        let _comps: Ptr<Vec<Chunk>> = (chunks.as_pointer());
        shrink_through_ptr_2(_comps)
    });
    assert!((*chunks.borrow()).is_empty());
    let w: Value<Writer> = Rc::new(RefCell::new(<Writer>::default()));
    (*(*(*w.borrow()).chunk.borrow()).data.borrow_mut()) = 42;
    (*(*w.borrow()).output.borrow_mut()) = (chunks.as_pointer());
    ({
        let _bw: Ptr<Writer> = (w.as_pointer());
        nested_push_move_3(_bw)
    });
    assert!(((*chunks.borrow()).len() as u64 == 1_u64));
    assert!(
        ((*(*(chunks.as_pointer() as Ptr<Chunk>)
            .offset(0_u64 as isize)
            .upgrade()
            .deref())
        .data
        .borrow())
            == 42)
    );
    ({
        let _jpg: Ptr<JPEGData> = (jpg.as_pointer());
        let _cond: bool = false;
        emplace_local_from_field_4(_jpg, _cond)
    });
    assert!(((*(*jpg.borrow()).app_data.borrow()).len() as u64 == 1_u64));
    assert!(
        ((*(((*jpg.borrow()).app_data.as_pointer() as Ptr<Value<Vec<u8>>>)
            .offset(0_u64 as isize)
            .upgrade()
            .deref()
            .as_pointer() as Ptr<Vec<u8>>)
            .upgrade()
            .deref())
        .len() as u64
            == 3_u64)
    );
    assert!(
        ((((((*jpg.borrow()).app_data.as_pointer() as Ptr<Value<Vec<u8>>>)
            .offset(0_u64 as isize)
            .upgrade()
            .deref()
            .as_pointer() as Ptr<u8>)
            .offset(0_u64 as isize)
            .read()) as i32)
            == 1)
    );
    assert!(
        ((((((*jpg.borrow()).app_data.as_pointer() as Ptr<Value<Vec<u8>>>)
            .offset(0_u64 as isize)
            .upgrade()
            .deref()
            .as_pointer() as Ptr<u8>)
            .offset(2_u64 as isize)
            .read()) as i32)
            == 3)
    );
    assert!(((*(*jpg.borrow()).com_data.borrow()).len() as u64 == 1_u64));
    (*(*(*w.borrow()).chunk.borrow()).data.borrow_mut()) = 99;
    (*(*w.borrow()).output.borrow_mut()) = (chunks.as_pointer());
    ({
        let _bw: Ptr<Writer> = (w.as_pointer());
        nested_emplace_move_5(_bw)
    });
    assert!(((*chunks.borrow()).len() as u64 == 2_u64));
    assert!(
        ((*(*(chunks.as_pointer() as Ptr<Chunk>)
            .offset(1_u64 as isize)
            .upgrade()
            .deref())
        .data
        .borrow())
            == 99)
    );
    ({
        let _comps: Ptr<Vec<Chunk>> = (chunks.as_pointer());
        self_ref_push_6(_comps)
    });
    assert!(((*chunks.borrow()).len() as u64 == 3_u64));
    assert!(
        ((*(*(chunks.as_pointer() as Ptr<Chunk>)
            .offset(2_u64 as isize)
            .upgrade()
            .deref())
        .data
        .borrow())
            == 42)
    );
    return 0;
}
