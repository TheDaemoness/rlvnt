mod args;
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
	for linesrc in linesources {
		let mut printer = output::BuffingPrinter::new();
		output::print_line(linesrc.name());
		let mut is_inside = false;
		linesrc.for_lines(|line| {
			use matcher::MatchType as Mt;
			let (matches, new_inside) = match matchers.match_on(&line, is_inside) {
				Mt::NoMatch => (false, is_inside),
				Mt::Start   => (true, true),
			};
			is_inside = new_inside;
			printer.push(line, matches);
		});
	}
}
