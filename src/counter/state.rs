// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of rlvnt. https://github.com/TheDaemoness/rlvnt

use super::CounterAction as Ca;
use crate::args::CounterOptions;
use crate::matcher::MatchType as Mt;

#[derive(Clone,Copy,PartialEq,Eq,Debug)]
pub enum State {
	Inside,
	OutsideDropping,
	OutsideBuffering(usize),
	OutsideCycling,
	OutsidePrinting(usize),
}

impl State {
	pub fn update(&mut self, opts: &CounterOptions, mt: &Mt) -> Ca {
		use State as Cs;
		match mt {
			Mt::Start => {
				*self = Cs::Inside;
				Ca::PrintAll
			},
			Mt::NoMatch => match self {
				Cs::Inside => Ca::Buffer,
				Cs::OutsideDropping => Ca::Ignore,
				Cs::OutsideBuffering(count) => {
					if *count == opts.before_context {
						*self = Cs::OutsideCycling;
						Ca::Cycle
					} else {
						*self = Cs::OutsideBuffering(*count+1);
						Ca::Buffer
					}
				}
				Cs::OutsideCycling => Ca::Cycle,
				Cs::OutsidePrinting(count) => {
					if *count == opts.after_context {
						*self = default_outside_state(opts);
						self.update(opts, mt)
					} else {
						*self = Cs::OutsidePrinting(*count+1);
						Ca::PrintOne
					}
				}
			},
			Mt::End => match self {
				Cs::Inside => {
					*self = if opts.after_context > 0 {
						Cs::OutsidePrinting(0)
					} else {
						default_outside_state(opts)
					};
					Ca::PrintAll
				},
				_ => self.update(opts, &Mt::NoMatch)
			}
		}
	}

	pub fn is_inside_block(&self) -> bool {
		use State::*;
		matches!(self, Inside)
	}
}

pub fn default_outside_state(args: &CounterOptions) -> State {
	use State::*;
	if args.before_context > 0 {
		OutsideBuffering(0)
	} else {
		OutsideDropping
	}
}
