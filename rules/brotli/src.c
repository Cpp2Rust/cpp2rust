// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#ifdef CPP2RUST_USE_INCLUDES
#include <brotli/decode.h>
#include <brotli/encode.h>
#endif

typedef BrotliDecoderResult CPP2RUST_TYPE_RULE(1);
typedef BrotliEncoderMode CPP2RUST_TYPE_RULE(2);
typedef struct BrotliDecoderStateStruct *CPP2RUST_TYPE_RULE(3);
typedef const struct BrotliDecoderStateStruct *CPP2RUST_TYPE_RULE(4);
typedef BrotliDecoderErrorCode CPP2RUST_TYPE_RULE(5);

BrotliEncoderMode CPP2RUST_EXPR_RULE(1)() { return BROTLI_MODE_FONT; }

BROTLI_BOOL CPP2RUST_EXPR_RULE(2)(
    int quality, int lgwin, BrotliEncoderMode mode, size_t input_size,
    const uint8_t input_buffer[BROTLI_ARRAY_PARAM(input_size)],
    size_t *encoded_size,
    uint8_t encoded_buffer[BROTLI_ARRAY_PARAM(*encoded_size)]) {
  return BrotliEncoderCompress(quality, lgwin, mode, input_size, input_buffer,
                               encoded_size, encoded_buffer);
}

BrotliEncoderMode CPP2RUST_EXPR_RULE(3)() { return BROTLI_MODE_TEXT; }

BrotliDecoderResult CPP2RUST_EXPR_RULE(4)() {
  return BROTLI_DECODER_RESULT_SUCCESS;
}

BrotliDecoderResult CPP2RUST_EXPR_RULE(5)(
    size_t encoded_size,
    const uint8_t encoded_buffer[BROTLI_ARRAY_PARAM(encoded_size)],
    size_t *decoded_size,
    uint8_t decoded_buffer[BROTLI_ARRAY_PARAM(*decoded_size)]) {
  return BrotliDecoderDecompress(encoded_size, encoded_buffer, decoded_size,
                                 decoded_buffer);
}

BrotliDecoderState *CPP2RUST_EXPR_RULE(6)(brotli_alloc_func alloc_func,
                                          brotli_free_func free_func,
                                          void *opaque) {
  return BrotliDecoderCreateInstance(alloc_func, free_func, opaque);
}

void CPP2RUST_EXPR_RULE(7)(BrotliDecoderState *state) {
  return BrotliDecoderDestroyInstance(state);
}

BrotliDecoderResult
CPP2RUST_EXPR_RULE(8)(BrotliDecoderState *state, size_t *available_in,
                      const uint8_t **next_in, size_t *available_out,
                      uint8_t **next_out, size_t *total_out) {
  return BrotliDecoderDecompressStream(state, available_out, next_in,
                                       available_out, next_out, total_out);
}

const uint8_t *CPP2RUST_EXPR_RULE(9)(BrotliDecoderState *state, size_t *size) {
  return BrotliDecoderTakeOutput(state, size);
}

BrotliDecoderResult CPP2RUST_EXPR_RULE(10)() {
  return BROTLI_DECODER_RESULT_ERROR;
}

BrotliDecoderResult CPP2RUST_EXPR_RULE(11)() {
  return BROTLI_DECODER_RESULT_NEEDS_MORE_OUTPUT;
}

BrotliEncoderMode CPP2RUST_EXPR_RULE(12)() { return BROTLI_MODE_GENERIC; }

BrotliDecoderResult CPP2RUST_EXPR_RULE(13)() {
  return BROTLI_DECODER_RESULT_NEEDS_MORE_INPUT;
}

