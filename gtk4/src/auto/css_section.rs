// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::CssLocation;
use glib::{prelude::*, translate::*};
use std::fmt;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct CssSection(Shared<ffi::GtkCssSection>);

    match fn {
        ref => |ptr| ffi::gtk_css_section_ref(ptr),
        unref => |ptr| ffi::gtk_css_section_unref(ptr),
        type_ => || ffi::gtk_css_section_get_type(),
    }
}

impl CssSection {
    #[doc(alias = "gtk_css_section_new")]
    pub fn new(
        file: Option<&impl IsA<gio::File>>,
        start: &CssLocation,
        end: &CssLocation,
    ) -> CssSection {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_css_section_new(
                file.map(|p| p.as_ref()).to_glib_none().0,
                start.to_glib_none().0,
                end.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_css_section_get_end_location")]
    #[doc(alias = "get_end_location")]
    pub fn end_location(&self) -> CssLocation {
        unsafe { from_glib_none(ffi::gtk_css_section_get_end_location(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_css_section_get_file")]
    #[doc(alias = "get_file")]
    pub fn file(&self) -> Option<gio::File> {
        unsafe { from_glib_none(ffi::gtk_css_section_get_file(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_css_section_get_parent")]
    #[doc(alias = "get_parent")]
    #[must_use]
    pub fn parent(&self) -> Option<CssSection> {
        unsafe { from_glib_none(ffi::gtk_css_section_get_parent(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_css_section_get_start_location")]
    #[doc(alias = "get_start_location")]
    pub fn start_location(&self) -> CssLocation {
        unsafe {
            from_glib_none(ffi::gtk_css_section_get_start_location(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_css_section_to_string")]
    #[doc(alias = "to_string")]
    pub fn to_str(&self) -> glib::GString {
        unsafe { from_glib_full(ffi::gtk_css_section_to_string(self.to_glib_none().0)) }
    }
}

impl fmt::Display for CssSection {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_str())
    }
}
