// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Event;
use glib::{prelude::*, translate::*};
use std::{fmt, mem};

glib::wrapper! {
    #[doc(alias = "GdkPadEvent")]
    pub struct PadEvent(Shared<ffi::GdkPadEvent>);

    match fn {
        ref => |ptr| ffi::gdk_event_ref(ptr as *mut ffi::GdkEvent),
        unref => |ptr| ffi::gdk_event_unref(ptr as *mut ffi::GdkEvent),
    }
}

impl glib::StaticType for PadEvent {
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gdk_pad_event_get_type()) }
    }
}

impl PadEvent {
    #[doc(alias = "gdk_pad_event_get_axis_value")]
    #[doc(alias = "get_axis_value")]
    pub fn axis_value(&self) -> (u32, f64) {
        unsafe {
            let mut index = mem::MaybeUninit::uninit();
            let mut value = mem::MaybeUninit::uninit();
            ffi::gdk_pad_event_get_axis_value(
                self.to_glib_none().0,
                index.as_mut_ptr(),
                value.as_mut_ptr(),
            );
            (index.assume_init(), value.assume_init())
        }
    }

    #[doc(alias = "gdk_pad_event_get_button")]
    #[doc(alias = "get_button")]
    pub fn button(&self) -> u32 {
        unsafe { ffi::gdk_pad_event_get_button(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_pad_event_get_group_mode")]
    #[doc(alias = "get_group_mode")]
    pub fn group_mode(&self) -> (u32, u32) {
        unsafe {
            let mut group = mem::MaybeUninit::uninit();
            let mut mode = mem::MaybeUninit::uninit();
            ffi::gdk_pad_event_get_group_mode(
                self.to_glib_none().0,
                group.as_mut_ptr(),
                mode.as_mut_ptr(),
            );
            (group.assume_init(), mode.assume_init())
        }
    }
}

impl fmt::Display for PadEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("PadEvent")
    }
}
