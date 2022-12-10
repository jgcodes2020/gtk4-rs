// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(target_os = "linux", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(target_os = "linux")))]
use crate::Printer;
use crate::{
    Accessible, AccessibleProperty, AccessibleRelation, AccessibleRole, AccessibleState,
    DebugFlags, PageSetup, PrintSettings, StyleContext, TextDirection, TreeModel, TreePath, Widget,
    Window,
};
use glib::{prelude::*, translate::*};
use std::{boxed::Box as Box_, mem, ptr};

#[doc(alias = "gtk_accelerator_get_default_mod_mask")]
pub fn accelerator_get_default_mod_mask() -> gdk::ModifierType {
    assert_initialized_main_thread!();
    unsafe { from_glib(ffi::gtk_accelerator_get_default_mod_mask()) }
}

#[doc(alias = "gtk_check_version")]
pub fn check_version(
    required_major: u32,
    required_minor: u32,
    required_micro: u32,
) -> Option<glib::GString> {
    skip_assert_initialized!();
    unsafe {
        from_glib_none(ffi::gtk_check_version(
            required_major,
            required_minor,
            required_micro,
        ))
    }
}

#[doc(alias = "gtk_disable_setlocale")]
pub fn disable_setlocale() {
    assert_not_initialized!();
    unsafe {
        ffi::gtk_disable_setlocale();
    }
}

//#[doc(alias = "gtk_distribute_natural_allocation")]
//pub fn distribute_natural_allocation(extra_space: i32, sizes: /*Ignored*/&[RequestedSize]) -> i32 {
//    unsafe { TODO: call ffi:gtk_distribute_natural_allocation() }
//}

#[cfg(any(target_os = "linux", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(target_os = "linux")))]
#[doc(alias = "gtk_enumerate_printers")]
pub fn enumerate_printers<P: Fn(&Printer) -> bool + Send + Sync + 'static>(func: P, wait: bool) {
    assert_initialized_main_thread!();
    let func_data: Box_<P> = Box_::new(func);
    unsafe extern "C" fn func_func<P: Fn(&Printer) -> bool + Send + Sync + 'static>(
        printer: *mut ffi::GtkPrinter,
        data: glib::ffi::gpointer,
    ) -> glib::ffi::gboolean {
        let printer = from_glib_borrow(printer);
        let callback: &P = &*(data as *mut _);
        let res = (*callback)(&printer);
        res.into_glib()
    }
    let func = Some(func_func::<P> as _);
    unsafe extern "C" fn destroy_func<P: Fn(&Printer) -> bool + Send + Sync + 'static>(
        data: glib::ffi::gpointer,
    ) {
        let _callback: Box_<P> = Box_::from_raw(data as *mut _);
    }
    let destroy_call2 = Some(destroy_func::<P> as _);
    let super_callback0: Box_<P> = func_data;
    unsafe {
        ffi::gtk_enumerate_printers(
            func,
            Box_::into_raw(super_callback0) as *mut _,
            destroy_call2,
            wait.into_glib(),
        );
    }
}

#[doc(alias = "gtk_get_binary_age")]
#[doc(alias = "get_binary_age")]
pub fn binary_age() -> u32 {
    skip_assert_initialized!();
    unsafe { ffi::gtk_get_binary_age() }
}

#[doc(alias = "gtk_get_debug_flags")]
#[doc(alias = "get_debug_flags")]
pub fn debug_flags() -> DebugFlags {
    assert_initialized_main_thread!();
    unsafe { from_glib(ffi::gtk_get_debug_flags()) }
}

#[doc(alias = "gtk_get_default_language")]
#[doc(alias = "get_default_language")]
pub fn default_language() -> pango::Language {
    assert_initialized_main_thread!();
    unsafe { from_glib_none(ffi::gtk_get_default_language()) }
}

#[doc(alias = "gtk_get_interface_age")]
#[doc(alias = "get_interface_age")]
pub fn interface_age() -> u32 {
    skip_assert_initialized!();
    unsafe { ffi::gtk_get_interface_age() }
}

#[doc(alias = "gtk_get_locale_direction")]
#[doc(alias = "get_locale_direction")]
pub fn locale_direction() -> TextDirection {
    assert_initialized_main_thread!();
    unsafe { from_glib(ffi::gtk_get_locale_direction()) }
}

