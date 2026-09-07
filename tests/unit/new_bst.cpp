// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>

struct node_t {
  node_t *left;
  node_t *right;
  int value;
};

node_t *find(node_t *node, int value);
node_t *insert(node_t *node, int value);

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

node_t *insert(node_t *node, int value) {
  if (node == nullptr)
    return new node_t{nullptr, nullptr, value};
  if (value < node->value) {
    node->left = insert(node->left, value);
  } else if (value > node->value) {
    node->right = insert(node->right, value);
  }
  return node;
}

void del(node_t *node) {
  if (node->left != nullptr)
    del(node->left);
  if (node->right != nullptr)
    del(node->right);
  delete node;
}

int main() {
  node_t *root = new node_t{nullptr, nullptr, 0};
  root = insert(root, 1);
  root = insert(root, 2);
  root = insert(root, 3);
  root = insert(root, 4);
  bool out = find(root, 0)->value == 0 && find(root, 1)->value == 1 &&
             find(root, 2)->value == 2 && find(root, 3)->value == 3 &&
             find(root, 4)->value == 4 && find(root, 5) == nullptr;
  del(root);
  assert(out);
  return 0;
}
