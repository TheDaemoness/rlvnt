use regex::{RegexSet,RegexSetBuilder};
use crate::args::MatcherOptions;

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

#[derive(Debug)]
pub enum MatcherError {
	BuildRegexSet(regex::Error)
}

impl MatcherEngine {
	pub fn from_regexes<S,T>(patterns: T, opts: &MatcherOptions) -> Result<MatcherEngine, MatcherError>
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
			Err(e) => Err(MatcherError::BuildRegexSet(e))
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
	pub fn from_exact<S,T>(patterns: T, opts: &MatcherOptions) -> Result<Matchers, MatcherError>
	where S: AsRef<str>, T: Iterator<Item = S> {
		Matchers::from_regexes(patterns.map(|r| regex::escape(r.as_ref())), opts)
	}

	pub fn from_regexes<S,T>(patterns: T, opts: &MatcherOptions) -> Result<Matchers, MatcherError>
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
