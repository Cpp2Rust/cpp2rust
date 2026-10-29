extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(48)]
pub struct Elem {
    #[offset(0)]
    pub id: i32,
    #[offset(4)]
    #[byte_size(16)]
    pub hist: Box<[i32]>,
    #[offset(24)]
    #[byte_size(24)]
    pub coeffs: Vec<i16>,
}
impl Elem {
    pub fn move_from(_a0: Ptr<Elem>) -> Self {
        Self {
            id: { (*_a0.upgrade().deref()).id },
            hist: Box::new(std::array::from_fn::<_, 4, _>(|__i: usize| {
                (*_a0.upgrade().deref()).hist[(__i) as usize]
            })),
            coeffs: field!(_a0, coeffs).with_mut(std::mem::take),
        }
    }
}
impl Default for Elem {
    fn default() -> Self {
        {
            Elem {
                id: 0_i32,
                hist: (0..4).map(|_| 0_i32).collect::<Box<[i32]>>(),
                coeffs: Default::default(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(88)]
pub struct State {
    #[offset(0)]
    pub count: i32,
    #[offset(4)]
    #[byte_size(24)]
    pub grid: Box<[Box<[i32]>]>,
    #[offset(28)]
    #[byte_size(12)]
    pub arr: Vec<i32>,
    #[offset(40)]
    #[byte_size(24)]
    pub elems: Vec<Elem>,
    #[offset(64)]
    #[byte_size(24)]
    pub rows: Vec<Vec<i32>>,
}
impl State {
    pub fn move_from(_a0: Ptr<State>) -> Self {
        Self {
            count: { (*_a0.upgrade().deref()).count },
            grid: Box::new(std::array::from_fn::<_, 2, _>(|__i: usize| {
                Box::<[_]>::from(std::array::from_fn::<_, 3, _>(|__i2: usize| {
                    (*_a0.upgrade().deref()).grid[(__i) as usize][(__i2) as usize]
                }))
            })),
            arr: field!(_a0, arr).with_mut(std::mem::take),
            elems: field!(_a0, elems).with_mut(std::mem::take),
            rows: field!(_a0, rows).with_mut(std::mem::take),
        }
    }
}
impl Default for State {
    fn default() -> Self {
        {
            State {
                count: 0_i32,
                grid: (0..2)
                    .map(|_| (0..3).map(|_| 0_i32).collect::<Box<[i32]>>())
                    .collect::<Box<[Box<[i32]>]>>(),
                arr: std::array::from_fn::<_, 3_usize, _>(|_| 0_i32).to_vec(),
                elems: Default::default(),
                rows: Default::default(),
            }
        }
    }
}
pub fn sum_0(mut p: Ptr<i32>, mut n: i32) -> i32 {
    let mut s: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while (i < n) {
        s += { (elem!(p, i).read()) };
        i.prefix_inc();
    }
    return s;
}
pub fn fill_1(mut s: Ptr<State>) {
    let mut i: i32 = 0;
    'loop_: while (i < 2) {
        let mut j: i32 = 0;
        'loop_: while (j < 3) {
            index!(index!(field!(s, grid), i), j).write({ ((i * 3) + j) });
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    vec_elem!(s, arr, 1_usize).write(7);
    {
        let __a0 = 3_usize as usize;
        field!(s, elems).with_mut(|__v: &mut Vec<Elem>| __v.resize_with(__a0, || <Elem>::default()))
    };
    let mut i: i32 = 0;
    'loop_: while (i < 3) {
        let __rhs = i;
        field!(vec_elem!(s, elems, (i as usize)), id).write(__rhs);
        let __rhs = (10 * i);
        index!(field!(vec_elem!(s, elems, (i as usize)), hist), i).write(__rhs);
        {
            let __a0 = 4_usize as usize;
            field!(vec_elem!(s, elems, (i as usize)), coeffs)
                .with_mut(|__v: &mut Vec<i16>| __v.resize(__a0, (i as i16)))
        };
        i.prefix_inc();
    }
    {
        let __a1 = Vec::from([1, 2, 3]);
        field!(s, rows).with_mut(|__v: &mut Vec<Vec<i32>>| __v.push(__a1))
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<State> = Rc::new(RefCell::new(<State>::default()));
    ({ fill_1((s.as_pointer())) });
    assert!(
        ((*s.borrow()).grid[(1) as usize][(2) as usize] == 5)
            && (({
                sum_0(
                    (((array_field_ptr!(s.as_pointer(), grid) as Ptr<Box<[i32]>>)
                        .offset(1)
                        .as_pointer()) as Ptr<i32>),
                    3,
                )
            }) == 12)
    );
    assert!(((vec_elem!(s.as_pointer(), arr, 1_usize).read()) == 7));
    let mut e: Ptr<Elem> = (field_ref!(s.as_pointer(), elems).decay().offset(2_usize));
    let mut h: Ptr<i32> = (array_field_ptr!(e, hist) as Ptr<i32>);
    let mut c: Ptr<i16> = (field_ref!(e, coeffs).decay().offset(1_usize));
    let mut id: Ptr<i32> = (field_ptr!(
        field_ref!(s.as_pointer(), elems).decay().offset(1_usize),
        id
    ));
    let mut k: i32 = 0;
    'loop_: while (k < 4) {
        {
            let __rhs = { k };
            elem!(h, k).with_mut(|__v| *__v = *__v + __rhs)
        };
        k.prefix_inc();
    }
    c.write({ (((c.read()) as i32) + 5) as i16 });
    id.write(11);
    (*s.borrow_mut()).count = 3;
    assert!(
        (({
            sum_0(
                (array_field_ptr!(
                    field_ref!(s.as_pointer(), elems).decay().offset(2_usize),
                    hist
                ) as Ptr<i32>),
                4,
            )
        }) == 26)
            && (((index!(
                field!(vec_elem!(s.as_pointer(), elems, 2_usize), coeffs),
                1_usize
            )
            .read()) as i32)
                == 7)
    );
    assert!(
        (vec_elem!(s.as_pointer(), elems, 1_usize).with(|__s| __s.id) == 11)
            && ({ h.offset((3) as isize) } == {
                ((array_field_ptr!(
                    field_ref!(s.as_pointer(), elems).decay().offset(2_usize),
                    hist
                ) as Ptr<i32>)
                    .offset((3) as isize))
            })
    );
    let mut row: Ptr<i32> = (field_ref!(s.as_pointer(), rows)
        .decay()
        .offset(0_usize)
        .decay()
        .offset(0_usize));
    elem!(row, 2).write(9);
    assert!(((index!(vec_elem!(s.as_pointer(), rows, 0_usize), 2_usize).read()) == 9));
    let t: Value<State> = Rc::new(RefCell::new((*s.borrow()).clone()));
    index!(field!(vec_elem!(t.as_pointer(), elems, 2_usize), hist), 0).write(100);
    (*t.borrow_mut()).grid[(0) as usize][(0) as usize] = 100;
    index!(vec_elem!(t.as_pointer(), rows, 0_usize), 0_usize).write(100);
    assert!(
        (((index!(field!(vec_elem!(s.as_pointer(), elems, 2_usize), hist), 0).read()) == 0)
            && ((*s.borrow()).grid[(0) as usize][(0) as usize] == 0))
            && ((index!(vec_elem!(s.as_pointer(), rows, 0_usize), 0_usize).read()) == 1)
    );
    assert!(
        (((index!(
            field!(vec_elem!(t.as_pointer(), elems, 2_usize), coeffs),
            1_usize
        )
        .read()) as i32)
            == 7)
    );
    let mut p: Ptr<State> = (t.as_pointer());
    let __rhs = ({ (*p.upgrade().deref()).grid[(1) as usize][(1) as usize] } + {
        (vec_elem!(p, arr, 1_usize).read())
    });
    index!(field!(vec_elem!(p, elems, 0_usize), hist), 3).write(__rhs);
    assert!(((index!(field!(vec_elem!(t.as_pointer(), elems, 0_usize), hist), 3).read()) == 11));
    {
        let a0_clone = (*vec_elem!(p, elems, 2_usize).upgrade().deref()).clone();
        field!(p, elems).with_mut(|__v: &mut Vec<Elem>| __v.push(a0_clone))
    };
    assert!(
        ((*p.upgrade().deref()).elems.len() == 4_usize)
            && ((index!(field!(vec_elem!(p, elems, 3_usize), hist), 0).read()) == 100)
    );
    let elems: Value<Vec<Elem>> = Rc::new(RefCell::new(
        (0..(2_usize) as usize)
            .map(|_| <Elem>::default())
            .collect::<Vec<_>>(),
    ));
    let mut first: Ptr<Elem> = (elems.decay().offset(0_usize));
    {
        let __a0 = 2_usize as usize;
        (*elems.borrow_mut())[1_usize]
            .coeffs
            .resize_with(__a0, || 0_i16)
    };
    vec_elem!(elems.decay().offset(1_usize), coeffs, 1_usize).write(4_i16);
    let __rhs = (((vec_elem!(elems.decay().offset(1_usize), coeffs, 1_usize).read()) as i32) + 1);
    array_elem!(elems.decay().offset(1_usize), hist, 2).write(__rhs);
    assert!(
        ((array_elem!(elems.decay().offset(1_usize), hist, 2).read()) == 5)
            && ((array_elem!(first, hist, 2).read()) == 0)
    );
    let moved: Value<State> = Rc::new(RefCell::new(State::move_from({ t.as_pointer() })));
    assert!(
        ((*moved.borrow()).grid[(0) as usize][(0) as usize] == 100)
            && ((*moved.borrow()).grid[(1) as usize][(2) as usize] == 5)
    );
    assert!(
        (((*moved.borrow()).elems.len() == 4_usize) && ((*t.borrow()).elems.is_empty()))
            && ((*t.borrow()).rows.is_empty())
    );
    let assigned: Value<State> = Rc::new(RefCell::new(<State>::default()));
    ({ StateImpl::move_assign(&assigned.as_pointer(), moved.as_pointer()) });
    assert!(
        ((*assigned.borrow()).grid[(0) as usize][(0) as usize] == 100)
            && ((*assigned.borrow()).grid[(1) as usize][(1) as usize] == 4)
    );
    assert!(
        (((index!(
            field!(vec_elem!(assigned.as_pointer(), elems, 2_usize), coeffs),
            1_usize
        )
        .read()) as i32)
            == 7)
            && ((*moved.borrow()).elems.is_empty())
    );
    return 0;
}
pub trait ElemImpl {
    fn move_assign(&self, _a0: Ptr<Elem>) -> Ptr<Elem>;
}
impl ElemImpl for Ptr<Elem> {
    fn move_assign(&self, _a0: Ptr<Elem>) -> Ptr<Elem> {
        field!((*self), id).write({ { (*_a0.upgrade().deref()).id } });
        {
            ((array_field_ptr!((*self), hist)) as Ptr<i32>)
                .to_any()
                .memcpy(
                    &((array_field_ptr!(_a0, hist)) as Ptr<i32>).to_any(),
                    16_usize as usize,
                );
            ((array_field_ptr!((*self), hist)) as Ptr<i32>).to_any()
        };
        field_ref!((*self), coeffs)
            .as_pointer()
            .write(field!(_a0, coeffs).with_mut(std::mem::take));
        return (*self).clone();
    }
}
pub trait StateImpl {
    fn move_assign(&self, _a0: Ptr<State>) -> Ptr<State>;
}
impl StateImpl for Ptr<State> {
    fn move_assign(&self, _a0: Ptr<State>) -> Ptr<State> {
        field!((*self), count).write({ { (*_a0.upgrade().deref()).count } });
        {
            ((array_field_ptr!((*self), grid)) as Ptr<Box<[i32]>>)
                .to_any()
                .memcpy(
                    &((array_field_ptr!(_a0, grid)) as Ptr<Box<[i32]>>).to_any(),
                    24_usize as usize,
                );
            ((array_field_ptr!((*self), grid)) as Ptr<Box<[i32]>>).to_any()
        };
        field_ref!((*self), arr)
            .as_pointer()
            .write(field!(_a0, arr).with_mut(std::mem::take));
        field_ref!((*self), elems)
            .as_pointer()
            .write(field!(_a0, elems).with_mut(std::mem::take));
        field_ref!((*self), rows)
            .as_pointer()
            .write(field!(_a0, rows).with_mut(std::mem::take));
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
