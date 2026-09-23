struct GraphNode {
  unsigned dst;
  GraphNode *next;
};

struct Graph {
  unsigned V;
  GraphNode **adj;

  void push(unsigned src, unsigned dst) const {
    adj[src] = new GraphNode{dst, adj[src]};
    adj[dst] = new GraphNode{src, adj[dst]};
  }
};

struct Partial {
  int *p = nullptr;

  Partial() = default;
  explicit Partial(int *q) : p(q) {}

  // Below methods are only declared
  int &get() const;
  Partial &next();
  Partial next(int);
};

class Declared {
public:
  Declared(int);
};

struct S {
  int i;
  Declared *d;
};

int main() {
  Graph g{5, nullptr};

  int arr[3] = {3, 1, 4};
  Partial it(arr);
  if (it.p != arr) {
    return 1;
  }
  Partial def;
  if (def.p != nullptr) {
    return 1;
  }

  S s{7, nullptr};
  if (s.i != 7 || s.d != nullptr) {
    return 1;
  }
  return 0;
}
