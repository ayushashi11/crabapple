use ::objc::runtime::*;
use std::os::raw::{c_char, c_void};

extern "C" {
	pub fn OBJC_NSString(str: *const c_char) -> *mut c_void;
	pub fn OBJC_NSLog(str: *const c_char);
	pub fn NSLogv(nsFormat: *mut c_void); // format from inside rust or it dies
	pub fn MSHookMessageEx(
		class: *const Class,
		selector: Sel,
		replacement: *mut c_void,
		result: *mut c_void,
	);
	pub fn ptr_strip(address: *mut c_void) -> *mut c_void;
}
