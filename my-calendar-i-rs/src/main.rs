// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// This Source Code Form is "Incompatible With Secondary Licenses", as
// defined by the Mozilla Public License, v. 2.0.
//
// Copyright © 2022 mumblingdrunkard
//
// solution for problem found at https://leetcode.com/problems/my-calendar-i/

use std::collections::BTreeMap;

struct MyCalendar {
    segments: BTreeMap<usize, usize>,
}

impl MyCalendar {
    fn new() -> Self {
        Self {
            segments: BTreeMap::new(),
        }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        let start = start as usize;
        let end = end as usize;
        if let Some((_, &u)) = self.segments.range(..end).next_back() {
            if u > start {
                return false;
            }
        }

        if let Some((&l, _)) = self.segments.range(start..).next() {
            if end > l {
                return false;
            }
        }

        self.segments.insert(start, end);

        true
    }
}

fn main() {
    let mut calendar = MyCalendar::new();
    println!("{}", calendar.book(10, 20));
    println!("{}", calendar.book(15, 25));
    println!("{}", calendar.book(20, 30));
}
