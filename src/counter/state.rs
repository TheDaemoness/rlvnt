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

pub fn default_outside_state(args: &CounterOptions) -> State {
	use State::*;
	if args.before_context > 0 {
		OutsideBuffering(0)
	} else {
		OutsideDropping
	}
}

pub fn update(state: &mut State, opts: &CounterOptions, mt: &Mt) -> Ca {
	use State as Cs;
	match mt {
		Mt::Start => {
			*state = Cs::Inside;
			Ca::PrintAll
		},
		Mt::NoMatch => match state {
			Cs::Inside => Ca::Buffer,
			Cs::OutsideDropping => Ca::Ignore,
			Cs::OutsideBuffering(count) => {
				if *count == opts.before_context {
					*state = Cs::OutsideCycling;
					Ca::Cycle
				} else {
					*state = Cs::OutsideBuffering(*count+1);
					Ca::Buffer
				}
			}
			Cs::OutsideCycling => Ca::Cycle,
			Cs::OutsidePrinting(count) => {
				if *count == opts.after_context {
					*state = default_outside_state(opts);
					update(state, opts, mt)
				} else {
					*state = Cs::OutsidePrinting(*count);
					Ca::PrintOne
				}
			}
		},
		Mt::End => {
			*state = if opts.after_context > 0 {
				Cs::OutsidePrinting(0)
			} else {
				default_outside_state(&opts)
			};
			Ca::PrintAll
		}
	}
}
