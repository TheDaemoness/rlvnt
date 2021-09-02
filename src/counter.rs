// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of rlvnt. https://github.com/TheDaemoness/rlvnt

use crate::args::CounterOptions;
use crate::matcher::MatchType;

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

enum CounterState {
	Inside,
	OutsideDropping,
	OutsideBuffering(usize),
	OutsideCycling,
	OutsidePrinting(usize),
}

pub struct Counter {
	opts: crate::args::CounterOptions,
	state: CounterState
}

impl Counter {
	pub fn new(opts: CounterOptions) -> Counter {
		let state = Self::default_outside_state(&opts);
		Counter {
			opts,
			state
		}
	}

	fn default_outside_state(args: &CounterOptions) -> CounterState {
		use CounterState::*;
		if args.before_context > 0 {
			OutsideBuffering(0)
		} else {
			OutsideDropping
		}
	}

	pub fn is_in_block(&self) -> bool {
		matches!(self.state, CounterState::Inside)
	}

	pub fn lines_after(&self) -> usize {
		if self.is_in_block() {
			self.opts.after_context
		} else {
			0
		}
	}

	pub fn action_for_line(&mut self, mt: &MatchType) -> CounterAction {
		use CounterState as Cs;
		use CounterAction as Ca;
		use MatchType as Mt;
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
				Cs::OutsideCycling => Ca::Cycle,
				Cs::OutsidePrinting(count) => {
					if count == self.opts.after_context {
						self.state = Self::default_outside_state(&self.opts);
						self.action_for_line(mt)
					} else {
						self.state = Cs::OutsidePrinting(count);
						Ca::PrintOne
					}
				}
			},
			Mt::End => {
				self.state = if self.opts.after_context > 0 {
					Cs::OutsidePrinting(0)
				} else {
					Self::default_outside_state(&self.opts)
				};
				Ca::PrintAll
			}
		}
	}
}
