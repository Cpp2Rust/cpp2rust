extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let src: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([1_u32, 2_u32, 3_u32])));
    let mut v1: Vec<u32> = {
        let mut __v = Vec::new();
        let mut __it = (src.as_pointer() as Ptr<u32>);
        while __it != (src.as_pointer() as Ptr<u32>).offset((3) as isize) {
            __v.push(<u32>::try_from(__it.iter_deref().read()).ok().unwrap());
            __it.iter_inc();
        }
        __v
    };
    assert!((v1.len() == 3_usize));
    assert!(((v1[0_usize] == 1_u32) && (v1[1_usize] == 2_u32)) && (v1[2_usize] == 3_u32));
    let mut v2: Vec<u64> = {
        let mut __v = Vec::new();
        let mut __it = (src.as_pointer() as Ptr<u32>);
        while __it != (src.as_pointer() as Ptr<u32>).offset((3) as isize) {
            __v.push(<u64>::try_from(__it.iter_deref().read()).ok().unwrap());
            __it.iter_inc();
        }
        __v
    };
    assert!((v2.len() == 3_usize));
    assert!(((v2[0_usize] == 1_u64) && (v2[1_usize] == 2_u64)) && (v2[2_usize] == 3_u64));
    let mut v3: Vec<i32> = {
        let mut __v = Vec::new();
        let mut __it = (src.as_pointer() as Ptr<u32>);
        while __it != (src.as_pointer() as Ptr<u32>).offset((3) as isize) {
            __v.push(<i32>::try_from(__it.iter_deref().read()).ok().unwrap());
            __it.iter_inc();
        }
        __v
    };
    assert!((v3.len() == 3_usize));
    assert!(((v3[0_usize] == 1) && (v3[1_usize] == 2)) && (v3[2_usize] == 3));
    let src1: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([1_u32, 2_u32, 3_u32])));
    let mut v4: Vec<u32> = {
        let mut __v = Vec::new();
        let mut __it = (src1.as_pointer() as Ptr<u32>);
        while __it != (src1.as_pointer() as Ptr<u32>).to_end() {
            __v.push(<u32>::try_from(__it.iter_deref().read()).ok().unwrap());
            __it.iter_inc();
        }
        __v
    };
    assert!((v4.len() == 3_usize));
    assert!(((v4[0_usize] == 1_u32) && (v4[1_usize] == 2_u32)) && (v4[2_usize] == 3_u32));
    let buf: Value<Box<[u8]>> =
        Rc::new(RefCell::new(Box::new([10_u8, 20_u8, 30_u8, 40_u8, 50_u8])));
    let mut start: Ptr<u8> = (buf.as_pointer() as Ptr<u8>);
    let mut len: usize = 5_usize;
    let mut v5: Vec<u8> = {
        let mut __v = Vec::new();
        let mut __it = (start).clone();
        while __it != start.offset((len) as isize) {
            __v.push(<u8>::try_from(__it.iter_deref().read()).ok().unwrap());
            __it.iter_inc();
        }
        __v
    };
    assert!((v5.len() == 5_usize));
    assert!(((v5[0_usize] as i32) == 10) && ((v5[4_usize] as i32) == 50));
    let m: Value<Map<i32, (Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(Map::new()));
    (m.as_pointer() as Ptr<Map<i32, (Value<i32>, Value<i32>)>>)
        .with_mut(|__m: &mut Map<i32, (Value<i32>, Value<i32>)>| {
            __m.get_or_insert_with(1.clone(), || {
                (
                    Rc::new(RefCell::new(1)),
                    Rc::new(RefCell::new(<i32>::default())),
                )
            })
            .borrow()
            .1
            .as_pointer()
        })
        .write(10);
    (m.as_pointer() as Ptr<Map<i32, (Value<i32>, Value<i32>)>>)
        .with_mut(|__m: &mut Map<i32, (Value<i32>, Value<i32>)>| {
            __m.get_or_insert_with(2.clone(), || {
                (
                    Rc::new(RefCell::new(2)),
                    Rc::new(RefCell::new(<i32>::default())),
                )
            })
            .borrow()
            .1
            .as_pointer()
        })
        .write(20);
    (m.as_pointer() as Ptr<Map<i32, (Value<i32>, Value<i32>)>>)
        .with_mut(|__m: &mut Map<i32, (Value<i32>, Value<i32>)>| {
            __m.get_or_insert_with(3.clone(), || {
                (
                    Rc::new(RefCell::new(3)),
                    Rc::new(RefCell::new(<i32>::default())),
                )
            })
            .borrow()
            .1
            .as_pointer()
        })
        .write(30);
    let v6: Value<Vec<(Value<i32>, Value<i32>)>> = Rc::new(RefCell::new({
        let mut __v = Vec::new();
        let mut __it = (m.as_pointer() as Ptr<Map<i32, (Value<i32>, Value<i32>)>>)
            .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin());
        while __it
            != (m.as_pointer() as Ptr<Map<i32, (Value<i32>, Value<i32>)>>)
                .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end())
        {
            __v.push(
                <(Value<i32>, Value<i32>)>::try_from(__it.iter_deref().read())
                    .ok()
                    .unwrap(),
            );
            __it.iter_inc();
        }
        __v
    }));
    assert!(((*v6.borrow()).len() == 3_usize));
    assert!(
        ((*(*v6.borrow())[0_usize].0.borrow()) == 1)
            && ((*(*v6.borrow())[0_usize].1.borrow()) == 10)
    );
    assert!(
        ((*(*v6.borrow())[2_usize].0.borrow()) == 3)
            && ((*(*v6.borrow())[2_usize].1.borrow()) == 30)
    );
    {
        let start_idx = <RandomAccessIterator<(Value<i32>, Value<i32>)>>::from(
            RandomAccessIterator::Vec((v6.as_pointer() as Ptr<(Value<i32>, Value<i32>)>))
                .iter_add(1_isize),
        )
        .index();
        let mut temp_vec: Vec<(Value<i32>, Value<i32>)> = Vec::new();
        let mut __it = (m.as_pointer() as Ptr<Map<i32, (Value<i32>, Value<i32>)>>)
            .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin());
        while __it
            != (m.as_pointer() as Ptr<Map<i32, (Value<i32>, Value<i32>)>>)
                .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end())
        {
            temp_vec.push(__it.iter_deref().read());
            __it.iter_inc();
        }
        (v6.as_pointer() as Ptr<Vec<(Value<i32>, Value<i32>)>>).with_mut(
            |v: &mut Vec<(Value<i32>, Value<i32>)>| {
                v.splice(start_idx..start_idx, temp_vec);
            },
        );
        RandomAccessIterator::Vec(
            (v6.as_pointer() as Ptr<Vec<(Value<i32>, Value<i32>)>>).decay() + start_idx,
        )
    };
    assert!(((*v6.borrow()).len() == 6_usize));
    assert!(
        (((*(*v6.borrow())[1_usize].0.borrow()) == 1)
            && ((*(*v6.borrow())[3_usize].0.borrow()) == 3))
            && ((*(*v6.borrow())[4_usize].0.borrow()) == 2)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
