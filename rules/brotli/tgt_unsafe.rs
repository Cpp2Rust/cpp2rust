// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn t1() -> BrotliDecoderResult {
    BROTLI_DECODER_RESULT_ERROR
}

fn t2() -> BrotliEncoderMode {
    BROTLI_MODE_GENERIC
}

fn t3() -> *mut BrotliDecoderState {
    std::ptr::null_mut()
}

fn t4() -> *const BrotliDecoderState {
    std::ptr::null()
}

fn t5() -> BrotliDecoderErrorCode {
    BROTLI_DECODER_NO_ERROR
}

fn t6() -> BrotliDecoderState {
    Default::default()
}

fn t7() -> BrotliDecoderState {
    Default::default()
}

fn f1() -> BrotliEncoderMode {
    BROTLI_MODE_FONT
}
unsafe fn f2(
    a0: libc::c_int,
    a1: libc::c_int,
    a2: BrotliEncoderMode,
    a3: usize,
    a4: *mut u8,
    a5: *mut usize,
    a6: *mut u8,
) -> libc::c_int {
    BrotliEncoderCompress_unsafe(a0, a1, a2, a3, a4, a5, a6)
}
fn f3() -> BrotliEncoderMode {
    BROTLI_MODE_TEXT
}
fn f4() -> BrotliDecoderResult {
    BROTLI_DECODER_RESULT_SUCCESS
}
unsafe fn f5(a0: usize, a1: *mut u8, a2: *mut usize, a3: *mut u8) -> BrotliDecoderResult {
    BrotliDecoderDecompress_unsafe(a0, a1, a2, a3)
}

fn f6() -> *mut BrotliDecoderState {
    BrotliDecoderCreateInstance_unsafe()
}

unsafe fn f7(a0: *mut BrotliDecoderState) {
    BrotliDecoderDestroyInstance_unsafe(a0)
}

unsafe fn f8(
    a0: *mut BrotliDecoderState,
    a1: *mut usize,
    a2: *mut *const u8,
    a3: *mut usize,
    a4: *mut *mut u8,
    a5: *mut usize,
) -> BrotliDecoderResult {
    BrotliDecoderDecompressStream_unsafe(a0, a1, a2, a3, a4, a5)
}

unsafe fn f9(a0: *mut BrotliDecoderState, a1: *mut usize) -> *const u8 {
    BrotliDecoderTakeOutput_unsafe(a0, a1)
}

fn f10() -> BrotliDecoderResult {
    BROTLI_DECODER_RESULT_ERROR
}

fn f11() -> BrotliDecoderResult {
    BROTLI_DECODER_RESULT_NEEDS_MORE_OUTPUT
}

fn f12() -> BrotliEncoderMode {
    BROTLI_MODE_GENERIC
}

fn f13() -> BrotliDecoderResult {
    BROTLI_DECODER_RESULT_NEEDS_MORE_INPUT
}

fn f14() -> BrotliDecoderErrorCode {
    BROTLI_DECODER_ERROR_FORMAT_EXUBERANT_NIBBLE
}
fn f15() -> BrotliDecoderErrorCode {
    BROTLI_DECODER_ERROR_FORMAT_EXUBERANT_META_NIBBLE
}
fn f16() -> BrotliDecoderErrorCode {
    BROTLI_DECODER_ERROR_FORMAT_SIMPLE_HUFFMAN_ALPHABET
}
fn f17() -> BrotliDecoderErrorCode {
    BROTLI_DECODER_ERROR_FORMAT_SIMPLE_HUFFMAN_SAME
}
fn f18() -> BrotliDecoderErrorCode {
    BROTLI_DECODER_ERROR_FORMAT_CL_SPACE
}
fn f19() -> BrotliDecoderErrorCode {
    BROTLI_DECODER_ERROR_FORMAT_HUFFMAN_SPACE
}
fn f20() -> BrotliDecoderErrorCode {
    BROTLI_DECODER_ERROR_FORMAT_CONTEXT_MAP_REPEAT
}
fn f21() -> BrotliDecoderErrorCode {
    BROTLI_DECODER_ERROR_FORMAT_BLOCK_LENGTH_1
}
fn f22() -> BrotliDecoderErrorCode {
    BROTLI_DECODER_ERROR_FORMAT_BLOCK_LENGTH_2
}
fn f23() -> BrotliDecoderErrorCode {
    BROTLI_DECODER_ERROR_FORMAT_TRANSFORM
}
fn f24() -> BrotliDecoderErrorCode {
    BROTLI_DECODER_ERROR_FORMAT_DICTIONARY
}
fn f25() -> BrotliDecoderErrorCode {
    BROTLI_DECODER_ERROR_FORMAT_WINDOW_BITS
}
fn f26() -> BrotliDecoderErrorCode {
    BROTLI_DECODER_ERROR_FORMAT_PADDING_1
}
fn f27() -> BrotliDecoderErrorCode {
    BROTLI_DECODER_ERROR_FORMAT_PADDING_2
}
fn f28() -> BrotliDecoderErrorCode {
    BROTLI_DECODER_ERROR_INVALID_ARGUMENTS
}
fn f29() -> BrotliDecoderErrorCode {
    BROTLI_DECODER_ERROR_ALLOC_CONTEXT_MODES
}
fn f30() -> BrotliDecoderErrorCode {
    BROTLI_DECODER_ERROR_ALLOC_TREE_GROUPS
}
fn f31() -> BrotliDecoderErrorCode {
    BROTLI_DECODER_ERROR_ALLOC_CONTEXT_MAP
}
fn f32() -> BrotliDecoderErrorCode {
    BROTLI_DECODER_ERROR_ALLOC_RING_BUFFER_1
}
fn f33() -> BrotliDecoderErrorCode {
    BROTLI_DECODER_ERROR_ALLOC_RING_BUFFER_2
}
fn f34() -> BrotliDecoderErrorCode {
    BROTLI_DECODER_ERROR_ALLOC_BLOCK_TYPE_TREES
}

unsafe fn f35(a0: *const BrotliDecoderState) -> BrotliDecoderErrorCode {
    BrotliDecoderGetErrorCode_unsafe(a0)
}

fn f36() -> u32 {
    BrotliDecoderVersion()
}
