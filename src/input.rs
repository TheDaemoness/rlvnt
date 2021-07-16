// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of rlvnt. https://github.com/TheDaemoness/rlvnt

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::ops::FnMut;

use crate::errorlist::ErrorList;

pub enum LineSource {
	Stdin(std::io::Stdin),
	File(String, File)
}

pub struct LineSources(Vec<LineSource>);

impl LineSource {
	pub fn from_stdin() -> LineSource {
		LineSource::Stdin(std::io::stdin())
	}

	pub fn from_filename(filename: String) -> Result<LineSource,(String,std::io::Error)> {
		//^ We do actually want to take a filename here.
		let file = match File::open(&filename) {
			Ok(f) => f,
			Err(e) => return Err((filename, e))
		};
		Ok(LineSource::File(filename, file))
	}

	pub fn name(&self) -> &str {
		match self {
			LineSource::Stdin(_) => "(standard input)",
			LineSource::File(name, _) => name
		}
	}

	pub fn for_lines<F>(&self, cb: F)
	where F: FnMut(String) -> () {
		match self {
			LineSource::Stdin(stdin) =>
				stdin.lock().lines().filter_map(|x| { x.ok() }).for_each(cb),
			LineSource::File(_, file) =>
				BufReader::new(file).lines().filter_map(|x| { x.ok() }).for_each(cb)
		}
	}
}

impl LineSources {
	pub fn new(mut names: Vec<String>) -> Result<LineSources, ErrorList> {
		if names.is_empty() {
			return Ok(LineSources(vec![LineSource::from_stdin()]))
		}
		let mut errors = ErrorList::new();
		let mut linesources = Vec::<LineSource>::with_capacity(names.len());
		let mut got_stdin = false;
		for name in names.drain(..) {
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
		errors.or(LineSources(linesources))
	}

	pub fn has_multiple(&self) -> bool {
		self.0.len() > 1
	}

	pub fn drain_all<'a>(&'a mut self) -> impl Iterator<Item = LineSource> + 'a {
		self.0.drain(..)
	}
}
