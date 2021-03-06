use crate::ffi::*;
use crate::util::*;
use ::objc::runtime::*;
use objc_foundation::{INSString, NSString};
use std::os::raw::c_void;
use std::ptr::NonNull;

/// Returns an Objective-C `Class` by it's name.
pub fn get_class(class: &str) -> *const Class {
	unsafe { objc_getClass(to_c_str(class)) }
}

/// Logs a string to the OSLog.
pub fn log(data: &str) {
	unsafe { NSLogv(&*NSString::from_str(data)) }
}

/// Hooks a Objective-C function, based on a `Class` and `Sel`.
pub fn hook(class: &str, selector: Sel, replacement: *mut c_void, orig: &mut Option<NonNull<Imp>>) {
	unsafe {
		MSHookMessageEx(get_class(class), selector, replacement, orig);
	}
}
