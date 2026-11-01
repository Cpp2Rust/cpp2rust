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
    let mut fd: i32 = 0;
    'loop_: while (((fd < 3) as i32) != 0) {
        assert!((((FdRegistry::close(fd) == 0) as i32) != 0));
        let mut reopened: i32 = {
            let __mode = match Box::<[VaArg]>::from([]).first() {
                Some(__m) => {
                    nix::sys::stat::Mode::from_bits_truncate(i32::get(__m) as ::libc::mode_t)
                }
                None => nix::sys::stat::Mode::empty(),
            };
            match nix::fcntl::open(
                Ptr::<i8>::from_string_literal(b"/dev/zero")
                    .to_rust_string()
                    .as_str(),
                nix::fcntl::OFlag::from_bits_retain(::libc::O_RDONLY),
                __mode,
            ) {
                Ok(__ofd) => FdRegistry::register(__ofd),
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            }
        };
        assert!((((reopened == fd) as i32) != 0));
        let buf: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([1_u8, 127_u8, 255_u8])));
        assert!(
            ((((match ({ ((buf.as_pointer() as Ptr::<u8>) as Ptr<u8>).to_any() },) {
                (mut __tmp_0,) => {
                    match FdRegistry::with_fd(reopened, |__fd| {
                        __tmp_0
                            .reinterpret_cast::<u8>()
                            .with_slice_mut(::std::mem::size_of::<[u8; 3]>(), |__buf| {
                                nix::unistd::read(__fd, __buf)
                            })
                    }) {
                        Ok(__n) => __n as isize,
                        Err(__e) => {
                            libcc2rs::cpp2rust_errno().write(__e as i32);
                            -1
                        }
                    }
                }
            } as usize)
                == ::std::mem::size_of::<[u8; 3]>()) as i32)
                != 0)
        );
        assert!(
            (((((((((((*buf.borrow())[(0) as usize] as i32) == 0) as i32) != 0)
                && (((((*buf.borrow())[(1) as usize] as i32) == 0) as i32) != 0))
                as i32)
                != 0)
                && (((((*buf.borrow())[(2) as usize] as i32) == 0) as i32) != 0))
                as i32)
                != 0)
        );
        fd.prefix_inc();
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
