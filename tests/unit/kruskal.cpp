// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>
#include <memory>

struct Edge {
  int u;
  int v;
  double weight;
};

int partition(std::unique_ptr<Edge[]> &arr, int start, int end) {
  Edge &pivot = arr[start];
  int count = 0;
  for (int i = start + 1; i <= end; ++i) {
    if (arr[i].weight <= pivot.weight)
      count++;
  }
  int pidx = start + count;
  Edge tmp = {arr[pidx].u, arr[pidx].v, arr[pidx].weight};
  arr[pidx] = {arr[start].u, arr[start].v, arr[start].weight};
  arr[start] = {tmp.u, tmp.v, tmp.weight};
  int i = start, j = end;
  while (i < pidx && j > pidx) {
    while (arr[i].weight <= pivot.weight) {
      ++i;
    }
    while (arr[j].weight > pivot.weight) {
      --j;
    }
    if (i < pidx && j > pidx) {
      tmp = {arr[i].u, arr[i].v, arr[i].weight};
      arr[i] = {arr[j].u, arr[j].v, arr[j].weight};
      arr[j] = {tmp.u, tmp.v, tmp.weight};
      i++;
      j--;
    }
  }
  return pidx;
}

void quicksort(std::unique_ptr<Edge[]> &arr, int start, int end) {
  if (start >= end)
    return;
  int p = partition(arr, start, end);
  quicksort(arr, start, p - 1);
  quicksort(arr, p + 1, end);
}

class DisjointSet {
public:
  std::unique_ptr<int[]> rank;
  std::unique_ptr<int[]> parent;
  int n;

  void makeSet() {
    for (int i = 0; i < n; i++) {
      this->parent[i] = i;
      this->rank[i] = 1;
    }
  }

  int find(int x) {
    if (parent[x] != x) {
      parent[x] = find(parent[x]);
    }
    return parent[x];
  }

  void merge(int x, int y) {
    int xset = find(x);
    int yset = find(y);
    if (xset == yset)
      return;
    if (rank[xset] < rank[yset]) {
      parent[xset] = yset;
    } else if (rank[xset] > rank[yset]) {
      parent[yset] = xset;
    } else {
      parent[yset] = xset;
      rank[xset] = rank[xset] + 1;
    }
  }
};

struct Graph {
  std::unique_ptr<Edge[]> edges;
  int V;
  int E;
};

double MSTKruskal(Graph &graph) {
  quicksort(graph.edges, 0, graph.E - 1);
  DisjointSet set{std::make_unique<int[]>(graph.V),
                  std::make_unique<int[]>(graph.V), graph.V};
  set.makeSet();
  double total_weight = 0;
  for (int i = 0; i < graph.E; ++i) {
    int x = graph.edges[i].u;
    int y = graph.edges[i].v;
    double w = graph.edges[i].weight;
    if (set.find(x) != set.find(y)) {
      set.merge(x, y);
      total_weight += w;
    }
  }
  return total_weight;
}

int main() {
  int V = 4;
  int E = 5;
  Graph graph{std::make_unique<Edge[]>(E), V, E};
  graph.edges[0] = {0, 1, 10};
  graph.edges[1] = {1, 3, 15};
  graph.edges[2] = {2, 3, 4};
  graph.edges[3] = {2, 0, 6};
  graph.edges[4] = {0, 3, 5};
  double total_weight = MSTKruskal(graph);
  assert(total_weight == 19);
  return 0;
}
