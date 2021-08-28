_rlvnt should be considered alpha-quality.
It is usable, but its interface and feature set will change._

# rlvnt
__A tool for extracting the broadly-defined "relevant" parts of logs.__

[![Build Status](https://github.com/TheDaemoness/rlvnt/actions/workflows/build.yml/badge.svg)](https://github.com/TheDaemoness/rlvnt/actions)

**rlvnt** is a tool to extract ranges of lines from text.
Its focus is on extracting lines until the last line
where some condition is met.

**rlvnt** will only fully buffer the input in worst-case scenarios.

## Regex Syntax

**rlvnt** uses the `regex` crate.
Documentation on its regex syntax can be found
[here](https://docs.rs/regex/1.5.*/regex/index.html#syntax).

## Supported Options

**rlvnt** tries to mimic **grep** usage patterns where possible.
The following flags/options match the behavior found in GNU grep:

* `-` representing standard input in the file list.
* `-A`/`--after-context`
* `-B`/`--before-context`
* `-e`/`--regexp`
* `-F`/`--fixed-strings`
* `-H`/`--with-filename`
* `-h`/`--no-filename`
* `-i`/`--ignore-case`
* `-v`/`--invert-match`
* `-x`/`--line-regexp`

## License

**rlvnt** is licened under the GPL v3 or any later version.
A copy can be found [here](https://www.gnu.org/licenses/gpl-3.0.txt).
