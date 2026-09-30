#include <cassert>
#include <memory>

struct Data {
  int v;
};

struct Holder {
  std::unique_ptr<Data> data;
  int n;
  void set(Data *p) { data.reset(p); }
};

int main() {
  Holder h;
  h.n = 1;
  Holder *hp = &h;
  hp->data.reset(new Data{3});
  assert(h.data->v == 3);
  h.set(new Data{4});
  assert(hp->data->v == 4);
  hp->data->v += hp->n;
  assert(h.data->v == 5);
  return 0;
}
