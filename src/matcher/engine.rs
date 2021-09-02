// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of rlvnt. https://github.com/TheDaemoness/rlvnt

use regex::{RegexSet,RegexSetBuilder};
use crate::args::MatcherOptions;
use crate::errorlist::ErrorList;

enum EngineInner {
	//TODO: Optimized plain text search?
	Regexes(RegexSet)
}

pub struct Engine {
	engine: EngineInner,
	invert: bool
}

impl Engine {
	pub fn from_regexes<S,T>(patterns: T, opts: &MatcherOptions) -> Result<Engine, ErrorList>
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
			Ok(v)  => Ok(Engine{
				engine: EngineInner::Regexes(v),
				invert: opts.invert_match
			}),
			Err(regex::Error::Syntax(e)) => {
				let mut errs = ErrorList::new();
				push_regex_error(&mut errs, e);
				Err(errs)
			}
			Err(e) => Err(ErrorList::wrap(e.to_string()))
		}
	}

	pub fn is_match(&self, what: &str) -> bool {
		use EngineInner::*;
		match self.engine {
			Regexes(ref r)  =>  r.is_match(what) != self.invert,
		}
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
