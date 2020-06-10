use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::ops::FnMut;

pub enum LineSource {
	Stdin(std::io::Stdin),
	File(String, File)
}

impl LineSource {
	fn from_stdin() -> LineSource {
		LineSource::Stdin(std::io::stdin())
	}

	fn from_file(filename: String) -> Option<LineSource> {
		//^ We do actually want to take a filename here.
		let file = File::open(&filename);
		if file.is_err() {
			eprintln!("Could not open '{}': {}", &filename, file.unwrap_err());
			None
		} else {
			Some(LineSource::File(filename, file.unwrap()))
		}
	}

	pub fn name(&self) -> &str {
		match self {
			LineSource::Stdin(_) => "stdin",
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

pub fn extend_with_linesources<It, C>(it: It, c: &mut C) -> bool
where It: Iterator<Item = String>, C: Extend<LineSource>{
	let mut got_stdin = false;
	let mut got_any = false;
	let mut had_problem = false;
	c.extend(it.filter_map(|name| {
		got_any = true;
		if name == "-" {
			if !got_stdin {
				got_stdin = true;
				Some(LineSource::from_stdin())
			} else {
				None
			}
		} else {
			let opened = LineSource::from_file(name);
			had_problem = had_problem && opened.is_none();
			opened
		}
	}));
	if !got_any {
		c.extend(std::iter::once(LineSource::from_stdin()));
	}
	had_problem
}
