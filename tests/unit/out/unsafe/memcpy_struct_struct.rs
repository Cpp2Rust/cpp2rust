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
pub struct Entry {
    pub bits: u8,
    pub value: u16,
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut table: [Entry; 8] = [
        Entry {
            bits: 1_u8,
            value: 4369_u16,
        },
        Entry {
            bits: 2_u8,
            value: 8738_u16,
        },
        Entry {
            bits: 3_u8,
            value: 13107_u16,
        },
        Entry {
            bits: 4_u8,
            value: 17476_u16,
        },
        Entry {
            bits: 0_u8,
            value: 0_u16,
        },
        Entry {
            bits: 0_u8,
            value: 0_u16,
        },
        Entry {
            bits: 0_u8,
            value: 0_u16,
        },
        Entry {
            bits: 0_u8,
            value: 0_u16,
        },
    ];
    let mut table_size: usize = 4_usize;
    {
        if (((table_size as u64).wrapping_mul(::std::mem::size_of::<Entry>() as u64)) as usize) != 0
        {
            ::std::ptr::copy_nonoverlapping(
                ((&mut table[(0) as usize] as *mut Entry) as *const Entry as *const ::libc::c_void),
                ((&mut table[(table_size) as usize] as *mut Entry) as *mut Entry
                    as *mut ::libc::c_void),
                (((table_size as u64).wrapping_mul(::std::mem::size_of::<Entry>() as u64)) as usize)
                    as usize,
            )
        }
        ((&mut table[(table_size) as usize] as *mut Entry) as *mut Entry as *mut ::libc::c_void)
    };
    assert!(
        ((table[(4) as usize].bits as i32) == (1))
            && ((table[(4) as usize].value as i32) == (4369))
    );
    assert!(
        ((table[(5) as usize].bits as i32) == (2))
            && ((table[(5) as usize].value as i32) == (8738))
    );
    assert!(
        ((table[(6) as usize].bits as i32) == (3))
            && ((table[(6) as usize].value as i32) == (13107))
    );
    assert!(
        ((table[(7) as usize].bits as i32) == (4))
            && ((table[(7) as usize].value as i32) == (17476))
    );
    return 0;
}
