// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of rlvnt. https://github.com/TheDaemoness/rlvnt

use crate::args::CounterOptions;
use crate::matcher::MatchType as Mt;

pub enum CounterAction {
	/// Discard the line.
	Ignore,
	/// Push the line onto the end of the buffer.
	Buffer,
	/// Pop a line from the buffer, then push a new one.
	Cycle,
	/// Print all lines from the buffer.
	PrintAll,
}

pub struct Counter {
	opts: crate::args::CounterOptions,
	in_block: bool
}

impl Counter {
	pub fn new(args: CounterOptions) -> Counter {
		Counter {
			opts: args,
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
				if self.in_block {
					CounterAction::Buffer
				} else {
					CounterAction::Ignore
				}
			}
		}
	}
}
