// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of rlvnt. https://github.com/TheDaemoness/rlvnt

use super::*;
use crate::args::CounterOptions;
use crate::matcher::MatchType;

use CounterAction::Buffer   as Cb;
//use CounterAction::Cycle    as Cc;
use CounterAction::Ignore   as Ci;
//use CounterAction::PrintAll as Cpa;
//use CounterAction::PrintOne as Cp1;

use MatchType::NoMatch as Mn;
//use MatchType::Start   as Ms;
//use MatchType::End     as Me;


fn test<IIt>(before_context: usize, after_context: usize, lines_after: usize, testpairs: IIt)
where IIt: IntoIterator<Item = (MatchType, CounterAction)> {
	let opts = CounterOptions{before_context, after_context};
	let mut counter = Counter::new(opts);
	let mut line = 1;
	for (input, expected) in testpairs {
		let output = counter.action_for_line(&input);
		assert!(
			output == expected,
			"incorrect action for #{} `{:?}`: expected `{:?}`, got `{:?}`",
			line, input, expected, output
		);
		line += 1;
	}
	let to_print = counter.lines_after();
	assert!(
		to_print == lines_after,
		"incorrect trailing printable lines: expected `{:?}`, got `{:?}`",
		lines_after, to_print
	);
}

#[test]
fn test_nomatch_once() {
	let contexts = [
		(0, 0, Ci, 0),
		(1, 0, Cb, 0),
		(0, 1, Ci, 0),
		(1, 1, Cb, 0)
	];
	for (before, after, result, lines_after) in contexts {
		test(before, after, lines_after, [(Mn, result)])
	}
}

