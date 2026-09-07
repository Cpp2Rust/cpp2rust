// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>
#include <memory>

// bst.h: BST node definition
struct node_t {
  node_t *left;
  node_t *right;
  int value;
};

node_t *find(node_t *node, int value);
node_t *insert(node_t *node, node_t *new_node);

// bst.cc: BST implementation
node_t *find(node_t *node, int value) {
  if (value < node->value && node->left != nullptr) {
    return find(node->left, value);
  } else if (value > node->value && node->right != nullptr) {
    return find(node->right, value);
  } else if (value == node->value) {
    return node;
  }
  return nullptr;
}

node_t *insert(node_t *node, node_t *new_node) {
  if (node == nullptr)
    return new_node;
  if (new_node->value < node->value) {
    node->left = insert(node->left, new_node);
  } else if (new_node->value > node->value) {
    node->right = insert(node->right, new_node);
  }
  return node;
}

// main.c: program entry point
int main() {
  std::unique_ptr<node_t> tree =
      std::make_unique<node_t>(node_t{nullptr, nullptr, 0});
  std::unique_ptr<node_t> n1 =
      std::make_unique<node_t>(node_t{nullptr, nullptr, 1});
  std::unique_ptr<node_t> n2 =
      std::make_unique<node_t>(node_t{nullptr, nullptr, 2});
  std::unique_ptr<node_t> n3 =
      std::make_unique<node_t>(node_t{nullptr, nullptr, 3});
  std::unique_ptr<node_t> n4 =
      std::make_unique<node_t>(node_t{nullptr, nullptr, 4});
  node_t *ptr1 = &*tree;
  ptr1 = insert(ptr1, &*n1);
  ptr1 = insert(ptr1, &*n2);
  ptr1 = insert(ptr1, &*n3);
  ptr1 = insert(ptr1, &*n4);
  assert(find(ptr1, 0)->value == 0 && find(ptr1, 1)->value == 1 &&
         find(ptr1, 2)->value == 2 && find(ptr1, 3)->value == 3 &&
         find(ptr1, 4)->value == 4 && find(ptr1, 5) == nullptr);
  return 0;
}
