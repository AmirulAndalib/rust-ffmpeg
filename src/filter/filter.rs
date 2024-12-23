use std::{ffi::CStr, marker::PhantomData, str::from_utf8_unchecked};

use super::{Flags, Pad};
use crate::ffi::*;

pub struct Filter {
	ptr: *const AVFilter,
}

impl Filter {
	pub unsafe fn wrap(ptr: *const AVFilter) -> Self {
		Filter { ptr }
	}

	pub unsafe fn as_ptr(&self) -> *const AVFilter {
		self.ptr as *const _
	}

	pub unsafe fn as_mut_ptr(&mut self) -> *mut AVFilter {
		self.ptr as *mut _
	}
}

impl Filter {
	pub fn name(&self) -> &str {
		unsafe { from_utf8_unchecked(CStr::from_ptr((*self.as_ptr()).name).to_bytes()) }
	}

	pub fn description(&self) -> Option<&str> {
		unsafe {
			let ptr = (*self.as_ptr()).description;

			if ptr.is_null() {
				None
			}
			else {
				Some(from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes()))
			}
		}
	}

	pub fn inputs(&self) -> Option<PadIter<'_>> {
		unsafe {
			let ptr = (*self.as_ptr()).inputs;

			#[cfg(feature = "ffmpeg_5_0")]
			let count = (*self.as_ptr()).nb_inputs;

			#[cfg(not(feature = "ffmpeg_5_0"))]
			let count = avfilter_pad_count((*self.as_ptr()).inputs);

			if ptr.is_null() {
				None
			}
			else {
				Some(PadIter::new(ptr, count as usize))
			}
		}
	}

	pub fn outputs(&self) -> Option<PadIter<'_>> {
		unsafe {
			let ptr = (*self.as_ptr()).outputs;

			#[cfg(feature = "ffmpeg_5_0")]
			let count = (*self.as_ptr()).nb_outputs;

			#[cfg(not(feature = "ffmpeg_5_0"))]
			let count = avfilter_pad_count((*self.as_ptr()).outputs);

			if ptr.is_null() {
				None
			}
			else {
				Some(PadIter::new(ptr, count as usize))
			}
		}
	}

	pub fn flags(&self) -> Flags {
		unsafe { Flags::from_bits_truncate((*self.as_ptr()).flags) }
	}
}

pub struct PadIter<'a> {
	ptr: *const AVFilterPad,
	count: usize,
	cur: usize,

	_marker: PhantomData<&'a ()>,
}

impl<'a> PadIter<'a> {
	pub fn new(ptr: *const AVFilterPad, count: usize) -> Self {
		PadIter {
			ptr,
			count,
			cur: 0,
			_marker: PhantomData,
		}
	}
}

impl<'a> Iterator for PadIter<'a> {
	type Item = Pad<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		unsafe {
			if self.cur >= self.count {
				return None;
			}

			let pad = Pad::wrap(self.ptr, self.cur);
			self.cur += 1;

			Some(pad)
		}
	}
}
