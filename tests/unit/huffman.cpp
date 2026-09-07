// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>
#include <memory>

#define MAX_TREE_HT 100

class MinHeapNode {
public:
  char data;
  int freq;
  MinHeapNode *left;
  MinHeapNode *right;
  bool IsLeaf() const { return left == nullptr && right == nullptr; }
};

void Swap(MinHeapNode &a, MinHeapNode &b) {
  MinHeapNode t = {a.data, a.freq, a.left, a.right};
  a = {b.data, b.freq, b.left, b.right};
  b = {t.data, t.freq, t.left, t.right};
}

class MinHeap {
public:
  int size;
  int capacity;
  std::unique_ptr<MinHeapNode *[]> arr;
  int next;
  std::unique_ptr<MinHeapNode[]> alloc;

  MinHeapNode *Alloc(char data, int freq) {
    alloc[next] = {data, freq, nullptr, nullptr};
    return &alloc[next++];
  }

  void Heapify(int idx) {
    int smallest = idx;
    int left = 2 * idx + 1;
    int right = 2 * idx + 2;
    if (left < size && arr[left]->freq < arr[smallest]->freq)
      smallest = left;
    if (right < size && arr[right]->freq < arr[smallest]->freq)
      smallest = right;
    if (smallest != idx) {
      Swap(*arr[smallest], *arr[idx]);
      Heapify(smallest);
    }
  }

  MinHeapNode *ExtractMin() {
    MinHeapNode *out = arr[0];
    --size;
    arr[0] = arr[size];
    Heapify(0);
    return out;
  }

  void Insert(MinHeapNode *node) {
    ++size;
    int i = size - 1;
    while (i != 0 && node->freq < arr[(i - 1) / 2]->freq) {
      arr[i] = arr[(i - 1) / 2];
      i = (i - 1) / 2;
    }
    arr[i] = node;
  }

  void Build(std::unique_ptr<char[]> &data, std::unique_ptr<int[]> &freq,
             int n) {
    for (int i = 0; i < n; ++i)
      arr[size++] = Alloc(data[i], freq[i]);
    for (int i = (size - 2) / 2; i >= 0; --i)
      Heapify(i);
  }
};

std::unique_ptr<MinHeap> AllocMinHeap(int capacity) {
  std::unique_ptr<MinHeap> minHeap = std::make_unique<MinHeap>(MinHeap{
      0,
      capacity,
      std::make_unique<MinHeapNode *[]>(capacity),
      0,
      std::make_unique<MinHeapNode[]>(10000),
  });
  return minHeap;
}

std::unique_ptr<MinHeap> Huffman(std::unique_ptr<char[]> &data,
                                 std::unique_ptr<int[]> &freq, int size) {
  std::unique_ptr<MinHeap> minHeap = AllocMinHeap(size);
  (*minHeap).Build(data, freq, size);
  while ((*minHeap).size != 1) {
    MinHeapNode *left = (*minHeap).ExtractMin();
    MinHeapNode *right = (*minHeap).ExtractMin();
    MinHeapNode *top = (*minHeap).Alloc('$', left->freq + right->freq);
    top->left = left;
    top->right = right;
    (*minHeap).Insert(top);
  }
  return minHeap;
}

void CollectCode(std::unique_ptr<int[]> &arr, int top,
                 std::unique_ptr<int[]> &out, int &next) {
  out[next] = 0;
  for (int i = 0; i < top; ++i) {
    out[next] = out[next] * 10;
    out[next] = out[next] + arr[i];
  }
  ++next;
}

void CollectCodes(MinHeapNode *root, std::unique_ptr<int[]> &arr, int top,
                  std::unique_ptr<int[]> &out, int &next) {
  if (root->left != nullptr) {
    arr[top] = 0;
    CollectCodes(root->left, arr, top + 1, out, next);
  }
  if (root->right != nullptr) {
    arr[top] = 1;
    CollectCodes(root->right, arr, top + 1, out, next);
  }
  if (root->IsLeaf()) {
    CollectCode(arr, top, out, next);
  }
}

std::unique_ptr<int[]> HuffmanCodes(std::unique_ptr<char[]> &data,
                                    std::unique_ptr<int[]> &freq, int size) {
  std::unique_ptr<MinHeap> minHeap = Huffman(data, freq, size);
  MinHeapNode *root = (*minHeap).ExtractMin();
  std::unique_ptr<int[]> arr = std::make_unique<int[]>(MAX_TREE_HT);
  std::unique_ptr<int[]> out = std::make_unique<int[]>(MAX_TREE_HT);
  int top = 0, next = 0;
  CollectCodes(root, arr, top, out, next);
  return out;
}

int main() {
  int size = 6;
  char arr1[] = {'a', 'b', 'c', 'd', 'e', 'f'};
  int arr2[] = {5, 9, 12, 13, 16, 45};
  std::unique_ptr<char[]> data = std::make_unique<char[]>(size);
  std::unique_ptr<int[]> freq = std::make_unique<int[]>(size);
  for (int i = 0; i < size; ++i) {
    data[i] = arr1[i];
    freq[i] = arr2[i];
  }
  std::unique_ptr<int[]> out = HuffmanCodes(data, freq, size);
  assert(out[0] == 0 && out[1] == 100 && out[2] == 101 && out[3] == 1100 &&
         out[4] == 1101 && out[5] == 111);
  return 0;
}
