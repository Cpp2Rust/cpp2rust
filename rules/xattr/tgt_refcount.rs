// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

#[cfg(target_os = "linux")]
fn f1(a0: i32, a1: Ptr<u8>, a2: AnyPtr, a3: usize, a4: i32) -> i32 {
    match a4 {
        0 => {}
        __f => panic!("fsetxattr: unsupported flags {}", __f),
    }
    let __name = a1.to_rust_string();
    match FdRegistry::with_fd(a0, |__fd| nix::unistd::dup(__fd)) {
        Ok(__dup) => {
            let __file = ::std::fs::File::from(__dup);
            match a2
                .reinterpret_cast::<u8>()
                .with_slice(a3, |__v| xattr::FileExt::set_xattr(&__file, &__name, __v))
            {
                Ok(()) => 0,
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e.raw_os_error().unwrap_or(::libc::EIO));
                    -1
                }
            }
        }
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    }
}
