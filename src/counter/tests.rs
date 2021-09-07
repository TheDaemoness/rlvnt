// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of rlvnt. https://github.com/TheDaemoness/rlvnt

use super::*;
use crate::args::CounterOptions;
use crate::matcher::MatchType;

use CounterAction::Buffer   as Cb;
use CounterAction::Cycle    as Cc;
use CounterAction::Ignore   as Ci;
use CounterAction::PrintAll as Cpa;
use CounterAction::PrintOne as Cp1;

use MatchType::NoMatch as Mn;
use MatchType::Start   as Ms;
use MatchType::End     as Me;


fn test<IIt>(
	case: usize,
	before_context: usize, after_context: usize, lines_after: usize,
	testpairs: IIt
) where IIt: IntoIterator<Item = (MatchType, CounterAction)> {
	let opts = CounterOptions{before_context, after_context};
	let mut counter = Counter::new(opts);
	let mut line = 1;
	for (input, expected) in testpairs {
		let output = counter.action_for_line(&input);
		assert!(
			output == expected,
			"incorrect action for case #{} line #{} `{:?}`: expected `{:?}`, got `{:?}`",
			case, line, input, expected, output
		);
		line += 1;
	}
	let to_print = counter.lines_after();
	assert!(
		to_print == lines_after,
		"incorrect trailing printable lines for case #{}: expected `{:?}`, got `{:?}`",
		case, lines_after, to_print
	);
}

#[test]
fn test_nomatch_once() {
	// before, after, result, lines_after
	let cases = [
		(0, 0, Ci),
		(1, 0, Cb),
		(0, 1, Ci),
		(1, 1, Cb)
	];
	let mut i = 1;
	for (before, after, result) in cases {
		test(i, before, after, 0, [(Mn, result)]);
		i += 1;
	}
}

#[test]
fn test_end() {
	// before, after, results for:
	// 2 lines before, 1 line before, 1 line after, 2 lines after
	let cases = [
		(0, 0, Ci, Ci, Ci, Ci),
		(1, 0, Cb, Cc, Cb, Cc),
		(0, 1, Ci, Ci, Cp1, Ci),
		(1, 1, Cb, Cc, Cp1, Cb),
	];
	let mut i = 1;
	for (before, after, bef2, bef1, aft1, aft2) in cases {
		test(i, before, after, 0, [
			(Mn, bef2), (Mn, bef1),
			(Ms, Cpa), (Mn, Cb), (Me, Cpa),
			(Mn, aft1), (Mn, aft2)
		]);
		i += 1;
	}
}
