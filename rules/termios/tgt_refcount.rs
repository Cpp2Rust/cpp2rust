// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn t1() -> libcc2rs::Termios {
    Default::default()
}

fn t2() -> libcc2rs::Winsize {
    Default::default()
}

fn f1(a0: i32, a1: i32, a2: Ptr<Termios>) -> i32 {
    let __action = match a1 {
        0 => nix::sys::termios::SetArg::TCSANOW,
        1 => nix::sys::termios::SetArg::TCSADRAIN,
        2 => nix::sys::termios::SetArg::TCSAFLUSH,
        __a => panic!("tcsetattr: unsupported action {__a}"),
    };
    let __t = nix::sys::termios::Termios::from(a2.with(|__src| __src.to_libc()));
    match FdRegistry::with_fd(a0, |__fd| {
        nix::sys::termios::tcsetattr(__fd, __action, &__t)
    }) {
        Ok(()) => 0,
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    }
}

fn f2(a0: i32, a1: Ptr<Termios>) -> i32 {
    match FdRegistry::with_fd(a0, |__fd| nix::sys::termios::tcgetattr(__fd)) {
        Ok(__t) => {
            a1.with_mut(|__dst| *__dst = Termios::from_libc(&__t.into()));
            0
        }
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    }
}
