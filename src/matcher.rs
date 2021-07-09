// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of rlvnt. https://github.com/TheDaemoness/rlvnt

use regex::{RegexSet,RegexSetBuilder};
use crate::args::MatcherOptions;
use crate::errorlist::ErrorList;

enum MatcherEngine {
	Regexes(RegexSet)
}

struct Matcher {
	engine: MatcherEngine,
	invert: bool
}

pub struct Matchers(Matcher);

#[derive(Clone,Copy,PartialEq,Eq)]
pub enum MatchType {
	NoMatch,
	Start,
}

impl MatcherEngine {
	pub fn from_regexes<S,T>(patterns: T, opts: &MatcherOptions) -> Result<MatcherEngine, ErrorList>
	where S: AsRef<str>, T: Iterator<Item = S> {
		let mut rsb = if !opts.line_regexp {
			RegexSetBuilder::new(patterns)
		} else {
			// https://github.com/rust-lang/regex/issues/675
			RegexSetBuilder::new(patterns.map(
				|pattern| format!("\\A(?:{})\\z", pattern.as_ref())
			))
		};
		rsb.case_insensitive(opts.ignore_case);
		match rsb.build() {
			Ok(v)  => Ok(MatcherEngine::Regexes(v)),
			Err(regex::Error::Syntax(e)) => {
				let mut errs = ErrorList::new();
				push_regex_error(&mut errs, e);
				Err(errs)
			}
			Err(e) => Err(ErrorList::wrap(e.to_string()))
		}
	}
}

impl Matcher {
	pub fn is_match(&self, what: &str) -> bool {
		use MatcherEngine::*;
		match self.engine {
			Regexes(ref r)  =>  r.is_match(what) != self.invert,
		}
	}
}

impl Matchers {
	pub fn from_exact<S,T>(patterns: T, opts: &MatcherOptions) -> Result<Matchers, ErrorList>
	where S: AsRef<str>, T: Iterator<Item = S> {
		Matchers::from_regexes(patterns.map(|r| regex::escape(r.as_ref())), opts)
	}

	pub fn from_regexes<S,T>(patterns: T, opts: &MatcherOptions) -> Result<Matchers, ErrorList>
	where S: AsRef<str>, T: Iterator<Item = S> {
		let me = MatcherEngine::from_regexes(patterns, opts)?;
		Ok(Matchers(Matcher{
			engine: me,
			invert: opts.invert_match
		}))
	}

	pub fn match_on(&self, what: &str, _is_inside: bool) -> MatchType {
		use MatchType as Mt;
		if self.0.is_match(what) {Mt::Start} else {Mt::NoMatch}
	}
}

pub fn push_regex_error(errs: &mut ErrorList, e: String) {
	//WARNING: Fragile error message parsing, but all well, blame `regex`.
	let prefix_to_strip = "error: ";
	let mut lines = e.lines();
	let mut next_or_panic = || lines.next().expect("regex error reporting changed");
	let _       = next_or_panic();
	let pattern = next_or_panic().trim_start();
	let _       = next_or_panic();
	let message = next_or_panic().split_at(prefix_to_strip.len()).1;
	errs.push_about("(patterns)", format_args!("{} in {}", message, pattern))
}
