// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <brotli/decode.h>
#include <brotli/encode.h>

typedef BrotliDecoderResult t1;
typedef BrotliEncoderMode t2;
typedef struct BrotliDecoderStateStruct *t3;
typedef const struct BrotliDecoderStateStruct *t4;
typedef BrotliDecoderErrorCode t5;

BrotliEncoderMode f1(void) { return BROTLI_MODE_FONT; }

BROTLI_BOOL f2(int quality, int lgwin, BrotliEncoderMode mode,
               size_t input_size,
               const uint8_t input_buffer[BROTLI_ARRAY_PARAM(input_size)],
               size_t *encoded_size,
               uint8_t encoded_buffer[BROTLI_ARRAY_PARAM(*encoded_size)]) {
  return BrotliEncoderCompress(quality, lgwin, mode, input_size, input_buffer,
                               encoded_size, encoded_buffer);
}

BrotliEncoderMode f3(void) { return BROTLI_MODE_TEXT; }

BrotliDecoderResult f4(void) { return BROTLI_DECODER_RESULT_SUCCESS; }

BrotliDecoderResult
f5(size_t encoded_size,
   const uint8_t encoded_buffer[BROTLI_ARRAY_PARAM(encoded_size)],
   size_t *decoded_size,
   uint8_t decoded_buffer[BROTLI_ARRAY_PARAM(*decoded_size)]) {
  return BrotliDecoderDecompress(encoded_size, encoded_buffer, decoded_size,
                                 decoded_buffer);
}

BrotliDecoderState *f6(brotli_alloc_func alloc_func, brotli_free_func free_func,
                       void *opaque) {
  return BrotliDecoderCreateInstance(alloc_func, free_func, opaque);
}

void f7(BrotliDecoderState *state) {
  return BrotliDecoderDestroyInstance(state);
}

BrotliDecoderResult f8(BrotliDecoderState *state, size_t *available_in,
                       const uint8_t **next_in, size_t *available_out,
                       uint8_t **next_out, size_t *total_out) {
  return BrotliDecoderDecompressStream(state, available_out, next_in,
                                       available_out, next_out, total_out);
}

const uint8_t *f9(BrotliDecoderState *state, size_t *size) {
  return BrotliDecoderTakeOutput(state, size);
}

BrotliDecoderResult f10(void) { return BROTLI_DECODER_RESULT_ERROR; }

BrotliDecoderResult f11(void) {
  return BROTLI_DECODER_RESULT_NEEDS_MORE_OUTPUT;
}

BrotliEncoderMode f12(void) { return BROTLI_MODE_GENERIC; }

BrotliDecoderResult f13(void) { return BROTLI_DECODER_RESULT_NEEDS_MORE_INPUT; }

BrotliDecoderErrorCode f14(void) {
  return BROTLI_DECODER_ERROR_FORMAT_EXUBERANT_NIBBLE;
}
BrotliDecoderErrorCode f15(void) {
  return BROTLI_DECODER_ERROR_FORMAT_EXUBERANT_META_NIBBLE;
}
BrotliDecoderErrorCode f16(void) {
  return BROTLI_DECODER_ERROR_FORMAT_SIMPLE_HUFFMAN_ALPHABET;
}
BrotliDecoderErrorCode f17(void) {
  return BROTLI_DECODER_ERROR_FORMAT_SIMPLE_HUFFMAN_SAME;
}
BrotliDecoderErrorCode f18(void) {
  return BROTLI_DECODER_ERROR_FORMAT_CL_SPACE;
}
BrotliDecoderErrorCode f19(void) {
  return BROTLI_DECODER_ERROR_FORMAT_HUFFMAN_SPACE;
}
BrotliDecoderErrorCode f20(void) {
  return BROTLI_DECODER_ERROR_FORMAT_CONTEXT_MAP_REPEAT;
}
BrotliDecoderErrorCode f21(void) {
  return BROTLI_DECODER_ERROR_FORMAT_BLOCK_LENGTH_1;
}
BrotliDecoderErrorCode f22(void) {
  return BROTLI_DECODER_ERROR_FORMAT_BLOCK_LENGTH_2;
}
BrotliDecoderErrorCode f23(void) {
  return BROTLI_DECODER_ERROR_FORMAT_TRANSFORM;
}
BrotliDecoderErrorCode f24(void) {
  return BROTLI_DECODER_ERROR_FORMAT_DICTIONARY;
}
BrotliDecoderErrorCode f25(void) {
  return BROTLI_DECODER_ERROR_FORMAT_WINDOW_BITS;
}
BrotliDecoderErrorCode f26(void) {
  return BROTLI_DECODER_ERROR_FORMAT_PADDING_1;
}
BrotliDecoderErrorCode f27(void) {
  return BROTLI_DECODER_ERROR_FORMAT_PADDING_2;
}
BrotliDecoderErrorCode f28(void) {
  return BROTLI_DECODER_ERROR_INVALID_ARGUMENTS;
}
BrotliDecoderErrorCode f29(void) {
  return BROTLI_DECODER_ERROR_ALLOC_CONTEXT_MODES;
}
BrotliDecoderErrorCode f30(void) {
  return BROTLI_DECODER_ERROR_ALLOC_TREE_GROUPS;
}
BrotliDecoderErrorCode f31(void) {
  return BROTLI_DECODER_ERROR_ALLOC_CONTEXT_MAP;
}
BrotliDecoderErrorCode f32(void) {
  return BROTLI_DECODER_ERROR_ALLOC_RING_BUFFER_1;
}
BrotliDecoderErrorCode f33(void) {
  return BROTLI_DECODER_ERROR_ALLOC_RING_BUFFER_2;
}
BrotliDecoderErrorCode f34(void) {
  return BROTLI_DECODER_ERROR_ALLOC_BLOCK_TYPE_TREES;
}

BrotliDecoderErrorCode f35(const BrotliDecoderState *state) {
  return BrotliDecoderGetErrorCode(state);
}

uint32_t f36(void) { return BrotliDecoderVersion(); }
