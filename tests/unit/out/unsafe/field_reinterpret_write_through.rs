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
pub struct In {
    pub a: i16,
    pub b: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct S {
    pub x: i32,
    pub in_: In,
    pub bytes: [u8; 4],
    pub arr: [i32; 3],
    pub tail: i64,
}
impl Default for S {
    fn default() -> Self {
        S {
            x: 0_i32,
            in_: <In>::default(),
            bytes: [0_u8; 4],
            arr: [0_i32; 3],
            tail: 0_i64,
        }
    }
}
pub unsafe fn set_bytes_0(mut p: *mut u8, mut n: usize, mut v: u8) {
    let mut i: usize = 0_usize;
    'loop_: while ((((i) < (n)) as i32) != 0) {
        (*p.offset((i) as isize)) = v;
        i.postfix_inc();
    }
}
pub unsafe fn all_bytes_1(mut p: *const u8, mut n: usize, mut v: u8) -> i32 {
    let mut i: usize = 0_usize;
    'loop_: while ((((i) < (n)) as i32) != 0) {
        if (((((*p.offset((i) as isize)) as i32) != (v as i32)) as i32) != 0) {
            return 0;
        }
        i.postfix_inc();
    }
    return 1;
}
pub unsafe fn check_final_2(mut s: *mut S) {
    assert!(
        (((((*s).x)
            == (((72340172838076673_u64 as u64).wrapping_mul((((19) as u8) as u64))) as i32))
            as i32)
            != 0)
    );
    assert!(
        (((((*s).in_.a as i32)
            == ((((72340172838076673_u64 as u64).wrapping_mul((((35) as u8) as u64))) as i16)
                as i32)) as i32)
            != 0)
    );
    assert!(
        (((((*s).in_.b)
            == (((72340172838076673_u64 as u64).wrapping_mul((((51) as u8) as u64))) as i32))
            as i32)
            != 0)
    );
    assert!(
        ((((((((*s).bytes[(0) as usize] as i32) == (67)) as i32) != 0)
            && (((((*s).bytes[(1) as usize] as i32) == (83)) as i32) != 0)) as i32)
            != 0)
    );
    assert!(
        ((((((((*s).bytes[(2) as usize] as i32) == (99)) as i32) != 0)
            && (((((*s).bytes[(3) as usize] as i32) == (115)) as i32) != 0)) as i32)
            != 0)
    );
    assert!(
        ((((((((*s).arr[(0) as usize])
            == (((72340172838076673_u64 as u64).wrapping_mul((((131) as u8) as u64))) as i32))
            as i32)
            != 0)
            && (((((*s).arr[(1) as usize])
                == (((72340172838076673_u64 as u64).wrapping_mul((((147) as u8) as u64))) as i32))
                as i32)
                != 0)) as i32)
            != 0)
    );
    assert!(
        (((((*s).arr[(2) as usize])
            == (((72340172838076673_u64 as u64).wrapping_mul((((11) as u8) as u64))) as i32))
            as i32)
            != 0)
    );
    assert!(
        (((((*s).tail)
            == (((72340172838076673_u64 as u64).wrapping_mul((((27) as u8) as u64))) as i64))
            as i32)
            != 0)
    );
}
pub unsafe fn check_struct_3(mut s: *mut S) {
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut fb: *mut u8 = ((&mut (*(s)).x as *mut i32) as *mut u8);
        let mut sb: *mut u8 = ((s) as *mut u8).offset((::std::mem::offset_of!(S, x)) as isize);
        (unsafe {
            let _p: *mut u8 = fb;
            set_bytes_0(_p, ::std::mem::size_of::<i32>(), ((16) as u8))
        });
        assert!(
            (((((*(s)).x)
                == (((72340172838076673_u64 as u64).wrapping_mul((((16) as u8) as u64))) as i32))
                as i32)
                != 0)
        );
        assert!(
            ((unsafe {
                let _p: *const u8 = (sb).cast_const();
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), ((16) as u8))
            }) != 0)
        );
        (*(s)).x =
            (((72340172838076673_u64 as u64).wrapping_mul(((((16) + (1)) as u8) as u64))) as i32);
        assert!(
            ((unsafe {
                let _p: *const u8 = (fb).cast_const();
                let _v: u8 = (((16) + (1)) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), _v)
            }) != 0)
        );
        assert!(
            ((unsafe {
                let _p: *const u8 = (sb).cast_const();
                let _v: u8 = (((16) + (1)) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), _v)
            }) != 0)
        );
        (unsafe {
            let _p: *mut u8 = sb;
            let _v: u8 = (((16) + (2)) as u8);
            set_bytes_0(_p, ::std::mem::size_of::<i32>(), _v)
        });
        assert!(
            (((((*(s)).x)
                == (((72340172838076673_u64 as u64).wrapping_mul(((((16) + (2)) as u8) as u64)))
                    as i32)) as i32)
                != 0)
        );
        assert!(
            ((unsafe {
                let _p: *const u8 = (fb).cast_const();
                let _v: u8 = (((16) + (2)) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), _v)
            }) != 0)
        );
        (*(s)).x =
            (((72340172838076673_u64 as u64).wrapping_mul(((((16) + (3)) as u8) as u64))) as i32);
        assert!(
            ((unsafe {
                let _p: *const u8 = (sb).cast_const();
                let _v: u8 = (((16) + (3)) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), _v)
            }) != 0)
        );
        assert!(
            ((unsafe {
                let _p: *const u8 = (fb).cast_const();
                let _v: u8 = (((16) + (3)) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), _v)
            }) != 0)
        );
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut fb: *mut u8 = ((&mut (*(s)).in_.a as *mut i16) as *mut u8);
        let mut sb: *mut u8 = ((s) as *mut u8).offset(
            ((::std::mem::offset_of!(S, in_) as usize)
                .wrapping_add((::std::mem::offset_of!(In, a) as usize))) as isize,
        );
        (unsafe {
            let _p: *mut u8 = fb;
            set_bytes_0(_p, ::std::mem::size_of::<i16>(), ((32) as u8))
        });
        assert!(
            (((((*(s)).in_.a as i32)
                == ((((72340172838076673_u64 as u64).wrapping_mul((((32) as u8) as u64))) as i16)
                    as i32)) as i32)
                != 0)
        );
        assert!(
            ((unsafe {
                let _p: *const u8 = (sb).cast_const();
                all_bytes_1(_p, ::std::mem::size_of::<i16>(), ((32) as u8))
            }) != 0)
        );
        (*(s)).in_.a =
            (((72340172838076673_u64 as u64).wrapping_mul(((((32) + (1)) as u8) as u64))) as i16);
        assert!(
            ((unsafe {
                let _p: *const u8 = (fb).cast_const();
                let _v: u8 = (((32) + (1)) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i16>(), _v)
            }) != 0)
        );
        assert!(
            ((unsafe {
                let _p: *const u8 = (sb).cast_const();
                let _v: u8 = (((32) + (1)) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i16>(), _v)
            }) != 0)
        );
        (unsafe {
            let _p: *mut u8 = sb;
            let _v: u8 = (((32) + (2)) as u8);
            set_bytes_0(_p, ::std::mem::size_of::<i16>(), _v)
        });
        assert!(
            (((((*(s)).in_.a as i32)
                == ((((72340172838076673_u64 as u64).wrapping_mul(((((32) + (2)) as u8) as u64)))
                    as i16) as i32)) as i32)
                != 0)
        );
        assert!(
            ((unsafe {
                let _p: *const u8 = (fb).cast_const();
                let _v: u8 = (((32) + (2)) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i16>(), _v)
            }) != 0)
        );
        (*(s)).in_.a =
            (((72340172838076673_u64 as u64).wrapping_mul(((((32) + (3)) as u8) as u64))) as i16);
        assert!(
            ((unsafe {
                let _p: *const u8 = (sb).cast_const();
                let _v: u8 = (((32) + (3)) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i16>(), _v)
            }) != 0)
        );
        assert!(
            ((unsafe {
                let _p: *const u8 = (fb).cast_const();
                let _v: u8 = (((32) + (3)) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i16>(), _v)
            }) != 0)
        );
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut fb: *mut u8 = ((&mut (*(s)).in_.b as *mut i32) as *mut u8);
        let mut sb: *mut u8 = ((s) as *mut u8).offset(
            ((::std::mem::offset_of!(S, in_) as usize)
                .wrapping_add((::std::mem::offset_of!(In, b) as usize))) as isize,
        );
        (unsafe {
            let _p: *mut u8 = fb;
            set_bytes_0(_p, ::std::mem::size_of::<i32>(), ((48) as u8))
        });
        assert!(
            (((((*(s)).in_.b)
                == (((72340172838076673_u64 as u64).wrapping_mul((((48) as u8) as u64))) as i32))
                as i32)
                != 0)
        );
        assert!(
            ((unsafe {
                let _p: *const u8 = (sb).cast_const();
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), ((48) as u8))
            }) != 0)
        );
        (*(s)).in_.b =
            (((72340172838076673_u64 as u64).wrapping_mul(((((48) + (1)) as u8) as u64))) as i32);
        assert!(
            ((unsafe {
                let _p: *const u8 = (fb).cast_const();
                let _v: u8 = (((48) + (1)) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), _v)
            }) != 0)
        );
        assert!(
            ((unsafe {
                let _p: *const u8 = (sb).cast_const();
                let _v: u8 = (((48) + (1)) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), _v)
            }) != 0)
        );
        (unsafe {
            let _p: *mut u8 = sb;
            let _v: u8 = (((48) + (2)) as u8);
            set_bytes_0(_p, ::std::mem::size_of::<i32>(), _v)
        });
        assert!(
            (((((*(s)).in_.b)
                == (((72340172838076673_u64 as u64).wrapping_mul(((((48) + (2)) as u8) as u64)))
                    as i32)) as i32)
                != 0)
        );
        assert!(
            ((unsafe {
                let _p: *const u8 = (fb).cast_const();
                let _v: u8 = (((48) + (2)) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), _v)
            }) != 0)
        );
        (*(s)).in_.b =
            (((72340172838076673_u64 as u64).wrapping_mul(((((48) + (3)) as u8) as u64))) as i32);
        assert!(
            ((unsafe {
                let _p: *const u8 = (sb).cast_const();
                let _v: u8 = (((48) + (3)) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), _v)
            }) != 0)
        );
        assert!(
            ((unsafe {
                let _p: *const u8 = (fb).cast_const();
                let _v: u8 = (((48) + (3)) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), _v)
            }) != 0)
        );
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut fb: *mut u8 = (&mut (*(s)).bytes[(0) as usize] as *mut u8);
        let mut sb: *mut u8 = ((s) as *mut u8)
            .offset(((::std::mem::offset_of!(S, bytes) as usize).wrapping_add(0_usize)) as isize);
        (unsafe {
            let _p: *mut u8 = fb;
            set_bytes_0(_p, ::std::mem::size_of::<u8>(), ((64) as u8))
        });
        assert!(
            (((((*(s)).bytes[(0) as usize] as i32)
                == ((((72340172838076673_u64 as u64).wrapping_mul((((64) as u8) as u64))) as u8)
                    as i32)) as i32)
                != 0)
        );
        assert!(
            ((unsafe {
                let _p: *const u8 = (sb).cast_const();
                all_bytes_1(_p, ::std::mem::size_of::<u8>(), ((64) as u8))
            }) != 0)
        );
        (*(s)).bytes[(0) as usize] =
            (((72340172838076673_u64 as u64).wrapping_mul(((((64) + (1)) as u8) as u64))) as u8);
        assert!(
            ((unsafe {
                let _p: *const u8 = (fb).cast_const();
                let _v: u8 = (((64) + (1)) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<u8>(), _v)
            }) != 0)
        );
        assert!(
            ((unsafe {
                let _p: *const u8 = (sb).cast_const();
                let _v: u8 = (((64) + (1)) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<u8>(), _v)
            }) != 0)
        );
        (unsafe {
            let _p: *mut u8 = sb;
            let _v: u8 = (((64) + (2)) as u8);
            set_bytes_0(_p, ::std::mem::size_of::<u8>(), _v)
        });
        assert!(
            (((((*(s)).bytes[(0) as usize] as i32)
                == ((((72340172838076673_u64 as u64).wrapping_mul(((((64) + (2)) as u8) as u64)))
                    as u8) as i32)) as i32)
                != 0)
        );
        assert!(
            ((unsafe {
                let _p: *const u8 = (fb).cast_const();
                let _v: u8 = (((64) + (2)) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<u8>(), _v)
            }) != 0)
        );
        (*(s)).bytes[(0) as usize] =
            (((72340172838076673_u64 as u64).wrapping_mul(((((64) + (3)) as u8) as u64))) as u8);
        assert!(
            ((unsafe {
                let _p: *const u8 = (sb).cast_const();
                let _v: u8 = (((64) + (3)) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<u8>(), _v)
            }) != 0)
        );
        assert!(
            ((unsafe {
                let _p: *const u8 = (fb).cast_const();
                let _v: u8 = (((64) + (3)) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<u8>(), _v)
            }) != 0)
        );
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut fb: *mut u8 = (&mut (*(s)).bytes[(1) as usize] as *mut u8);
        let mut sb: *mut u8 = ((s) as *mut u8)
            .offset(((::std::mem::offset_of!(S, bytes) as usize).wrapping_add(1_usize)) as isize);
        (unsafe {
            let _p: *mut u8 = fb;
            set_bytes_0(_p, ::std::mem::size_of::<u8>(), ((80) as u8))
        });
        assert!(
            (((((*(s)).bytes[(1) as usize] as i32)
                == ((((72340172838076673_u64 as u64).wrapping_mul((((80) as u8) as u64))) as u8)
                    as i32)) as i32)
                != 0)
        );
        assert!(
            ((unsafe {
                let _p: *const u8 = (sb).cast_const();
                all_bytes_1(_p, ::std::mem::size_of::<u8>(), ((80) as u8))
            }) != 0)
        );
        (*(s)).bytes[(1) as usize] =
            (((72340172838076673_u64 as u64).wrapping_mul(((((80) + (1)) as u8) as u64))) as u8);
        assert!(
            ((unsafe {
                let _p: *const u8 = (fb).cast_const();
                let _v: u8 = (((80) + (1)) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<u8>(), _v)
            }) != 0)
        );
        assert!(
            ((unsafe {
                let _p: *const u8 = (sb).cast_const();
                let _v: u8 = (((80) + (1)) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<u8>(), _v)
            }) != 0)
        );
        (unsafe {
            let _p: *mut u8 = sb;
            let _v: u8 = (((80) + (2)) as u8);
            set_bytes_0(_p, ::std::mem::size_of::<u8>(), _v)
        });
        assert!(
            (((((*(s)).bytes[(1) as usize] as i32)
                == ((((72340172838076673_u64 as u64).wrapping_mul(((((80) + (2)) as u8) as u64)))
                    as u8) as i32)) as i32)
                != 0)
        );
        assert!(
            ((unsafe {
                let _p: *const u8 = (fb).cast_const();
                let _v: u8 = (((80) + (2)) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<u8>(), _v)
            }) != 0)
        );
        (*(s)).bytes[(1) as usize] =
            (((72340172838076673_u64 as u64).wrapping_mul(((((80) + (3)) as u8) as u64))) as u8);
        assert!(
            ((unsafe {
                let _p: *const u8 = (sb).cast_const();
                let _v: u8 = (((80) + (3)) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<u8>(), _v)
            }) != 0)
        );
        assert!(
            ((unsafe {
                let _p: *const u8 = (fb).cast_const();
                let _v: u8 = (((80) + (3)) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<u8>(), _v)
            }) != 0)
        );
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut fb: *mut u8 = (&mut (*(s)).bytes[(2) as usize] as *mut u8);
        let mut sb: *mut u8 = ((s) as *mut u8)
            .offset(((::std::mem::offset_of!(S, bytes) as usize).wrapping_add(2_usize)) as isize);
        (unsafe {
            let _p: *mut u8 = fb;
            set_bytes_0(_p, ::std::mem::size_of::<u8>(), ((96) as u8))
        });
        assert!(
            (((((*(s)).bytes[(2) as usize] as i32)
                == ((((72340172838076673_u64 as u64).wrapping_mul((((96) as u8) as u64))) as u8)
                    as i32)) as i32)
                != 0)
        );
        assert!(
            ((unsafe {
                let _p: *const u8 = (sb).cast_const();
                all_bytes_1(_p, ::std::mem::size_of::<u8>(), ((96) as u8))
            }) != 0)
        );
        (*(s)).bytes[(2) as usize] =
            (((72340172838076673_u64 as u64).wrapping_mul(((((96) + (1)) as u8) as u64))) as u8);
        assert!(
            ((unsafe {
                let _p: *const u8 = (fb).cast_const();
                let _v: u8 = (((96) + (1)) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<u8>(), _v)
            }) != 0)
        );
        assert!(
            ((unsafe {
                let _p: *const u8 = (sb).cast_const();
                let _v: u8 = (((96) + (1)) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<u8>(), _v)
            }) != 0)
        );
        (unsafe {
            let _p: *mut u8 = sb;
            let _v: u8 = (((96) + (2)) as u8);
            set_bytes_0(_p, ::std::mem::size_of::<u8>(), _v)
        });
        assert!(
            (((((*(s)).bytes[(2) as usize] as i32)
                == ((((72340172838076673_u64 as u64).wrapping_mul(((((96) + (2)) as u8) as u64)))
                    as u8) as i32)) as i32)
                != 0)
        );
        assert!(
            ((unsafe {
                let _p: *const u8 = (fb).cast_const();
                let _v: u8 = (((96) + (2)) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<u8>(), _v)
            }) != 0)
        );
        (*(s)).bytes[(2) as usize] =
            (((72340172838076673_u64 as u64).wrapping_mul(((((96) + (3)) as u8) as u64))) as u8);
        assert!(
            ((unsafe {
                let _p: *const u8 = (sb).cast_const();
                let _v: u8 = (((96) + (3)) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<u8>(), _v)
            }) != 0)
        );
        assert!(
            ((unsafe {
                let _p: *const u8 = (fb).cast_const();
                let _v: u8 = (((96) + (3)) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<u8>(), _v)
            }) != 0)
        );
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut fb: *mut u8 = (&mut (*(s)).bytes[(3) as usize] as *mut u8);
        let mut sb: *mut u8 = ((s) as *mut u8)
            .offset(((::std::mem::offset_of!(S, bytes) as usize).wrapping_add(3_usize)) as isize);
        (unsafe {
            let _p: *mut u8 = fb;
            set_bytes_0(_p, ::std::mem::size_of::<u8>(), ((112) as u8))
        });
        assert!(
            (((((*(s)).bytes[(3) as usize] as i32)
                == ((((72340172838076673_u64 as u64).wrapping_mul((((112) as u8) as u64))) as u8)
                    as i32)) as i32)
                != 0)
        );
        assert!(
            ((unsafe {
                let _p: *const u8 = (sb).cast_const();
                all_bytes_1(_p, ::std::mem::size_of::<u8>(), ((112) as u8))
            }) != 0)
        );
        (*(s)).bytes[(3) as usize] =
            (((72340172838076673_u64 as u64).wrapping_mul(((((112) + (1)) as u8) as u64))) as u8);
        assert!(
            ((unsafe {
                let _p: *const u8 = (fb).cast_const();
                let _v: u8 = (((112) + (1)) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<u8>(), _v)
            }) != 0)
        );
        assert!(
            ((unsafe {
                let _p: *const u8 = (sb).cast_const();
                let _v: u8 = (((112) + (1)) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<u8>(), _v)
            }) != 0)
        );
        (unsafe {
            let _p: *mut u8 = sb;
            let _v: u8 = (((112) + (2)) as u8);
            set_bytes_0(_p, ::std::mem::size_of::<u8>(), _v)
        });
        assert!(
            (((((*(s)).bytes[(3) as usize] as i32)
                == ((((72340172838076673_u64 as u64).wrapping_mul(((((112) + (2)) as u8) as u64)))
                    as u8) as i32)) as i32)
                != 0)
        );
        assert!(
            ((unsafe {
                let _p: *const u8 = (fb).cast_const();
                let _v: u8 = (((112) + (2)) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<u8>(), _v)
            }) != 0)
        );
        (*(s)).bytes[(3) as usize] =
            (((72340172838076673_u64 as u64).wrapping_mul(((((112) + (3)) as u8) as u64))) as u8);
        assert!(
            ((unsafe {
                let _p: *const u8 = (sb).cast_const();
                let _v: u8 = (((112) + (3)) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<u8>(), _v)
            }) != 0)
        );
        assert!(
            ((unsafe {
                let _p: *const u8 = (fb).cast_const();
                let _v: u8 = (((112) + (3)) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<u8>(), _v)
            }) != 0)
        );
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut fb: *mut u8 = ((&mut (*(s)).arr[(0) as usize] as *mut i32) as *mut i32 as *mut u8);
        let mut sb: *mut u8 = ((s) as *mut u8).offset(
            ((::std::mem::offset_of!(S, arr) as usize).wrapping_add(
                ((0_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)) as usize),
            )) as isize,
        );
        (unsafe {
            let _p: *mut u8 = fb;
            set_bytes_0(_p, ::std::mem::size_of::<i32>(), ((128) as u8))
        });
        assert!(
            (((((*(s)).arr[(0) as usize])
                == (((72340172838076673_u64 as u64).wrapping_mul((((128) as u8) as u64))) as i32))
                as i32)
                != 0)
        );
        assert!(
            ((unsafe {
                let _p: *const u8 = (sb).cast_const();
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), ((128) as u8))
            }) != 0)
        );
        (*(s)).arr[(0) as usize] =
            (((72340172838076673_u64 as u64).wrapping_mul(((((128) + (1)) as u8) as u64))) as i32);
        assert!(
            ((unsafe {
                let _p: *const u8 = (fb).cast_const();
                let _v: u8 = (((128) + (1)) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), _v)
            }) != 0)
        );
        assert!(
            ((unsafe {
                let _p: *const u8 = (sb).cast_const();
                let _v: u8 = (((128) + (1)) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), _v)
            }) != 0)
        );
        (unsafe {
            let _p: *mut u8 = sb;
            let _v: u8 = (((128) + (2)) as u8);
            set_bytes_0(_p, ::std::mem::size_of::<i32>(), _v)
        });
        assert!(
            (((((*(s)).arr[(0) as usize])
                == (((72340172838076673_u64 as u64).wrapping_mul(((((128) + (2)) as u8) as u64)))
                    as i32)) as i32)
                != 0)
        );
        assert!(
            ((unsafe {
                let _p: *const u8 = (fb).cast_const();
                let _v: u8 = (((128) + (2)) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), _v)
            }) != 0)
        );
        (*(s)).arr[(0) as usize] =
            (((72340172838076673_u64 as u64).wrapping_mul(((((128) + (3)) as u8) as u64))) as i32);
        assert!(
            ((unsafe {
                let _p: *const u8 = (sb).cast_const();
                let _v: u8 = (((128) + (3)) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), _v)
            }) != 0)
        );
        assert!(
            ((unsafe {
                let _p: *const u8 = (fb).cast_const();
                let _v: u8 = (((128) + (3)) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), _v)
            }) != 0)
        );
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut fb: *mut u8 = ((&mut (*(s)).arr[(1) as usize] as *mut i32) as *mut i32 as *mut u8);
        let mut sb: *mut u8 = ((s) as *mut u8).offset(
            ((::std::mem::offset_of!(S, arr) as usize).wrapping_add(
                ((1_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)) as usize),
            )) as isize,
        );
        (unsafe {
            let _p: *mut u8 = fb;
            set_bytes_0(_p, ::std::mem::size_of::<i32>(), ((144) as u8))
        });
        assert!(
            (((((*(s)).arr[(1) as usize])
                == (((72340172838076673_u64 as u64).wrapping_mul((((144) as u8) as u64))) as i32))
                as i32)
                != 0)
        );
        assert!(
            ((unsafe {
                let _p: *const u8 = (sb).cast_const();
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), ((144) as u8))
            }) != 0)
        );
        (*(s)).arr[(1) as usize] =
            (((72340172838076673_u64 as u64).wrapping_mul(((((144) + (1)) as u8) as u64))) as i32);
        assert!(
            ((unsafe {
                let _p: *const u8 = (fb).cast_const();
                let _v: u8 = (((144) + (1)) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), _v)
            }) != 0)
        );
        assert!(
            ((unsafe {
                let _p: *const u8 = (sb).cast_const();
                let _v: u8 = (((144) + (1)) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), _v)
            }) != 0)
        );
        (unsafe {
            let _p: *mut u8 = sb;
            let _v: u8 = (((144) + (2)) as u8);
            set_bytes_0(_p, ::std::mem::size_of::<i32>(), _v)
        });
        assert!(
            (((((*(s)).arr[(1) as usize])
                == (((72340172838076673_u64 as u64).wrapping_mul(((((144) + (2)) as u8) as u64)))
                    as i32)) as i32)
                != 0)
        );
        assert!(
            ((unsafe {
                let _p: *const u8 = (fb).cast_const();
                let _v: u8 = (((144) + (2)) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), _v)
            }) != 0)
        );
        (*(s)).arr[(1) as usize] =
            (((72340172838076673_u64 as u64).wrapping_mul(((((144) + (3)) as u8) as u64))) as i32);
        assert!(
            ((unsafe {
                let _p: *const u8 = (sb).cast_const();
                let _v: u8 = (((144) + (3)) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), _v)
            }) != 0)
        );
        assert!(
            ((unsafe {
                let _p: *const u8 = (fb).cast_const();
                let _v: u8 = (((144) + (3)) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), _v)
            }) != 0)
        );
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut fb: *mut u8 = ((&mut (*(s)).arr[(2) as usize] as *mut i32) as *mut i32 as *mut u8);
        let mut sb: *mut u8 = ((s) as *mut u8).offset(
            ((::std::mem::offset_of!(S, arr) as usize).wrapping_add(
                ((2_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)) as usize),
            )) as isize,
        );
        (unsafe {
            let _p: *mut u8 = fb;
            set_bytes_0(_p, ::std::mem::size_of::<i32>(), ((8) as u8))
        });
        assert!(
            (((((*(s)).arr[(2) as usize])
                == (((72340172838076673_u64 as u64).wrapping_mul((((8) as u8) as u64))) as i32))
                as i32)
                != 0)
        );
        assert!(
            ((unsafe {
                let _p: *const u8 = (sb).cast_const();
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), ((8) as u8))
            }) != 0)
        );
        (*(s)).arr[(2) as usize] =
            (((72340172838076673_u64 as u64).wrapping_mul(((((8) + (1)) as u8) as u64))) as i32);
        assert!(
            ((unsafe {
                let _p: *const u8 = (fb).cast_const();
                let _v: u8 = (((8) + (1)) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), _v)
            }) != 0)
        );
        assert!(
            ((unsafe {
                let _p: *const u8 = (sb).cast_const();
                let _v: u8 = (((8) + (1)) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), _v)
            }) != 0)
        );
        (unsafe {
            let _p: *mut u8 = sb;
            let _v: u8 = (((8) + (2)) as u8);
            set_bytes_0(_p, ::std::mem::size_of::<i32>(), _v)
        });
        assert!(
            (((((*(s)).arr[(2) as usize])
                == (((72340172838076673_u64 as u64).wrapping_mul(((((8) + (2)) as u8) as u64)))
                    as i32)) as i32)
                != 0)
        );
        assert!(
            ((unsafe {
                let _p: *const u8 = (fb).cast_const();
                let _v: u8 = (((8) + (2)) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), _v)
            }) != 0)
        );
        (*(s)).arr[(2) as usize] =
            (((72340172838076673_u64 as u64).wrapping_mul(((((8) + (3)) as u8) as u64))) as i32);
        assert!(
            ((unsafe {
                let _p: *const u8 = (sb).cast_const();
                let _v: u8 = (((8) + (3)) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), _v)
            }) != 0)
        );
        assert!(
            ((unsafe {
                let _p: *const u8 = (fb).cast_const();
                let _v: u8 = (((8) + (3)) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i32>(), _v)
            }) != 0)
        );
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut fb: *mut u8 = ((&mut (*(s)).tail as *mut i64) as *mut u8);
        let mut sb: *mut u8 = ((s) as *mut u8).offset((::std::mem::offset_of!(S, tail)) as isize);
        (unsafe {
            let _p: *mut u8 = fb;
            set_bytes_0(_p, ::std::mem::size_of::<i64>(), ((24) as u8))
        });
        assert!(
            (((((*(s)).tail)
                == (((72340172838076673_u64 as u64).wrapping_mul((((24) as u8) as u64))) as i64))
                as i32)
                != 0)
        );
        assert!(
            ((unsafe {
                let _p: *const u8 = (sb).cast_const();
                all_bytes_1(_p, ::std::mem::size_of::<i64>(), ((24) as u8))
            }) != 0)
        );
        (*(s)).tail =
            (((72340172838076673_u64 as u64).wrapping_mul(((((24) + (1)) as u8) as u64))) as i64);
        assert!(
            ((unsafe {
                let _p: *const u8 = (fb).cast_const();
                let _v: u8 = (((24) + (1)) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i64>(), _v)
            }) != 0)
        );
        assert!(
            ((unsafe {
                let _p: *const u8 = (sb).cast_const();
                let _v: u8 = (((24) + (1)) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i64>(), _v)
            }) != 0)
        );
        (unsafe {
            let _p: *mut u8 = sb;
            let _v: u8 = (((24) + (2)) as u8);
            set_bytes_0(_p, ::std::mem::size_of::<i64>(), _v)
        });
        assert!(
            (((((*(s)).tail)
                == (((72340172838076673_u64 as u64).wrapping_mul(((((24) + (2)) as u8) as u64)))
                    as i64)) as i32)
                != 0)
        );
        assert!(
            ((unsafe {
                let _p: *const u8 = (fb).cast_const();
                let _v: u8 = (((24) + (2)) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i64>(), _v)
            }) != 0)
        );
        (*(s)).tail =
            (((72340172838076673_u64 as u64).wrapping_mul(((((24) + (3)) as u8) as u64))) as i64);
        assert!(
            ((unsafe {
                let _p: *const u8 = (sb).cast_const();
                let _v: u8 = (((24) + (3)) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i64>(), _v)
            }) != 0)
        );
        assert!(
            ((unsafe {
                let _p: *const u8 = (fb).cast_const();
                let _v: u8 = (((24) + (3)) as u8);
                all_bytes_1(_p, ::std::mem::size_of::<i64>(), _v)
            }) != 0)
        );
    }
    (unsafe { check_final_2(s) });
    (unsafe {
        set_bytes_0(
            ((&mut (*s).in_ as *mut In) as *mut u8),
            ::std::mem::size_of::<In>(),
            33_u8,
        )
    });
    assert!(
        ((((((((*s).in_.a as i32)
            == ((((72340172838076673_u64 as u64).wrapping_mul((((33) as u8) as u64))) as i16)
                as i32)) as i32)
            != 0)
            && (((((*s).in_.b)
                == (((72340172838076673_u64 as u64).wrapping_mul((((33) as u8) as u64))) as i32))
                as i32)
                != 0)) as i32)
            != 0)
    );
    let mut in_: In = In {
        a: (((72340172838076673_u64 as u64).wrapping_mul((((35) as u8) as u64))) as i16),
        b: (((72340172838076673_u64 as u64).wrapping_mul((((51) as u8) as u64))) as i32),
    };
    (*s).in_ = in_;
    assert!(
        ((unsafe {
            all_bytes_1(
                ((&mut (*s).in_.a as *mut i16) as *mut u8).cast_const(),
                ::std::mem::size_of::<i16>(),
                35_u8,
            )
        }) != 0)
    );
    assert!(
        ((unsafe {
            all_bytes_1(
                ((&mut (*s).in_.b as *mut i32) as *mut u8).cast_const(),
                ::std::mem::size_of::<i32>(),
                51_u8,
            )
        }) != 0)
    );
    (unsafe {
        let _p: *mut u8 = ((*s).arr.as_mut_ptr() as *mut u8);
        let _n: usize = ::std::mem::size_of::<[i32; 3]>();
        set_bytes_0(_p, _n, 133_u8)
    });
    assert!(
        ((((((((*s).arr[(0) as usize])
            == (((72340172838076673_u64 as u64).wrapping_mul((((133) as u8) as u64))) as i32))
            as i32)
            != 0)
            && (((((*s).arr[(2) as usize])
                == (((72340172838076673_u64 as u64).wrapping_mul((((133) as u8) as u64))) as i32))
                as i32)
                != 0)) as i32)
            != 0)
    );
    (unsafe {
        let _p: *mut u8 = ((&mut (*s).arr as *mut [i32; 3]) as *mut u8);
        let _n: usize = ::std::mem::size_of::<[i32; 3]>();
        set_bytes_0(_p, _n, 134_u8)
    });
    assert!(
        (((((*s).arr[(1) as usize])
            == (((72340172838076673_u64 as u64).wrapping_mul((((134) as u8) as u64))) as i32))
            as i32)
            != 0)
    );
    (*s).arr[(0) as usize] =
        (((72340172838076673_u64 as u64).wrapping_mul((((131) as u8) as u64))) as i32);
    (*s).arr[(1) as usize] =
        (((72340172838076673_u64 as u64).wrapping_mul((((147) as u8) as u64))) as i32);
    (*s).arr[(2) as usize] =
        (((72340172838076673_u64 as u64).wrapping_mul((((11) as u8) as u64))) as i32);
    let mut ib: *mut i32 = ((*s).bytes.as_mut_ptr() as *mut i32);
    (*ib) = (((72340172838076673_u64 as u64).wrapping_mul((((69) as u8) as u64))) as i32);
    assert!(
        ((unsafe {
            let _p: *const u8 = ((*s).bytes.as_mut_ptr()).cast_const();
            let _n: usize = ::std::mem::size_of::<[u8; 4]>();
            all_bytes_1(_p, _n, 69_u8)
        }) != 0)
    );
    (*s).bytes[(0) as usize] = 67_u8;
    (*s).bytes[(1) as usize] = 83_u8;
    (*s).bytes[(2) as usize] = 99_u8;
    (*s).bytes[(3) as usize] = 115_u8;
    assert!(
        ((((*ib) == (*(((&mut (*s).bytes as *mut [u8; 4]) as *mut u8) as *mut i32))) as i32) != 0)
    );
    let mut v: *mut ::libc::c_void = ((&mut (*s).in_.b as *mut i32) as *mut ::libc::c_void);
    (*(v as *mut i32)) = 11;
    assert!((((((*s).in_.b) == (11)) as i32) != 0));
    let mut twelve: i32 = 12;
    {
        if ::std::mem::size_of::<i32>() != 0 {
            ::std::ptr::copy_nonoverlapping(
                ((&mut twelve as *mut i32) as *const ::libc::c_void),
                ((&mut (*s).x as *mut i32) as *mut ::libc::c_void),
                ::std::mem::size_of::<i32>() as usize,
            )
        }
        ((&mut (*s).x as *mut i32) as *mut ::libc::c_void)
    };
    assert!((((((*s).x) == (12)) as i32) != 0));
    {
        let byte_0 = ((&mut (*s).in_.b as *mut i32) as *mut ::libc::c_void) as *mut u8;
        for offset in 0..::std::mem::size_of::<i32>() {
            *byte_0.offset(offset as isize) = 51 as u8;
        }
        ((&mut (*s).in_.b as *mut i32) as *mut ::libc::c_void)
    };
    {
        let byte_0 = ((&mut (*s).x as *mut i32) as *mut ::libc::c_void) as *mut u8;
        for offset in 0..::std::mem::size_of::<i32>() {
            *byte_0.offset(offset as isize) = 19 as u8;
        }
        ((&mut (*s).x as *mut i32) as *mut ::libc::c_void)
    };
    (unsafe { check_final_2(s) });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut local: S = <S>::default();
    {
        let byte_0 = ((&mut local as *mut S) as *mut ::libc::c_void) as *mut u8;
        for offset in 0..::std::mem::size_of::<S>() {
            *byte_0.offset(offset as isize) = 0 as u8;
        }
        ((&mut local as *mut S) as *mut ::libc::c_void)
    };
    (unsafe { check_struct_3((&mut local as *mut S)) });
    let mut arr: [S; 2] = [<S>::default(); 2];
    {
        let byte_0 = (arr.as_mut_ptr() as *mut ::libc::c_void) as *mut u8;
        for offset in 0..::std::mem::size_of::<[S; 2]>() {
            *byte_0.offset(offset as isize) = 0 as u8;
        }
        (arr.as_mut_ptr() as *mut ::libc::c_void)
    };
    (unsafe { check_struct_3((&mut arr[(1) as usize] as *mut S)) });
    assert!(
        ((unsafe {
            all_bytes_1(
                ((&mut arr[(0) as usize] as *mut S) as *mut S as *mut u8).cast_const(),
                ::std::mem::size_of::<S>(),
                0_u8,
            )
        }) != 0)
    );
    let mut heap: *mut S = (libcc2rs::calloc_unsafe(1_usize, ::std::mem::size_of::<S>()) as *mut S);
    assert!((((!((heap).is_null())) as i32) != 0));
    (unsafe { check_struct_3(heap) });
    libcc2rs::free_unsafe((heap as *mut ::libc::c_void));
    let mut raw_: *mut u8 = (libcc2rs::malloc_unsafe(::std::mem::size_of::<S>()) as *mut u8);
    assert!((((!((raw_).is_null())) as i32) != 0));
    {
        let byte_0 = (raw_ as *mut ::libc::c_void) as *mut u8;
        for offset in 0..::std::mem::size_of::<S>() {
            *byte_0.offset(offset as isize) = 0 as u8;
        }
        (raw_ as *mut ::libc::c_void)
    };
    let mut view: *mut S = (raw_ as *mut S);
    (unsafe { check_struct_3(view) });
    assert!(
        (((((*raw_.offset((::std::mem::offset_of!(S, tail)) as isize)) as i32) == (27)) as i32)
            != 0)
    );
    (unsafe {
        let _p: *mut u8 = raw_.offset((::std::mem::offset_of!(S, arr)) as isize);
        set_bytes_0(_p, ::std::mem::size_of::<i32>(), 119_u8)
    });
    assert!(
        (((((*view).arr[(0) as usize])
            == (((72340172838076673_u64 as u64).wrapping_mul((((119) as u8) as u64))) as i32))
            as i32)
            != 0)
    );
    (*view).bytes[(3) as usize] = 17_u8;
    assert!(
        (((((*raw_
            .offset(((::std::mem::offset_of!(S, bytes) as usize).wrapping_add(3_usize)) as isize))
            as i32)
            == (17)) as i32)
            != 0)
    );
    libcc2rs::free_unsafe((raw_ as *mut ::libc::c_void));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
