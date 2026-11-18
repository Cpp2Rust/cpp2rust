extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type E = u32;
pub const E_A: E = 0;
pub const E_B: E = 1;
pub const E_C: E = 2;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Pair {
    pub x: i32,
    pub y: i32,
}
#[repr(C, align(64))]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Wide {
    pub c: libc::c_char,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Grid {
    pub h: i32,
    pub row: [i32; 3],
    pub cells: [[i32; 3]; 2],
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct Bag {
    pub n: i32,
    pub items: Vec<i32>,
}
impl Bag {
    pub unsafe fn move_from(_a0: *mut Bag) -> Self {
        let mut this = Self {
            n: (*_a0).n,
            items: std::mem::take(&mut (*_a0).items),
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Bag) -> *mut Bag {
        self.n = (*_a0).n;
        self.items = std::mem::take(&mut (*_a0).items);
        return &mut (*(self as *mut Bag));
    }
}
pub unsafe fn adjacent_arrays_0() {
    let mut a: [i32; 4] = [1, 2, 3, 4];
    let mut b: [i32; 4] = [5, 6, 7, 8];
    let mut ia: usize = (a.as_mut_ptr() as usize);
    let mut ib: usize = (b.as_mut_ptr() as usize);
    assert!(((ia) != (ib)));
    assert!(
        ((((a.as_mut_ptr().offset((3) as isize)) as usize).wrapping_sub(ia))
            == ((3_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize))))
    );
    assert!(
        (((&mut a[(2) as usize] as *mut i32) as *mut i32 as usize)
            == ((ia as u64).wrapping_add(
                ((2_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)) as u64)
            ) as usize))
    );
    let mut end: *mut i32 =
        (((ia as u64).wrapping_add((::std::mem::size_of::<[i32; 4]>() as u64))) as *mut i32);
    assert!(((end) == (a.as_mut_ptr().offset((4) as isize))));
    let mut last: *mut i32 = end.offset(-((1) as isize));
    assert!(((*last) == (4)));
    (*end.offset((-2_i32) as isize)) = 30;
    assert!(((a[(2) as usize]) == (30)));
    end.prefix_dec();
    (*end) = 40;
    assert!(((a[(3) as usize]) == (40)));
    let mut pb: *mut i32 = (ib as *mut i32);
    (*pb.offset((3) as isize)) = 80;
    assert!(((b[(3) as usize]) == (80)));
    assert!(
        ((*(((ib as u64).wrapping_add((::std::mem::size_of::<i32>() as u64))) as *mut i32)) == (6))
    );
    let mut pa: *mut [i32; 4] = (&mut a as *mut [i32; 4]);
    assert!(((pa as usize) == (ia)));
    assert!(((*((pa as usize) as *mut i32).offset((1) as isize)) == (2)));
}
pub unsafe fn compare_physical_1() {
    let mut a: [i32; 4] = [1, 2, 3, 4];
    let mut b: [i32; 2] = [5, 6];
    let mut ia: usize = (a.as_mut_ptr() as usize);
    let mut p1: *mut i32 =
        (((ia as u64).wrapping_add((::std::mem::size_of::<i32>() as u64))) as *mut i32);
    let mut p2: *mut i32 = (((ia as u64)
        .wrapping_add(((2_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)) as u64)))
        as *mut i32);
    assert!(((p1) != (p2)));
    assert!(((p1) < (p2)));
    assert!(((p2) > (p1)));
    assert!(((p1) <= (p1)));
    assert!(((p1.offset((1) as isize)) == (p2)));
    assert!(((((p2 as usize - p1 as usize) / ::std::mem::size_of::<i32>()) as i64) == (1_i64)));
    assert!(
        ((p1) == (((ia as u64).wrapping_add((::std::mem::size_of::<i32>() as u64))) as *mut i32))
    );
    assert!(((p1) == (&mut a[(1) as usize] as *mut i32)));
    assert!(((&mut a[(2) as usize] as *mut i32) == (p2)));
    assert!(((p1) != (a.as_mut_ptr())));
    assert!(((p1) > (a.as_mut_ptr())));
    assert!(((p2) < (a.as_mut_ptr().offset((3) as isize))));
    assert!(
        ((a.as_mut_ptr().offset((4) as isize))
            == (((ia as u64).wrapping_add((::std::mem::size_of::<[i32; 4]>() as u64)))
                as *mut i32))
    );
    assert!(
        ((((p2 as usize - a.as_mut_ptr() as usize) / ::std::mem::size_of::<i32>()) as i64)
            == (2_i64))
    );
    assert!(
        ((((a.as_mut_ptr().offset((4) as isize) as usize - p1 as usize)
            / ::std::mem::size_of::<i32>()) as i64)
            == (3_i64))
    );
    let mut pb: *mut i32 = ((b.as_mut_ptr() as usize) as *mut i32);
    assert!(((pb) == (b.as_mut_ptr())));
    assert!(((pb) != (p1)));
    assert!(((pb as *mut ::libc::c_void) == (b.as_mut_ptr() as *mut ::libc::c_void)));
    assert!(((pb as *mut ::libc::c_void) != (p1 as *mut ::libc::c_void)));
    assert!(!((pb).is_null()));
    let mut null: *mut i32 = std::ptr::null_mut();
    assert!(((null as usize) == (0_usize)));
    assert!(((null as usize) as *mut i32).is_null());
}
pub unsafe fn order_physical_2() {
    let mut x: i32 = 1;
    let mut y: i32 = 2;
    let mut px: *mut i32 = (&mut x as *mut i32);
    let mut py: *mut i32 = (&mut y as *mut i32);
    let mut qx: *mut i32 = ((px as usize) as *mut i32);
    let mut qy: *mut i32 = ((py as usize) as *mut i32);
    assert!((((qx) == (px)) && (!((qx) < (px)))) && (!((px) < (qx))));
    assert!(((((px) < (py)) as i32) == (((qx) < (py)) as i32)));
    assert!(((((px) < (py)) as i32) == (((px) < (qy)) as i32)));
    assert!(((((px) < (py)) as i32) == (((qx) < (qy)) as i32)));
    assert!(((((py) < (px)) as i32) == (((qy) < (qx)) as i32)));
    let mut ps: [*mut i32; 4] = [py, qx, px, qy];
    match ({ ps.as_mut_ptr() },) {
        (mut __tmp_0,) => {
            let __n = ps.as_mut_ptr().offset((4) as isize).iter_distance(&__tmp_0);
            let mut __order: Vec<isize> = (0..__n).collect();
            __order.sort_by(|&__x, &__y| (*__tmp_0.iter_at(__x)).cmp(&*__tmp_0.iter_at(__y)));
            let __values: Vec<_> = __order
                .iter()
                .map(|&__i| std::ptr::read(__tmp_0.iter_at(__i)))
                .collect();
            for (__i, __v) in (0..__n).zip(__values) {
                std::ptr::write(__tmp_0.iter_at(__i), __v);
            }
        }
    };
    assert!(
        (((ps[(0) as usize]) == (ps[(1) as usize])) && ((ps[(2) as usize]) == (ps[(3) as usize])))
            && ((ps[(1) as usize]) < (ps[(2) as usize]))
    );
    let mut sentinel: *mut i32 = ((-1_i32 as usize) as *mut i32);
    assert!(((sentinel) != (px)) && (!((sentinel).is_null())));
    let mut kept: *mut ::libc::c_void = (42_i64 as *mut ::libc::c_void);
    assert!(((kept as i64) == (42_i64)));
    assert!(((kept) != (px as *mut ::libc::c_void)) && (!((kept).is_null())));
    assert!(((((px as usize) as *mut ::libc::c_void) as *mut i32) == (px)));
}
pub unsafe fn over_aligned_3() {
    let mut c: libc::c_char = ('c' as libc::c_char);
    assert!((((&mut c as *mut libc::c_char) as usize) != (0_usize)));
    let mut w: [Wide; 3] = [<Wide>::default(); 3];
    let mut iw: usize = (w.as_mut_ptr() as usize);
    assert!((((iw as u64).wrapping_rem((::std::mem::align_of::<Wide>() as u64))) == (0_u64)));
    assert!(
        ((((&mut w[(1) as usize] as *mut Wide) as *mut Wide as usize).wrapping_sub(iw))
            == (::std::mem::size_of::<Wide>()))
    );
    let mut h: *mut Wide = Box::leak(
        (0..2_usize)
            .map(|_| <Wide>::default())
            .collect::<Box<[Wide]>>(),
    )
    .as_mut_ptr();
    assert!(
        ((((h as usize) as u64).wrapping_rem((::std::mem::align_of::<Wide>() as u64))) == (0_u64))
    );
    {
        let __p = h;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<Wide>(),
            )))
        }
    };
}
pub unsafe fn vector_capacity_4() {
    let mut v: Vec<i32> = Vec::new();
    if 4_usize > v.capacity() as usize {
        let len_0 = v.len();
        v.reserve_exact(4_usize - len_0 as usize);
    };
    {
        let __a1 = 1;
        v.push(__a1)
    };
    let mut i0: usize = ((&mut v[(0_usize)] as *mut i32) as *mut i32 as usize);
    {
        let __a1 = 2;
        v.push(__a1)
    };
    {
        let __a1 = 3;
        v.push(__a1)
    };
    assert!(
        (((&mut v[(2_usize)] as *mut i32) as *mut i32 as usize)
            == ((i0 as u64).wrapping_add(
                ((2_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)) as u64)
            ) as usize))
    );
    assert!(
        ((*(((i0 as u64).wrapping_add((::std::mem::size_of::<i32>() as u64))) as *mut i32)) == (2))
    );
    if 100_usize > v.capacity() as usize {
        let len_0 = v.len();
        v.reserve_exact(100_usize - len_0 as usize);
    };
    let mut j0: usize = ((&mut v[(0_usize)] as *mut i32) as *mut i32 as usize);
    assert!(
        ((((&mut v[(2_usize)] as *mut i32) as *mut i32 as usize).wrapping_sub(j0))
            == ((2_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize))))
    );
    assert!(
        ((*(((j0 as u64)
            .wrapping_add(((2_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)) as u64)))
            as *mut i32))
            == (3))
    );
}
pub unsafe fn nested_5() {
    let mut g: Grid = <Grid>::default();
    let mut ig: usize = ((&mut g as *mut Grid) as usize);
    assert!(
        (((&mut g.row[(2) as usize] as *mut i32) as *mut i32 as usize)
            == (((ig as u64).wrapping_add((::std::mem::offset_of!(Grid, row) as u64))).wrapping_add(
                ((2_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)) as u64)
            ) as usize))
    );
    let mut pr: *mut i32 = ((((ig as u64).wrapping_add((::std::mem::offset_of!(Grid, row) as u64)))
        .wrapping_add((::std::mem::size_of::<i32>() as u64)))
        as *mut i32);
    (*pr) = 7;
    assert!(((g.row[(1) as usize]) == (7)));
    assert!(((pr) == (&mut g.row[(1) as usize] as *mut i32)));
    let mut cell: usize = (((ig as u64).wrapping_add((::std::mem::offset_of!(Grid, cells) as u64)))
        .wrapping_add(((4_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)) as u64))
        as usize);
    assert!(
        (((&mut g.cells[(1) as usize][(1) as usize] as *mut i32) as *mut i32 as usize) == (cell))
    );
    (*(cell as *mut i32)) = 8;
    assert!(((g.cells[(1) as usize][(1) as usize]) == (8)));
    let mut m: [[i32; 4]; 3] = [[0; 4], [0; 4], [0; 4]];
    let mut im: usize = (m.as_mut_ptr() as usize);
    assert!(
        (((&mut m[(2) as usize][(1) as usize] as *mut i32) as *mut i32 as usize)
            == ((im as u64).wrapping_add(
                ((9_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)) as u64)
            ) as usize))
    );
    let mut pm: *mut i32 = (((im as u64)
        .wrapping_add(((5_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)) as u64)))
        as *mut i32);
    (*pm) = 42;
    assert!(((m[(1) as usize][(1) as usize]) == (42)));
    assert!(((pm) == (&mut m[(1) as usize][(1) as usize] as *mut i32)));
    assert!(((pm) < (&mut m[(2) as usize][(0) as usize] as *mut i32)));
    let mut b: Bag = <Bag>::default();
    if 4_usize > b.items.capacity() as usize {
        let len_0 = b.items.len();
        b.items.reserve_exact(4_usize - len_0 as usize);
    };
    {
        let __a1 = 1;
        b.items.push(__a1)
    };
    {
        let __a1 = 2;
        b.items.push(__a1)
    };
    let mut ib: usize = ((&mut b.items[(0_usize)] as *mut i32) as *mut i32 as usize);
    assert!(
        (((&mut b.items[(1_usize)] as *mut i32) as *mut i32 as usize)
            == ((ib as u64).wrapping_add((::std::mem::size_of::<i32>() as u64)) as usize))
    );
    {
        let __a1 = 3;
        b.items.push(__a1)
    };
    assert!(
        (((&mut b.items[(2_usize)] as *mut i32) as *mut i32 as usize)
            == ((ib as u64).wrapping_add(
                ((2_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)) as u64)
            ) as usize))
    );
    (*(((ib as u64).wrapping_add((::std::mem::size_of::<i32>() as u64))) as *mut i32)) = 20;
    assert!(((b.items[(1_usize)]) == (20)));
    assert!(((ib as *mut i32) == (&mut b.items[(0_usize)] as *mut i32)));
    assert!((((&mut b as *mut Bag) as usize) != (ib)));
    let mut vv: Vec<Vec<i32>> = vec![vec![5; 3_usize]; 2_usize];
    let mut iv: usize = ((&mut vv[(1_usize)][(0_usize)] as *mut i32) as *mut i32 as usize);
    assert!(
        (((&mut vv[(1_usize)][(2_usize)] as *mut i32) as *mut i32 as usize)
            == ((iv as u64).wrapping_add(
                ((2_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)) as u64)
            ) as usize))
    );
    assert!((((&mut vv[(0_usize)][(0_usize)] as *mut i32) as *mut i32 as usize) != (iv)));
    (*(((iv as u64).wrapping_add((::std::mem::size_of::<i32>() as u64))) as *mut i32)) = 9;
    assert!(((vv[(1_usize)][(1_usize)]) == (9)));
    assert!(
        ((((iv as u64)
            .wrapping_add(((2_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)) as u64)))
            as *mut i32)
            == (&mut vv[(1_usize)][(2_usize)] as *mut i32))
    );
}
pub unsafe fn struct_fields_6() {
    let mut pr: Pair = Pair { x: 1, y: 2 };
    let mut ip: usize = ((&mut pr as *mut Pair) as usize);
    assert!(
        (((&mut pr.y as *mut i32) as usize)
            == ((ip as u64).wrapping_add((::std::mem::offset_of!(Pair, y) as u64)) as usize))
    );
    let mut py: *mut i32 =
        (((ip as u64).wrapping_add((::std::mem::offset_of!(Pair, y) as u64))) as *mut i32);
    assert!(((*py) == (2)));
    (*py) = 20;
    assert!(((pr.y) == (20)));
    let mut pp: *mut Pair = (ip as *mut Pair);
    (*pp).x = 10;
    assert!(((pr.x) == (10)));
    assert!((((*pp).y) == (20)));
    assert!(((pp) == (&mut pr as *mut Pair)));
}
pub unsafe fn heap_7() {
    let mut h: *mut i32 = Box::leak(Box::new([1, 2, 3])).as_mut_ptr();
    let mut ih: usize = (h as usize);
    (*(ih as *mut i32).offset((2) as isize)) = 9;
    assert!(((*h.offset((2) as isize)) == (9)));
    {
        let __p = (ih as *mut i32);
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<i32>(),
            )))
        }
    };
    let mut l: *mut i64 =
        (libcc2rs::malloc_unsafe((2_usize).wrapping_mul((::std::mem::size_of::<i64>() as usize)))
            as *mut i64);
    (*l.offset((1) as isize)) = 7_i64;
    let mut back: *mut i64 = (((l.offset((1) as isize)) as usize) as *mut i64);
    assert!(((*back) == (7_i64)));
    libcc2rs::free_unsafe(
        ((((back as usize) as u64).wrapping_sub((::std::mem::size_of::<i64>() as u64)))
            as *mut ::libc::c_void),
    );
}
pub unsafe fn c_strings_8() {
    let mut s: [libc::c_char; 6] = std::mem::transmute(*b"hello\0");
    let mut p: *mut libc::c_char = ((s.as_mut_ptr() as usize) as *mut libc::c_char);
    assert!(((libc::strlen((p).cast_const())) == (5_usize)));
    assert!(((libc::strcmp((p).cast_const(), c"hello".as_ptr())) == (0)));
    let mut d: [libc::c_char; 6] = [(0 as libc::c_char); 6];
    match (
        { (((d.as_mut_ptr() as usize) as *mut libc::c_char) as *mut ::libc::c_void) },
        { (p as *const ::libc::c_void) },
    ) {
        (mut __tmp_1, mut __tmp_2) => {
            if ::std::mem::size_of::<[libc::c_char; 6]>() != 0 {
                ::std::ptr::copy_nonoverlapping(
                    __tmp_2,
                    __tmp_1,
                    ::std::mem::size_of::<[libc::c_char; 6]>(),
                )
            }
            __tmp_1
        }
    };
    assert!(
        (({
            let sa = core::slice::from_raw_parts(
                (((d.as_mut_ptr() as usize) as *mut libc::c_char) as *const ::libc::c_void)
                    as *const u8,
                ::std::mem::size_of::<[libc::c_char; 6]>(),
            );
            let sb = core::slice::from_raw_parts(
                (p as *const ::libc::c_void) as *const u8,
                ::std::mem::size_of::<[libc::c_char; 6]>(),
            );
            let mut diff = 0_i32;
            for (x, y) in sa.iter().zip(sb.iter()) {
                if x != y {
                    diff = (*x as i32) - (*y as i32);
                    break;
                }
            }
            diff
        }) == (0))
    );
    assert!(((libc::strcmp((d.as_mut_ptr()).cast_const(), c"hello".as_ptr())) == (0)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut size: usize = 1_usize;
    assert!(((size) == (1_usize)));
    let mut ul: u64 = 5_u64;
    let mut s1: usize = (ul as usize);
    assert!(((s1) == (5_usize)));
    ul = (7_usize as u64);
    assert!(((ul) == (7_u64)));
    let mut i: i32 = 2;
    let mut e: E = ((i) as E);
    assert!(((e as i32) == (E_C as i32)));
    assert!(((e as i32) == (2)));
    (unsafe { adjacent_arrays_0() });
    (unsafe { compare_physical_1() });
    (unsafe { order_physical_2() });
    (unsafe { over_aligned_3() });
    (unsafe { vector_capacity_4() });
    (unsafe { nested_5() });
    (unsafe { struct_fields_6() });
    (unsafe { heap_7() });
    (unsafe { c_strings_8() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
