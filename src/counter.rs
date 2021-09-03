// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of rlvnt. https://github.com/TheDaemoness/rlvnt

mod state;
#[cfg(test)]
mod tests;

use state::State;
use crate::args::CounterOptions;
use crate::matcher::MatchType;

#[derive(Clone,Copy,PartialEq,Eq,Debug)]
pub enum CounterAction {
	/// Discard the line.
	Ignore,
	/// Push the line onto the end of the buffer.
	Buffer,
	/// Pop a line from the buffer, then push a new one.
	Cycle,
	/// Print all lines from the buffer.
	PrintAll,
	/// Print this line only.
	PrintOne,
}

pub struct Counter {
	opts: crate::args::CounterOptions,
	state: State
}

impl Counter {
	pub fn new(opts: CounterOptions) -> Counter {
		let state = state::default_outside_state(&opts);
		Counter {
			opts,
			state
		}
	}

	pub fn is_in_block(&self) -> bool {
		self.state.is_inside_block()
	}

	pub fn lines_after(&self) -> usize {
		if self.is_in_block() {
			self.opts.after_context
		} else {
			0
		}
	}

	pub fn action_for_line(&mut self, mt: &MatchType) -> CounterAction {
		self.state.update(&self.opts, mt)
	}
}
