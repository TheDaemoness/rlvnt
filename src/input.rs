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
	where F: FnMut(String) {
		match self {
			LineSource::Stdin(stdin) =>
				stdin.lock().lines().filter_map(|x| { x.ok() }).for_each(cb),
			LineSource::File(_, file) =>
				BufReader::new(file).lines().filter_map(|x| { x.ok() }).for_each(cb)
		}
	}
}

//NOTE: Performance microoptimization is possible here,
//but isn't worth the code quality degradation.

impl LineSources {
	pub fn new<IIt, AsStr>(names: IIt) -> Result<LineSources, ErrorList>
	where IIt: IntoIterator<Item = AsStr>, AsStr: AsRef<str> {
		let it = names.into_iter();
		let size = it.size_hint().1.unwrap_or(1);
		let mut linesources = Vec::<LineSource>::with_capacity(size);
		let mut errors = ErrorList::new();
		let mut got_stdin = false;
		for name_raw in it {
			let name: &str = name_raw.as_ref();
			if name == "-" {
				if !got_stdin {
					got_stdin = true;
					linesources.push(LineSource::from_stdin())
				}
			} else { match LineSource::from_filename(name.to_owned()) {
				Ok(ls)         => linesources.push(ls),
				Err((name, e)) => errors.push_about(&name, e)
			}}
		}
		if linesources.is_empty() {
			linesources.push(LineSource::from_stdin())
		}
		errors.or(LineSources(linesources))
	}

	pub fn has_multiple(&self) -> bool {
		self.0.len() > 1
	}

	pub fn drain_all(&mut self) -> impl Iterator<Item = LineSource> + '_ {
		self.0.drain(..)
	}
}
