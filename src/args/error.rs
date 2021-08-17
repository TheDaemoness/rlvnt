// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of rlvnt. https://github.com/TheDaemoness/rlvnt

fn message(error: clap::Error) -> Result<String, clap::Error> {
	use clap::ErrorKind as Ek;
	let info = &error.info;
	match error.kind {
		Ek::InvalidValue            => Ok(format!("invalid value for `{}`: {}", info[0], info[1])),
		Ek::UnknownArgument         => Ok(format!("unknown argument `{}`", info[0])),
		Ek::InvalidSubcommand       => Ok(format!("invalid subcommand `{}`", info[0])),
		Ek::UnrecognizedSubcommand  => Ok(format!("unknown subcommand `{}`", info[0])),
		Ek::EmptyValue              => Ok(format!("invalid empty value for `{}`", info[0])),
		Ek::ValueValidation         => Ok(format!("invalid value for `{}`: {}", info[0], info[1])),
		Ek::TooManyValues           => Ok(format!("wrong value count for `{}`", info[0])),
		Ek::TooFewValues            => Ok(format!("wrong value count for `{}`", info[0])),
		Ek::WrongNumberOfValues     => Ok(format!("wrong value count for `{}`", info[0])),
		Ek::ArgumentConflict        => Ok(format!("conflicting arguments: {:?}", info)),
		Ek::NoEquals                => Ok(format!("missing equals for `{}`", info[0])),
		Ek::MissingRequiredArgument => Ok(format!("missing argument `{}`", info[0])),
		Ek::MissingSubcommand       => Ok(format!("missing subcommand")),
		Ek::UnexpectedMultipleUsage => Ok(format!("invalid multiple usage")),
		Ek::ArgumentNotFound        => Ok(format!("missing argument `{}`", info[0])),
		Ek::TooManyOccurrences      => Ok(format!("too many occurrences of `{}`", info[0])),
		Ek::InvalidUtf8             => Ok(format!("invalid UTF-8 in arguments")),
		Ek::Io                      => Err(error),
		Ek::Format                  => Err(error),
		Ek::DisplayHelp             => Err(error),
		Ek::DisplayVersion          => Err(error),
		Ek::DisplayHelpOnMissingArgumentOrSubcommand => Err(error),
	}
}

pub fn into_errorlist_or_exit(error: clap::Error) -> crate::errorlist::ErrorList {
	match message(error) {
		Ok(m)  => crate::errorlist::ErrorList::wrap(m),
		Err(e) => e.exit()
	}
}
