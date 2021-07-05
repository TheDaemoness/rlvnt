use regex::{RegexSet,RegexSetBuilder};

#[derive(clap::Clap)]
pub struct MatcherOptions {
	#[clap(long, short='i')]
	ignore_case: bool,
	#[clap(long, short='v')]
	invert_match: bool,
}

enum Matcher {
	Basic(RegexSet),
	Invert(RegexSet)
}

impl Matcher {
	pub fn from_regexes<S,T>(patterns: T, opts: &MatcherOptions) -> Result<Matcher, MatcherError>
	where S: AsRef<str>, T: Iterator<Item = S> {
		let mut rsb = RegexSetBuilder::new(patterns);
		rsb.case_insensitive(opts.ignore_case);
		match rsb.build() {
			Ok(v)  => Ok(
				if opts.invert_match {
					Matcher::Invert(v)
				} else {
					Matcher::Basic(v)
				}
			),
			Err(e) => Err(MatcherError::BuildRegexSet(e))
		}
	}

	pub fn is_match(&self, what: &str) -> bool {
		use Matcher::*;
		match self {
			Basic(m)  =>  m.is_match(what),
			Invert(m) => !m.is_match(what)
		}
	}
}

#[derive(Debug)]
pub enum MatcherError {
	BuildRegexSet(regex::Error)
}

pub struct Matchers {
	m: Matcher
}

#[derive(Clone,Copy,PartialEq,Eq)]
pub enum MatchType {
	NoMatch,
	Start,
}

impl Matchers {
	pub fn from_exact<S,T>(patterns: T, opts: &MatcherOptions) -> Result<Matchers, MatcherError>
	where S: AsRef<str>, T: Iterator<Item = S> {
		Matchers::from_regexes(patterns.map(|r| regex::escape(r.as_ref())), opts)
	}

	pub fn from_regexes<S,T>(patterns: T, opts: &MatcherOptions) -> Result<Matchers, MatcherError>
	where S: AsRef<str>, T: Iterator<Item = S> {
		Matcher::from_regexes(patterns, opts).map(|v| Matchers{m:v})
	}

	pub fn match_on(&self, what: &str, _is_inside: bool) -> MatchType {
		use MatchType as Mt;
		if self.m.is_match(what) {Mt::Start} else {Mt::NoMatch}
	}
}
