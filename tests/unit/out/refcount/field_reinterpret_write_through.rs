extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct In {
    #[offset(0)]
    pub a: i16,
    #[offset(4)]
    pub b: i32,
}
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(40)]
pub struct S {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    #[byte_size(8)]
    pub in_: In,
    #[offset(12)]
    #[byte_size(4)]
    pub bytes: Value<Box<[u8]>>,
    #[offset(16)]
    #[byte_size(12)]
    pub arr: Value<Box<[i32]>>,
    #[offset(32)]
    pub tail: i64,
}
impl Default for S {
    fn default() -> Self {
        S {
            x: 0_i32,
            in_: <In>::default(),
            bytes: Rc::new(RefCell::new((0..4).map(|_| 0_u8).collect::<Box<[u8]>>())),
            arr: Rc::new(RefCell::new((0..3).map(|_| 0_i32).collect::<Box<[i32]>>())),
            tail: 0_i64,
        }
    }
}
pub fn set_bytes_0(mut p: Ptr<u8>, mut n: usize, mut v: u8) {
    let mut i: usize = 0_usize;
    'loop_: while (((i < n) as i32) != 0) {
        elem!(p, i).write({ v });
        i.postfix_inc();
    }
}
pub fn all_bytes_1(mut p: Ptr<u8>, mut n: usize, mut v: u8) -> i32 {
    let mut i: usize = 0_usize;
    'loop_: while (((i < n) as i32) != 0) {
        if ((({ ((elem!(p, i).read()) as i32) } != { (v as i32) }) as i32) != 0) {
            return 0;
        }
        i.postfix_inc();
    }
    return 1;
}
pub fn check_final_2(mut s: Ptr<S>) {
    assert!(
        (((s.with(|__s| __s.x)
            == (((72340172838076673_u64 as u64).wrapping_mul((((19) as u8) as u64))) as i32))
            as i32)
            != 0)
    );
    assert!(
        ((((s.with(|__s| __s.in_.a) as i32)
            == ((((72340172838076673_u64 as u64).wrapping_mul((((35) as u8) as u64))) as i16)
                as i32)) as i32)
            != 0)
    );
    assert!(
        (((s.with(|__s| __s.in_.b)
            == (((72340172838076673_u64 as u64).wrapping_mul((((51) as u8) as u64))) as i32))
            as i32)
            != 0)
    );
    assert!(
        ((((((((elem!((array_field_ptr!(s, bytes) as Ptr::<u8>), 0).read()) as i32) == 67) as i32)
            != 0)
            && (((((elem!((array_field_ptr!(s, bytes) as Ptr::<u8>), 1).read()) as i32) == 83)
                as i32)
                != 0)) as i32)
            != 0)
    );
    assert!(
        ((((((((elem!((array_field_ptr!(s, bytes) as Ptr::<u8>), 2).read()) as i32) == 99) as i32)
            != 0)
            && (((((elem!((array_field_ptr!(s, bytes) as Ptr::<u8>), 3).read()) as i32) == 115)
                as i32)
                != 0)) as i32)
            != 0)
    );
    assert!(
        (((((((elem!((array_field_ptr!(s, arr) as Ptr::<i32>), 0).read())
            == (((72340172838076673_u64 as u64).wrapping_mul((((131) as u8) as u64))) as i32))
            as i32)
            != 0)
            && ((((elem!((array_field_ptr!(s, arr) as Ptr::<i32>), 1).read())
                == (((72340172838076673_u64 as u64).wrapping_mul((((147) as u8) as u64))) as i32))
                as i32)
                != 0)) as i32)
            != 0)
    );
    assert!(
        ((((elem!((array_field_ptr!(s, arr) as Ptr::<i32>), 2).read())
            == (((72340172838076673_u64 as u64).wrapping_mul((((11) as u8) as u64))) as i32))
            as i32)
            != 0)
    );
    assert!(
        (((s.with(|__s| __s.tail)
            == (((72340172838076673_u64 as u64).wrapping_mul((((27) as u8) as u64))) as i64))
            as i32)
            != 0)
    );
}
pub fn check_struct_3(mut s: Ptr<S>) {
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut fb: Ptr<u8> = (field_ptr!((s), x)).reinterpret_cast::<u8>();
        let mut sb: Ptr<u8> = (s).reinterpret_cast::<u8>().offset((0_usize) as isize);
        ({
            let _p: Ptr<u8> = (fb).clone();
            set_bytes_0(_p, ::std::mem::size_of::<i32>(), ((16) as u8))
        });
        assert!(
            ((((s).with(|__s| __s.x)
                == (((72340172838076673_u64 as u64).wrapping_mul((((16) as u8) as u64))) as i32))
                as i32)
                != 0)
        );
        assert!(
            (({
                let _p: Ptr<u8> = (sb).clone();
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), ((16) as u8))
            }) != 0)
        );
        field!((s), x).write(
            (((72340172838076673_u64 as u64).wrapping_mul(((((16) + 1) as u8) as u64))) as i32),
        );
        assert!(
            (({
                let _p: Ptr<u8> = (fb).clone();
                let _v: u8 = (((16) + 1) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), _v)
            }) != 0)
        );
        assert!(
            (({
                let _p: Ptr<u8> = (sb).clone();
                let _v: u8 = (((16) + 1) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), _v)
            }) != 0)
        );
        ({
            let _p: Ptr<u8> = (sb).clone();
            let _v: u8 = (((16) + 2) as u8);
            set_bytes_0(_p, ::std::mem::size_of::<i32>(), _v)
        });
        assert!(
            ((((s).with(|__s| __s.x)
                == (((72340172838076673_u64 as u64).wrapping_mul(((((16) + 2) as u8) as u64)))
                    as i32)) as i32)
                != 0)
        );
        assert!(
            (({
                let _p: Ptr<u8> = (fb).clone();
                let _v: u8 = (((16) + 2) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), _v)
            }) != 0)
        );
        field!((s), x).write(
            (((72340172838076673_u64 as u64).wrapping_mul(((((16) + 3) as u8) as u64))) as i32),
        );
        assert!(
            (({
                let _p: Ptr<u8> = (sb).clone();
                let _v: u8 = (((16) + 3) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), _v)
            }) != 0)
        );
        assert!(
            (({
                let _p: Ptr<u8> = (fb).clone();
                let _v: u8 = (((16) + 3) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), _v)
            }) != 0)
        );
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut fb: Ptr<u8> = (field_ptr!(field_ptr!((s), in_), a)).reinterpret_cast::<u8>();
        let mut sb: Ptr<u8> = (s)
            .reinterpret_cast::<u8>()
            .offset(((4_usize as usize).wrapping_add((0_usize as usize))) as isize);
        ({
            let _p: Ptr<u8> = (fb).clone();
            set_bytes_0(_p, ::std::mem::size_of::<i16>(), ((32) as u8))
        });
        assert!(
            (((((s).with(|__s| __s.in_.a) as i32)
                == ((((72340172838076673_u64 as u64).wrapping_mul((((32) as u8) as u64))) as i16)
                    as i32)) as i32)
                != 0)
        );
        assert!(
            (({
                let _p: Ptr<u8> = (sb).clone();
                all_bytes_1(_p, ::std::mem::size_of::<i16>(), ((32) as u8))
            }) != 0)
        );
        field!(field!((s), in_), a).write(
            (((72340172838076673_u64 as u64).wrapping_mul(((((32) + 1) as u8) as u64))) as i16),
        );
        assert!(
            (({
                let _p: Ptr<u8> = (fb).clone();
                let _v: u8 = (((32) + 1) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i16>(), _v)
            }) != 0)
        );
        assert!(
            (({
                let _p: Ptr<u8> = (sb).clone();
                let _v: u8 = (((32) + 1) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i16>(), _v)
            }) != 0)
        );
        ({
            let _p: Ptr<u8> = (sb).clone();
            let _v: u8 = (((32) + 2) as u8);
            set_bytes_0(_p, ::std::mem::size_of::<i16>(), _v)
        });
        assert!(
            (((((s).with(|__s| __s.in_.a) as i32)
                == ((((72340172838076673_u64 as u64).wrapping_mul(((((32) + 2) as u8) as u64)))
                    as i16) as i32)) as i32)
                != 0)
        );
        assert!(
            (({
                let _p: Ptr<u8> = (fb).clone();
                let _v: u8 = (((32) + 2) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i16>(), _v)
            }) != 0)
        );
        field!(field!((s), in_), a).write(
            (((72340172838076673_u64 as u64).wrapping_mul(((((32) + 3) as u8) as u64))) as i16),
        );
        assert!(
            (({
                let _p: Ptr<u8> = (sb).clone();
                let _v: u8 = (((32) + 3) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i16>(), _v)
            }) != 0)
        );
        assert!(
            (({
                let _p: Ptr<u8> = (fb).clone();
                let _v: u8 = (((32) + 3) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i16>(), _v)
            }) != 0)
        );
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut fb: Ptr<u8> = (field_ptr!(field_ptr!((s), in_), b)).reinterpret_cast::<u8>();
        let mut sb: Ptr<u8> = (s)
            .reinterpret_cast::<u8>()
            .offset(((4_usize as usize).wrapping_add((4_usize as usize))) as isize);
        ({
            let _p: Ptr<u8> = (fb).clone();
            set_bytes_0(_p, ::std::mem::size_of::<i32>(), ((48) as u8))
        });
        assert!(
            ((((s).with(|__s| __s.in_.b)
                == (((72340172838076673_u64 as u64).wrapping_mul((((48) as u8) as u64))) as i32))
                as i32)
                != 0)
        );
        assert!(
            (({
                let _p: Ptr<u8> = (sb).clone();
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), ((48) as u8))
            }) != 0)
        );
        field!(field!((s), in_), b).write(
            (((72340172838076673_u64 as u64).wrapping_mul(((((48) + 1) as u8) as u64))) as i32),
        );
        assert!(
            (({
                let _p: Ptr<u8> = (fb).clone();
                let _v: u8 = (((48) + 1) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), _v)
            }) != 0)
        );
        assert!(
            (({
                let _p: Ptr<u8> = (sb).clone();
                let _v: u8 = (((48) + 1) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), _v)
            }) != 0)
        );
        ({
            let _p: Ptr<u8> = (sb).clone();
            let _v: u8 = (((48) + 2) as u8);
            set_bytes_0(_p, ::std::mem::size_of::<i32>(), _v)
        });
        assert!(
            ((((s).with(|__s| __s.in_.b)
                == (((72340172838076673_u64 as u64).wrapping_mul(((((48) + 2) as u8) as u64)))
                    as i32)) as i32)
                != 0)
        );
        assert!(
            (({
                let _p: Ptr<u8> = (fb).clone();
                let _v: u8 = (((48) + 2) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), _v)
            }) != 0)
        );
        field!(field!((s), in_), b).write(
            (((72340172838076673_u64 as u64).wrapping_mul(((((48) + 3) as u8) as u64))) as i32),
        );
        assert!(
            (({
                let _p: Ptr<u8> = (sb).clone();
                let _v: u8 = (((48) + 3) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), _v)
            }) != 0)
        );
        assert!(
            (({
                let _p: Ptr<u8> = (fb).clone();
                let _v: u8 = (((48) + 3) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), _v)
            }) != 0)
        );
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut fb: Ptr<u8> = ((array_field_ptr!((s), bytes) as Ptr<u8>).offset((0) as isize))
            .reinterpret_cast::<u8>();
        let mut sb: Ptr<u8> = (s)
            .reinterpret_cast::<u8>()
            .offset(((12_usize as usize).wrapping_add(0_usize)) as isize);
        ({
            let _p: Ptr<u8> = (fb).clone();
            set_bytes_0(_p, ::std::mem::size_of::<u8>(), ((64) as u8))
        });
        assert!(
            (((((elem!((array_field_ptr!((s), bytes) as Ptr::<u8>), 0).read()) as i32)
                == ((((72340172838076673_u64 as u64).wrapping_mul((((64) as u8) as u64))) as u8)
                    as i32)) as i32)
                != 0)
        );
        assert!(
            (({
                let _p: Ptr<u8> = (sb).clone();
                all_bytes_1(_p, ::std::mem::size_of::<u8>(), ((64) as u8))
            }) != 0)
        );
        elem!((array_field_ptr!((s), bytes) as Ptr::<u8>), 0).write(
            (((72340172838076673_u64 as u64).wrapping_mul(((((64) + 1) as u8) as u64))) as u8),
        );
        assert!(
            (({
                let _p: Ptr<u8> = (fb).clone();
                let _v: u8 = (((64) + 1) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<u8>(), _v)
            }) != 0)
        );
        assert!(
            (({
                let _p: Ptr<u8> = (sb).clone();
                let _v: u8 = (((64) + 1) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<u8>(), _v)
            }) != 0)
        );
        ({
            let _p: Ptr<u8> = (sb).clone();
            let _v: u8 = (((64) + 2) as u8);
            set_bytes_0(_p, ::std::mem::size_of::<u8>(), _v)
        });
        assert!(
            (((((elem!((array_field_ptr!((s), bytes) as Ptr::<u8>), 0).read()) as i32)
                == ((((72340172838076673_u64 as u64).wrapping_mul(((((64) + 2) as u8) as u64)))
                    as u8) as i32)) as i32)
                != 0)
        );
        assert!(
            (({
                let _p: Ptr<u8> = (fb).clone();
                let _v: u8 = (((64) + 2) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<u8>(), _v)
            }) != 0)
        );
        elem!((array_field_ptr!((s), bytes) as Ptr::<u8>), 0).write(
            (((72340172838076673_u64 as u64).wrapping_mul(((((64) + 3) as u8) as u64))) as u8),
        );
        assert!(
            (({
                let _p: Ptr<u8> = (sb).clone();
                let _v: u8 = (((64) + 3) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<u8>(), _v)
            }) != 0)
        );
        assert!(
            (({
                let _p: Ptr<u8> = (fb).clone();
                let _v: u8 = (((64) + 3) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<u8>(), _v)
            }) != 0)
        );
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut fb: Ptr<u8> = ((array_field_ptr!((s), bytes) as Ptr<u8>).offset((1) as isize))
            .reinterpret_cast::<u8>();
        let mut sb: Ptr<u8> = (s)
            .reinterpret_cast::<u8>()
            .offset(((12_usize as usize).wrapping_add(1_usize)) as isize);
        ({
            let _p: Ptr<u8> = (fb).clone();
            set_bytes_0(_p, ::std::mem::size_of::<u8>(), ((80) as u8))
        });
        assert!(
            (((((elem!((array_field_ptr!((s), bytes) as Ptr::<u8>), 1).read()) as i32)
                == ((((72340172838076673_u64 as u64).wrapping_mul((((80) as u8) as u64))) as u8)
                    as i32)) as i32)
                != 0)
        );
        assert!(
            (({
                let _p: Ptr<u8> = (sb).clone();
                all_bytes_1(_p, ::std::mem::size_of::<u8>(), ((80) as u8))
            }) != 0)
        );
        elem!((array_field_ptr!((s), bytes) as Ptr::<u8>), 1).write(
            (((72340172838076673_u64 as u64).wrapping_mul(((((80) + 1) as u8) as u64))) as u8),
        );
        assert!(
            (({
                let _p: Ptr<u8> = (fb).clone();
                let _v: u8 = (((80) + 1) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<u8>(), _v)
            }) != 0)
        );
        assert!(
            (({
                let _p: Ptr<u8> = (sb).clone();
                let _v: u8 = (((80) + 1) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<u8>(), _v)
            }) != 0)
        );
        ({
            let _p: Ptr<u8> = (sb).clone();
            let _v: u8 = (((80) + 2) as u8);
            set_bytes_0(_p, ::std::mem::size_of::<u8>(), _v)
        });
        assert!(
            (((((elem!((array_field_ptr!((s), bytes) as Ptr::<u8>), 1).read()) as i32)
                == ((((72340172838076673_u64 as u64).wrapping_mul(((((80) + 2) as u8) as u64)))
                    as u8) as i32)) as i32)
                != 0)
        );
        assert!(
            (({
                let _p: Ptr<u8> = (fb).clone();
                let _v: u8 = (((80) + 2) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<u8>(), _v)
            }) != 0)
        );
        elem!((array_field_ptr!((s), bytes) as Ptr::<u8>), 1).write(
            (((72340172838076673_u64 as u64).wrapping_mul(((((80) + 3) as u8) as u64))) as u8),
        );
        assert!(
            (({
                let _p: Ptr<u8> = (sb).clone();
                let _v: u8 = (((80) + 3) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<u8>(), _v)
            }) != 0)
        );
        assert!(
            (({
                let _p: Ptr<u8> = (fb).clone();
                let _v: u8 = (((80) + 3) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<u8>(), _v)
            }) != 0)
        );
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut fb: Ptr<u8> = ((array_field_ptr!((s), bytes) as Ptr<u8>).offset((2) as isize))
            .reinterpret_cast::<u8>();
        let mut sb: Ptr<u8> = (s)
            .reinterpret_cast::<u8>()
            .offset(((12_usize as usize).wrapping_add(2_usize)) as isize);
        ({
            let _p: Ptr<u8> = (fb).clone();
            set_bytes_0(_p, ::std::mem::size_of::<u8>(), ((96) as u8))
        });
        assert!(
            (((((elem!((array_field_ptr!((s), bytes) as Ptr::<u8>), 2).read()) as i32)
                == ((((72340172838076673_u64 as u64).wrapping_mul((((96) as u8) as u64))) as u8)
                    as i32)) as i32)
                != 0)
        );
        assert!(
            (({
                let _p: Ptr<u8> = (sb).clone();
                all_bytes_1(_p, ::std::mem::size_of::<u8>(), ((96) as u8))
            }) != 0)
        );
        elem!((array_field_ptr!((s), bytes) as Ptr::<u8>), 2).write(
            (((72340172838076673_u64 as u64).wrapping_mul(((((96) + 1) as u8) as u64))) as u8),
        );
        assert!(
            (({
                let _p: Ptr<u8> = (fb).clone();
                let _v: u8 = (((96) + 1) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<u8>(), _v)
            }) != 0)
        );
        assert!(
            (({
                let _p: Ptr<u8> = (sb).clone();
                let _v: u8 = (((96) + 1) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<u8>(), _v)
            }) != 0)
        );
        ({
            let _p: Ptr<u8> = (sb).clone();
            let _v: u8 = (((96) + 2) as u8);
            set_bytes_0(_p, ::std::mem::size_of::<u8>(), _v)
        });
        assert!(
            (((((elem!((array_field_ptr!((s), bytes) as Ptr::<u8>), 2).read()) as i32)
                == ((((72340172838076673_u64 as u64).wrapping_mul(((((96) + 2) as u8) as u64)))
                    as u8) as i32)) as i32)
                != 0)
        );
        assert!(
            (({
                let _p: Ptr<u8> = (fb).clone();
                let _v: u8 = (((96) + 2) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<u8>(), _v)
            }) != 0)
        );
        elem!((array_field_ptr!((s), bytes) as Ptr::<u8>), 2).write(
            (((72340172838076673_u64 as u64).wrapping_mul(((((96) + 3) as u8) as u64))) as u8),
        );
        assert!(
            (({
                let _p: Ptr<u8> = (sb).clone();
                let _v: u8 = (((96) + 3) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<u8>(), _v)
            }) != 0)
        );
        assert!(
            (({
                let _p: Ptr<u8> = (fb).clone();
                let _v: u8 = (((96) + 3) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<u8>(), _v)
            }) != 0)
        );
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut fb: Ptr<u8> = ((array_field_ptr!((s), bytes) as Ptr<u8>).offset((3) as isize))
            .reinterpret_cast::<u8>();
        let mut sb: Ptr<u8> = (s)
            .reinterpret_cast::<u8>()
            .offset(((12_usize as usize).wrapping_add(3_usize)) as isize);
        ({
            let _p: Ptr<u8> = (fb).clone();
            set_bytes_0(_p, ::std::mem::size_of::<u8>(), ((112) as u8))
        });
        assert!(
            (((((elem!((array_field_ptr!((s), bytes) as Ptr::<u8>), 3).read()) as i32)
                == ((((72340172838076673_u64 as u64).wrapping_mul((((112) as u8) as u64))) as u8)
                    as i32)) as i32)
                != 0)
        );
        assert!(
            (({
                let _p: Ptr<u8> = (sb).clone();
                all_bytes_1(_p, ::std::mem::size_of::<u8>(), ((112) as u8))
            }) != 0)
        );
        elem!((array_field_ptr!((s), bytes) as Ptr::<u8>), 3).write(
            (((72340172838076673_u64 as u64).wrapping_mul(((((112) + 1) as u8) as u64))) as u8),
        );
        assert!(
            (({
                let _p: Ptr<u8> = (fb).clone();
                let _v: u8 = (((112) + 1) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<u8>(), _v)
            }) != 0)
        );
        assert!(
            (({
                let _p: Ptr<u8> = (sb).clone();
                let _v: u8 = (((112) + 1) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<u8>(), _v)
            }) != 0)
        );
        ({
            let _p: Ptr<u8> = (sb).clone();
            let _v: u8 = (((112) + 2) as u8);
            set_bytes_0(_p, ::std::mem::size_of::<u8>(), _v)
        });
        assert!(
            (((((elem!((array_field_ptr!((s), bytes) as Ptr::<u8>), 3).read()) as i32)
                == ((((72340172838076673_u64 as u64).wrapping_mul(((((112) + 2) as u8) as u64)))
                    as u8) as i32)) as i32)
                != 0)
        );
        assert!(
            (({
                let _p: Ptr<u8> = (fb).clone();
                let _v: u8 = (((112) + 2) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<u8>(), _v)
            }) != 0)
        );
        elem!((array_field_ptr!((s), bytes) as Ptr::<u8>), 3).write(
            (((72340172838076673_u64 as u64).wrapping_mul(((((112) + 3) as u8) as u64))) as u8),
        );
        assert!(
            (({
                let _p: Ptr<u8> = (sb).clone();
                let _v: u8 = (((112) + 3) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<u8>(), _v)
            }) != 0)
        );
        assert!(
            (({
                let _p: Ptr<u8> = (fb).clone();
                let _v: u8 = (((112) + 3) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<u8>(), _v)
            }) != 0)
        );
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut fb: Ptr<u8> = ((array_field_ptr!((s), arr) as Ptr<i32>).offset((0) as isize))
            .reinterpret_cast::<u8>();
        let mut sb: Ptr<u8> = (s).reinterpret_cast::<u8>().offset(
            ((16_usize as usize).wrapping_add(
                ((0_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)) as usize),
            )) as isize,
        );
        ({
            let _p: Ptr<u8> = (fb).clone();
            set_bytes_0(_p, ::std::mem::size_of::<i32>(), ((128) as u8))
        });
        assert!(
            ((((elem!((array_field_ptr!((s), arr) as Ptr::<i32>), 0).read())
                == (((72340172838076673_u64 as u64).wrapping_mul((((128) as u8) as u64))) as i32))
                as i32)
                != 0)
        );
        assert!(
            (({
                let _p: Ptr<u8> = (sb).clone();
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), ((128) as u8))
            }) != 0)
        );
        elem!((array_field_ptr!((s), arr) as Ptr::<i32>), 0).write(
            (((72340172838076673_u64 as u64).wrapping_mul(((((128) + 1) as u8) as u64))) as i32),
        );
        assert!(
            (({
                let _p: Ptr<u8> = (fb).clone();
                let _v: u8 = (((128) + 1) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), _v)
            }) != 0)
        );
        assert!(
            (({
                let _p: Ptr<u8> = (sb).clone();
                let _v: u8 = (((128) + 1) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), _v)
            }) != 0)
        );
        ({
            let _p: Ptr<u8> = (sb).clone();
            let _v: u8 = (((128) + 2) as u8);
            set_bytes_0(_p, ::std::mem::size_of::<i32>(), _v)
        });
        assert!(
            ((((elem!((array_field_ptr!((s), arr) as Ptr::<i32>), 0).read())
                == (((72340172838076673_u64 as u64).wrapping_mul(((((128) + 2) as u8) as u64)))
                    as i32)) as i32)
                != 0)
        );
        assert!(
            (({
                let _p: Ptr<u8> = (fb).clone();
                let _v: u8 = (((128) + 2) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), _v)
            }) != 0)
        );
        elem!((array_field_ptr!((s), arr) as Ptr::<i32>), 0).write(
            (((72340172838076673_u64 as u64).wrapping_mul(((((128) + 3) as u8) as u64))) as i32),
        );
        assert!(
            (({
                let _p: Ptr<u8> = (sb).clone();
                let _v: u8 = (((128) + 3) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), _v)
            }) != 0)
        );
        assert!(
            (({
                let _p: Ptr<u8> = (fb).clone();
                let _v: u8 = (((128) + 3) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), _v)
            }) != 0)
        );
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut fb: Ptr<u8> = ((array_field_ptr!((s), arr) as Ptr<i32>).offset((1) as isize))
            .reinterpret_cast::<u8>();
        let mut sb: Ptr<u8> = (s).reinterpret_cast::<u8>().offset(
            ((16_usize as usize).wrapping_add(
                ((1_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)) as usize),
            )) as isize,
        );
        ({
            let _p: Ptr<u8> = (fb).clone();
            set_bytes_0(_p, ::std::mem::size_of::<i32>(), ((144) as u8))
        });
        assert!(
            ((((elem!((array_field_ptr!((s), arr) as Ptr::<i32>), 1).read())
                == (((72340172838076673_u64 as u64).wrapping_mul((((144) as u8) as u64))) as i32))
                as i32)
                != 0)
        );
        assert!(
            (({
                let _p: Ptr<u8> = (sb).clone();
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), ((144) as u8))
            }) != 0)
        );
        elem!((array_field_ptr!((s), arr) as Ptr::<i32>), 1).write(
            (((72340172838076673_u64 as u64).wrapping_mul(((((144) + 1) as u8) as u64))) as i32),
        );
        assert!(
            (({
                let _p: Ptr<u8> = (fb).clone();
                let _v: u8 = (((144) + 1) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), _v)
            }) != 0)
        );
        assert!(
            (({
                let _p: Ptr<u8> = (sb).clone();
                let _v: u8 = (((144) + 1) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), _v)
            }) != 0)
        );
        ({
            let _p: Ptr<u8> = (sb).clone();
            let _v: u8 = (((144) + 2) as u8);
            set_bytes_0(_p, ::std::mem::size_of::<i32>(), _v)
        });
        assert!(
            ((((elem!((array_field_ptr!((s), arr) as Ptr::<i32>), 1).read())
                == (((72340172838076673_u64 as u64).wrapping_mul(((((144) + 2) as u8) as u64)))
                    as i32)) as i32)
                != 0)
        );
        assert!(
            (({
                let _p: Ptr<u8> = (fb).clone();
                let _v: u8 = (((144) + 2) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), _v)
            }) != 0)
        );
        elem!((array_field_ptr!((s), arr) as Ptr::<i32>), 1).write(
            (((72340172838076673_u64 as u64).wrapping_mul(((((144) + 3) as u8) as u64))) as i32),
        );
        assert!(
            (({
                let _p: Ptr<u8> = (sb).clone();
                let _v: u8 = (((144) + 3) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), _v)
            }) != 0)
        );
        assert!(
            (({
                let _p: Ptr<u8> = (fb).clone();
                let _v: u8 = (((144) + 3) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), _v)
            }) != 0)
        );
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut fb: Ptr<u8> = ((array_field_ptr!((s), arr) as Ptr<i32>).offset((2) as isize))
            .reinterpret_cast::<u8>();
        let mut sb: Ptr<u8> = (s).reinterpret_cast::<u8>().offset(
            ((16_usize as usize).wrapping_add(
                ((2_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)) as usize),
            )) as isize,
        );
        ({
            let _p: Ptr<u8> = (fb).clone();
            set_bytes_0(_p, ::std::mem::size_of::<i32>(), ((8) as u8))
        });
        assert!(
            ((((elem!((array_field_ptr!((s), arr) as Ptr::<i32>), 2).read())
                == (((72340172838076673_u64 as u64).wrapping_mul((((8) as u8) as u64))) as i32))
                as i32)
                != 0)
        );
        assert!(
            (({
                let _p: Ptr<u8> = (sb).clone();
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), ((8) as u8))
            }) != 0)
        );
        elem!((array_field_ptr!((s), arr) as Ptr::<i32>), 2).write(
            (((72340172838076673_u64 as u64).wrapping_mul(((((8) + 1) as u8) as u64))) as i32),
        );
        assert!(
            (({
                let _p: Ptr<u8> = (fb).clone();
                let _v: u8 = (((8) + 1) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), _v)
            }) != 0)
        );
        assert!(
            (({
                let _p: Ptr<u8> = (sb).clone();
                let _v: u8 = (((8) + 1) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), _v)
            }) != 0)
        );
        ({
            let _p: Ptr<u8> = (sb).clone();
            let _v: u8 = (((8) + 2) as u8);
            set_bytes_0(_p, ::std::mem::size_of::<i32>(), _v)
        });
        assert!(
            ((((elem!((array_field_ptr!((s), arr) as Ptr::<i32>), 2).read())
                == (((72340172838076673_u64 as u64).wrapping_mul(((((8) + 2) as u8) as u64)))
                    as i32)) as i32)
                != 0)
        );
        assert!(
            (({
                let _p: Ptr<u8> = (fb).clone();
                let _v: u8 = (((8) + 2) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), _v)
            }) != 0)
        );
        elem!((array_field_ptr!((s), arr) as Ptr::<i32>), 2).write(
            (((72340172838076673_u64 as u64).wrapping_mul(((((8) + 3) as u8) as u64))) as i32),
        );
        assert!(
            (({
                let _p: Ptr<u8> = (sb).clone();
                let _v: u8 = (((8) + 3) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), _v)
            }) != 0)
        );
        assert!(
            (({
                let _p: Ptr<u8> = (fb).clone();
                let _v: u8 = (((8) + 3) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), _v)
            }) != 0)
        );
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut fb: Ptr<u8> = (field_ptr!((s), tail)).reinterpret_cast::<u8>();
        let mut sb: Ptr<u8> = (s).reinterpret_cast::<u8>().offset((32_usize) as isize);
        ({
            let _p: Ptr<u8> = (fb).clone();
            set_bytes_0(_p, ::std::mem::size_of::<i64>(), ((24) as u8))
        });
        assert!(
            ((((s).with(|__s| __s.tail)
                == (((72340172838076673_u64 as u64).wrapping_mul((((24) as u8) as u64))) as i64))
                as i32)
                != 0)
        );
        assert!(
            (({
                let _p: Ptr<u8> = (sb).clone();
                all_bytes_1(_p, ::std::mem::size_of::<i64>(), ((24) as u8))
            }) != 0)
        );
        field!((s), tail).write(
            (((72340172838076673_u64 as u64).wrapping_mul(((((24) + 1) as u8) as u64))) as i64),
        );
        assert!(
            (({
                let _p: Ptr<u8> = (fb).clone();
                let _v: u8 = (((24) + 1) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i64>(), _v)
            }) != 0)
        );
        assert!(
            (({
                let _p: Ptr<u8> = (sb).clone();
                let _v: u8 = (((24) + 1) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i64>(), _v)
            }) != 0)
        );
        ({
            let _p: Ptr<u8> = (sb).clone();
            let _v: u8 = (((24) + 2) as u8);
            set_bytes_0(_p, ::std::mem::size_of::<i64>(), _v)
        });
        assert!(
            ((((s).with(|__s| __s.tail)
                == (((72340172838076673_u64 as u64).wrapping_mul(((((24) + 2) as u8) as u64)))
                    as i64)) as i32)
                != 0)
        );
        assert!(
            (({
                let _p: Ptr<u8> = (fb).clone();
                let _v: u8 = (((24) + 2) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i64>(), _v)
            }) != 0)
        );
        field!((s), tail).write(
            (((72340172838076673_u64 as u64).wrapping_mul(((((24) + 3) as u8) as u64))) as i64),
        );
        assert!(
            (({
                let _p: Ptr<u8> = (sb).clone();
                let _v: u8 = (((24) + 3) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i64>(), _v)
            }) != 0)
        );
        assert!(
            (({
                let _p: Ptr<u8> = (fb).clone();
                let _v: u8 = (((24) + 3) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i64>(), _v)
            }) != 0)
        );
    }
    ({ check_final_2((s).clone()) });
    ({ set_bytes_0((field_ptr!(s, in_)).reinterpret_cast::<u8>(), 8usize, 33_u8) });
    assert!(
        (((((((s.with(|__s| __s.in_.a) as i32)
            == ((((72340172838076673_u64 as u64).wrapping_mul((((33) as u8) as u64))) as i16)
                as i32)) as i32)
            != 0)
            && (((s.with(|__s| __s.in_.b)
                == (((72340172838076673_u64 as u64).wrapping_mul((((33) as u8) as u64))) as i32))
                as i32)
                != 0)) as i32)
            != 0)
    );
    let in_: Value<In> = Rc::new(RefCell::new(In {
        a: (((72340172838076673_u64 as u64).wrapping_mul((((35) as u8) as u64))) as i16),
        b: (((72340172838076673_u64 as u64).wrapping_mul((((51) as u8) as u64))) as i32),
    }));
    field!(s, in_).write((*in_.borrow()).clone());
    assert!(
        (({
            all_bytes_1(
                (field_ptr!(field_ptr!(s, in_), a)).reinterpret_cast::<u8>(),
                ::std::mem::size_of::<i16>(),
                35_u8,
            )
        }) != 0)
    );
    assert!(
        (({
            all_bytes_1(
                (field_ptr!(field_ptr!(s, in_), b)).reinterpret_cast::<u8>(),
                ::std::mem::size_of::<i32>(),
                51_u8,
            )
        }) != 0)
    );
    ({
        let _p: Ptr<u8> = (array_field_ptr!(s, arr) as Ptr<i32>).reinterpret_cast::<u8>();
        let _n: usize = ::std::mem::size_of::<[i32; 3]>();
        set_bytes_0(_p, _n, 133_u8)
    });
    assert!(
        (((((((elem!((array_field_ptr!(s, arr) as Ptr::<i32>), 0).read())
            == (((72340172838076673_u64 as u64).wrapping_mul((((133) as u8) as u64))) as i32))
            as i32)
            != 0)
            && ((((elem!((array_field_ptr!(s, arr) as Ptr::<i32>), 2).read())
                == (((72340172838076673_u64 as u64).wrapping_mul((((133) as u8) as u64))) as i32))
                as i32)
                != 0)) as i32)
            != 0)
    );
    ({
        let _p: Ptr<u8> = ((array_field_ptr!(s, arr)) as Ptr<i32>).reinterpret_cast::<u8>();
        let _n: usize = ::std::mem::size_of::<[i32; 3]>();
        set_bytes_0(_p, _n, 134_u8)
    });
    assert!(
        ((((elem!((array_field_ptr!(s, arr) as Ptr::<i32>), 1).read())
            == (((72340172838076673_u64 as u64).wrapping_mul((((134) as u8) as u64))) as i32))
            as i32)
            != 0)
    );
    elem!((array_field_ptr!(s, arr) as Ptr::<i32>), 0)
        .write((((72340172838076673_u64 as u64).wrapping_mul((((131) as u8) as u64))) as i32));
    elem!((array_field_ptr!(s, arr) as Ptr::<i32>), 1)
        .write((((72340172838076673_u64 as u64).wrapping_mul((((147) as u8) as u64))) as i32));
    elem!((array_field_ptr!(s, arr) as Ptr::<i32>), 2)
        .write((((72340172838076673_u64 as u64).wrapping_mul((((11) as u8) as u64))) as i32));
    let mut ib: Ptr<i32> = (array_field_ptr!(s, bytes) as Ptr<u8>).reinterpret_cast::<i32>();
    ib.write((((72340172838076673_u64 as u64).wrapping_mul((((69) as u8) as u64))) as i32));
    assert!(
        (({
            let _p: Ptr<u8> = (array_field_ptr!(s, bytes) as Ptr<u8>);
            let _n: usize = ::std::mem::size_of::<[u8; 4]>();
            all_bytes_1(_p, _n, 69_u8)
        }) != 0)
    );
    elem!((array_field_ptr!(s, bytes) as Ptr::<u8>), 0).write(67_u8);
    elem!((array_field_ptr!(s, bytes) as Ptr::<u8>), 1).write(83_u8);
    elem!((array_field_ptr!(s, bytes) as Ptr::<u8>), 2).write(99_u8);
    elem!((array_field_ptr!(s, bytes) as Ptr::<u8>), 3).write(115_u8);
    assert!(
        ((({ (ib.read()) } == {
            (((array_field_ptr!(s, bytes)) as Ptr<u8>)
                .reinterpret_cast::<i32>()
                .read())
        }) as i32)
            != 0)
    );
    let mut v: AnyPtr = ((field_ptr!(field_ptr!(s, in_), b)) as Ptr<i32>).to_any();
    v.reinterpret_cast::<i32>().write(11);
    assert!((((s.with(|__s| __s.in_.b) == 11) as i32) != 0));
    let twelve: Value<i32> = Rc::new(RefCell::new(12));
    {
        ((field_ptr!(s, x)) as Ptr<i32>).to_any().memcpy(
            &((twelve.as_pointer()) as Ptr<i32>).to_any(),
            ::std::mem::size_of::<i32>() as usize,
        );
        ((field_ptr!(s, x)) as Ptr<i32>).to_any()
    };
    assert!((((s.with(|__s| __s.x) == 12) as i32) != 0));
    {
        ((field_ptr!(field_ptr!(s, in_), b)) as Ptr<i32>)
            .to_any()
            .memset((51) as u8, ::std::mem::size_of::<i32>() as usize);
        ((field_ptr!(field_ptr!(s, in_), b)) as Ptr<i32>).to_any()
    };
    {
        ((field_ptr!(s, x)) as Ptr<i32>)
            .to_any()
            .memset((19) as u8, ::std::mem::size_of::<i32>() as usize);
        ((field_ptr!(s, x)) as Ptr<i32>).to_any()
    };
    ({ check_final_2((s).clone()) });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let local: Value<S> = <Value<S>>::default();
    {
        ((local.as_pointer()) as Ptr<S>)
            .to_any()
            .memset((0) as u8, 40usize as usize);
        ((local.as_pointer()) as Ptr<S>).to_any()
    };
    ({ check_struct_3((local.as_pointer())) });
    let arr: Value<Box<[S]>> = Rc::new(RefCell::new(
        (0..2).map(|_| <S>::default()).collect::<Box<[S]>>(),
    ));
    {
        ((arr.as_pointer() as Ptr<S>) as Ptr<S>)
            .to_any()
            .memset((0) as u8, 80usize as usize);
        ((arr.as_pointer() as Ptr<S>) as Ptr<S>).to_any()
    };
    ({ check_struct_3(((arr.as_pointer() as Ptr<S>).offset(1))) });
    assert!(
        (({
            all_bytes_1(
                ((arr.as_pointer() as Ptr<S>).offset(0)).reinterpret_cast::<u8>(),
                40usize,
                0_u8,
            )
        }) != 0)
    );
    let mut heap: Ptr<S> = libcc2rs::calloc_refcount(1_usize, 40usize).reinterpret_cast::<S>();
    assert!((((!((heap).is_null())) as i32) != 0));
    ({ check_struct_3((heap).clone()) });
    libcc2rs::free_refcount((heap).to_any());
    let mut raw_: Ptr<u8> = libcc2rs::malloc_refcount(40usize).reinterpret_cast::<u8>();
    assert!((((!((raw_).is_null())) as i32) != 0));
    {
        (raw_).to_any().memset((0) as u8, 40usize as usize);
        (raw_).to_any()
    };
    let mut view: Ptr<S> = raw_.reinterpret_cast::<S>();
    ({ check_struct_3((view).clone()) });
    assert!((((((elem!(raw_, 32_usize).read()) as i32) == 27) as i32) != 0));
    ({
        let _p: Ptr<u8> = raw_.offset((16_usize) as isize);
        set_bytes_0(_p, ::std::mem::size_of::<i32>(), 119_u8)
    });
    assert!(
        ((((elem!((array_field_ptr!(view, arr) as Ptr::<i32>), 0).read())
            == (((72340172838076673_u64 as u64).wrapping_mul((((119) as u8) as u64))) as i32))
            as i32)
            != 0)
    );
    elem!((array_field_ptr!(view, bytes) as Ptr::<u8>), 3).write(17_u8);
    assert!(
        (((((elem!(raw_, (12_usize as usize).wrapping_add(3_usize)).read()) as i32) == 17) as i32)
            != 0)
    );
    libcc2rs::free_refcount((raw_).to_any());
    return 0;
}
pub fn __cpp2rust_init_globals() {}
