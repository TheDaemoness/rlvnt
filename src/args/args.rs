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
	/// Include the filename as a prefix on matching lines.
	#[clap(long, short='H', overrides_with="no-filename")]
	pub with_filename: bool,
	/// Suppress inclusion of the filename as a prefix on matching lines.
	#[clap(long, short='h', overrides_with="with-filename")]
	pub no_filename: bool,
	#[clap(flatten)]
	pub pattern_opts: PatternOptions,
}

/// Contains all pattern options AND the positional option list.
/// ALSO contains the files-to-search list for technical reasons.
#[derive(Clap)]
pub struct PatternOptions {
	/*
	/// Add a pattern indicating the end of a block.
	/// If used multiple times, match any of the specified patterns.
	#[clap(
		long="regexp-end", short='E', value_name="PATTERN",
		multiple_values=false, multiple_occurrences=true
	)]
	patterns_end: Vec<String>,
	*/
	/// Add a pattern indicating the start of a block.
	/// If used multiple times, match any of the specified patterns.
	#[clap(
		long="regexp", short='e', value_name="PATTERN",
		multiple_values=false, multiple_occurrences=true
	)]
	patterns: Vec<String>,
	#[clap(hidden=true)]
	positional: Vec<String>
}

#[derive(Clap)]
pub struct MatcherOptions {
	/// Treat patterns as a strings to find instead of regexes.
	#[clap(long, short='F')]
	pub fixed_strings: bool,
	/// Ignore case when matching.
	/// Equivalent to lowercasing the text and the pattern before matching.
	#[clap(long, short='i')]
	pub ignore_case: bool,
	/// Invert matching.
	/// Treat lines that match as if they do not match, and lines that don't match as if they match.
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
				crate::util::empty_slice,
				std::slice::from_ref
			)
		} else {
			self.patterns.as_slice()
		}
	}

	//pub fn patterns_end(&self) -> &[String] {
	//	self.patterns_end.as_slice()
	//}

	pub fn filenames(&self) -> &[String] {
		if self.positional.is_empty() {
			crate::util::empty_slice()
		} else {
			let idx: usize = if self.has_positional_pattern() {1} else {0};
			self.positional.as_slice().split_at(idx).1
		}
	}
}
