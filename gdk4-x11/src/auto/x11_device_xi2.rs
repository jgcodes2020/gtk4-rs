// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::{prelude::*, translate::*};
use std::fmt;

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
        X11DeviceXI2Builder::default()
    }

    #[doc(alias = "device-id")]
    pub fn device_id(&self) -> i32 {
        glib::ObjectExt::property(self, "device-id")
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`X11DeviceXI2`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct X11DeviceXI2Builder {
    device_id: Option<i32>,
    display: Option<gdk::Display>,
    has_cursor: Option<bool>,
    name: Option<String>,
    num_touches: Option<u32>,
    product_id: Option<String>,
    seat: Option<gdk::Seat>,
    source: Option<gdk::InputSource>,
    vendor_id: Option<String>,
}

impl X11DeviceXI2Builder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`X11DeviceXI2Builder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`X11DeviceXI2`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> X11DeviceXI2 {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref device_id) = self.device_id {
            properties.push(("device-id", device_id));
        }
        if let Some(ref display) = self.display {
            properties.push(("display", display));
        }
        if let Some(ref has_cursor) = self.has_cursor {
            properties.push(("has-cursor", has_cursor));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref num_touches) = self.num_touches {
            properties.push(("num-touches", num_touches));
        }
        if let Some(ref product_id) = self.product_id {
            properties.push(("product-id", product_id));
        }
        if let Some(ref seat) = self.seat {
            properties.push(("seat", seat));
        }
        if let Some(ref source) = self.source {
            properties.push(("source", source));
        }
        if let Some(ref vendor_id) = self.vendor_id {
            properties.push(("vendor-id", vendor_id));
        }
        glib::Object::new::<X11DeviceXI2>(&properties)
    }

    pub fn device_id(mut self, device_id: i32) -> Self {
        self.device_id = Some(device_id);
        self
    }

    pub fn display(mut self, display: &impl IsA<gdk::Display>) -> Self {
        self.display = Some(display.clone().upcast());
        self
    }

    pub fn has_cursor(mut self, has_cursor: bool) -> Self {
        self.has_cursor = Some(has_cursor);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn num_touches(mut self, num_touches: u32) -> Self {
        self.num_touches = Some(num_touches);
        self
    }

    pub fn product_id(mut self, product_id: &str) -> Self {
        self.product_id = Some(product_id.to_string());
        self
    }

    pub fn seat(mut self, seat: &gdk::Seat) -> Self {
        self.seat = Some(seat.clone());
        self
    }

    pub fn source(mut self, source: gdk::InputSource) -> Self {
        self.source = Some(source);
        self
    }

    pub fn vendor_id(mut self, vendor_id: &str) -> Self {
        self.vendor_id = Some(vendor_id.to_string());
        self
    }
}

impl fmt::Display for X11DeviceXI2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("X11DeviceXI2")
    }
}
