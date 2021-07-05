mod args;
mod counter;
mod input;
mod matcher;
mod output;

fn main() {
	let (matchers, linesources) = {
		use clap::Clap;
		let mut opts = args::Args::parse();
		let mut linesources = Vec::<input::LineSource>::new();
		linesources.reserve(opts.files.len());
		if input::extend_with_linesources(opts.files.drain(0..), &mut linesources) {
			std::process::exit(1);
		}
		let patterns = std::iter::once(&opts.pattern);
		(
			if opts.fixed_strings {
				matcher::Matchers::from_exact(patterns, &opts.match_opts)
			} else {
				matcher::Matchers::from_regexes(patterns, &opts.match_opts)
			}.unwrap(),
			linesources,
		)
	};
	if linesources.len() > 1 {
		for linesrc in linesources {
			process_lines(&matchers, &linesrc, true)
		}
	} else {
		process_lines(&matchers, linesources.first().expect("linesources is somehow empty"), false)
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
