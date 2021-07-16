// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of rlvnt. https://github.com/TheDaemoness/rlvnt

use crate::*;

pub struct Engine {
	matchers:     matcher::Matchers,
	counter_opts: args::CounterOptions,
	printer:      printer::Printer,
}

impl Engine {
	pub fn new(opts: crate::args::Args) -> Result<Engine, errorlist::ErrorList> {
		let m = {
			let patterns = std::iter::once(&opts.pattern);
			if opts.fixed_strings {
				matcher::Matchers::from_exact(patterns, &opts.match_opts)
			} else {
				matcher::Matchers::from_regexes(patterns, &opts.match_opts)
			}?
		};
		Ok(Engine{
			matchers: m,
			counter_opts: opts.counter_opts,
			printer: printer::Printer::new()
		})
	}

	pub fn process(&mut self, linesrc: input::LineSource, print_prefix: bool) {
		let mut closure = if print_prefix {
			self.printer.closure_with_prefix(linesrc.name())
		} else {
			self.printer.closure()
		};
		let mut counter = counter::Counter::new(self.counter_opts.clone());
		let mut buffer = buffer::Lines::new();
		let matchers = &self.matchers;
		linesrc.for_lines(|line| {
			use counter::CounterAction as Ca;
			use buffer::Buffer;
			let in_block = counter.is_in_block();
			match counter.action_for_line(&matchers.match_on(&line, in_block)) {
				Ca::Ignore   => (),
				Ca::Buffer   => buffer.push(line),
				Ca::Cycle    => {
					buffer.drop();
					buffer.push(line);
				}
				Ca::PrintAll => {
					buffer.for_all(&mut closure);
					closure(line.as_str())
				}
			};
		});
	}
}
