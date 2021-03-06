use Error;
use glib::GString;
use glib::translate::*;
use std::ptr;
use vte_sys;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Regex(Shared<vte_sys::VteRegex>);

    match fn {
        ref => |ptr| vte_sys::vte_regex_ref(ptr);
        unref => |ptr| vte_sys::vte_regex_unref(ptr),
        get_type => || vte_sys::vte_regex_get_type(),
    }
}

impl Regex {
    pub fn new_for_match(pattern: &str, flags: u32) -> Result<Regex, Error> {
        assert_initialized_main_thread!();
        let pattern_length = pattern.len() as isize;
        unsafe {
            let mut error = ptr::null_mut();
            let ret = vte_sys::vte_regex_new_for_match(pattern.to_glib_none().0, pattern_length, flags, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn new_for_search(pattern: &str, flags: u32) -> Result<Regex, Error> {
        assert_initialized_main_thread!();
        let pattern_length = pattern.len() as isize;
        unsafe {
            let mut error = ptr::null_mut();
            let ret = vte_sys::vte_regex_new_for_search(pattern.to_glib_none().0, pattern_length, flags, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn jit(&self, flags: u32) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = vte_sys::vte_regex_jit(self.to_glib_none().0, flags, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn substitute(&self, subject: &str, replacement: &str, flags: u32) -> Result<GString, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = vte_sys::vte_regex_substitute(self.to_glib_none().0, subject.to_glib_none().0, replacement.to_glib_none().0, flags, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }
}
