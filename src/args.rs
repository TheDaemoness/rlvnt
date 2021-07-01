use clap::Clap;

#[derive(Clap)]
#[clap(author, about, version)]
pub struct Args {
	#[clap(flatten)]
	pub match_opts: crate::matcher::MatcherOptions,
	#[clap(long, short='F')]
	pub fixed_strings: bool,
	#[clap(required = true)]
	pub pattern: String,
	#[clap()]
	pub files: Vec<String>
}
