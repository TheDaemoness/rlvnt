// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of rlvnt. https://github.com/TheDaemoness/rlvnt

use clap::Clap;
use crate::errorlist::ErrorList;

#[derive(Clap)]
#[clap(author, about, version)]
pub struct Args {
	#[clap(flatten)]
	pub counter_opts: CounterOptions,
	#[clap(flatten)]
	pub match_opts: MatcherOptions,
	#[clap(long, short='F')]
	pub fixed_strings: bool,
	#[clap(long, short='H', overrides_with="no-filename")]
	pub with_filename: bool,
	#[clap(long, short='h', overrides_with="with-filename")]
	pub no_filename: bool,
	#[clap(required = true)]
	pub pattern: String,
	#[clap()]
	pub files: Vec<String>
}

#[derive(Clap)]
pub struct MatcherOptions {
	#[clap(long, short='i')]
	pub ignore_case: bool,
	#[clap(long, short='v')]
	pub invert_match: bool,
	#[clap(long, short='x')]
	pub line_regexp: bool
}

#[derive(Clap, Clone)]
pub struct CounterOptions {
	//#[clap(long, short='A', default = 0)]
	//pub after_context: usize,
	//#[clap(long, short='B', default = 0)]
	//pub before_context: usize,
}

impl Args {
	pub fn should_prefix_lines(&self) -> Option<bool> {
		if !self.with_filename && !self.no_filename {
			None
		} else {
			Some(self.with_filename)
		}
	}
}
