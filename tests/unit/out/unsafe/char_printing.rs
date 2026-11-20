extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut made_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub unsafe fn make_1(mut s: *const libc::c_char) -> Vec<libc::c_char> {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut made_0)).prefix_inc();
    return {
        let mut __tmp_0 = { s };
        std::slice::from_raw_parts(
            __tmp_0,
            (0..).take_while(|&i| *__tmp_0.add(i) != 0).count() + 1,
        )
        .to_vec()
    };
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut vec_: Vec<u8> = Vec::from([195_u8, 167_u8]);
    let mut i: i32 = 27;
    let mut str: Vec<libc::c_char> = std::slice::from_raw_parts(
        c"foo.".as_ptr(),
        (0..).take_while(|&i| *c"foo.".as_ptr().add(i) != 0).count() + 1,
    )
    .to_vec();
    write!((*libcc2rs::cout_unsafe()), "{:} a", i,);
    (*libcc2rs::cout_unsafe()).write_all(
        &([
            (&[vec_[(0_usize)] as u8] as &[u8]),
            (&[vec_[(1_usize)] as u8] as &[u8]),
            (&[('o' as libc::c_char) as u8] as &[u8]),
            (&(str)
                .split_last()
                .unwrap()
                .1
                .iter()
                .map(|&c| c as u8)
                .collect::<Vec<u8>>() as &[u8]),
            (b"\n" as &[u8]),
        ]
        .concat()),
    );
    write!((*libcc2rs::cout_unsafe()), "0x{:x}", 27,);
    (*libcc2rs::cout_unsafe()).write_all(
        &([
            (b" a\xc3\xa7ordas?" as &[u8]),
            (&[('\n' as libc::c_char) as u8] as &[u8]),
            (b"Sim, 0x" as &[u8]),
        ]
        .concat()),
    );
    write!((*libcc2rs::cout_unsafe()), "{:x}.\n", i,);
    write!((*libcc2rs::cout_unsafe()), "Hello, World!\n",);
    (*libcc2rs::cout_unsafe()).write_all(
        &([
            (&[vec_[(0_usize)] as u8] as &[u8]),
            (&[('\n' as libc::c_char) as u8] as &[u8]),
            (&[vec_[(1_usize)] as u8] as &[u8]),
            (&[('\n' as libc::c_char) as u8] as &[u8]),
        ]
        .concat()),
    );
    (*libcc2rs::cout_unsafe()).write_all(
        &([
            (&(unsafe { make_1(c"once".as_ptr()) })
                .split_last()
                .unwrap()
                .1
                .iter()
                .map(|&c| c as u8)
                .collect::<Vec<u8>>() as &[u8]),
            (&[('\n' as libc::c_char) as u8] as &[u8]),
        ]
        .concat()),
    );
    (*libcc2rs::cout_unsafe()).write_all(
        &(unsafe { make_1(c"alone".as_ptr()) })
            .split_last()
            .unwrap()
            .1
            .iter()
            .map(|&c| c as u8)
            .collect::<Vec<u8>>(),
    );
    write!((*libcc2rs::cout_unsafe()), "\n",);
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut made_0)) == (2)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const made_0);
}
