// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of rlvnt. https://github.com/TheDaemoness/rlvnt

use std::collections::{VecDeque,vec_deque};

pub struct Lines(VecDeque<String>);

impl Lines {
	pub fn new() -> Lines {Lines(VecDeque::new())}

	fn drain_max(&mut self, count: usize) -> vec_deque::Drain<String> {
		let end = self.0.len().min(count);
		self.0.drain(..end)
	}
}

impl super::Buffer for Lines {
	fn push(&mut self, line: String) {
		self.0.push_back(line);
	}

	fn for_n<F: FnMut(&str)>(&mut self, count: usize, mut f: F) {
		for line in self.drain_max(count) {
			f(line.as_str())
		}
	}
	fn for_all<F: FnMut(&str)>(&mut self, mut f: F) {
		for line in self.0.iter() {
			f(line.as_str())
		}
		self.0.clear()
	}

	fn drop(&mut self) {
		let _ = self.0.pop_front();
	}
	fn drop_n(&mut self, count: usize) {
		self.drain_max(count).for_each(std::mem::drop)
	}
	fn drop_all(&mut self) {
		self.0.clear();
	}
}
