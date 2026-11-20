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
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let mut sibling: Ptr<i8> = Ptr::<i8>::from_string_literal(b"unlink-\xef\xbf\xbd.tmp");
    let mut fp: Ptr<CFile> = match CFile::open(
        &sibling.to_c_u8_bytes(),
        &Ptr::<i8>::from_string_literal(b"wb").to_c_u8_bytes(),
    ) {
        Some(__f) => Ptr::alloc(__f),
        None => Ptr::null(),
    };
    assert!((!((fp).is_null())));
    assert!(
        (match Ptr::<i8>::from_string_literal(b"sentinel")
            .with_c_bytes(|__bytes| fp.with_mut(|__f| __f.write(__bytes)) == __bytes.len())
        {
            true => 0,
            false => -1,
        } >= 0)
    );
    assert!(
        ({
            let __r = fp.with(|__f| __f.close());
            fp.delete();
            __r
        } == 0)
    );
    libcc2rs::cpp2rust_errno().write(0);
    assert!(
        (match Ptr::<i8>::from_string_literal(b"unlink-\xff.tmp")
            .with_c_bytes(|__path| nix::unistd::unlink(__path))
        {
            Ok(()) => 0,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == -1_i32)
    );
    assert!(((libcc2rs::cpp2rust_errno().read()) == libc::ENOENT));
    fp = match CFile::open(
        &sibling.to_c_u8_bytes(),
        &Ptr::<i8>::from_string_literal(b"rb").to_c_u8_bytes(),
    ) {
        Some(__f) => Ptr::alloc(__f),
        None => Ptr::null(),
    };
    assert!((!((fp).is_null())));
    assert!(
        ({
            let __r = fp.with(|__f| __f.close());
            fp.delete();
            __r
        } == 0)
    );
    fp = match CFile::open(
        &Ptr::<i8>::from_string_literal(b"unlink-\xff.tmp").to_c_u8_bytes(),
        &Ptr::<i8>::from_string_literal(b"wb").to_c_u8_bytes(),
    ) {
        Some(__f) => Ptr::alloc(__f),
        None => Ptr::null(),
    };
    assert!((!((fp).is_null())));
    assert!(
        ({
            let __r = fp.with(|__f| __f.close());
            fp.delete();
            __r
        } == 0)
    );
    let st: Value<libcc2rs::Stat> = Rc::new(RefCell::new(Default::default()));
    assert!(
        ({
            (({
                let mut __tmp_0 = { (st.as_pointer()) };
                match Ptr::<i8>::from_string_literal(b"unlink-\xff.tmp")
                    .with_c_bytes(nix::sys::stat::stat)
                {
                    Ok(__s) => {
                        __tmp_0.with_mut(|__st| *__st = Stat::from_libc(&__s));
                        0
                    }
                    Err(__e) => {
                        libcc2rs::cpp2rust_errno().write(__e as i32);
                        -1
                    }
                }
            } == 0)
                && ({ (*st.borrow()).st_size } == (0 as ::libc::off_t)))
        })
    );
    assert!(
        ({
            (({
                let mut __tmp_1 = { (st.as_pointer()) };
                match sibling.with_c_bytes(nix::sys::stat::stat) {
                    Ok(__s) => {
                        __tmp_1.with_mut(|__st| *__st = Stat::from_libc(&__s));
                        0
                    }
                    Err(__e) => {
                        libcc2rs::cpp2rust_errno().write(__e as i32);
                        -1
                    }
                }
            } == 0)
                && ({ (*st.borrow()).st_size } == (8 as ::libc::off_t)))
        })
    );
    assert!(
        (match Ptr::<i8>::from_string_literal(b"unlink-\xff.tmp")
            .with_c_os_str(|__from| Ptr::<i8>::from_string_literal(b"unlink-\xfe.tmp")
                .with_c_os_str(|__to| ::std::fs::rename(__from, __to)))
        {
            Ok(()) => 0,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e.raw_os_error().unwrap_or(::libc::EIO));
                -1
            }
        } == 0)
    );
    let mut fd: i32 = {
        let __mode = match Box::<[VaArg]>::from([]).first() {
            Some(__m) => nix::sys::stat::Mode::from_bits_truncate(i32::get(__m) as ::libc::mode_t),
            None => nix::sys::stat::Mode::empty(),
        };
        match Ptr::<i8>::from_string_literal(b"unlink-\xfe.tmp").with_c_bytes(|__path| {
            nix::fcntl::open(
                __path,
                nix::fcntl::OFlag::from_bits_retain(::libc::O_RDONLY),
                __mode,
            )
        }) {
            Ok(__ofd) => FdRegistry::register(__ofd),
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        }
    };
    assert!((fd >= 0));
    assert!((FdRegistry::close(fd) == 0));
    let mut path: Ptr<i8> = match Ptr::<i8>::from_string_literal(b"unlink-\xfe.tmp")
        .with_c_os_str(|__path| ::std::fs::canonicalize(__path))
    {
        Ok(__p) => {
            let mut __bytes = __p.into_os_string().into_encoded_bytes();
            __bytes.push(0);
            if Ptr::<i8>::null().is_null() {
                Ptr::alloc_array(CChar::from_byte_vec(__bytes).into_boxed_slice())
            } else {
                Ptr::<i8>::null().write_c_bytes(&__bytes);
                Ptr::<i8>::null()
            }
        }
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e.raw_os_error().unwrap_or(::libc::EIO));
            Ptr::null()
        }
    };
    assert!((!((path).is_null())));
    assert!(
        ({
            let mut __it1 = {
                let mut __tmp_2 = { (path).clone() };
                let __t = ('/' as i32) as i8;
                match __tmp_2
                    .to_c_string_iterator()
                    .enumerate()
                    .filter(|__e| __e.1 == __t)
                    .last()
                {
                    Some((__i, _)) => __tmp_2.offset(__i),
                    None => {
                        if __t == 0 {
                            __tmp_2.offset(__tmp_2.to_c_string_iterator().count())
                        } else {
                            Ptr::null()
                        }
                    }
                }
            }
            .to_c_string_iterator();
            let mut __it2 =
                Ptr::<i8>::from_string_literal(b"/unlink-\xfe.tmp").to_c_string_iterator();
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
    libcc2rs::free_refcount((path).to_any());
    assert!(
        (match Ptr::<i8>::from_string_literal(b"unlink-\xfe.tmp")
            .with_c_bytes(|__path| nix::unistd::unlink(__path))
        {
            Ok(()) => 0,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == 0)
    );
    assert!(
        ({
            (({
                let mut __tmp_3 = { (st.as_pointer()) };
                match sibling.with_c_bytes(nix::sys::stat::stat) {
                    Ok(__s) => {
                        __tmp_3.with_mut(|__st| *__st = Stat::from_libc(&__s));
                        0
                    }
                    Err(__e) => {
                        libcc2rs::cpp2rust_errno().write(__e as i32);
                        -1
                    }
                }
            } == 0)
                && ({ (*st.borrow()).st_size } == (8 as ::libc::off_t)))
        })
    );
    assert!(
        (match Ptr::<i8>::from_string_literal(b"unlink-\xfd").with_c_bytes(|__path| {
            nix::unistd::mkdir(__path, nix::sys::stat::Mode::from_bits_truncate(493_u32))
        }) {
            Ok(()) => 0,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == 0)
    );
    let mut dir: Ptr<libcc2rs::CDir> = match Ptr::<i8>::from_string_literal(b"unlink-\xfd")
        .with_c_bytes(|__path| {
            nix::dir::Dir::open(
                __path,
                nix::fcntl::OFlag::O_RDONLY,
                nix::sys::stat::Mode::empty(),
            )
        }) {
        Ok(__dir) => Ptr::alloc(CDir::from_dir(__dir)),
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            Ptr::null()
        }
    };
    assert!((!((dir).is_null())));
    assert!(
        ({
            dir.delete();
            0
        } == 0)
    );
    assert!(
        (match Ptr::<i8>::from_string_literal(b"unlink-\xfd")
            .with_c_os_str(|__path| ::std::fs::remove_dir(__path))
        {
            Ok(()) => 0,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e.raw_os_error().unwrap_or(::libc::EIO));
                -1
            }
        } == 0)
    );
    assert!(
        (match sibling.with_c_bytes(|__path| nix::unistd::unlink(__path)) {
            Ok(()) => 0,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == 0)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
