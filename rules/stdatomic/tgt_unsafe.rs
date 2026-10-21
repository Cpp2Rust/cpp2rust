// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

unsafe fn f1(a0: *mut ::std::sync::atomic::AtomicBool) {
    (*a0).store(false, ::std::sync::atomic::Ordering::SeqCst)
}
