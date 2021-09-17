// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of rlvnt. https://github.com/TheDaemoness/rlvnt

mod engine;

use engine::Engine;
use crate::args::Patterns;
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
				if is_inside && e.is_match(what) {return Mt::End}
				if s.is_match(what) {return Mt::Start}
			}
		}
		Mt::NoMatch
	}
}

impl Matcher {
	pub fn new(start: Patterns<'_>, end: Patterns<'_>) -> Result<Matcher, ErrorList> {
		if start.is_empty() {
			return Err(crate::errorlist::ErrorList::wrap("no patterns specified"))
		}
		let engine_start = Engine::from_patterns(start)?;
		let matcher_inner =	if end.is_empty() {
			MatcherInner::StartOnly(engine_start)
		} else {
			let engine_end = Engine::from_patterns(end)?;
			MatcherInner::StartEnd(engine_start, engine_end)
		};
		Ok(Matcher(matcher_inner))
	}

	pub fn match_on(&self, what: &str, is_inside: bool) -> MatchType {
		self.0.match_on(what, is_inside)
	}
}
