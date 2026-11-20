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

void emplace_ref_through_ptr(std::vector<Pair> *pairs) {
  Pair &p = pairs->emplace_back(1, 2);
  p.second = 3;
  pairs->emplace_back(4).second += 1;
}

int *emplace_addr(std::vector<int> &values) { return &values.emplace_back(9); }

void emplace_self_reads(std::vector<long> *values, std::vector<Pair> *pairs) {
  values->emplace_back(values->size());
  pairs->emplace_back((*pairs)[0]);
  pairs->emplace_back(pairs->back().first, (int)pairs->size());
}

long &emplace_ref_self_read(std::vector<long> &values) {
  return values.emplace_back(values.front());
}

struct Bytes {
  std::vector<std::vector<uint8_t>> v;
  uint8_t b[3];
};

void emplace_from_element(std::vector<std::vector<uint8_t>> *v) {
  uint8_t *p = (*v)[0].data();
  v->emplace_back(p, p + 3);
}

struct Plain {
  int a;
  double b[2];
};

void emplace_plain(std::vector<Plain> *v) {
  Plain p{1, {2, 3}};
  v->emplace_back(p);
}

struct Counted;
std::vector<Counted> *all_counted;

struct Counted {
  std::size_t seen;
  explicit Counted(int);
};

Counted::Counted(int) : seen(all_counted->size()) {}

void emplace_counted(std::vector<Counted> *v) { v->emplace_back(1); }

void emplace_from_sibling(Bytes *s) {
  uint8_t *p = s->b;
  std::vector<std::vector<uint8_t>> *v = &s->v;
  v->emplace_back(p, p + 3);
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

  std::vector<int> ints;
  ints.emplace_back(0);
  int &r = ints.emplace_back(1);
  r = 5;
  int *q = emplace_addr(ints);
  *q += 1;
  assert(ints.size() == 3 && ints[1] == 5 && ints[2] == 10);

  std::deque<int> ints_queue;
  ints_queue.emplace_back(2) += 3;
  assert(ints_queue.front() == 5);

  std::vector<Pair> more_pairs;
  emplace_ref_through_ptr(&more_pairs);
  assert(more_pairs[0].first == 1 && more_pairs[0].second == 3);
  assert(more_pairs[1].first == 4 && more_pairs[1].second == 1);

  std::vector<long> self_values{7};
  std::vector<Pair> self_pairs;
  self_pairs.emplace_back(1, 2);
  emplace_self_reads(&self_values, &self_pairs);
  emplace_ref_self_read(self_values) += 1;
  assert(self_values.size() == 3 && self_values[1] == 1 && self_values[2] == 8);
  assert(self_pairs[1].first == 1 && self_pairs[1].second == 4);
  assert(self_pairs[2].first == 1 && self_pairs[2].second == 4);

  std::vector<std::vector<uint8_t>> rows{{1, 2, 3}};
  emplace_from_element(&rows);
  assert(rows.size() == 2 && rows[1][2] == 3);
  Bytes bytes{{}, {4, 5, 6}};
  emplace_from_sibling(&bytes);
  assert(bytes.v.size() == 1 && bytes.v[0][2] == 6);

  std::vector<Plain> plains;
  emplace_plain(&plains);
  assert(plains[0].a == 1 && plains[0].b[1] == 3);

  std::vector<Counted> counted;
  all_counted = &counted;
  emplace_counted(&counted);
  emplace_counted(&counted);
  assert(counted[0].seen == 0 && counted[1].seen == 1);

  return 0;
}