#[doc(alias = "gtk_get_major_version")]
#[doc(alias = "get_major_version")]
pub fn major_version() -> u32 {
    skip_assert_initialized!();
    unsafe { ffi::gtk_get_major_version() }
}

#[doc(alias = "gtk_get_micro_version")]
#[doc(alias = "get_micro_version")]
pub fn micro_version() -> u32 {
    skip_assert_initialized!();
    unsafe { ffi::gtk_get_micro_version() }
}

#[doc(alias = "gtk_get_minor_version")]
#[doc(alias = "get_minor_version")]
pub fn minor_version() -> u32 {
    skip_assert_initialized!();
    unsafe { ffi::gtk_get_minor_version() }
}

#[doc(alias = "gtk_hsv_to_rgb")]
pub fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    assert_initialized_main_thread!();
    unsafe {
        let mut r = mem::MaybeUninit::uninit();
        let mut g = mem::MaybeUninit::uninit();
        let mut b = mem::MaybeUninit::uninit();
        ffi::gtk_hsv_to_rgb(h, s, v, r.as_mut_ptr(), g.as_mut_ptr(), b.as_mut_ptr());
        (r.assume_init(), g.assume_init(), b.assume_init())
    }
}

#[doc(alias = "gtk_print_run_page_setup_dialog")]
pub fn print_run_page_setup_dialog(
    parent: Option<&impl IsA<Window>>,
    page_setup: Option<&PageSetup>,
    settings: &PrintSettings,
) -> PageSetup {
    skip_assert_initialized!();
    unsafe {
        from_glib_full(ffi::gtk_print_run_page_setup_dialog(
            parent.map(|p| p.as_ref()).to_glib_none().0,
            page_setup.to_glib_none().0,
            settings.to_glib_none().0,
        ))
    }
}

#[doc(alias = "gtk_print_run_page_setup_dialog_async")]
pub fn print_run_page_setup_dialog_async<P: FnOnce(&PageSetup) + Send + Sync + 'static>(
    parent: Option<&impl IsA<Window>>,
    page_setup: Option<&PageSetup>,
    settings: &PrintSettings,
    done_cb: P,
) {
    skip_assert_initialized!();
    let done_cb_data: Box_<P> = Box_::new(done_cb);
    unsafe extern "C" fn done_cb_func<P: FnOnce(&PageSetup) + Send + Sync + 'static>(
        page_setup: *mut ffi::GtkPageSetup,
        data: glib::ffi::gpointer,
    ) {
        let page_setup = from_glib_borrow(page_setup);
        let callback: Box_<P> = Box_::from_raw(data as *mut _);
        (*callback)(&page_setup);
    }
    let done_cb = Some(done_cb_func::<P> as _);
    let super_callback0: Box_<P> = done_cb_data;
    unsafe {
        ffi::gtk_print_run_page_setup_dialog_async(
            parent.map(|p| p.as_ref()).to_glib_none().0,
            page_setup.to_glib_none().0,
            settings.to_glib_none().0,
            done_cb,
            Box_::into_raw(super_callback0) as *mut _,
        );
    }
}

#[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
#[allow(deprecated)]
#[doc(alias = "gtk_render_activity")]
pub fn render_activity(
    context: &impl IsA<StyleContext>,
    cr: &cairo::Context,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_activity(
            context.as_ref().to_glib_none().0,
            mut_override(cr.to_glib_none().0),
            x,
            y,
            width,
            height,
        );
    }
}

#[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
#[allow(deprecated)]
#[doc(alias = "gtk_render_arrow")]
pub fn render_arrow(
    context: &impl IsA<StyleContext>,
    cr: &cairo::Context,
    angle: f64,
    x: f64,
    y: f64,
    size: f64,
) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_arrow(
            context.as_ref().to_glib_none().0,
            mut_override(cr.to_glib_none().0),
            angle,
            x,
            y,
            size,
        );
    }
}

#[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
#[allow(deprecated)]
#[doc(alias = "gtk_render_background")]
pub fn render_background(
    context: &impl IsA<StyleContext>,
    cr: &cairo::Context,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_background(
            context.as_ref().to_glib_none().0,
            mut_override(cr.to_glib_none().0),
            x,
            y,
            width,
            height,
        );
    }
}

