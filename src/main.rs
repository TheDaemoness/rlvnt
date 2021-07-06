mod args;
mod errorlist;
mod counter;
mod input;
mod matcher;
mod output;

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
	if has_multiple {
		for linesrc in linesources {
			process_lines(&matchers, &linesrc, should_prefix)
		}
	} else {
		process_lines(&matchers, linesources.first().expect("linesources is somehow empty"), should_prefix)
	}
}

// This is temporary.
fn process_lines(matchers: &matcher::Matchers, linesrc: &input::LineSource, print_prefix: bool) {
	let mut printer = output::BuffingPrinter::new();
	let prefix = if print_prefix {linesrc.name()} else {""};
	let mut counter = counter::Counter::new();
	linesrc.for_lines(|line| {
		use counter::CounterAction as Ca;
		let in_block = counter.is_in_block();
		match counter.action_for_line(&matchers.match_on(&line, in_block)) {
			Ca::Ignore   => (),
			Ca::Buffer   => printer.push(line),
			Ca::PrintAll => printer.print_all(line, prefix)
		};
	});
}
