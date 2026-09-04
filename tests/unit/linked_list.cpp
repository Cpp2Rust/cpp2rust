// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>

struct Node {
  int val;
  Node *next;

  void SetNext(Node *next) { this->next = next; }
};

Node *Find(Node *head, int idx) {
  Node *curr = head;
  for (int i = 0; i < idx; i++)
    curr = curr->next;
  return curr;
}

void Append(Node &head, Node &new_node) {
  Node *curr = &head;
  while (curr->next != nullptr)
    curr = curr->next;
  curr->SetNext(&new_node);
}

Node *Delete(Node *head, int val) {
  Node *curr = head;
  Node *prev = nullptr;
  while (curr != nullptr) {
    if (curr->val == val) {
      if (prev != nullptr) {
        prev->next = curr->next;
        return head;
      } else {
        return curr->next;
      }
    }
    prev = curr;
    curr = curr->next;
  }
  return head;
}

int main() {
  Node n0 = {5, nullptr};
  Node *head = &n0;
  Node n1 = {4, nullptr};
  Node n2 = {3, nullptr};
  Node n3 = {2, nullptr};
  Node n4 = {1, nullptr};
  Node n5 = {0, nullptr};
  Node n6 = {-1, nullptr};
  Node n7 = {-2, nullptr};
  Append(*head, n1);
  Append(*head, n2);
  Append(*head, n3);
  Append(*head, n4);
  Append(*head, n5);
  Append(*head, n6);
  Append(*head, n7);
  head = Delete(head, 5);
  head = Delete(head, 0);
  head = Delete(head, -2);
  assert(Find(head, 0)->val == 4 && Find(head, 1)->val == 3 &&
         Find(head, 2)->val == 2 && Find(head, 3)->val == 1 &&
         Find(head, 4)->val == -1 && Find(head, 5) == nullptr);
  return 0;
}