#[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
#[allow(deprecated)]
#[doc(alias = "gtk_render_check")]
pub fn render_check(
    context: &impl IsA<StyleContext>,
    cr: &cairo::Context,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_check(
            context.as_ref().to_glib_none().0,
            mut_override(cr.to_glib_none().0),
            x,
            y,
            width,
            height,
        );
    }
}

#[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
#[allow(deprecated)]
#[doc(alias = "gtk_render_expander")]
pub fn render_expander(
    context: &impl IsA<StyleContext>,
    cr: &cairo::Context,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_expander(
            context.as_ref().to_glib_none().0,
            mut_override(cr.to_glib_none().0),
            x,
            y,
            width,
            height,
        );
    }
}

#[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
#[allow(deprecated)]
#[doc(alias = "gtk_render_focus")]
pub fn render_focus(
    context: &impl IsA<StyleContext>,
    cr: &cairo::Context,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_focus(
            context.as_ref().to_glib_none().0,
            mut_override(cr.to_glib_none().0),
            x,
            y,
            width,
            height,
        );
    }
}

#[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
#[allow(deprecated)]
#[doc(alias = "gtk_render_frame")]
pub fn render_frame(
    context: &impl IsA<StyleContext>,
    cr: &cairo::Context,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_frame(
            context.as_ref().to_glib_none().0,
            mut_override(cr.to_glib_none().0),
            x,
            y,
            width,
            height,
        );
    }
}

#[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
#[allow(deprecated)]
#[doc(alias = "gtk_render_handle")]
pub fn render_handle(
    context: &impl IsA<StyleContext>,
    cr: &cairo::Context,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_handle(
            context.as_ref().to_glib_none().0,
            mut_override(cr.to_glib_none().0),
            x,
            y,
            width,
            height,
        );
    }
}

#[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
#[allow(deprecated)]
#[doc(alias = "gtk_render_icon")]
pub fn render_icon(
    context: &impl IsA<StyleContext>,
    cr: &cairo::Context,
    texture: &impl IsA<gdk::Texture>,
    x: f64,
    y: f64,
) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_icon(
            context.as_ref().to_glib_none().0,
            mut_override(cr.to_glib_none().0),
            texture.as_ref().to_glib_none().0,
            x,
            y,
        );
    }
}

#[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
#[allow(deprecated)]
#[doc(alias = "gtk_render_layout")]
pub fn render_layout(
    context: &impl IsA<StyleContext>,
    cr: &cairo::Context,
    x: f64,
    y: f64,
    layout: &pango::Layout,
) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_layout(
            context.as_ref().to_glib_none().0,
            mut_override(cr.to_glib_none().0),
            x,
            y,
            layout.to_glib_none().0,
        );
    }
}

#[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
#[allow(deprecated)]
#[doc(alias = "gtk_render_line")]
pub fn render_line(
    context: &impl IsA<StyleContext>,
    cr: &cairo::Context,
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_line(
            context.as_ref().to_glib_none().0,
            mut_override(cr.to_glib_none().0),
            x0,
            y0,
            x1,
            y1,
        );
    }
}

#[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
#[allow(deprecated)]
#[doc(alias = "gtk_render_option")]
pub fn render_option(
    context: &impl IsA<StyleContext>,
    cr: &cairo::Context,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_option(
            context.as_ref().to_glib_none().0,
            mut_override(cr.to_glib_none().0),
            x,
            y,
            width,
            height,
        );
    }
}

#[doc(alias = "gtk_rgb_to_hsv")]
pub fn rgb_to_hsv(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    assert_initialized_main_thread!();
    unsafe {
        let mut h = mem::MaybeUninit::uninit();
        let mut s = mem::MaybeUninit::uninit();
        let mut v = mem::MaybeUninit::uninit();
        ffi::gtk_rgb_to_hsv(r, g, b, h.as_mut_ptr(), s.as_mut_ptr(), v.as_mut_ptr());
        (h.assume_init(), s.assume_init(), v.assume_init())
    }
}

#[doc(alias = "gtk_set_debug_flags")]
pub fn set_debug_flags(flags: DebugFlags) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gtk_set_debug_flags(flags.into_glib());
    }
}

