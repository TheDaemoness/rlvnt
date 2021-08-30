// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of rlvnt. https://github.com/TheDaemoness/rlvnt

pub fn empty_slice<'a, T>() -> &'a [T] {
	unsafe {
		std::slice::from_raw_parts(std::ptr::NonNull::dangling().as_ptr(), 0)
	}
}
