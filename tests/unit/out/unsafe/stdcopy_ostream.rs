extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut str: Vec<libc::c_char> = std::slice::from_raw_parts(
        c"Hello, world!\n".as_ptr(),
        (0..)
            .take_while(|&i| *c"Hello, world!\n".as_ptr().add(i) != 0)
            .count()
            + 1,
    )
    .to_vec();
    let file: [libc::c_char; 25] = std::mem::transmute(*b"test_stdcopy_ostream.txt\0");
    {
        let mut ofs: libcc2rs::CppStream =
            ::std::fs::File::create(::std::ffi::CStr::from_ptr(file.as_ptr()).to_str().unwrap())
                .map(libcc2rs::CppStream::from)
                .unwrap();
        match ({ UnsafeRandomAccessIterator::Vec(str.as_mut_ptr()) }, {
            UnsafeRandomAccessIterator::Vec(str.as_mut_ptr().add(str.len() - 1))
        }) {
            (mut __tmp_0, mut __tmp_1) => {
                let mut __it = (__tmp_0).clone();
                let __out = &mut ofs;
                let mut __bytes: Vec<u8> = Vec::new();
                while __it != __tmp_1 {
                    __bytes.push(*__it.iter_deref() as u8);
                    __it.iter_inc();
                }
                __out.write_all(&__bytes);
                __out.try_clone().unwrap()
            }
        };
    }
    let mut fd: i32 = (unsafe { libc::open(file.as_ptr() as *const i8, ::libc::O_RDONLY as i32) });
    let mut buf: [libc::c_char; 15] = [
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
    ];
    assert!(
        ((libc::read(
            fd,
            (buf.as_mut_ptr() as *mut ::libc::c_void),
            ::std::mem::size_of::<[libc::c_char; 15]>()
        ) as usize)
            == (str.len() - 1))
    );
    assert!(match ({ (buf.as_mut_ptr()).cast_const() },) {
        (mut __tmp_2,) => {
            str == {
                std::slice::from_raw_parts(
                    __tmp_2,
                    (0..).take_while(|&i| *__tmp_2.add(i) != 0).count() + 1,
                )
                .to_vec()
            }
        }
    });
    assert!(((libc::close(fd)) == (0)));
    libc::unlink(file.as_ptr());
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
