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
        .to_vec(),
    );
}
pub unsafe fn shrink_through_ptr_2(mut comps: *mut Vec<Chunk>) {
    (*comps).shrink_to_fit();
}
pub unsafe fn nested_push_move_3(mut bw: *mut Writer) {
    (*(*bw).output).push(std::mem::take(&mut (*bw).chunk));
}
pub unsafe fn emplace_local_from_field_4(mut jpg: *mut JPEGData, mut cond: bool) {
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
        .to_vec(),
    );
}
pub unsafe fn nested_emplace_move_5(mut bw: *mut Writer) {
    (*(*bw).output).push(std::mem::take(&mut (*bw).chunk));
}
pub unsafe fn self_ref_push_6(mut comps: *mut Vec<Chunk>) {
    {
        let a0_clone = (*((*comps).first_mut().unwrap())).clone();
        (*comps).push(a0_clone)
    };
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut vecs: Vec<Vec<u8>> = Vec::new();
    (unsafe {
        let _dest: *mut Vec<Vec<u8>> = (&mut vecs as *mut Vec<Vec<u8>>);
        push_param_0(_dest)
    });
    assert!(((vecs.len() as u64) == (1_u64)));
    assert!(vecs[(0_u64) as usize].is_empty());
    let mut jpg: JPEGData = <JPEGData>::default();
    (unsafe {
        let _jpg: *mut JPEGData = (&mut jpg as *mut JPEGData);
        let _cond: bool = true;
        push_local_from_field_1(_jpg, _cond)
    });
    assert!(((jpg.com_data.len() as u64) == (1_u64)));
    assert!(((jpg.com_data[(0_u64) as usize].len() as u64) == (3_u64)));
    assert!(((jpg.com_data[(0_u64) as usize][(0_u64) as usize] as i32) == (1)));
    assert!(((jpg.com_data[(0_u64) as usize][(1_u64) as usize] as i32) == (2)));
    assert!(((jpg.com_data[(0_u64) as usize][(2_u64) as usize] as i32) == (3)));
    assert!(jpg.app_data.is_empty());
    let mut chunks: Vec<Chunk> = Vec::new();
    (unsafe {
        let _comps: *mut Vec<Chunk> = (&mut chunks as *mut Vec<Chunk>);
        shrink_through_ptr_2(_comps)
    });
    assert!(chunks.is_empty());
    let mut w: Writer = <Writer>::default();
    w.chunk.data = 42;
    w.output = (&mut chunks as *mut Vec<Chunk>);
    (unsafe {
        let _bw: *mut Writer = (&mut w as *mut Writer);
        nested_push_move_3(_bw)
    });
    assert!(((chunks.len() as u64) == (1_u64)));
    assert!(((chunks[(0_u64) as usize].data) == (42)));
    (unsafe {
        let _jpg: *mut JPEGData = (&mut jpg as *mut JPEGData);
        let _cond: bool = false;
        emplace_local_from_field_4(_jpg, _cond)
    });
    assert!(((jpg.app_data.len() as u64) == (1_u64)));
    assert!(((jpg.app_data[(0_u64) as usize].len() as u64) == (3_u64)));
    assert!(((jpg.app_data[(0_u64) as usize][(0_u64) as usize] as i32) == (1)));
    assert!(((jpg.app_data[(0_u64) as usize][(2_u64) as usize] as i32) == (3)));
    assert!(((jpg.com_data.len() as u64) == (1_u64)));
    w.chunk.data = 99;
    w.output = (&mut chunks as *mut Vec<Chunk>);
    (unsafe {
        let _bw: *mut Writer = (&mut w as *mut Writer);
        nested_emplace_move_5(_bw)
    });
    assert!(((chunks.len() as u64) == (2_u64)));
    assert!(((chunks[(1_u64) as usize].data) == (99)));
    (unsafe {
        let _comps: *mut Vec<Chunk> = (&mut chunks as *mut Vec<Chunk>);
        self_ref_push_6(_comps)
    });
    assert!(((chunks.len() as u64) == (3_u64)));
    assert!(((chunks[(2_u64) as usize].data) == (42)));
    return 0;
}
