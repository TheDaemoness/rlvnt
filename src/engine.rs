// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of rlvnt. https://github.com/TheDaemoness/rlvnt

use crate::*;

pub struct Engine {
	matchers:     matcher::Matcher,
	counter_opts: args::CounterOptions,
	printer:      printer::Printer,
}

impl Engine {
	pub fn new(opts: crate::args::Args) -> Result<Engine, errorlist::ErrorList> {
		let matchers = {
			let patterns = opts.pattern_opts.patterns_start();
			if patterns.is_empty() {
				return Err(errorlist::ErrorList::wrap("no patterns specified"))
			}
			if opts.match_opts.fixed_strings {
				matcher::Matcher::from_exact(patterns, &opts.match_opts)
			} else {
				matcher::Matcher::from_regexes(patterns, &opts.match_opts)
			}?
		};
		Ok(Engine{
			matchers,
			counter_opts: opts.counter_opts,
			printer: printer::Printer::new()
		})
	}

	pub fn process(&mut self, linesrc: input::LineSource, print_prefix: bool) {
		use buffer::Buffer;
		let mut print = if print_prefix {
			self.printer.closure_with_prefix(linesrc.name())
		} else {
			self.printer.closure()
		};
		let mut counter = counter::Counter::new(self.counter_opts.clone());
		let mut buffer = buffer::Lines::new();
		let matchers = &self.matchers;
		linesrc.for_lines(|line| {
			use counter::CounterAction as Ca;
			let is_in_bl = counter.is_in_block();
			match counter.action_for_line(&matchers.match_on(&line, is_in_bl)) {
				Ca::Ignore   => (),
				Ca::Buffer   => buffer.push(line),
				Ca::Cycle    => {
					buffer.drop();
					buffer.push(line);
				}
				Ca::PrintAll => {
					buffer.for_all(&mut print);
					print(line.as_str())
				}
				Ca::PrintOne => print(line.as_str())
			};
		});
		buffer.for_n(counter.lines_after(), &mut print);
	}
}
