// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of rlvnt. https://github.com/TheDaemoness/rlvnt

use clap::Clap;
use clap::AppSettings as As;

const USAGE: &str = "rlvnt [OPTIONS...] <PATTERN> [FILES...]
    rlvnt [OPTIONS...] -e <PATTERN>... [FILES...]";

#[derive(Clap)]
#[clap(
	author, about, version,
	setting = As::UnifiedHelpMessage,
	setting = As::ColoredHelp,
	setting = As::NextLineHelp,
	override_usage = USAGE
)]
pub struct Args {
	#[clap(flatten)]
	pub counter_opts: CounterOptions,
	#[clap(flatten)]
	pub match_opts: MatcherOptions,
	/// Include the filename as a prefix on output lines.
	#[clap(long, short='H', overrides_with="no-filename")]
	pub with_filename: bool,
	/// Suppress inclusion of the filename as a prefix on output lines.
	#[clap(long, short='h', overrides_with="with-filename")]
	pub no_filename: bool,
	#[clap(flatten)]
	pub pattern_opts: PatternOptions,
}

/// Contains all pattern options AND the positional option list.
/// ALSO contains the files-to-search list for technical reasons.
#[derive(Clap)]
pub struct PatternOptions {
	/// Specify an ending pattern.
	/// If used multiple times, only one of the patterns must match.
	#[clap(
		long="regexp-end", short='E', value_name="PATTERN",
		multiple_values=false, multiple_occurrences=true,
		allow_hyphen_values=true
	)]
	patterns_end: Vec<String>,
	/// Specify a starting pattern.
	/// If used multiple times, only one of the patterns must match.
	#[clap(
		long="regexp", short='e', value_name="PATTERN",
		multiple_values=false, multiple_occurrences=true,
		allow_hyphen_values=true
	)]
	patterns: Vec<String>,
	#[clap(hidden=true)]
	positional: Vec<String>
}

#[derive(Clap)]
pub struct MatcherOptions {
	/// Treat patterns as fixed strings to find instead of regexes.
	#[clap(long, short='F')]
	pub fixed_strings: bool,
	/// Ignore case when matching.
	#[clap(long, short='i')]
	pub ignore_case: bool,
	/// Invert matching of starting patterns.
	/// Treat lines that match as if they do not match, and lines that don't match as if they match.
	#[clap(long, short='v')]
	pub invert_match: bool,
	/// Invert matching of ending patterns.
	/// Treat lines that match as if they do not match, and lines that don't match as if they match.
	#[clap(long, short='V')]
	pub invert_match_end: bool,
	/// Require patterns to match entire lines to be considered matches.
	#[clap(long, short='x')]
	pub line_regexp: bool
}

#[derive(Clap, Clone)]
pub struct CounterOptions {
	/// Include COUNT additional lines after the last line in a range.
	#[clap(long, short='A', default_value = "0", hide_default_value = true, value_name = "COUNT")]
	pub after_context: usize,
	/// Include COUNT additional lines before the first line in a range.
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

	pub fn filenames(&self) -> &[String] {
		self.pattern_opts.filenames()
	}
}

impl PatternOptions {
	fn has_positional_pattern(&self) -> bool {
		self.patterns.is_empty()
	}

	pub fn patterns_start(&self) -> &[String] {
		if self.has_positional_pattern() {
			self.positional.first().map_or_else(
				crate::util::slice::empty,
				std::slice::from_ref
			)
		} else {
			self.patterns.as_slice()
		}
	}

	pub fn patterns_end(&self) -> &[String] {
		self.patterns_end.as_slice()
	}

	pub fn filenames(&self) -> &[String] {
		if self.positional.is_empty() {
			crate::util::slice::empty()
		} else {
			let idx: usize = if self.has_positional_pattern() {1} else {0};
			self.positional.as_slice().split_at(idx).1
		}
	}
}
