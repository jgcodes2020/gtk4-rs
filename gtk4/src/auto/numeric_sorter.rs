// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::SortType;
use crate::Sorter;
use glib::object::Cast;
use glib::object::IsA;
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
    pub struct NumericSorter(Object<ffi::GtkNumericSorter, ffi::GtkNumericSorterClass>) @extends Sorter;

    match fn {
        get_type => || ffi::gtk_numeric_sorter_get_type(),
    }
}

impl NumericSorter {
    //#[doc(alias = "gtk_numeric_sorter_new")]
    //pub fn new(expression: /*Ignored*/Option<&Expression>) -> NumericSorter {
    //    unsafe { TODO: call ffi:gtk_numeric_sorter_new() }
    //}

    //#[doc(alias = "gtk_numeric_sorter_get_expression")]
    //pub fn get_expression(&self) -> /*Ignored*/Option<Expression> {
    //    unsafe { TODO: call ffi:gtk_numeric_sorter_get_expression() }
    //}

    #[doc(alias = "gtk_numeric_sorter_get_sort_order")]
    pub fn get_sort_order(&self) -> SortType {
        unsafe {
            from_glib(ffi::gtk_numeric_sorter_get_sort_order(
                self.to_glib_none().0,
            ))
        }
    }

    //#[doc(alias = "gtk_numeric_sorter_set_expression")]
    //pub fn set_expression(&self, expression: /*Ignored*/Option<&Expression>) {
    //    unsafe { TODO: call ffi:gtk_numeric_sorter_set_expression() }
    //}

    #[doc(alias = "gtk_numeric_sorter_set_sort_order")]
    pub fn set_sort_order(&self, sort_order: SortType) {
        unsafe {
            ffi::gtk_numeric_sorter_set_sort_order(self.to_glib_none().0, sort_order.to_glib());
        }
    }

    pub fn connect_property_expression_notify<F: Fn(&NumericSorter) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_expression_trampoline<F: Fn(&NumericSorter) + 'static>(
            this: *mut ffi::GtkNumericSorter,
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
                b"notify::expression\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_expression_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_sort_order_notify<F: Fn(&NumericSorter) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_sort_order_trampoline<F: Fn(&NumericSorter) + 'static>(
            this: *mut ffi::GtkNumericSorter,
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
                b"notify::sort-order\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_sort_order_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[derive(Clone, Default)]
pub struct NumericSorterBuilder {
    //expression: /*Unknown type*/,
    sort_order: Option<SortType>,
}

impl NumericSorterBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> NumericSorter {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref sort_order) = self.sort_order {
            properties.push(("sort-order", sort_order));
        }
        let ret = glib::Object::new::<NumericSorter>(&properties).expect("object new");
        ret
    }

    pub fn sort_order(mut self, sort_order: SortType) -> Self {
        self.sort_order = Some(sort_order);
        self
    }
}

impl fmt::Display for NumericSorter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("NumericSorter")
    }
}
