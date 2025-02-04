// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`Popover`].

use glib::translate::*;

use crate::{ffi, prelude::*, subclass::prelude::*, Native, Popover, ShortcutManager};

pub trait PopoverImpl:
    WidgetImpl + ObjectSubclass<Type: IsA<Popover> + IsA<Native> + IsA<ShortcutManager>>
{
    fn activate_default(&self) {
        self.parent_activate_default()
    }

    fn closed(&self) {
        self.parent_closed()
    }
}

pub trait PopoverImplExt: PopoverImpl {
    fn parent_activate_default(&self) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkPopoverClass;
            if let Some(f) = (*parent_class).activate_default {
                f(self.obj().unsafe_cast_ref::<Popover>().to_glib_none().0)
            }
        }
    }

    fn parent_closed(&self) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkPopoverClass;
            if let Some(f) = (*parent_class).closed {
                f(self.obj().unsafe_cast_ref::<Popover>().to_glib_none().0)
            }
        }
    }
}

impl<T: PopoverImpl> PopoverImplExt for T {}

unsafe impl<T: PopoverImpl> IsSubclassable<T> for Popover {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        let klass = class.as_mut();
        klass.activate_default = Some(popover_activate_default::<T>);
        klass.closed = Some(popover_closed::<T>);
    }
}

unsafe extern "C" fn popover_activate_default<T: PopoverImpl>(ptr: *mut ffi::GtkPopover) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.activate_default()
}

unsafe extern "C" fn popover_closed<T: PopoverImpl>(ptr: *mut ffi::GtkPopover) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.closed()
}
