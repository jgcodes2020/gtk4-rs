// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Display;
use crate::Rectangle;
use crate::SubpixelLayout;
use glib::object::Cast;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::glib_wrapper! {
    pub struct Monitor(Object<ffi::GdkMonitor, ffi::GdkMonitorClass>);

    match fn {
        get_type => || ffi::gdk_monitor_get_type(),
    }
}

impl Monitor {
    #[doc(alias = "gdk_monitor_get_connector")]
    pub fn get_connector(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gdk_monitor_get_connector(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_monitor_get_display")]
    pub fn get_display(&self) -> Option<Display> {
        unsafe { from_glib_none(ffi::gdk_monitor_get_display(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_monitor_get_geometry")]
    pub fn get_geometry(&self) -> Rectangle {
        unsafe {
            let mut geometry = Rectangle::uninitialized();
            ffi::gdk_monitor_get_geometry(self.to_glib_none().0, geometry.to_glib_none_mut().0);
            geometry
        }
    }

    #[doc(alias = "gdk_monitor_get_height_mm")]
    pub fn get_height_mm(&self) -> i32 {
        unsafe { ffi::gdk_monitor_get_height_mm(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_monitor_get_manufacturer")]
    pub fn get_manufacturer(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gdk_monitor_get_manufacturer(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_monitor_get_model")]
    pub fn get_model(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gdk_monitor_get_model(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_monitor_get_refresh_rate")]
    pub fn get_refresh_rate(&self) -> i32 {
        unsafe { ffi::gdk_monitor_get_refresh_rate(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_monitor_get_scale_factor")]
    pub fn get_scale_factor(&self) -> i32 {
        unsafe { ffi::gdk_monitor_get_scale_factor(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_monitor_get_subpixel_layout")]
    pub fn get_subpixel_layout(&self) -> SubpixelLayout {
        unsafe { from_glib(ffi::gdk_monitor_get_subpixel_layout(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_monitor_get_width_mm")]
    pub fn get_width_mm(&self) -> i32 {
        unsafe { ffi::gdk_monitor_get_width_mm(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_monitor_is_valid")]
    pub fn is_valid(&self) -> bool {
        unsafe { from_glib(ffi::gdk_monitor_is_valid(self.to_glib_none().0)) }
    }

    pub fn get_property_valid(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"valid\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `valid` getter")
                .unwrap()
        }
    }

    pub fn connect_invalidate<F: Fn(&Monitor) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn invalidate_trampoline<F: Fn(&Monitor) + 'static>(
            this: *mut ffi::GdkMonitor,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"invalidate\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    invalidate_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_connector_notify<F: Fn(&Monitor) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_connector_trampoline<F: Fn(&Monitor) + 'static>(
            this: *mut ffi::GdkMonitor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::connector\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_connector_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_geometry_notify<F: Fn(&Monitor) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_geometry_trampoline<F: Fn(&Monitor) + 'static>(
            this: *mut ffi::GdkMonitor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::geometry\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_geometry_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_height_mm_notify<F: Fn(&Monitor) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_height_mm_trampoline<F: Fn(&Monitor) + 'static>(
            this: *mut ffi::GdkMonitor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::height-mm\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_height_mm_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_manufacturer_notify<F: Fn(&Monitor) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_manufacturer_trampoline<F: Fn(&Monitor) + 'static>(
            this: *mut ffi::GdkMonitor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::manufacturer\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_manufacturer_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_model_notify<F: Fn(&Monitor) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_model_trampoline<F: Fn(&Monitor) + 'static>(
            this: *mut ffi::GdkMonitor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::model\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_model_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_refresh_rate_notify<F: Fn(&Monitor) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_refresh_rate_trampoline<F: Fn(&Monitor) + 'static>(
            this: *mut ffi::GdkMonitor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::refresh-rate\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_refresh_rate_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_scale_factor_notify<F: Fn(&Monitor) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_scale_factor_trampoline<F: Fn(&Monitor) + 'static>(
            this: *mut ffi::GdkMonitor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::scale-factor\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_scale_factor_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_subpixel_layout_notify<F: Fn(&Monitor) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_subpixel_layout_trampoline<F: Fn(&Monitor) + 'static>(
            this: *mut ffi::GdkMonitor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::subpixel-layout\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_subpixel_layout_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_valid_notify<F: Fn(&Monitor) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_valid_trampoline<F: Fn(&Monitor) + 'static>(
            this: *mut ffi::GdkMonitor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::valid\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_valid_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_width_mm_notify<F: Fn(&Monitor) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_width_mm_trampoline<F: Fn(&Monitor) + 'static>(
            this: *mut ffi::GdkMonitor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::width-mm\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_width_mm_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[derive(Clone, Default)]
pub struct MonitorBuilder {
    display: Option<Display>,
}

impl MonitorBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Monitor {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref display) = self.display {
            properties.push(("display", display));
        }
        let ret = glib::Object::new::<Monitor>(&properties).expect("object new");
        ret
    }

    pub fn display(mut self, display: &Display) -> Self {
        self.display = Some(display.clone());
        self
    }
}

impl fmt::Display for Monitor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Monitor")
    }
}
