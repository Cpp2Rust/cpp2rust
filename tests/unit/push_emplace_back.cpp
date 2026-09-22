#include <cassert>
#include <cstdint>
#include <deque>
#include <vector>

struct Chunk {
  int data;
};

struct Writer {
  std::vector<Chunk> *output;
  Chunk chunk;
};

struct JPEGData {
  std::vector<std::vector<uint8_t>> com_data;
  std::vector<std::vector<uint8_t>> app_data;
};

void push_param(std::vector<std::vector<uint8_t>> *dest) {
  dest->push_back(std::vector<uint8_t>());
}

void push_local_from_field(JPEGData *jpg, bool cond) {
  uint8_t head[3] = {1, 2, 3};
  std::vector<std::vector<uint8_t>> *dest;
  if (cond) {
    dest = &jpg->com_data;
  } else {
    dest = &jpg->app_data;
  }
  dest->push_back(std::vector<uint8_t>(head, head + 3));
}

void shrink_through_ptr(std::vector<Chunk> *comps) { comps->shrink_to_fit(); }

void nested_push_move(Writer *bw) {
  bw->output->push_back(std::move(bw->chunk));
}

void emplace_local_from_field(JPEGData *jpg, bool cond) {
  uint8_t head[3] = {1, 2, 3};
  std::vector<std::vector<uint8_t>> *dest;
  if (cond) {
    dest = &jpg->com_data;
  } else {
    dest = &jpg->app_data;
  }
  dest->emplace_back(head, head + 3);
}

void nested_emplace_move(Writer *bw) {
  bw->output->emplace_back(std::move(bw->chunk));
}

void self_ref_push(std::vector<Chunk> *comps) {
  comps->push_back(comps->front());
}

struct Pair {
  int first;
  int second;
  Pair() : first(-1), second(-1) {}
  Pair(int a) : first(a), second(0) {}
  Pair(int a, int b) : first(a), second(b * 2) {}
};

void emplace_ctor_args(std::vector<Pair> *pairs) {
  pairs->emplace_back();
  pairs->emplace_back(3);
  pairs->emplace_back(4, 5);
}

void emplace_deque(std::deque<Pair> *queue) {
  queue->emplace_back(6, 7);
  queue->emplace_back();
}

void emplace_scalar(std::vector<long> *values, int x) {
  values->emplace_back();
  values->emplace_back(x);
}

int main() {
  std::vector<std::vector<uint8_t>> vecs;
  push_param(&vecs);
  assert(vecs.size() == 1);
  assert(vecs[0].empty());

  JPEGData jpg;
  push_local_from_field(&jpg, true);
  assert(jpg.com_data.size() == 1);
  assert(jpg.com_data[0].size() == 3);
  assert(jpg.com_data[0][0] == 1);
  assert(jpg.com_data[0][1] == 2);
  assert(jpg.com_data[0][2] == 3);
  assert(jpg.app_data.empty());

  std::vector<Chunk> chunks;
  shrink_through_ptr(&chunks);
  assert(chunks.empty());

  Writer w;
  w.chunk.data = 42;
  w.output = &chunks;
  nested_push_move(&w);
  assert(chunks.size() == 1);
  assert(chunks[0].data == 42);

  emplace_local_from_field(&jpg, false);
  assert(jpg.app_data.size() == 1);
  assert(jpg.app_data[0].size() == 3);
  assert(jpg.app_data[0][0] == 1);
  assert(jpg.app_data[0][2] == 3);
  assert(jpg.com_data.size() == 1);

  w.chunk.data = 99;
  w.output = &chunks;
  nested_emplace_move(&w);
  assert(chunks.size() == 2);
  assert(chunks[1].data == 99);

  self_ref_push(&chunks);
  assert(chunks.size() == 3);
  assert(chunks[2].data == 42);

  std::vector<Pair> pairs;
  emplace_ctor_args(&pairs);
  assert(pairs.size() == 3);
  assert(pairs[0].first == -1 && pairs[0].second == -1);
  assert(pairs[1].first == 3 && pairs[1].second == 0);
  assert(pairs[2].first == 4 && pairs[2].second == 10);

  std::deque<Pair> queue;
  emplace_deque(&queue);
  assert(queue.front().first == 6 && queue.front().second == 14);
  assert(queue.back().first == -1 && queue.back().second == -1);

  std::vector<long> values;
  emplace_scalar(&values, 7);
  assert(values.size() == 2);
  assert(values[0] == 0);
  assert(values[1] == 7);

  return 0;
}
