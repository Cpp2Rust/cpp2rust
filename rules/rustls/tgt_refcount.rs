// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn t4() -> libcc2rs::RustlsStr {
    Default::default()
}

fn f55() -> libcc2rs::RustlsStr {
    libcc2rs::rustls_version_refcount()
}
