// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn t3() -> Ptr<BrotliDecoderState> {
    Ptr::null()
}

fn t4() -> Ptr<BrotliDecoderState> {
    Ptr::null()
}

fn t6() -> BrotliDecoderState {
    Default::default()
}

fn t7() -> BrotliDecoderState {
    Default::default()
}

fn f2(
    a0: libc::c_int,
    a1: libc::c_int,
    a2: ::brotli_sys::BrotliEncoderMode,
    a3: usize,
    a4: Ptr<u8>,
    a5: Ptr<usize>,
    a6: Ptr<u8>,
) -> libc::c_int {
    BrotliEncoderCompress(a0, a1, a2, a3, a4, a5, a6)
}

fn f5(a0: usize, a1: Ptr<u8>, a2: Ptr<usize>, a3: Ptr<u8>) -> ::brotli_sys::BrotliDecoderResult {
    BrotliDecoderDecompress(a0, a1, a2, a3)
}

fn f6(
    a0: ::brotli_sys::brotli_alloc_func,
    a1: ::brotli_sys::brotli_free_func,
    a2: *mut std::ffi::c_void,
) -> Ptr<BrotliDecoderState> {
    Ptr::alloc(BrotliDecoderState::default())
}

fn f7(a0: Ptr<BrotliDecoderState>) {
    BrotliDecoderDestroyInstance(a0)
}

fn f8(
    a0: Ptr<BrotliDecoderState>,
    a1: Ptr<usize>,
    a2: Ptr<Ptr<u8>>,
    a3: Ptr<usize>,
    a4: Ptr<Ptr<u8>>,
    a5: Ptr<usize>,
) -> ::brotli_sys::BrotliDecoderResult {
    BrotliDecoderDecompressStream(a0, a1, a2, a3, a4, a5)
}

fn f9(a0: Ptr<BrotliDecoderState>, a1: Ptr<usize>) -> Ptr<u8> {
    BrotliDecoderTakeOutput(a0, a1)
}

fn f35(a0: Ptr<BrotliDecoderState>) -> ::brotli_sys::BrotliDecoderErrorCode {
    a0.with(|__s| __s.state.error_code as i32)
}

fn f36() -> u32 {
    BrotliDecoderVersion()
}
