// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of rlvnt. https://github.com/TheDaemoness/rlvnt

mod lines;

pub use lines::Lines;

pub trait Buffer {
	fn push(&mut self, line: String);

	fn for_n<F: FnMut(&str)>(&mut self, count: usize, f: F);
	fn for_all<F: FnMut(&str)>(&mut self, f: F);

	fn drop(&mut self) {self.drop_n(1)}
	fn drop_n(&mut self, count: usize);
	fn drop_all(&mut self);
}

