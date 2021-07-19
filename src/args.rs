// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of rlvnt. https://github.com/TheDaemoness/rlvnt

use clap::Clap;
use clap::AppSettings as As;

#[derive(Clap)]
#[clap(
	author, about, version,
	setting = As::UnifiedHelpMessage,
	setting = As::ColoredHelp,
	setting = As::NextLineHelp,
	override_usage = "rlvnt [OPTIONS] <pattern> [files]..."
)]
pub struct Args {
	#[clap(flatten)]
	pub counter_opts: CounterOptions,
	#[clap(flatten)]
	pub match_opts: MatcherOptions,
	/// Treat patterns as a strings to find instead of regexes.
	#[clap(long, short='F')]
	pub fixed_strings: bool,
	/// Include the filename as a prefix on matching lines.
	#[clap(long, short='H', overrides_with="no-filename")]
	pub with_filename: bool,
	/// Suppress inclusion of the filename as a prefix on matching lines.
	#[clap(long, short='h', overrides_with="with-filename")]
	pub no_filename: bool,
	/// The pattern to match.
	#[clap(required = true)]
	pub pattern: String,
	/// The list of files to examine. Use `-` for standard input.
	#[clap()]
	pub files: Vec<String>
}

#[derive(Clap)]
pub struct MatcherOptions {
	/// Ignore case when matching. Equivalent to lowercasing the text and the pattern before matching.
	#[clap(long, short='i')]
	pub ignore_case: bool,
	/// Consider lines that do NOT match as matching, and likes that do match as NOT matching.
	#[clap(long, short='v')]
	pub invert_match: bool,
	/// Require patterns to match entire lines to be considered matches.
	#[clap(long, short='x')]
	pub line_regexp: bool
}

#[derive(Clap, Clone)]
pub struct CounterOptions {
	/// Include COUNT additional lines after the last matching line in a block.
	#[clap(long, short='A', default_value = "0", hide_default_value = true, value_name = "COUNT")]
	pub after_context: usize,
	/// Include COUNT additional lines before the first matching line in a block.
	#[clap(long, short='B', default_value = "0", hide_default_value = true, value_name = "COUNT")]
	pub before_context: usize,
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