#[doc(alias = "gtk_show_uri")]
pub fn show_uri(parent: Option<&impl IsA<Window>>, uri: &str, timestamp: u32) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gtk_show_uri(
            parent.map(|p| p.as_ref()).to_glib_none().0,
            uri.to_glib_none().0,
            timestamp,
        );
    }
}

#[doc(alias = "gtk_test_accessible_assertion_message_role")]
pub fn test_accessible_assertion_message_role(
    domain: &str,
    file: &str,
    line: i32,
    func: &str,
    expr: &str,
    accessible: &impl IsA<Accessible>,
    expected_role: AccessibleRole,
    actual_role: AccessibleRole,
) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_test_accessible_assertion_message_role(
            domain.to_glib_none().0,
            file.to_glib_none().0,
            line,
            func.to_glib_none().0,
            expr.to_glib_none().0,
            accessible.as_ref().to_glib_none().0,
            expected_role.into_glib(),
            actual_role.into_glib(),
        );
    }
}

#[doc(alias = "gtk_test_accessible_has_property")]
pub fn test_accessible_has_property(
    accessible: &impl IsA<Accessible>,
    property: AccessibleProperty,
) -> bool {
    skip_assert_initialized!();
    unsafe {
        from_glib(ffi::gtk_test_accessible_has_property(
            accessible.as_ref().to_glib_none().0,
            property.into_glib(),
        ))
    }
}

#[doc(alias = "gtk_test_accessible_has_relation")]
pub fn test_accessible_has_relation(
    accessible: &impl IsA<Accessible>,
    relation: AccessibleRelation,
) -> bool {
    skip_assert_initialized!();
    unsafe {
        from_glib(ffi::gtk_test_accessible_has_relation(
            accessible.as_ref().to_glib_none().0,
            relation.into_glib(),
        ))
    }
}

#[doc(alias = "gtk_test_accessible_has_role")]
pub fn test_accessible_has_role(accessible: &impl IsA<Accessible>, role: AccessibleRole) -> bool {
    skip_assert_initialized!();
    unsafe {
        from_glib(ffi::gtk_test_accessible_has_role(
            accessible.as_ref().to_glib_none().0,
            role.into_glib(),
        ))
    }
}

#[doc(alias = "gtk_test_accessible_has_state")]
pub fn test_accessible_has_state(
    accessible: &impl IsA<Accessible>,
    state: AccessibleState,
) -> bool {
    skip_assert_initialized!();
    unsafe {
        from_glib(ffi::gtk_test_accessible_has_state(
            accessible.as_ref().to_glib_none().0,
            state.into_glib(),
        ))
    }
}

//#[doc(alias = "gtk_test_init")]
//pub fn test_init(argvp: /*Unimplemented*/Vec<glib::GString>, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) {
//    unsafe { TODO: call ffi:gtk_test_init() }
//}

#[doc(alias = "gtk_test_register_all_types")]
pub fn test_register_all_types() {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gtk_test_register_all_types();
    }
}

#[doc(alias = "gtk_test_widget_wait_for_draw")]
pub fn test_widget_wait_for_draw(widget: &impl IsA<Widget>) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_test_widget_wait_for_draw(widget.as_ref().to_glib_none().0);
    }
}

#[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
#[allow(deprecated)]
#[doc(alias = "gtk_tree_create_row_drag_content")]
pub fn tree_create_row_drag_content(
    tree_model: &impl IsA<TreeModel>,
    path: &TreePath,
) -> gdk::ContentProvider {
    skip_assert_initialized!();
    unsafe {
        from_glib_full(ffi::gtk_tree_create_row_drag_content(
            tree_model.as_ref().to_glib_none().0,
            mut_override(path.to_glib_none().0),
        ))
    }
}

#[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
#[allow(deprecated)]
#[doc(alias = "gtk_tree_get_row_drag_data")]
pub fn tree_get_row_drag_data(
    value: &glib::Value,
) -> Option<(Option<TreeModel>, Option<TreePath>)> {
    assert_initialized_main_thread!();
    unsafe {
        let mut tree_model = ptr::null_mut();
        let mut path = ptr::null_mut();
        let ret = from_glib(ffi::gtk_tree_get_row_drag_data(
            value.to_glib_none().0,
            &mut tree_model,
            &mut path,
        ));
        if ret {
            Some((from_glib_none(tree_model), from_glib_full(path)))
        } else {
            None
        }
    }
}
