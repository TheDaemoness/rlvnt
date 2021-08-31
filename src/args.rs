// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of rlvnt. https://github.com/TheDaemoness/rlvnt

#[allow(clippy::module_inception)]
mod args;
mod error;

pub use args::*;
pub use error::*;

/// Parse args. Exit if either --version or --help are specified, printing a message.
pub fn parse_args<IIt, Str>(what: IIt) -> Result<Args, crate::errorlist::ErrorList>
where IIt: IntoIterator<Item = Str>, Str: Into<std::ffi::OsString> + Clone {
	use clap::Clap;
	Args::try_parse_from(what).map_err(into_errorlist_or_exit)
}
