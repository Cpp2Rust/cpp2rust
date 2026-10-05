// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

fn t3() -> ::std::fs::File {
    ::std::fs::File::open("").unwrap()
}

unsafe fn f2(a0: ::std::fs::File) -> ::std::fs::File {
    a0.try_clone().unwrap()
}

unsafe fn f3(a0: &mut ::std::fs::File) -> &mut ::std::fs::File {
    a0
}
