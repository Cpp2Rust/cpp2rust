// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn t1() -> Ptr<libcc2rs::CDir> {
    Ptr::null()
}

fn t2() -> libcc2rs::Dirent {
    Default::default()
}

fn f1(a0: Ptr<i8>) -> Ptr<CDir> {
    match nix::dir::Dir::open(
        a0.to_rust_string().as_str(),
        nix::fcntl::OFlag::O_RDONLY,
        nix::sys::stat::Mode::empty(),
    ) {
        Ok(__dir) => Ptr::alloc(CDir::from_dir(__dir)),
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            Ptr::null()
        }
    }
}

fn f2(a0: Ptr<CDir>) -> Ptr<Dirent> {
    a0.with(|__d| {
        let __i = __d.pos.get();
        if __i >= __d.entries.len() {
            Ptr::null()
        } else {
            __d.pos.set(__i + 1);
            let __e = &__d.entries[__i];
            Ptr::alloc(Dirent::from_entry(__e.0, &__e.1, __e.2))
        }
    })
}

fn f3(a0: Ptr<CDir>) -> i32 {
    a0.delete();
    0
}
