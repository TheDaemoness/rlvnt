// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of rlvnt. https://github.com/TheDaemoness/rlvnt

use super::*;

// Begin boilerplate for tests.

const NADA: [&'static str; 0] = [];

use std::ffi::OsString;
macro_rules! parse {
	($($arg:expr),*) => {{
		let input = [OsString::from("rlvnt") $(,OsString::from($arg))*];
		parse_args(input)
	}}
}

fn assert_posargs(args: &PatternOptions, a: &[&str], b: &[&str]) {
	// Skip using assert_eq, we get nicer error messages this way.
	assert!(
		args.patterns_start() == a,
		"incorrect patterns: expected `{:?}`, got `{:?}`", a, args.patterns_start()
	);
	assert!(
		args.filenames() == b,
		"incorrect filenames: expected `{:?}`, got `{:?}`", b, args.filenames()
	);
}

// End boilerplate for tests.
// Begin tests.

#[test]
pub fn test_empty() {
	let args = parse!().expect("Parsing should not fail");
	assert_posargs(&args.pattern_opts, &NADA, &NADA);
}

#[test]
pub fn test_patterns_one() {
	let args = parse!("foo").expect("Parsing should not fail");
	assert_posargs(&args.pattern_opts, &["foo"], &NADA);
}

#[test]
pub fn test_patterns_many() {
	let args = parse!("-e", "foo", "-e", "bar").expect("Parsing should not fail");
	assert_posargs(&args.pattern_opts, &["foo", "bar"], &NADA);
}

#[test]
pub fn test_filenames() {
	let args = parse!("foo", "bar", "baz").expect("Parsing should not fail");
	assert_posargs(&args.pattern_opts, &["foo"], &["bar", "baz"]);
}

#[test]
pub fn test_e_and_filenames() {
	let args = parse!("-e", "foo", "bar", "baz").expect("Parsing should not fail");
	assert_posargs(&args.pattern_opts, &["foo"], &["bar", "baz"]);
}
