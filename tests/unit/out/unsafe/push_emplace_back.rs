extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Chunk {
    pub data: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Writer {
    pub output: *mut Vec<Chunk>,
    pub chunk: Chunk,
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct JPEGData {
    pub com_data: Vec<Vec<u8>>,
    pub app_data: Vec<Vec<u8>>,
}
impl JPEGData {
    pub unsafe fn move_from(_a0: *mut JPEGData) -> Self {
        let mut this = Self {
            com_data: std::mem::take(&mut (*_a0).com_data),
            app_data: std::mem::take(&mut (*_a0).app_data),
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut JPEGData) -> *mut JPEGData {
        {
            self.com_data = std::mem::take(&mut (*_a0).com_data);
        };
        {
            self.app_data = std::mem::take(&mut (*_a0).app_data);
        };
        return &mut (*(self as *mut JPEGData));
    }
}
pub unsafe fn push_param_0(mut dest: *mut Vec<Vec<u8>>) {
    {
        let __a1 = Vec::new();
        (*dest).push(__a1)
    };
}
pub unsafe fn push_local_from_field_1(mut jpg: *mut JPEGData, mut cond: bool) {
    let mut head: [u8; 3] = [1_u8, 2_u8, 3_u8];
    let mut dest: *mut Vec<Vec<u8>> = std::ptr::null_mut();
    if cond {
        dest = (&mut (*jpg).com_data as *mut Vec<Vec<u8>>);
    } else {
        dest = (&mut (*jpg).app_data as *mut Vec<Vec<u8>>);
    }
    {
        let __a1 = {
            let mut __tmp_0 = { head.as_mut_ptr().offset((3) as isize) };
            let mut __v = Vec::new();
            let mut __it = head.as_mut_ptr();
            while __it != __tmp_0 {
                __v.push((*__it.iter_deref()));
                __it.iter_inc();
            }
            __v
        };
        (*dest).push(__a1)
    };
}
pub unsafe fn shrink_through_ptr_2(mut comps: *mut Vec<Chunk>) {
    (*comps).shrink_to_fit();
}
pub unsafe fn nested_push_move_3(mut bw: *mut Writer) {
    {
        let __a1 = (*bw).chunk;
        (*(*bw).output).push(__a1)
    };
}
pub unsafe fn emplace_local_from_field_4(mut jpg: *mut JPEGData, mut cond: bool) {
    let mut head: [u8; 3] = [1_u8, 2_u8, 3_u8];
    let mut dest: *mut Vec<Vec<u8>> = std::ptr::null_mut();
    if cond {
        dest = (&mut (*jpg).com_data as *mut Vec<Vec<u8>>);
    } else {
        dest = (&mut (*jpg).app_data as *mut Vec<Vec<u8>>);
    }
    (*dest).push_mut({
        let mut __tmp_1 = { head.as_mut_ptr().offset((3) as isize) };
        let mut __v = Vec::new();
        let mut __it = head.as_mut_ptr();
        while __it != __tmp_1 {
            __v.push((*__it.iter_deref()));
            __it.iter_inc();
        }
        __v
    });
}
pub unsafe fn nested_emplace_move_5(mut bw: *mut Writer) {
    (*(*bw).output).push_mut((*bw).chunk);
}
pub unsafe fn self_ref_push_6(mut comps: *mut Vec<Chunk>) {
    {
        let a0_clone = (&mut (*comps))[0].clone();
        (*comps).push(a0_clone)
    };
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe)]
#[cpp_from(i32, new_1, value)]
pub struct Pair {
    pub first: i32,
    pub second: i32,
}
impl Pair {
    pub unsafe fn new_2() -> Self {
        let mut this = Self {
            first: -1_i32,
            second: -1_i32,
        };
        this
    }
    pub unsafe fn new_1(mut a: i32) -> Self {
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
        unsafe { Pair::new_2() }
    }
}
pub unsafe fn emplace_ctor_args_7(mut pairs: *mut Vec<Pair>) {
    (*pairs).push_mut(Pair::new_2());
    (*pairs).push_mut(Pair::new_1({ 3 }));
    (*pairs).push_mut(Pair::new_3({ 4 }, { 5 }));
}
pub unsafe fn emplace_deque_8(mut queue: *mut Vec<Pair>) {
    (*queue).push_mut(Pair::new_3({ 6 }, { 7 }));
    (*queue).push_mut(Pair::new_2());
}
pub unsafe fn emplace_scalar_9(mut values: *mut Vec<i64>, mut x: i32) {
    (*values).push_mut(0_i64);
    (*values).push_mut((x as i64));
}
pub unsafe fn emplace_ref_through_ptr_10(mut pairs: *mut Vec<Pair>) {
    let p: *mut Pair = (*pairs).push_mut(Pair::new_3({ 1 }, { 2 }));
    (*p).second = 3;
    (*(*pairs).push_mut(Pair::new_1({ 4 }))).second += 1;
}
pub unsafe fn emplace_addr_11(values: *mut Vec<i32>) -> *mut i32 {
    return ((*values).push_mut(9));
}
pub unsafe fn emplace_self_reads_12(mut values: *mut Vec<i64>, mut pairs: *mut Vec<Pair>) {
    (*values).push_mut((((*(values).cast_const()).len()) as i64));
    (*pairs).push_mut((&mut (*pairs))[(0_usize)]);
    (*pairs).push_mut(Pair::new_3(
        {
            (*{
                let __n = (*pairs).len() - 1;
                &mut (&mut (*pairs))[__n]
            })
            .first
        },
        { (((*(pairs).cast_const()).len()) as i32) },
    ));
}
pub unsafe fn emplace_ref_self_read_13(values: *mut Vec<i64>) -> *mut i64 {
    return (*values).push_mut((&mut (*values))[0]);
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct Bytes {
    pub v: Vec<Vec<u8>>,
    pub b: [u8; 3],
}
impl Bytes {
    pub unsafe fn move_from(_a0: *mut Bytes) -> Self {
        let mut this = Self {
            v: std::mem::take(&mut (*_a0).v),
            b: std::array::from_fn::<_, 3, _>(|__i: usize| (*_a0).b[(__i)]),
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Bytes) -> *mut Bytes {
        {
            self.v = std::mem::take(&mut (*_a0).v);
        };
        {
            let mut __tmp_2 = { ((&mut self.b as *mut [u8; 3]) as *mut ::libc::c_void) };
            let mut __tmp_3 = { ((&mut (*_a0).b as *mut [u8; 3]) as *const ::libc::c_void) };
            if 3_usize != 0 {
                ::std::ptr::copy_nonoverlapping(__tmp_3, __tmp_2, 3_usize)
            }
            __tmp_2
        };
        return &mut (*(self as *mut Bytes));
    }
}
pub unsafe fn emplace_from_element_14(mut v: *mut Vec<Vec<u8>>) {
    let mut p: *mut u8 = (&mut (*v))[(0_usize)].as_mut_ptr();
    (*v).push_mut({
        let mut __tmp_4 = { p.offset((3) as isize) };
        let mut __v = Vec::new();
        let mut __it = p;
        while __it != __tmp_4 {
            __v.push((*__it.iter_deref()));
            __it.iter_inc();
        }
        __v
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Plain {
    pub a: i32,
    pub b: [f64; 2],
}
pub unsafe fn emplace_plain_15(mut v: *mut Vec<Plain>) {
    let mut p: Plain = Plain {
        a: 1,
        b: [2_f64, 3_f64],
    };
    (*v).push_mut(p);
}
pub static mut all_counted_16: std::cell::LazyCell<*mut Vec<Counted>> =
    std::cell::LazyCell::new(|| unsafe { std::ptr::null_mut() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Counted {
    pub seen: usize,
}
impl Counted {
    pub unsafe fn new(mut _a0: i32) -> Self {
        let mut this = Self {
            seen: (*(*std::cell::LazyCell::force_mut(&mut *&raw mut all_counted_16)).cast_const())
                .len(),
        };
        this
    }
}
impl Counted {}
pub unsafe fn emplace_counted_17(mut v: *mut Vec<Counted>) {
    (*v).push_mut(Counted::new({ 1 }));
}
pub unsafe fn emplace_from_sibling_18(mut s: *mut Bytes) {
    let mut p: *mut u8 = (*s).b.as_mut_ptr();
    let mut v: *mut Vec<Vec<u8>> = (&mut (*s).v as *mut Vec<Vec<u8>>);
    (*v).push_mut({
        let mut __tmp_5 = { p.offset((3) as isize) };
        let mut __v = Vec::new();
        let mut __it = p;
        while __it != __tmp_5 {
            __v.push((*__it.iter_deref()));
            __it.iter_inc();
        }
        __v
    });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
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
    assert!(((queue[0].first) == (6)) && ((queue[0].second) == (14)));
    assert!(
        (((*{
            let __n = queue.len() - 1;
            &mut queue[__n]
        })
        .first)
            == (-1_i32))
            && (((*{
                let __n = queue.len() - 1;
                &mut queue[__n]
            })
            .second)
                == (-1_i32))
    );
    let mut values: Vec<i64> = Vec::new();
    (unsafe { emplace_scalar_9((&mut values as *mut Vec<i64>), 7) });
    assert!(((values.len()) == (2_usize)));
    assert!(((values[(0_usize)]) == (0_i64)));
    assert!(((values[(1_usize)]) == (7_i64)));
    let mut ints: Vec<i32> = Vec::new();
    ints.push_mut(0);
    let r: *mut i32 = ints.push_mut(1);
    (*r) = 5;
    let mut q: *mut i32 = (unsafe { emplace_addr_11(&mut ints) });
    (*q) += 1;
    assert!(
        (((ints.len()) == (3_usize)) && ((ints[(1_usize)]) == (5))) && ((ints[(2_usize)]) == (10))
    );
    let mut ints_queue: Vec<i32> = Vec::new();
    (*ints_queue.push_mut(2)) += 3;
    assert!(((ints_queue[0]) == (5)));
    let mut more_pairs: Vec<Pair> = Vec::new();
    (unsafe { emplace_ref_through_ptr_10((&mut more_pairs as *mut Vec<Pair>)) });
    assert!(((more_pairs[(0_usize)].first) == (1)) && ((more_pairs[(0_usize)].second) == (3)));
    assert!(((more_pairs[(1_usize)].first) == (4)) && ((more_pairs[(1_usize)].second) == (1)));
    let mut self_values: Vec<i64> = Vec::from([7_i64]);
    let mut self_pairs: Vec<Pair> = Vec::new();
    self_pairs.push_mut(Pair::new_3({ 1 }, { 2 }));
    (unsafe {
        emplace_self_reads_12(
            (&mut self_values as *mut Vec<i64>),
            (&mut self_pairs as *mut Vec<Pair>),
        )
    });
    (*(unsafe { emplace_ref_self_read_13(&mut self_values) })) += 1_i64;
    assert!(
        (((self_values.len()) == (3_usize)) && ((self_values[(1_usize)]) == (1_i64)))
            && ((self_values[(2_usize)]) == (8_i64))
    );
    assert!(((self_pairs[(1_usize)].first) == (1)) && ((self_pairs[(1_usize)].second) == (4)));
    assert!(((self_pairs[(2_usize)].first) == (1)) && ((self_pairs[(2_usize)].second) == (4)));
    let mut rows: Vec<Vec<u8>> = Vec::from([Vec::from([1_u8, 2_u8, 3_u8])]);
    (unsafe { emplace_from_element_14((&mut rows as *mut Vec<Vec<u8>>)) });
    assert!(((rows.len()) == (2_usize)) && ((rows[(1_usize)][(2_usize)] as i32) == (3)));
    let mut bytes: Bytes = Bytes {
        v: Vec::new(),
        b: [4_u8, 5_u8, 6_u8],
    };
    (unsafe { emplace_from_sibling_18((&mut bytes as *mut Bytes)) });
    assert!(((bytes.v.len()) == (1_usize)) && ((bytes.v[(0_usize)][(2_usize)] as i32) == (6)));
    let mut plains: Vec<Plain> = Vec::new();
    (unsafe { emplace_plain_15((&mut plains as *mut Vec<Plain>)) });
    assert!(((plains[(0_usize)].a) == (1)) && ((plains[(0_usize)].b[(1) as usize]) == (3_f64)));
    let mut counted: Vec<Counted> = Vec::new();
    (*std::cell::LazyCell::force_mut(&mut *&raw mut all_counted_16)) =
        (&mut counted as *mut Vec<Counted>);
    (unsafe { emplace_counted_17((&mut counted as *mut Vec<Counted>)) });
    (unsafe { emplace_counted_17((&mut counted as *mut Vec<Counted>)) });
    assert!(((counted[(0_usize)].seen) == (0_usize)) && ((counted[(1_usize)].seen) == (1_usize)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const all_counted_16);
}
