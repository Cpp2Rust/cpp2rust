// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

fn t1() -> libcc2rs::CppStream {
    ::std::fs::File::open("")
        .map(libcc2rs::CppStream::from)
        .unwrap()
}

fn t2() -> libcc2rs::CppStream {
    ::std::fs::File::open("")
        .map(libcc2rs::CppStream::from)
        .unwrap()
}

fn t3<T1>() -> libcc2rs::CppStream {
    ::std::fs::File::open("")
        .map(libcc2rs::CppStream::from)
        .unwrap()
}

fn t4() -> libcc2rs::CppStream {
    ::std::fs::File::open("")
        .map(libcc2rs::CppStream::from)
        .unwrap()
}

unsafe fn f1(a0: *const libc::c_char) -> libcc2rs::CppStream {
    ::std::fs::File::create(::std::ffi::CStr::from_ptr(a0).to_str().unwrap())
        .map(libcc2rs::CppStream::from)
        .unwrap()
}

unsafe fn f2(a0: libcc2rs::CppStream) -> libcc2rs::CppStream {
    a0.try_clone().unwrap()
}
unsafe fn f3(a0: &mut libcc2rs::CppStream) -> &mut libcc2rs::CppStream {
    a0
}
unsafe fn f4(a0: libcc2rs::CppStream) -> libcc2rs::CppStream {
    a0
}
unsafe fn f5(a0: *const libc::c_char) -> libcc2rs::CppStream {
    ::std::fs::File::open(::std::ffi::CStr::from_ptr(a0).to_str().unwrap())
        .map(libcc2rs::CppStream::from)
        .unwrap()
}

unsafe fn f6(a0: libcc2rs::CppStream) -> libcc2rs::CppStream {
    a0.try_clone().unwrap()
}

unsafe fn f7(a0: libcc2rs::CppStream) -> libcc2rs::CppStream {
    a0.try_clone().unwrap()
}

unsafe fn f8(a0: libcc2rs::CppStream) -> libcc2rs::CppStream {
    a0
}
