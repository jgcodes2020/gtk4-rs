// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{EventController, EventControllerScrollFlags, PropagationLimit, PropagationPhase};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GtkEventControllerScroll")]
    pub struct EventControllerScroll(Object<ffi::GtkEventControllerScroll, ffi::GtkEventControllerScrollClass>) @extends EventController;

    match fn {
        type_ => || ffi::gtk_event_controller_scroll_get_type(),
    }
}

impl EventControllerScroll {
    #[doc(alias = "gtk_event_controller_scroll_new")]
    pub fn new(flags: EventControllerScrollFlags) -> EventControllerScroll {
        assert_initialized_main_thread!();
        unsafe {
            EventController::from_glib_full(ffi::gtk_event_controller_scroll_new(flags.into_glib()))
                .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`EventControllerScroll`] objects.
    ///
    /// This method returns an instance of [`EventControllerScrollBuilder`](crate::builders::EventControllerScrollBuilder) which can be used to create [`EventControllerScroll`] objects.
    pub fn builder() -> EventControllerScrollBuilder {
        EventControllerScrollBuilder::new()
    }

    #[doc(alias = "gtk_event_controller_scroll_get_flags")]
    #[doc(alias = "get_flags")]
    pub fn flags(&self) -> EventControllerScrollFlags {
        unsafe {
            from_glib(ffi::gtk_event_controller_scroll_get_flags(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v4_8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_8")))]
    #[doc(alias = "gtk_event_controller_scroll_get_unit")]
    #[doc(alias = "get_unit")]
    pub fn unit(&self) -> gdk::ScrollUnit {
        unsafe {
            from_glib(ffi::gtk_event_controller_scroll_get_unit(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_event_controller_scroll_set_flags")]
    pub fn set_flags(&self, flags: EventControllerScrollFlags) {
        unsafe {
            ffi::gtk_event_controller_scroll_set_flags(self.to_glib_none().0, flags.into_glib());
        }
    }

    #[doc(alias = "decelerate")]
    pub fn connect_decelerate<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn decelerate_trampoline<
            F: Fn(&EventControllerScroll, f64, f64) + 'static,
        >(
            this: *mut ffi::GtkEventControllerScroll,
            vel_x: libc::c_double,
            vel_y: libc::c_double,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), vel_x, vel_y)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"decelerate\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    decelerate_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "scroll")]
    pub fn connect_scroll<F: Fn(&Self, f64, f64) -> glib::ControlFlow + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn scroll_trampoline<
            F: Fn(&EventControllerScroll, f64, f64) -> glib::ControlFlow + 'static,
        >(
            this: *mut ffi::GtkEventControllerScroll,
            dx: libc::c_double,
            dy: libc::c_double,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), dx, dy).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"scroll\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    scroll_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "scroll-begin")]
    pub fn connect_scroll_begin<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn scroll_begin_trampoline<F: Fn(&EventControllerScroll) + 'static>(
            this: *mut ffi::GtkEventControllerScroll,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"scroll-begin\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    scroll_begin_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "scroll-end")]
    pub fn connect_scroll_end<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn scroll_end_trampoline<F: Fn(&EventControllerScroll) + 'static>(
            this: *mut ffi::GtkEventControllerScroll,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"scroll-end\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    scroll_end_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "flags")]
    pub fn connect_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_flags_trampoline<F: Fn(&EventControllerScroll) + 'static>(
            this: *mut ffi::GtkEventControllerScroll,
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
                b"notify::flags\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_flags_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for EventControllerScroll {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`EventControllerScroll`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct EventControllerScrollBuilder {
    builder: glib::object::ObjectBuilder<'static, EventControllerScroll>,
}

impl EventControllerScrollBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn flags(self, flags: EventControllerScrollFlags) -> Self {
        Self {
            builder: self.builder.property("flags", flags),
        }
    }

    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }

    pub fn propagation_limit(self, propagation_limit: PropagationLimit) -> Self {
        Self {
            builder: self
                .builder
                .property("propagation-limit", propagation_limit),
        }
    }

    pub fn propagation_phase(self, propagation_phase: PropagationPhase) -> Self {
        Self {
            builder: self
                .builder
                .property("propagation-phase", propagation_phase),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`EventControllerScroll`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> EventControllerScroll {
        self.builder.build()
    }
}

impl fmt::Display for EventControllerScroll {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("EventControllerScroll")
    }
}
