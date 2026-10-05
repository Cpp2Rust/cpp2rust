// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

unsafe fn f7(a0: ::std::fs::File) -> ::std::fs::File {
    a0.try_clone().unwrap()
}

unsafe fn f8(a0: ::std::fs::File) -> ::std::fs::File {
    a0
}
