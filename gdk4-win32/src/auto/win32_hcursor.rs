// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::{prelude::*, translate::*};
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GdkWin32HCursor")]
    pub struct Win32HCursor(Object<ffi::GdkWin32HCursor, ffi::GdkWin32HCursorClass>);

    match fn {
        type_ => || ffi::gdk_win32_hcursor_get_type(),
    }
}

impl Win32HCursor {
    pub fn is_destroyable(&self) -> bool {
        glib::ObjectExt::property(self, "destroyable")
    }

    pub fn display(&self) -> Option<gdk::Display> {
        glib::ObjectExt::property(self, "display")
    }
}

impl fmt::Display for Win32HCursor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Win32HCursor")
    }
}
