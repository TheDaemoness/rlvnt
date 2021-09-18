_rlvnt should be considered alpha-quality.
It is usable, but its interface and feature set will change._

# rlvnt
__A tool for extracting the broadly-defined "relevant" parts of logs.__

[![Build Status](https://github.com/TheDaemoness/rlvnt/actions/workflows/build.yml/badge.svg)](https://github.com/TheDaemoness/rlvnt/actions)

**rlvnt** is a tool to extract ranges of lines from text,
starting with the first line matches one of a set of "starting" patterns,
and ending with one of the following:
* The last line that matches one of the "starting" patterns.
* The next line that matches one of the "ending" patterns.

**rlvnt** will only fully buffer the input in worst-case scenarios.

## Regex Syntax

**rlvnt** uses the `regex` crate.
Documentation on its regex syntax can be found
[here](https://docs.rs/regex/1.5.*/regex/index.html#syntax).

## Basic Usage

Patterns can be specified using the following options:

* `-e`/`--regexp`: Specify a "starting" pattern.
* `-E`/`--regexp-end`: Specify an "ending" pattern.
* The first positional argument is assumed to be a "starting" pattern if `-e` is not used.
* `-v`/`--invert-match`: Invert matching for "starting" patterns.
* `-V`/`--invert-match-end`: Invert matching for "ending" patterns.

**rlvnt** tries to mimic **grep** usage patterns where possible.
The following flags/options roughly match the behavior found in GNU grep:

* `-` representing standard input in the file list.
* `-A`/`--after-context`
* `-B`/`--before-context`
* `-F`/`--fixed-strings`
* `-H`/`--with-filename`
* `-h`/`--no-filename`
* `-i`/`--ignore-case`
* `-x`/`--line-regexp`

For more information, run `rlvnt --help`.

## License

**rlvnt** is licened under the GPL v3 or any later version.
A copy can be found [here](https://www.gnu.org/licenses/gpl-3.0.txt).
