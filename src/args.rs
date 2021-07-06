use clap::Clap;
use crate::errorlist::ErrorList;

#[derive(Clap)]
#[clap(author, about, version)]
pub struct Args {
	#[clap(flatten)]
	pub match_opts: MatcherOptions,
	#[clap(long, short='F')]
	pub fixed_strings: bool,
	#[clap(long, short='H', overrides_with="no-filename")]
	pub with_filename: bool,
	#[clap(long, short='h', overrides_with="with-filename")]
	pub no_filename: bool,
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

impl Args {
	pub fn build_linesources(&mut self) -> Result<Vec<crate::input::LineSource>,ErrorList> {
		use crate::input::LineSource;
		if self.files.is_empty() {
			return Ok(vec![LineSource::from_stdin()])
		}
		let mut errors = ErrorList::new();
		let mut linesources = Vec::<LineSource>::with_capacity(self.files.len());
		let mut got_stdin = false;
		for name in self.files.drain(0..) {
			if name == "-" {
				if !got_stdin {
					got_stdin = true;
					linesources.push(LineSource::from_stdin())
				}
			} else { match LineSource::from_filename(name) {
				Ok(ls) => linesources.push(ls),
				Err((name, e)) => errors.push_about(&name, e)
			}}
		}
		errors.or(linesources)
	}

	pub fn should_prefix_lines(&self) -> Option<bool> {
		if !self.with_filename && !self.no_filename {
			None
		} else {
			Some(self.with_filename)
		}
	}
}
