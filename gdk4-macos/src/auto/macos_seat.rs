// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;

glib::wrapper! {
    #[doc(alias = "GdkMacosSeat")]
    pub struct MacosSeat(Object<ffi::GdkMacosSeat, ffi::GdkMacosSeatClass>) @extends gdk::Seat;

    match fn {
        type_ => || ffi::gdk_macos_seat_get_type(),
    }
}

impl MacosSeat {}
