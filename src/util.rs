// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of rlvnt. https://github.com/TheDaemoness/rlvnt

pub mod slice {
	pub fn empty<'a, T>() -> &'a [T] {
		unsafe {
			std::slice::from_raw_parts(std::ptr::NonNull::dangling().as_ptr(), 0)
		}
	}

	pub fn take_first<'a, T>(slice: &mut &'a [T]) -> Option<&'a T> {
		if let Some((head, tail)) = slice.split_first() {
			*slice = tail;
			Some(head)
		} else {None}
	}
}
