use clap::Clap;

#[derive(Clap)]
#[clap(author, about, version)]
pub struct Args {
	#[clap(flatten)]
	pub match_opts: MatcherOptions,
	#[clap(long, short='F')]
	pub fixed_strings: bool,
	#[clap(required = true)]
	pub pattern: String,
	#[clap()]
	pub files: Vec<String>
}

#[derive(Clap)]
pub struct MatcherOptions {
	#[clap(long, short='i')]
	pub ignore_case: bool,
	#[clap(long, short='v')]
	pub invert_match: bool,
	#[clap(long, short='x')]
	pub line_regexp: bool
}
