// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of rlvnt. https://github.com/TheDaemoness/rlvnt

mod args;
mod buffer;
mod errorlist;
mod counter;
mod input;
mod matcher;
mod printer;

fn main() {
	use clap::Clap;
	let mut opts = args::Args::parse();
	//TODO: We have two unwraps in here. Improve stuff.
	let matchers = {
		let patterns = std::iter::once(&opts.pattern);
		errorlist::exit_if_err(if opts.fixed_strings {
			matcher::Matchers::from_exact(patterns, &opts.match_opts)
		} else {
			matcher::Matchers::from_regexes(patterns, &opts.match_opts)
		})
	};
	let linesources = errorlist::exit_if_err(opts.build_linesources());
	let has_multiple = linesources.len() > 1;
	let should_prefix = opts.should_prefix_lines().unwrap_or(has_multiple);
	let mut printer = printer::Printer::new();
	if has_multiple {
		for linesrc in linesources {
			process_lines(&matchers, &linesrc, &mut printer, should_prefix)
		}
	} else {
		process_lines(&matchers, linesources.first().expect("linesources is somehow empty"), &mut printer, should_prefix)
	}
}

// This is temporary.
fn process_lines(matchers: &matcher::Matchers, linesrc: &input::LineSource, printer: &mut printer::Printer, print_prefix: bool) {
	let mut closure = if print_prefix {
		printer.closure_with_prefix(linesrc.name())
	} else {
		printer.closure()
	};
	let mut counter = counter::Counter::new();
	let mut buffer = buffer::Lines::new();
	linesrc.for_lines(|line| {
		use counter::CounterAction as Ca;
		use buffer::Buffer;
		let in_block = counter.is_in_block();
		match counter.action_for_line(&matchers.match_on(&line, in_block)) {
			Ca::Ignore   => (),
			Ca::Buffer   => buffer.push(line),
			Ca::PrintAll => {
				buffer.for_all(&mut closure);
				closure(line.as_str())
			}
		};
	});
}
