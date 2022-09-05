// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/n-ary-tree-level-order-traversal/

#include <deque>
#include <vector>
#include <iostream>

class Node {
public:
  int val;
  std::vector<Node *> children;

  Node() {}
  Node(int _val) : val(_val) {}
  Node(int _val, std::vector<Node *> _children)
      : val(_val), children(_children) {}
};

class Solution {
  struct Entry {
    Node *n;
    std::size_t level;
  };

public:
  std::vector<std::vector<int>> levelOrder(Node *root) {
    auto result = std::vector<std::vector<int>>();
    auto q = std::deque<Entry>();
    q.push_back({root, 0});

    while (!q.empty()) {
      auto e = q.front();
      q.pop_front();
      if (e.n == nullptr)
        continue;

      if (e.level >= result.size()) {
        result.push_back(std::vector<int>());
      }

      result[e.level].push_back(e.n->val);

      for (auto child : e.n->children) {
        q.push_back({child, e.level + 1});
      }
    }

    return result;
  }
};

int main(void) {
  std::vector<Node> nodes = {Node(1), Node(3), Node(2),
                             Node(4), Node(5), Node(6)};
  nodes[0].children = {&nodes[1], &nodes[2], &nodes[3]};
  nodes[1].children = {&nodes[4], &nodes[5]};

  auto solution = Solution();
  auto res = solution.levelOrder(&nodes[0]);
  for (auto level : res) {
      for (auto i : level) {
          std::cout << i << " ";
      }
      std::cout << std::endl;
  }
}
