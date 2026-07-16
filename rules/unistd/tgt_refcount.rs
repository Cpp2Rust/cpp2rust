// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn f4(a0: Ptr<u8>) -> i32 {
    match nix::unistd::unlink(
        ::std::ffi::CString::new(a0.to_c_string_iterator().collect::<Vec<u8>>())
            .unwrap()
            .as_c_str(),
    ) {
        Ok(()) => 0,
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    }
}

fn f8() -> u32 {
    nix::unistd::geteuid().as_raw()
}

fn f9(a0: Ptr<u8>, a1: usize) -> i32 {
    match nix::unistd::gethostname() {
        Ok(__name) => {
            let __bytes = __name.as_encoded_bytes();
            let __n = __bytes.len().min(a1.saturating_sub(1));
            let mut __dst = a0.clone();
            for __b in &__bytes[..__n] {
                __dst.write(*__b);
                __dst += 1;
            }
            if a1 > 0 {
                __dst.write(0);
            }
            0
        }
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    }
}

fn f11(a0: Ptr<u8>) -> i32 {
    match ::std::fs::remove_dir(a0.to_rust_string()) {
        Ok(()) => 0,
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e.raw_os_error().unwrap_or(::libc::EIO));
            -1
        }
    }
}
