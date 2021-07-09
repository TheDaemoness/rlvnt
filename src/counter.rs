// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of rlvnt. https://github.com/TheDaemoness/rlvnt

use super::matcher::MatchType as Mt;

pub enum CounterAction {
	Ignore,
	Buffer,
	PrintAll,
}

pub struct Counter {
	in_block: bool
}

impl Counter {
	pub fn new() -> Counter {
		Counter {
			in_block: false
		}
	}
	pub fn is_in_block(&self) -> bool {
		self.in_block
	}
	pub fn action_for_line(&mut self, mt: &Mt) -> CounterAction {
		match mt {
			Mt::Start => {
				self.in_block = true;
				CounterAction::PrintAll
			},
			Mt::NoMatch => {
				if self.in_block {CounterAction::Buffer} else {CounterAction::Ignore}
			}
		}
	}
}
