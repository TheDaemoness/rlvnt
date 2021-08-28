pub fn empty_slice<'a, T>() -> &'a [T] {
	unsafe {
		std::slice::from_raw_parts(std::ptr::NonNull::dangling().as_ptr(), 0)
	}
}
