extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type E = u32;
pub const E_A: E = 0;
pub const E_B: E = 1;
pub const E_C: E = 2;
#[derive(Copy, Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Pair {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
#[derive(Copy, Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(64)]
#[byte_align(64)]
pub struct Wide {
    #[offset(0)]
    pub c: i8,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(40)]
pub struct Grid {
    #[offset(0)]
    pub h: i32,
    #[offset(4)]
    #[byte_size(12)]
    pub row: Box<[i32]>,
    #[offset(16)]
    #[byte_size(24)]
    pub cells: Box<[Box<[i32]>]>,
}
impl Default for Grid {
    fn default() -> Self {
        {
            Grid {
                h: 0_i32,
                row: (0..3).map(|_| 0_i32).collect::<Box<[i32]>>(),
                cells: (0..2)
                    .map(|_| (0..3).map(|_| 0_i32).collect::<Box<[i32]>>())
                    .collect::<Box<[Box<[i32]>]>>(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(32)]
pub struct Bag {
    #[offset(0)]
    pub n: i32,
    #[offset(8)]
    #[byte_size(24)]
    pub items: Vec<i32>,
}
impl Bag {
    pub fn move_from(_a0: Ptr<Bag>) -> Self {
        Self {
            n: { (*_a0.upgrade().deref()).n },
            items: field!(_a0, items).with_mut(std::mem::take),
        }
    }
}
pub fn adjacent_arrays_0() {
    let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4])));
    let b: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([5, 6, 7, 8])));
    let mut ia: usize = a.decay().to_int();
    let mut ib: usize = b.decay().to_int();
    assert!((ia != ib));
    assert!(
        (((a.decay().offset((3) as isize)).to_int()).wrapping_sub(ia)
            == (3_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)))
    );
    assert!(
        ((a.decay().offset(2)).to_int()
            == ((ia as u64).wrapping_add(
                ((2_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)) as u64)
            ) as usize))
    );
    let mut end: Ptr<i32> = <Ptr<i32>>::from_int(
        ((ia as u64).wrapping_add((::std::mem::size_of::<[i32; 4]>() as u64))) as usize,
    );
    assert!(({ (end).clone() } == { a.decay().offset((4) as isize) }));
    let mut last: Ptr<i32> = end.offset(-((1) as isize));
    assert!(((last.read()) == 4));
    elem!(end, -2_i32).write(30);
    assert!(({ ((*a.borrow())[(2) as usize] == 30) }));
    end.prefix_dec();
    end.write(40);
    assert!(({ ((*a.borrow())[(3) as usize] == 40) }));
    let mut pb: Ptr<i32> = <Ptr<i32>>::from_int(ib);
    elem!(pb, 3).write(80);
    assert!(({ ((*b.borrow())[(3) as usize] == 80) }));
    assert!(
        ((<Ptr<i32>>::from_int(
            ((ib as u64).wrapping_add((::std::mem::size_of::<i32>() as u64))) as usize
        )
        .read())
            == 6)
    );
    let mut pa: Ptr<Box<[i32]>> = (a.as_pointer());
    assert!(({ pa.to_int() } == { ia }));
    assert!(((elem!((<Ptr<i32>>::from_int(pa.to_int())), 1).read()) == 2));
}
pub fn compare_physical_1() {
    let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4])));
    let b: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([5, 6])));
    let mut ia: usize = a.decay().to_int();
    let mut p1: Ptr<i32> = <Ptr<i32>>::from_int(
        ((ia as u64).wrapping_add((::std::mem::size_of::<i32>() as u64))) as usize,
    );
    let mut p2: Ptr<i32> = <Ptr<i32>>::from_int(
        ((ia as u64)
            .wrapping_add(((2_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)) as u64)))
            as usize,
    );
    assert!(({ (p1).clone() } != { (p2).clone() }));
    assert!(({ (p1).clone() } < { (p2).clone() }));
    assert!(({ (p2).clone() } > { (p1).clone() }));
    assert!((p1 <= p1));
    assert!(({ p1.offset((1) as isize) } == { (p2).clone() }));
    assert!((((p2).clone() - (p1).clone()) as i64 == 1_i64));
    assert!(
        ({ (p1).clone() } == {
            <Ptr<i32>>::from_int(
                ((ia as u64).wrapping_add((::std::mem::size_of::<i32>() as u64))) as usize,
            )
        })
    );
    assert!(({ (p1).clone() } == { (a.decay().offset(1)) }));
    assert!(({ (a.decay().offset(2)) } == { (p2).clone() }));
    assert!(({ (p1).clone() } != { a.decay() }));
    assert!(({ (p1).clone() } > { a.decay() }));
    assert!(({ (p2).clone() } < { a.decay().offset((3) as isize) }));
    assert!(
        (a.decay().offset((4) as isize)
            == <Ptr<i32>>::from_int(
                ((ia as u64).wrapping_add((::std::mem::size_of::<[i32; 4]>() as u64))) as usize
            ))
    );
    assert!((((p2).clone() - a.decay()) as i64 == 2_i64));
    assert!(((a.decay().offset((4) as isize) - (p1).clone()) as i64 == 3_i64));
    let mut pb: Ptr<i32> = <Ptr<i32>>::from_int(b.decay().to_int());
    assert!(({ (pb).clone() } == { b.decay() }));
    assert!(({ (pb).clone() } != { (p1).clone() }));
    assert!(({ (pb).to_any() } == { (b.decay()).to_any() }));
    assert!(({ (pb).to_any() } != { (p1).to_any() }));
    assert!(!((pb).is_null()));
    let mut null: Ptr<i32> = Ptr::<i32>::null();
    assert!((null.to_int() == 0_usize));
    assert!((<Ptr<i32>>::from_int(null.to_int())).is_null());
}
pub fn order_physical_2() {
    let x: Value<i32> = Rc::new(RefCell::new(1));
    let y: Value<i32> = Rc::new(RefCell::new(2));
    let mut px: Ptr<i32> = (x.as_pointer());
    let mut py: Ptr<i32> = (y.as_pointer());
    let mut qx: Ptr<i32> = <Ptr<i32>>::from_int(px.to_int());
    let mut qy: Ptr<i32> = <Ptr<i32>>::from_int(py.to_int());
    assert!(
        (({ (qx).clone() } == { (px).clone() }) && (!({ (qx).clone() } < { (px).clone() })))
            && (!({ (px).clone() } < { (qx).clone() }))
    );
    assert!(
        ({ (({ (px).clone() } < { (py).clone() }) as i32) } == {
            (({ (qx).clone() } < { (py).clone() }) as i32)
        })
    );
    assert!(
        ({ (({ (px).clone() } < { (py).clone() }) as i32) } == {
            (({ (px).clone() } < { (qy).clone() }) as i32)
        })
    );
    assert!(
        ({ (({ (px).clone() } < { (py).clone() }) as i32) } == {
            (({ (qx).clone() } < { (qy).clone() }) as i32)
        })
    );
    assert!(
        ({ (({ (py).clone() } < { (px).clone() }) as i32) } == {
            (({ (qy).clone() } < { (qx).clone() }) as i32)
        })
    );
    let ps: Value<Box<[Ptr<i32>]>> = Rc::new(RefCell::new(Box::new([
        (py).clone(),
        (qx).clone(),
        (px).clone(),
        (qy).clone(),
    ])));
    match ({ ps.decay() },) {
        (mut __tmp_0,) => {
            let __n = ps.decay().offset((4) as isize).iter_distance(&__tmp_0);
            let mut __order: Vec<isize> = (0..__n).collect();
            __order.sort_by(|&__x, &__y| cpp_cmp(&__tmp_0.iter_at(__x), &__tmp_0.iter_at(__y)));
            reorder(__order, |__i| __tmp_0.iter_at(__i));
        }
    };
    assert!(
        ({
            (((*ps.borrow())[(0) as usize] == (*ps.borrow())[(1) as usize])
                && ((*ps.borrow())[(2) as usize] == (*ps.borrow())[(3) as usize]))
                && ((*ps.borrow())[(1) as usize] < (*ps.borrow())[(2) as usize])
        })
    );
    let mut sentinel: Ptr<i32> = <Ptr<i32>>::from_int((-1_i32 as usize));
    assert!(({ (sentinel).clone() } != { (px).clone() }) && (!((sentinel).is_null())));
    let mut kept: AnyPtr = <AnyPtr>::from_int((42_i64) as usize);
    assert!(((kept.to_int() as i64) == 42_i64));
    assert!(({ (kept).clone() } != { (px).to_any() }) && (!((kept).is_null())));
    assert!((<AnyPtr>::from_int(px.to_int()).reinterpret_cast::<i32>() == px));
}
pub fn over_aligned_3() {
    let c: Value<i8> = Rc::new(RefCell::new(('c' as i8)));
    assert!(((c.as_pointer()).to_int() != 0_usize));
    let w: Value<Box<[Wide]>> = Rc::new(RefCell::new(
        (0..3).map(|_| <Wide>::default()).collect::<Box<[Wide]>>(),
    ));
    let mut iw: usize = w.decay().to_int();
    assert!(((iw as u64).wrapping_rem((64usize as u64)) == 0_u64));
    assert!((((w.decay().offset(1)).to_int()).wrapping_sub(iw) == 64usize));
    let mut h: Ptr<Wide> = Ptr::alloc_array(
        (0..2_usize)
            .map(|_| <Wide>::default())
            .collect::<Box<[Wide]>>(),
    );
    assert!(((h.to_int() as u64).wrapping_rem((64usize as u64)) == 0_u64));
    h.delete();
}
pub fn vector_capacity_4() {
    let v: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    if 4_usize > (*v.borrow()).capacity() as usize {
        let len_0 = (*v.borrow()).len();
        (*v.borrow_mut()).reserve_exact(4_usize - len_0 as usize);
    };
    {
        let __a1 = 1;
        (*v.borrow_mut()).push(__a1)
    };
    let mut i0: usize = (v.decay().offset(0_usize)).to_int();
    {
        let __a1 = 2;
        (*v.borrow_mut()).push(__a1)
    };
    {
        let __a1 = 3;
        (*v.borrow_mut()).push(__a1)
    };
    assert!(
        ((v.decay().offset(2_usize)).to_int()
            == ((i0 as u64).wrapping_add(
                ((2_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)) as u64)
            ) as usize))
    );
    assert!(
        ((<Ptr<i32>>::from_int(
            ((i0 as u64).wrapping_add((::std::mem::size_of::<i32>() as u64))) as usize
        )
        .read())
            == 2)
    );
    if 100_usize > (*v.borrow()).capacity() as usize {
        let len_0 = (*v.borrow()).len();
        (*v.borrow_mut()).reserve_exact(100_usize - len_0 as usize);
    };
    let mut j0: usize = (v.decay().offset(0_usize)).to_int();
    assert!(
        (((v.decay().offset(2_usize)).to_int()).wrapping_sub(j0)
            == (2_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)))
    );
    assert!(
        ((<Ptr<i32>>::from_int(
            ((j0 as u64).wrapping_add(
                ((2_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)) as u64)
            )) as usize
        )
        .read())
            == 3)
    );
}
pub fn nested_5() {
    let g: Value<Grid> = Rc::new(RefCell::new(<Grid>::default()));
    let mut ig: usize = (g.as_pointer()).to_int();
    assert!(
        ((array_field_ptr!(g.as_pointer(), row).offset((2) as isize)).to_int()
            == (((ig as u64).wrapping_add((4_usize as u64))).wrapping_add(
                ((2_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)) as u64)
            ) as usize))
    );
    let mut pr: Ptr<i32> = <Ptr<i32>>::from_int(
        (((ig as u64).wrapping_add((4_usize as u64)))
            .wrapping_add((::std::mem::size_of::<i32>() as u64))) as usize,
    );
    pr.write(7);
    assert!(((array_elem!(g.as_pointer(), row, 1).read()) == 7));
    assert!(({ (pr).clone() } == { (array_field_ptr!(g.as_pointer(), row).offset((1) as isize)) }));
    let mut cell: usize = (((ig as u64).wrapping_add((16_usize as u64)))
        .wrapping_add(((4_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)) as u64))
        as usize);
    assert!(
        ((array_field_ptr!(g.as_pointer(), cells)
            .offset(1)
            .decay()
            .offset(1))
        .to_int()
            == cell)
    );
    <Ptr<i32>>::from_int(cell).write(8);
    assert!(({ ((*g.borrow()).cells[(1) as usize][(1) as usize] == 8) }));
    let m: Value<Box<[Box<[i32]>]>> = Rc::new(RefCell::new(Box::new([
        Box::new([0; 4]),
        Box::new([0; 4]),
        Box::new([0; 4]),
    ])));
    let mut im: usize = m.decay().to_int();
    assert!(
        ((m.decay().offset(2).decay().offset(1)).to_int()
            == ((im as u64).wrapping_add(
                ((9_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)) as u64)
            ) as usize))
    );
    let mut pm: Ptr<i32> = <Ptr<i32>>::from_int(
        ((im as u64)
            .wrapping_add(((5_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)) as u64)))
            as usize,
    );
    pm.write(42);
    assert!(({ ((*m.borrow())[(1) as usize][(1) as usize] == 42) }));
    assert!(({ (pm).clone() } == { (m.decay().offset(1).decay().offset(1)) }));
    assert!(({ (pm).clone() } < { (m.decay().offset(2).decay().offset(0)) }));
    let b: Value<Bag> = Rc::new(RefCell::new(<Bag>::default()));
    if 4_usize > (*b.borrow()).items.capacity() as usize {
        let len_0 = (*b.borrow()).items.len();
        (*b.borrow_mut())
            .items
            .reserve_exact(4_usize - len_0 as usize);
    };
    {
        let __a1 = 1;
        (*b.borrow_mut()).items.push(__a1)
    };
    {
        let __a1 = 2;
        (*b.borrow_mut()).items.push(__a1)
    };
    let mut ib: usize = (field_ref!(b.as_pointer(), items).decay().offset(0_usize)).to_int();
    assert!(
        ((field_ref!(b.as_pointer(), items).decay().offset(1_usize)).to_int()
            == ((ib as u64).wrapping_add((::std::mem::size_of::<i32>() as u64)) as usize))
    );
    {
        let __a1 = 3;
        (*b.borrow_mut()).items.push(__a1)
    };
    assert!(
        ((field_ref!(b.as_pointer(), items).decay().offset(2_usize)).to_int()
            == ((ib as u64).wrapping_add(
                ((2_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)) as u64)
            ) as usize))
    );
    <Ptr<i32>>::from_int(
        ((ib as u64).wrapping_add((::std::mem::size_of::<i32>() as u64))) as usize,
    )
    .write(20);
    assert!(((vec_elem!(b.as_pointer(), items, 1_usize).read()) == 20));
    assert!(
        (<Ptr<i32>>::from_int(ib) == (field_ref!(b.as_pointer(), items).decay().offset(0_usize)))
    );
    assert!(((b.as_pointer()).to_int() != ib));
    let vv: Value<Vec<Vec<i32>>> = Rc::new(RefCell::new(vec![vec![5; 3_usize]; 2_usize]));
    let mut iv: usize = (vv.decay().offset(1_usize).decay().offset(0_usize)).to_int();
    assert!(
        ((vv.decay().offset(1_usize).decay().offset(2_usize)).to_int()
            == ((iv as u64).wrapping_add(
                ((2_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)) as u64)
            ) as usize))
    );
    assert!(((vv.decay().offset(0_usize).decay().offset(0_usize)).to_int() != iv));
    <Ptr<i32>>::from_int(
        ((iv as u64).wrapping_add((::std::mem::size_of::<i32>() as u64))) as usize,
    )
    .write(9);
    assert!(((index!(elem!(vv.decay(), 1_usize), 1_usize).read()) == 9));
    assert!(
        (<Ptr<i32>>::from_int(
            ((iv as u64).wrapping_add(
                ((2_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)) as u64)
            )) as usize
        ) == (vv.decay().offset(1_usize).decay().offset(2_usize)))
    );
}
pub fn struct_fields_6() {
    let pr: Value<Pair> = Rc::new(RefCell::new(Pair { x: 1, y: 2 }));
    let mut ip: usize = (pr.as_pointer()).to_int();
    assert!(
        ((field_ptr!(pr.as_pointer(), y)).to_int()
            == ((ip as u64).wrapping_add((4_usize as u64)) as usize))
    );
    let mut py: Ptr<i32> =
        <Ptr<i32>>::from_int(((ip as u64).wrapping_add((4_usize as u64))) as usize);
    assert!(((py.read()) == 2));
    py.write(20);
    assert!(({ ({ (*pr.borrow()).y } == 20) }));
    let mut pp: Ptr<Pair> = <Ptr<Pair>>::from_int(ip);
    field!(pp, x).write(10);
    assert!(({ ({ (*pr.borrow()).x } == 10) }));
    assert!((pp.with(|__s| __s.y) == 20));
    assert!(({ (pp).clone() } == { (pr.as_pointer()) }));
}
pub fn heap_7() {
    let mut h: Ptr<i32> = Ptr::alloc_array(Box::new([1, 2, 3]));
    let mut ih: usize = h.to_int();
    elem!((<Ptr<i32>>::from_int(ih)), 2).write(9);
    assert!(((elem!(h, 2).read()) == 9));
    <Ptr<i32>>::from_int(ih).delete();
    let mut l: Ptr<i64> =
        libcc2rs::malloc_refcount((2_usize).wrapping_mul((::std::mem::size_of::<i64>() as usize)))
            .reinterpret_cast::<i64>();
    elem!(l, 1).write(7_i64);
    let mut back: Ptr<i64> = <Ptr<i64>>::from_int((l.offset((1) as isize)).to_int());
    assert!(((back.read()) == 7_i64));
    libcc2rs::free_refcount(<AnyPtr>::from_int(
        ((back.to_int() as u64).wrapping_sub((::std::mem::size_of::<i64>() as u64))) as usize,
    ));
}
pub fn c_strings_8() {
    let s: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"hello\0")));
    let mut p: Ptr<i8> = <Ptr<i8>>::from_int(s.decay().to_int());
    assert!((p.to_c_string_iterator().count() == 5_usize));
    assert!(
        ({
            let mut __it1 = p.to_c_string_iterator();
            let mut __it2 = Ptr::<i8>::from_string_literal(b"hello").to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as u8 as i32) - (__c2.unwrap_or(0) as u8 as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0)
    );
    let d: Value<Box<[i8]>> = Rc::new(RefCell::new((0..6).map(|_| 0_i8).collect::<Box<[i8]>>()));
    match ({ (<Ptr<i8>>::from_int(d.decay().to_int())).to_any() },) {
        (mut __tmp_1,) => {
            __tmp_1.memcpy(&(p).to_any(), ::std::mem::size_of::<[i8; 6]>());
            (__tmp_1).clone()
        }
    };
    assert!(
        ((<Ptr<i8>>::from_int(d.decay().to_int()))
            .to_any()
            .memcmp(&(p).to_any(), ::std::mem::size_of::<[i8; 6]>())
            == 0)
    );
    assert!(
        ({
            let mut __it1 = d.decay().to_c_string_iterator();
            let mut __it2 = Ptr::<i8>::from_string_literal(b"hello").to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as u8 as i32) - (__c2.unwrap_or(0) as u8 as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0)
    );
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut size: usize = 1_usize;
    assert!((size == 1_usize));
    let mut ul: u64 = 5_u64;
    let mut s1: usize = (ul as usize);
    assert!((s1 == 5_usize));
    ul = (7_usize as u64);
    assert!((ul == 7_u64));
    let mut i: i32 = 2;
    let mut e: E = ((i) as E);
    assert!(((e as i32) == (E_C as i32)));
    assert!(((e as i32) == 2));
    ({ adjacent_arrays_0() });
    ({ compare_physical_1() });
    ({ order_physical_2() });
    ({ over_aligned_3() });
    ({ vector_capacity_4() });
    ({ nested_5() });
    ({ struct_fields_6() });
    ({ heap_7() });
    ({ c_strings_8() });
    return 0;
}
pub trait BagImpl {
    fn move_assign(&self, _a0: Ptr<Bag>) -> Ptr<Bag>;
}
impl BagImpl for Ptr<Bag> {
    fn move_assign(&self, _a0: Ptr<Bag>) -> Ptr<Bag> {
        field!((*self), n).write({ { (*_a0.upgrade().deref()).n } });
        {
            field_ref!((*self), items)
                .as_pointer()
                .write(field!(_a0, items).with_mut(std::mem::take));
            field_ref!((*self), items).as_pointer()
        };
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
