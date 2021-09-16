// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of rlvnt. https://github.com/TheDaemoness/rlvnt

use std::borrow::Cow;
use regex::escape as regex_escape;

enum PatternInner<'a> {
	Fixed    (&'a str),
	FixedLine(&'a str),
	Regex    (&'a str),
	RegexLine(&'a str)
}

pub struct Pattern<'a>(PatternInner<'a>);

impl<'a> Pattern<'a> {
	pub fn new_fixed(string: &str, anchored: bool) -> Pattern<'_> {
		Pattern(
			if !anchored {PatternInner::Fixed(string)}
			else {PatternInner::FixedLine(string)}
		)
	}
	pub fn new_regex(regex: &str, anchored: bool) -> Pattern<'_> {
		Pattern(
			if !anchored {PatternInner::Regex(regex)}
			else {PatternInner::RegexLine(regex)}
		)
	}

	// Get the contents of regex patterns.
	pub fn get_regex(&self) ->  Option<Cow<'a, str>> {
		use PatternInner::*;
		match self.0 {
			Regex(s)     => Some(Cow::from(s)),
			RegexLine(s) => Some(Cow::from(regex_anchor(s))),
			_ => None
		}
	}

	// Convert other patterns into regex patterns where possible.
	pub fn make_regex(&self) -> Option<Cow<'a, str>> {
		use PatternInner::*;
		match self.0 {
			Fixed(s)     => Some(Cow::from(regex_escape(s))),
			FixedLine(s) => Some(Cow::from(regex_anchor(regex_escape(s).as_str()))),
			_ => self.get_regex()
		}
	}
}

fn regex_anchor(regex: &str) -> String {
	// https://github.com/rust-lang/regex/issues/675
	format!("\\A(?:{})\\z", regex)
}
