// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of rlvnt. https://github.com/TheDaemoness/rlvnt

#[derive(Clone, Debug)]
pub struct ErrorList(Vec<String>);

impl ErrorList {
	pub fn new() -> Self {
		ErrorList(Vec::with_capacity(0))
	}
	pub fn wrap<E: ToString>(e: E) -> Self {
		ErrorList(vec![e.to_string()])
	}
	pub fn push<E: ToString>(&mut self, e: E) {
		self.0.push(e.to_string())
	}
	pub fn push_about<E: std::fmt::Display>(&mut self, name: &str, e: E) {
		self.push(format!("{}: {}", name, e))
	}

	pub fn or<T>(self, ok: T) -> Result<T, Self> {
		if self.0.is_empty() {Ok(ok)}
		else {Err(self)}
	}

	pub fn print_all(self) {
		for e in self.0 {
			eprintln!("error: {}", e);
		}
	}
}
