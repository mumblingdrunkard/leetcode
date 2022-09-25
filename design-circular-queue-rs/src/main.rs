// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/design-circular-queue/

#[allow(unused)]
struct MyCircularQueue {
    front: usize,
    len: usize,
    data: Vec<i32>,
}

#[allow(unused)]
impl MyCircularQueue {
    fn new(k: i32) -> Self {
        Self {
            front: 0,
            len: 0,
            data: vec![0; k as usize],
        }
    }

    fn en_queue(&mut self, value: i32) -> bool {
        if self.len == self.data.len() {
            return false;
        }
        let capacity = self.data.len();
        self.data[(self.front + self.len) % capacity] = value;
        self.len += 1;
        true
    }

    fn de_queue(&mut self) -> bool {
        if self.len == 0 {
            return false;
        }

        self.len -= 1;
        self.front += 1;
        self.front %= self.data.len();
        true
    }

    fn front(&self) -> i32 {
        if self.len == 0 {
            -1
        } else {
            self.data[self.front]
        }
    }

    fn rear(&self) -> i32 {
        if self.len == 0 {
            -1
        } else {
            self.data[(self.front + self.len + self.data.len() - 1) % self.data.len()]
        }
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn is_full(&self) -> bool {
        self.len == self.data.len()
    }
}

fn main() {}
