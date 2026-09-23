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
pub struct Chunk {
    pub data: i32,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Writer {
    pub output: *mut Vec<Chunk>,
    pub chunk: Chunk,
}
#[repr(C)]
#[derive(Clone, Default)]
pub struct JPEGData {
    pub com_data: Vec<Vec<u8>>,
    pub app_data: Vec<Vec<u8>>,
}
pub unsafe fn push_param_0(mut dest: *mut Vec<Vec<u8>>) {
    (*dest).push(Vec::new());
}
pub unsafe fn push_local_from_field_1(mut jpg: *mut JPEGData, mut cond: bool) {
    let mut head: [u8; 3] = [1_u8, 2_u8, 3_u8];
    let mut dest: *mut Vec<Vec<u8>> = std::ptr::null_mut();
    if cond {
        dest = (&mut (*jpg).com_data as *mut Vec<Vec<u8>>);
    } else {
        dest = (&mut (*jpg).app_data as *mut Vec<Vec<u8>>);
    }
    (*dest).push(
        core::slice::from_raw_parts(
            head.as_mut_ptr(),
            (head.as_mut_ptr().offset((3) as isize)).offset_from(head.as_mut_ptr()) as usize,
        )
        .iter()
        .map(|x| u8::try_from(x.clone()).ok().unwrap())
        .collect(),
    );
}
pub unsafe fn shrink_through_ptr_2(mut comps: *mut Vec<Chunk>) {
    (*comps).shrink_to_fit();
}
pub unsafe fn nested_push_move_3(mut bw: *mut Writer) {
    (*(*bw).output).push((*bw).chunk);
}
pub unsafe fn emplace_local_from_field_4(mut jpg: *mut JPEGData, mut cond: bool) {
    let mut head: [u8; 3] = [1_u8, 2_u8, 3_u8];
    let mut dest: *mut Vec<Vec<u8>> = std::ptr::null_mut();
    if cond {
        dest = (&mut (*jpg).com_data as *mut Vec<Vec<u8>>);
    } else {
        dest = (&mut (*jpg).app_data as *mut Vec<Vec<u8>>);
    }
    {
        let __init = core::slice::from_raw_parts(
            head.as_mut_ptr(),
            (head.as_mut_ptr().offset((3) as isize)).offset_from(head.as_mut_ptr()) as usize,
        )
        .iter()
        .map(|x| u8::try_from(x.clone()).ok().unwrap())
        .collect();
        (*dest).push(__init)
    };
}
pub unsafe fn nested_emplace_move_5(mut bw: *mut Writer) {
    {
        let __init = (*bw).chunk;
        (*(*bw).output).push(__init)
    };
}
pub unsafe fn self_ref_push_6(mut comps: *mut Vec<Chunk>) {
    {
        let a0_clone = (*((*comps).first_mut().unwrap())).clone();
        (*comps).push(a0_clone)
    };
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Pair {
    pub first: i32,
    pub second: i32,
}
impl Pair {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {
            first: -1_i32,
            second: -1_i32,
        };
        this
    }
    pub unsafe fn new_2(mut a: i32) -> Self {
        let mut this = Self {
            first: a,
            second: 0,
        };
        this
    }
    pub unsafe fn new_3(mut a: i32, mut b: i32) -> Self {
        let mut this = Self {
            first: a,
            second: ((b) * (2)),
        };
        this
    }
}
impl Default for Pair {
    fn default() -> Self {
        unsafe { Pair::new_1() }
    }
}
pub unsafe fn emplace_ctor_args_7(mut pairs: *mut Vec<Pair>) {
    {
        let __init = Pair::new_1();
        (*pairs).push(__init)
    };
    {
        let __init = Pair::new_2({ 3 });
        (*pairs).push(__init)
    };
    {
        let __init = Pair::new_3({ 4 }, { 5 });
        (*pairs).push(__init)
    };
}
pub unsafe fn emplace_deque_8(mut queue: *mut Vec<Pair>) {
    {
        let __init = Pair::new_3({ 6 }, { 7 });
        (*queue).push(__init)
    };
    {
        let __init = Pair::new_1();
        (*queue).push(__init)
    };
}
pub unsafe fn emplace_scalar_9(mut values: *mut Vec<i64>, mut x: i32) {
    {
        let __init = 0_i64;
        (*values).push(__init)
    };
    {
        let __init = (x as i64);
        (*values).push(__init)
    };
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut vecs: Vec<Vec<u8>> = Vec::new();
    (unsafe { push_param_0((&mut vecs as *mut Vec<Vec<u8>>)) });
    assert!(((vecs.len()) == (1_usize)));
    assert!(vecs[(0_usize)].is_empty());
    let mut jpg: JPEGData = <JPEGData>::default();
    (unsafe { push_local_from_field_1((&mut jpg as *mut JPEGData), true) });
    assert!(((jpg.com_data.len()) == (1_usize)));
    assert!(((jpg.com_data[(0_usize)].len()) == (3_usize)));
    assert!(((jpg.com_data[(0_usize)][(0_usize)] as i32) == (1)));
    assert!(((jpg.com_data[(0_usize)][(1_usize)] as i32) == (2)));
    assert!(((jpg.com_data[(0_usize)][(2_usize)] as i32) == (3)));
    assert!(jpg.app_data.is_empty());
    let mut chunks: Vec<Chunk> = Vec::new();
    (unsafe { shrink_through_ptr_2((&mut chunks as *mut Vec<Chunk>)) });
    assert!(chunks.is_empty());
    let mut w: Writer = <Writer>::default();
    w.chunk.data = 42;
    w.output = (&mut chunks as *mut Vec<Chunk>);
    (unsafe { nested_push_move_3((&mut w as *mut Writer)) });
    assert!(((chunks.len()) == (1_usize)));
    assert!(((chunks[(0_usize)].data) == (42)));
    (unsafe { emplace_local_from_field_4((&mut jpg as *mut JPEGData), false) });
    assert!(((jpg.app_data.len()) == (1_usize)));
    assert!(((jpg.app_data[(0_usize)].len()) == (3_usize)));
    assert!(((jpg.app_data[(0_usize)][(0_usize)] as i32) == (1)));
    assert!(((jpg.app_data[(0_usize)][(2_usize)] as i32) == (3)));
    assert!(((jpg.com_data.len()) == (1_usize)));
    w.chunk.data = 99;
    w.output = (&mut chunks as *mut Vec<Chunk>);
    (unsafe { nested_emplace_move_5((&mut w as *mut Writer)) });
    assert!(((chunks.len()) == (2_usize)));
    assert!(((chunks[(1_usize)].data) == (99)));
    (unsafe { self_ref_push_6((&mut chunks as *mut Vec<Chunk>)) });
    assert!(((chunks.len()) == (3_usize)));
    assert!(((chunks[(2_usize)].data) == (42)));
    let mut pairs: Vec<Pair> = Vec::new();
    (unsafe { emplace_ctor_args_7((&mut pairs as *mut Vec<Pair>)) });
    assert!(((pairs.len()) == (3_usize)));
    assert!(((pairs[(0_usize)].first) == (-1_i32)) && ((pairs[(0_usize)].second) == (-1_i32)));
    assert!(((pairs[(1_usize)].first) == (3)) && ((pairs[(1_usize)].second) == (0)));
    assert!(((pairs[(2_usize)].first) == (4)) && ((pairs[(2_usize)].second) == (10)));
    let mut queue: Vec<Pair> = Vec::new();
    (unsafe { emplace_deque_8((&mut queue as *mut Vec<Pair>)) });
    assert!(
        (((*((queue).first_mut().unwrap())).first) == (6))
            && (((*((queue).first_mut().unwrap())).second) == (14))
    );
    assert!(
        (((*(queue.last_mut().unwrap())).first) == (-1_i32))
            && (((*(queue.last_mut().unwrap())).second) == (-1_i32))
    );
    let mut values: Vec<i64> = Vec::new();
    (unsafe { emplace_scalar_9((&mut values as *mut Vec<i64>), 7) });
    assert!(((values.len()) == (2_usize)));
    assert!(((values[(0_usize)]) == (0_i64)));
    assert!(((values[(1_usize)]) == (7_i64)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
