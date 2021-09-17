// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of rlvnt. https://github.com/TheDaemoness/rlvnt

use crate::pattern::Pattern;

pub struct Patterns<'a> {
	opts: &'a super::MatcherOptions,
	patterns: &'a [String]
}

impl<'a> Patterns<'a> {
	pub fn new_start(opts: &'a super::Args) -> Patterns<'a> {
		Patterns {
			opts: &opts.match_opts,
			patterns: opts.pattern_opts.patterns_start()
		}
	}

	pub fn is_empty(&self) -> bool {
		//TODO: When reading from files is implemented, this will be incorrect.
		self.patterns.is_empty()
	}

	pub fn matcher_opts(&self) -> &'a super::MatcherOptions {
		self.opts
	}

	fn wrap_plain_pattern(&self, pattern: &'a str) -> Pattern<'a> {
		if self.opts.fixed_strings {
			Pattern::new_fixed(pattern, self.opts.line_regexp)
		} else {
			Pattern::new_regex(pattern, self.opts.line_regexp)
		}
	}
}

impl<'a> Iterator for Patterns<'a> {
	type Item = crate::pattern::Pattern<'a>;
	fn next(&mut self) -> Option<Self::Item> {
		let string = crate::util::slice::take_first(&mut self.patterns)?;
		Some(self.wrap_plain_pattern(string.as_str()))
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		let len = self.patterns.len();
		//TODO: When reading from files is implemented, this will be incorrect.
		(len, Some(len))
	}
}

impl<'a> std::iter::FusedIterator for Patterns<'a> {}
