// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn t1() -> libcc2rs::Passwd {
    Default::default()
}

fn f2(a0: u32, a1: Ptr<Passwd>, a2: Ptr<i8>, a3: usize, a4: Ptr<Ptr<Passwd>>) -> i32 {
    match nix::unistd::User::from_uid(nix::unistd::Uid::from_raw(a0)) {
        Ok(Some(__u)) => {
            let __strs: [Vec<u8>; 5] = [
                __u.name.as_bytes().to_vec(),
                __u.passwd.as_bytes().to_vec(),
                __u.gecos.as_bytes().to_vec(),
                __u.dir.as_os_str().as_encoded_bytes().to_vec(),
                __u.shell.as_os_str().as_encoded_bytes().to_vec(),
            ];
            let __needed: usize = __strs.iter().map(|__s| __s.len() + 1).sum();
            if __needed > a3 {
                a4.write(Ptr::null());
                ::libc::ERANGE
            } else {
                let mut __ptrs: Vec<Ptr<i8>> = Vec::new();
                let mut __off: usize = 0;
                for __s in &__strs {
                    __ptrs.push(a2.offset(__off));
                    let __end = __s.len();
                    a2.offset(__off).write_c_bytes(__s);
                    elem!(a2, __off + __end).write(0);
                    __off += __end + 1;
                }
                a1.with_mut(|__pw| *__pw = Passwd::from_user_in(&__u, &__ptrs));
                a4.write(a1.clone());
                0
            }
        }
        Ok(None) => {
            a4.write(Ptr::null());
            0
        }
        Err(__e) => {
            a4.write(Ptr::null());
            __e as i32
        }
    }
}

fn f1(a0: u32) -> Ptr<Passwd> {
    match nix::unistd::User::from_uid(nix::unistd::Uid::from_raw(a0)) {
        Ok(Some(__u)) => Ptr::alloc(Passwd::from_user(&__u)),
        Ok(None) => Ptr::null(),
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            Ptr::null()
        }
    }
}
