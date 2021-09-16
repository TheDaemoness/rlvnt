// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of rlvnt. https://github.com/TheDaemoness/rlvnt

mod engine;

use engine::Engine;
use crate::args::MatcherOptions;
use crate::errorlist::ErrorList;

enum MatcherInner {
	StartOnly(Engine),
	StartEnd(Engine, Engine)
}

pub struct Matcher(MatcherInner);

#[derive(Clone,Copy,PartialEq,Eq,Debug)]
pub enum MatchType {
	NoMatch,
	Start,
	End,
}

impl MatcherInner {
	pub fn match_on(&self, what: &str, is_inside: bool) -> MatchType {
		use MatcherInner::*;
		use MatchType as Mt;
		match self {
			StartOnly(s)  => if s.is_match(what) {return Mt::Start}
			StartEnd(s,e) => {
				let (matcher, result) = if is_inside {
					(&e, Mt::End)
				} else {
					(&s, Mt::Start)
				};
				if matcher.is_match(what) {return result}
			}
		}
		Mt::NoMatch
	}
}

impl Matcher {
	pub fn from_exact<IIt,Str>(patterns: IIt, opts: &MatcherOptions) -> Result<Matcher, ErrorList>
	where IIt: IntoIterator<Item = Str>, Str: AsRef<str> {
		Matcher::from_regexes(patterns.into_iter().map(|r| regex::escape(r.as_ref())), opts)
	}

	pub fn from_regexes<IIt,Str>(patterns: IIt, opts: &MatcherOptions) -> Result<Matcher, ErrorList>
	where IIt: IntoIterator<Item = Str>, Str: AsRef<str> {
		let engine = Engine::from_regexes(patterns.into_iter(), opts)?;
		Ok(Matcher(MatcherInner::StartOnly(engine)))
	}

	pub fn match_on(&self, what: &str, is_inside: bool) -> MatchType {
		self.0.match_on(what, is_inside)
	}
}
