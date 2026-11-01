extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct Degrees {
    #[offset(0)]
    pub value: i32,
}
impl Degrees {
    pub fn new(mut v: i32) -> Self {
        Self { value: v }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_into(i64, to_i64)]
#[byte_size(4)]
pub struct Reading {
    #[offset(0)]
    pub raw_: i32,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let src: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([1_u32, 2_u32, 3_u32])));
    let mut v1: Vec<u32> = match ({ (src.as_pointer() as Ptr<u32>).offset((3) as isize) },) {
        (mut __tmp_0,) => {
            let mut __v = Vec::new();
            let mut __it = (src.as_pointer() as Ptr<u32>);
            while __it != __tmp_0 {
                __v.push((__it.iter_deref().read()));
                __it.iter_inc();
            }
            __v
        }
    };
    assert!((v1.len() == 3_usize));
    assert!(((v1[0_usize] == 1_u32) && (v1[1_usize] == 2_u32)) && (v1[2_usize] == 3_u32));
    let mut v2: Vec<u64> = match ({ (src.as_pointer() as Ptr<u32>).offset((3) as isize) },) {
        (mut __tmp_1,) => {
            let mut __v = Vec::new();
            let mut __it = (src.as_pointer() as Ptr<u32>);
            while __it != __tmp_1 {
                __v.push(((__it.iter_deref().read()) as u64));
                __it.iter_inc();
            }
            __v
        }
    };
    assert!((v2.len() == 3_usize));
    assert!(((v2[0_usize] == 1_u64) && (v2[1_usize] == 2_u64)) && (v2[2_usize] == 3_u64));
    let mut v3: Vec<i32> = match ({ (src.as_pointer() as Ptr<u32>).offset((3) as isize) },) {
        (mut __tmp_2,) => {
            let mut __v = Vec::new();
            let mut __it = (src.as_pointer() as Ptr<u32>);
            while __it != __tmp_2 {
                __v.push(((__it.iter_deref().read()) as i32));
                __it.iter_inc();
            }
            __v
        }
    };
    assert!((v3.len() == 3_usize));
    assert!(((v3[0_usize] == 1) && (v3[1_usize] == 2)) && (v3[2_usize] == 3));
    let src1: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([1_u32, 2_u32, 3_u32])));
    let mut v4: Vec<u32> = match ({ (src1.as_pointer() as Ptr<u32>).to_end() },) {
        (mut __tmp_3,) => {
            let mut __v = Vec::new();
            let mut __it = (src1.as_pointer() as Ptr<u32>);
            while __it != __tmp_3 {
                __v.push((__it.iter_deref().read()));
                __it.iter_inc();
            }
            __v
        }
    };
    assert!((v4.len() == 3_usize));
    assert!(((v4[0_usize] == 1_u32) && (v4[1_usize] == 2_u32)) && (v4[2_usize] == 3_u32));
    let buf: Value<Box<[u8]>> =
        Rc::new(RefCell::new(Box::new([10_u8, 20_u8, 30_u8, 40_u8, 50_u8])));
    let mut start: Ptr<u8> = (buf.as_pointer() as Ptr<u8>);
    let mut len: usize = 5_usize;
    let mut v5: Vec<u8> = match ({ start.offset((len) as isize) },) {
        (mut __tmp_4,) => {
            let mut __v = Vec::new();
            let mut __it = (start).clone();
            while __it != __tmp_4 {
                __v.push((__it.iter_deref().read()));
                __it.iter_inc();
            }
            __v
        }
    };
    assert!((v5.len() == 5_usize));
    assert!(((v5[0_usize] as i32) == 10) && ((v5[4_usize] as i32) == 50));
    let m: Value<Map<i32, (Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(Map::new()));
    Map::subscript_move(
        m.as_pointer(),
        Rc::new(RefCell::new(1)).as_pointer(),
        |__k| cpp_move!(__k),
    )
    .write(10);
    Map::subscript_move(
        m.as_pointer(),
        Rc::new(RefCell::new(2)).as_pointer(),
        |__k| cpp_move!(__k),
    )
    .write(20);
    Map::subscript_move(
        m.as_pointer(),
        Rc::new(RefCell::new(3)).as_pointer(),
        |__k| cpp_move!(__k),
    )
    .write(30);
    let v6: Value<Vec<(Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(
        match (
            {
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin())
            },
            {
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end())
            },
        ) {
            (mut __tmp_5, mut __tmp_6) => {
                let mut __v = Vec::new();
                let mut __it = (__tmp_5).clone();
                while __it != __tmp_6 {
                    __v.push(<(Value<i32>, Value<i32>) as CppFrom<(
                        Value<i32>,
                        Value<i32>,
                    )>>::cpp_from(__it.iter_deref()));
                    __it.iter_inc();
                }
                __v
            }
        },
    ));
    assert!(((*v6.borrow()).len() == 3_usize));
    assert!(
        ((*(*v6.borrow())[0_usize].0.borrow()) == 1)
            && ((*(*v6.borrow())[0_usize].1.borrow()) == 10)
    );
    assert!(
        ((*(*v6.borrow())[2_usize].0.borrow()) == 3)
            && ((*(*v6.borrow())[2_usize].1.borrow()) == 30)
    );
    match (
        {
            <RandomAccessIterator<(Value<i32>, Value<i32>)>>::from(
                v6.decay().begin().iter_add(1_isize),
            )
        },
        {
            m.as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin())
        },
        {
            m.as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end())
        },
    ) {
        (mut __tmp_7, mut __tmp_8, mut __tmp_9) => {
            let start_idx = __tmp_7.index();
            let mut temp_vec: Vec<(Value<i32>, Value<i32>)> = Vec::new();
            let mut __it = (__tmp_8).clone();
            while __it != __tmp_9 {
                temp_vec.push(__it.iter_deref().read());
                __it.iter_inc();
            }
            v6.as_pointer()
                .with_mut(|v: &mut Vec<(Value<i32>, Value<i32>)>| {
                    v.splice(start_idx..start_idx, temp_vec);
                });
            (__tmp_7).clone()
        }
    };
    assert!(((*v6.borrow()).len() == 6_usize));
    assert!(
        (((*(*v6.borrow())[1_usize].0.borrow()) == 1)
            && ((*(*v6.borrow())[3_usize].0.borrow()) == 3))
            && ((*(*v6.borrow())[4_usize].0.borrow()) == 2)
    );
    let ints: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([4, 5, 6])));
    let mut v7: Vec<Degrees> = match ({ (ints.as_pointer() as Ptr<i32>).offset((3) as isize) },) {
        (mut __tmp_10,) => {
            let mut __v = Vec::new();
            let mut __it = (ints.as_pointer() as Ptr<i32>);
            while __it != __tmp_10 {
                __v.push(<Degrees as CppFrom<i32>>::cpp_from(__it.iter_deref()));
                __it.iter_inc();
            }
            __v
        }
    };
    assert!((v7[1_usize].value == 5));
    let readings: Value<Box<[Reading]>> = Rc::new(RefCell::new(Box::new([
        Reading { raw_: 4 },
        Reading { raw_: 5 },
    ])));
    let mut v8: Vec<i64> = match ({ (readings.as_pointer() as Ptr<Reading>).offset((2) as isize) },)
    {
        (mut __tmp_11,) => {
            let mut __v = Vec::new();
            let mut __it = (readings.as_pointer() as Ptr<Reading>);
            while __it != __tmp_11 {
                __v.push(<i64 as CppFrom<Reading>>::cpp_from(__it.iter_deref()));
                __it.iter_inc();
            }
            __v
        }
    };
    assert!((v8[0_usize] == 40_i64) && (v8[1_usize] == 50_i64));
    return 0;
}
pub trait ReadingImpl {
    fn to_i64(&self) -> i64;
}
impl ReadingImpl for Ptr<Reading> {
    fn to_i64(&self) -> i64 {
        return (((*self).with(|__s| __s.raw_) as i64) * 10_i64);
    }
}
pub fn __cpp2rust_init_globals() {}
