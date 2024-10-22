// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, ApplicationInhibitFlags, Window};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkApplication")]
    pub struct Application(Object<ffi::GtkApplication, ffi::GtkApplicationClass>) @extends gio::Application, @implements gio::ActionGroup, gio::ActionMap;

    match fn {
        type_ => || ffi::gtk_application_get_type(),
    }
}

impl Application {
    pub const NONE: Option<&'static Application> = None;

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Application`] objects.
    ///
    /// This method returns an instance of [`ApplicationBuilder`](crate::builders::ApplicationBuilder) which can be used to create [`Application`] objects.
    pub fn builder() -> ApplicationBuilder {
        ApplicationBuilder::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Application`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct ApplicationBuilder {
    builder: glib::object::ObjectBuilder<'static, Application>,
}

impl ApplicationBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn menubar(self, menubar: &impl IsA<gio::MenuModel>) -> Self {
        Self {
            builder: self.builder.property("menubar", menubar.clone().upcast()),
        }
    }

    pub fn register_session(self, register_session: bool) -> Self {
        Self {
            builder: self.builder.property("register-session", register_session),
        }
    }

    pub fn application_id(self, application_id: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("application-id", application_id.into()),
        }
    }

    pub fn flags(self, flags: gio::ApplicationFlags) -> Self {
        Self {
            builder: self.builder.property("flags", flags),
        }
    }

    pub fn inactivity_timeout(self, inactivity_timeout: u32) -> Self {
        Self {
            builder: self
                .builder
                .property("inactivity-timeout", inactivity_timeout),
        }
    }

    pub fn resource_base_path(self, resource_base_path: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("resource-base-path", resource_base_path.into()),
        }
    }

    #[cfg(feature = "gio_v2_80")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gio_v2_80")))]
    pub fn version(self, version: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("version", version.into()),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Application`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Application {
        assert_initialized_main_thread!();
        let ret = self.builder.build();
        {
            Application::register_startup_hook(&ret);
        }
        ret
    }
}

pub trait GtkApplicationExt: IsA<Application> + 'static {
    #[doc(alias = "gtk_application_add_window")]
    fn add_window(&self, window: &impl IsA<Window>) {
        unsafe {
            ffi::gtk_application_add_window(
                self.as_ref().to_glib_none().0,
                window.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_application_get_accels_for_action")]
    #[doc(alias = "get_accels_for_action")]
    fn accels_for_action(&self, detailed_action_name: &str) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_application_get_accels_for_action(
                self.as_ref().to_glib_none().0,
                detailed_action_name.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_application_get_actions_for_accel")]
    #[doc(alias = "get_actions_for_accel")]
    fn actions_for_accel(&self, accel: &str) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_application_get_actions_for_accel(
                self.as_ref().to_glib_none().0,
                accel.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_application_get_active_window")]
    #[doc(alias = "get_active_window")]
    #[doc(alias = "active-window")]
    fn active_window(&self) -> Option<Window> {
        unsafe {
            from_glib_none(ffi::gtk_application_get_active_window(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_application_get_menu_by_id")]
    #[doc(alias = "get_menu_by_id")]
    fn menu_by_id(&self, id: &str) -> Option<gio::Menu> {
        unsafe {
            from_glib_none(ffi::gtk_application_get_menu_by_id(
                self.as_ref().to_glib_none().0,
                id.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_application_get_menubar")]
    #[doc(alias = "get_menubar")]
    fn menubar(&self) -> Option<gio::MenuModel> {
        unsafe {
            from_glib_none(ffi::gtk_application_get_menubar(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_application_get_window_by_id")]
    #[doc(alias = "get_window_by_id")]
    fn window_by_id(&self, id: u32) -> Option<Window> {
        unsafe {
            from_glib_none(ffi::gtk_application_get_window_by_id(
                self.as_ref().to_glib_none().0,
                id,
            ))
        }
    }

    #[doc(alias = "gtk_application_get_windows")]
    #[doc(alias = "get_windows")]
    fn windows(&self) -> Vec<Window> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gtk_application_get_windows(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_application_inhibit")]
    fn inhibit(
        &self,
        window: Option<&impl IsA<Window>>,
        flags: ApplicationInhibitFlags,
        reason: Option<&str>,
    ) -> u32 {
        unsafe {
            ffi::gtk_application_inhibit(
                self.as_ref().to_glib_none().0,
                window.map(|p| p.as_ref()).to_glib_none().0,
                flags.into_glib(),
                reason.to_glib_none().0,
            )
        }
    }

    #[doc(alias = "gtk_application_list_action_descriptions")]
    fn list_action_descriptions(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_application_list_action_descriptions(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_application_remove_window")]
    fn remove_window(&self, window: &impl IsA<Window>) {
        unsafe {
            ffi::gtk_application_remove_window(
                self.as_ref().to_glib_none().0,
                window.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_application_set_accels_for_action")]
    fn set_accels_for_action(&self, detailed_action_name: &str, accels: &[&str]) {
        unsafe {
            ffi::gtk_application_set_accels_for_action(
                self.as_ref().to_glib_none().0,
                detailed_action_name.to_glib_none().0,
                accels.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_application_set_menubar")]
    #[doc(alias = "menubar")]
    fn set_menubar(&self, menubar: Option<&impl IsA<gio::MenuModel>>) {
        unsafe {
            ffi::gtk_application_set_menubar(
                self.as_ref().to_glib_none().0,
                menubar.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_application_uninhibit")]
    fn uninhibit(&self, cookie: u32) {
        unsafe {
            ffi::gtk_application_uninhibit(self.as_ref().to_glib_none().0, cookie);
        }
    }

    #[doc(alias = "register-session")]
    fn is_register_session(&self) -> bool {
        ObjectExt::property(self.as_ref(), "register-session")
    }

    #[doc(alias = "register-session")]
    fn set_register_session(&self, register_session: bool) {
        ObjectExt::set_property(self.as_ref(), "register-session", register_session)
    }

    #[doc(alias = "screensaver-active")]
    fn is_screensaver_active(&self) -> bool {
        ObjectExt::property(self.as_ref(), "screensaver-active")
    }

    #[doc(alias = "query-end")]
    fn connect_query_end<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn query_end_trampoline<P: IsA<Application>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkApplication,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Application::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"query-end\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    query_end_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "window-added")]
    fn connect_window_added<F: Fn(&Self, &Window) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn window_added_trampoline<
            P: IsA<Application>,
            F: Fn(&P, &Window) + 'static,
        >(
            this: *mut ffi::GtkApplication,
            window: *mut ffi::GtkWindow,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Application::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(window),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"window-added\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    window_added_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "window-removed")]
    fn connect_window_removed<F: Fn(&Self, &Window) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn window_removed_trampoline<
            P: IsA<Application>,
            F: Fn(&P, &Window) + 'static,
        >(
            this: *mut ffi::GtkApplication,
            window: *mut ffi::GtkWindow,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Application::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(window),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"window-removed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    window_removed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "active-window")]
    fn connect_active_window_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_active_window_trampoline<
            P: IsA<Application>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkApplication,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Application::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::active-window\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_active_window_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "menubar")]
    fn connect_menubar_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_menubar_trampoline<P: IsA<Application>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkApplication,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Application::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::menubar\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_menubar_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "register-session")]
    fn connect_register_session_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_register_session_trampoline<
            P: IsA<Application>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkApplication,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Application::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::register-session\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_register_session_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "screensaver-active")]
    fn connect_screensaver_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_screensaver_active_trampoline<
            P: IsA<Application>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkApplication,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Application::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::screensaver-active\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_screensaver_active_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<Application>> GtkApplicationExt for O {}
