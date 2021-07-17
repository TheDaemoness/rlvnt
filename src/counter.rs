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

enum CounterState {
	Inside,
	OutsideDropping,
	OutsideBuffering(usize),
	OutsideCycling,
}

pub struct Counter {
	opts: crate::args::CounterOptions,
	state: CounterState
}

impl Counter {
	pub fn new(args: CounterOptions) -> Counter {
		use CounterState::*;
		let default_state = if args.before_context > 0 {
			OutsideBuffering(0)
		} else {
			OutsideDropping
		};
		Counter {
			opts: args,
			state: default_state
		}
	}
	pub fn is_in_block(&self) -> bool {
		matches!(self.state, CounterState::Inside)
	}
	pub fn action_for_line(&mut self, mt: &Mt) -> CounterAction {
		use CounterState as Cs;
		use CounterAction as Ca;
		match mt {
			Mt::Start => {
				self.state = Cs::Inside;
				Ca::PrintAll
			},
			Mt::NoMatch => match self.state {
				Cs::Inside => Ca::Buffer,
				Cs::OutsideDropping => Ca::Ignore,
				Cs::OutsideBuffering(count) => {
					if count == self.opts.before_context {
						self.state = Cs::OutsideCycling;
						Ca::Cycle
					} else {
						self.state = Cs::OutsideBuffering(count+1);
						Ca::Buffer
					}
				}
				Cs::OutsideCycling => Ca::Cycle
			}
		}
	}
	pub fn lines_after(&self) -> usize {
		if self.is_in_block() {
			self.opts.after_context
		} else {
			0
		}
	}
}
