use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::ops::FnMut;

pub enum LineSource {
	Stdin(std::io::Stdin),
	File(String, File)
}

impl LineSource {
	pub fn from_stdin() -> LineSource {
		LineSource::Stdin(std::io::stdin())
	}

	pub fn from_filename(filename: String) -> Result<LineSource,std::io::Error> {
		//^ We do actually want to take a filename here.
		let file = File::open(&filename)?;
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