BrotliDecoderErrorCode CPP2RUST_EXPR_RULE(14)() {
  return BROTLI_DECODER_ERROR_FORMAT_EXUBERANT_NIBBLE;
}
BrotliDecoderErrorCode CPP2RUST_EXPR_RULE(15)() {
  return BROTLI_DECODER_ERROR_FORMAT_EXUBERANT_META_NIBBLE;
}
BrotliDecoderErrorCode CPP2RUST_EXPR_RULE(16)() {
  return BROTLI_DECODER_ERROR_FORMAT_SIMPLE_HUFFMAN_ALPHABET;
}
BrotliDecoderErrorCode CPP2RUST_EXPR_RULE(17)() {
  return BROTLI_DECODER_ERROR_FORMAT_SIMPLE_HUFFMAN_SAME;
}
BrotliDecoderErrorCode CPP2RUST_EXPR_RULE(18)() {
  return BROTLI_DECODER_ERROR_FORMAT_CL_SPACE;
}
BrotliDecoderErrorCode CPP2RUST_EXPR_RULE(19)() {
  return BROTLI_DECODER_ERROR_FORMAT_HUFFMAN_SPACE;
}
BrotliDecoderErrorCode CPP2RUST_EXPR_RULE(20)() {
  return BROTLI_DECODER_ERROR_FORMAT_CONTEXT_MAP_REPEAT;
}
BrotliDecoderErrorCode CPP2RUST_EXPR_RULE(21)() {
  return BROTLI_DECODER_ERROR_FORMAT_BLOCK_LENGTH_1;
}
BrotliDecoderErrorCode CPP2RUST_EXPR_RULE(22)() {
  return BROTLI_DECODER_ERROR_FORMAT_BLOCK_LENGTH_2;
}
BrotliDecoderErrorCode CPP2RUST_EXPR_RULE(23)() {
  return BROTLI_DECODER_ERROR_FORMAT_TRANSFORM;
}
BrotliDecoderErrorCode CPP2RUST_EXPR_RULE(24)() {
  return BROTLI_DECODER_ERROR_FORMAT_DICTIONARY;
}
BrotliDecoderErrorCode CPP2RUST_EXPR_RULE(25)() {
  return BROTLI_DECODER_ERROR_FORMAT_WINDOW_BITS;
}
BrotliDecoderErrorCode CPP2RUST_EXPR_RULE(26)() {
  return BROTLI_DECODER_ERROR_FORMAT_PADDING_1;
}
BrotliDecoderErrorCode CPP2RUST_EXPR_RULE(27)() {
  return BROTLI_DECODER_ERROR_FORMAT_PADDING_2;
}
BrotliDecoderErrorCode CPP2RUST_EXPR_RULE(28)() {
  return BROTLI_DECODER_ERROR_INVALID_ARGUMENTS;
}
BrotliDecoderErrorCode CPP2RUST_EXPR_RULE(29)() {
  return BROTLI_DECODER_ERROR_ALLOC_CONTEXT_MODES;
}
BrotliDecoderErrorCode CPP2RUST_EXPR_RULE(30)() {
  return BROTLI_DECODER_ERROR_ALLOC_TREE_GROUPS;
}
BrotliDecoderErrorCode CPP2RUST_EXPR_RULE(31)() {
  return BROTLI_DECODER_ERROR_ALLOC_CONTEXT_MAP;
}
BrotliDecoderErrorCode CPP2RUST_EXPR_RULE(32)() {
  return BROTLI_DECODER_ERROR_ALLOC_RING_BUFFER_1;
}
BrotliDecoderErrorCode CPP2RUST_EXPR_RULE(33)() {
  return BROTLI_DECODER_ERROR_ALLOC_RING_BUFFER_2;
}
BrotliDecoderErrorCode CPP2RUST_EXPR_RULE(34)() {
  return BROTLI_DECODER_ERROR_ALLOC_BLOCK_TYPE_TREES;
}

BrotliDecoderErrorCode CPP2RUST_EXPR_RULE(35)(const BrotliDecoderState *state) {
  return BrotliDecoderGetErrorCode(state);
}

uint32_t CPP2RUST_EXPR_RULE(36)() { return BrotliDecoderVersion(); }
