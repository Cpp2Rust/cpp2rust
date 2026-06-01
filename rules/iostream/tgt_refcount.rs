// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;
use std::os::fd::AsFd;

// TODO: t2 and t3 should be translated to Ptr<dyn Traits>
fn t2() -> Ptr<std::fs::File> {
    Ptr::null()
}

fn t3() -> Ptr<std::fs::File> {
    Ptr::null()
}

fn f1() -> ::std::fs::File {
    std::fs::File::from(std::io::stdout().as_fd().try_clone_to_owned().unwrap())
}

fn f2() -> ::std::fs::File {
    std::fs::File::from(std::io::stderr().as_fd().try_clone_to_owned().unwrap())
}

fn f3() -> Ptr<::std::fs::File> {
    libcc2rs::cout()
}

fn f4() -> Ptr<::std::fs::File> {
    libcc2rs::cerr()
}
