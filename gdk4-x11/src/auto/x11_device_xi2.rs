// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GdkX11DeviceXI2")]
    pub struct X11DeviceXI2(Object<ffi::GdkX11DeviceXI2, ffi::GdkX11DeviceXI2Class>) @extends gdk::Device;

    match fn {
        type_ => || ffi::gdk_x11_device_xi2_get_type(),
    }
}

impl X11DeviceXI2 {
    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`X11DeviceXI2`] objects.
    ///
    /// This method returns an instance of [`X11DeviceXI2Builder`](crate::builders::X11DeviceXI2Builder) which can be used to create [`X11DeviceXI2`] objects.
    pub fn builder() -> X11DeviceXI2Builder {
        X11DeviceXI2Builder::new()
    }

    #[doc(alias = "device-id")]
    pub fn device_id(&self) -> i32 {
        ObjectExt::property(self, "device-id")
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`X11DeviceXI2`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct X11DeviceXI2Builder {
    builder: glib::object::ObjectBuilder<'static, X11DeviceXI2>,
}

impl X11DeviceXI2Builder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn device_id(self, device_id: i32) -> Self {
        Self {
            builder: self.builder.property("device-id", device_id),
        }
    }

    pub fn display(self, display: &impl IsA<gdk::Display>) -> Self {
        Self {
            builder: self.builder.property("display", display.clone().upcast()),
        }
    }

    pub fn has_cursor(self, has_cursor: bool) -> Self {
        Self {
            builder: self.builder.property("has-cursor", has_cursor),
        }
    }

    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }

    pub fn num_touches(self, num_touches: u32) -> Self {
        Self {
            builder: self.builder.property("num-touches", num_touches),
        }
    }

    pub fn product_id(self, product_id: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("product-id", product_id.into()),
        }
    }

    pub fn seat(self, seat: &gdk::Seat) -> Self {
        Self {
            builder: self.builder.property("seat", seat.clone()),
        }
    }

    pub fn source(self, source: gdk::InputSource) -> Self {
        Self {
            builder: self.builder.property("source", source),
        }
    }

    pub fn vendor_id(self, vendor_id: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("vendor-id", vendor_id.into()),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`X11DeviceXI2`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> X11DeviceXI2 {
        assert_initialized_main_thread!();
        self.builder.build()
    }
}
