mod input;
mod output;

use clap::Clap;

#[derive(Clap)]
#[clap(version = "0.1.0")]
struct Opts {
	#[clap(long, short="i")]
	ignore_case: bool,
	#[clap(long, short="v")]
	invert_match: bool,
	#[clap(long, short="F")]
	fixed_strings: bool,
	#[clap(required = true)]
	pattern: String,
	#[clap()]
	files: Vec<String>
}

fn main() {
	let (matchers, linesources, should_match) = {
		let mut opts = Opts::parse();
		let patterns = std::iter::once(opts.pattern);
		let mut rsb = if opts.fixed_strings {
			regex::RegexSetBuilder::new(patterns.map(|p| { regex::escape(&p) }))
		} else {
			regex::RegexSetBuilder::new(patterns)
		};
		rsb.case_insensitive(opts.ignore_case);
		let mut linesources = Vec::<input::LineSource>::new();
		linesources.reserve(opts.files.len());
		if input::extend_with_linesources(opts.files.drain(0..), &mut linesources) {
			std::process::exit(1);
		}
		(
			rsb.build().unwrap(),
			linesources,
			!opts.invert_match
		)
	};
	for linesrc in linesources {
		let mut printer = output::BuffingPrinter::new();
		output::print_line(linesrc.name());
		linesrc.for_lines(|line| {
			let matched = matchers.is_match(&line) == should_match;
			printer.push(line, matched);
		});
	}
}
