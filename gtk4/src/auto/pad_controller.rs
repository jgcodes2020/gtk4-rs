// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::EventController;
use crate::PadActionType;
use crate::PropagationLimit;
use crate::PropagationPhase;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::fmt;

glib::glib_wrapper! {
    pub struct PadController(Object<ffi::GtkPadController, ffi::GtkPadControllerClass>) @extends EventController;

    match fn {
        get_type => || ffi::gtk_pad_controller_get_type(),
    }
}

impl PadController {
    #[doc(alias = "gtk_pad_controller_new")]
    pub fn new<P: IsA<gio::ActionGroup>>(group: &P, pad: Option<&gdk::Device>) -> PadController {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_pad_controller_new(
                group.as_ref().to_glib_none().0,
                pad.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_pad_controller_set_action")]
    pub fn set_action(
        &self,
        type_: PadActionType,
        index: i32,
        mode: i32,
        label: &str,
        action_name: &str,
    ) {
        unsafe {
            ffi::gtk_pad_controller_set_action(
                self.to_glib_none().0,
                type_.to_glib(),
                index,
                mode,
                label.to_glib_none().0,
                action_name.to_glib_none().0,
            );
        }
    }

    pub fn get_property_action_group(&self) -> Option<gio::ActionGroup> {
        unsafe {
            let mut value = glib::Value::from_type(<gio::ActionGroup as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"action-group\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `action-group` getter")
        }
    }

    pub fn get_property_pad(&self) -> Option<gdk::Device> {
        unsafe {
            let mut value = glib::Value::from_type(<gdk::Device as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"pad\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value.get().expect("Return Value for property `pad` getter")
        }
    }
}

#[derive(Clone, Default)]
pub struct PadControllerBuilder {
    action_group: Option<gio::ActionGroup>,
    pad: Option<gdk::Device>,
    name: Option<String>,
    propagation_limit: Option<PropagationLimit>,
    propagation_phase: Option<PropagationPhase>,
}

impl PadControllerBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> PadController {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref action_group) = self.action_group {
            properties.push(("action-group", action_group));
        }
        if let Some(ref pad) = self.pad {
            properties.push(("pad", pad));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref propagation_limit) = self.propagation_limit {
            properties.push(("propagation-limit", propagation_limit));
        }
        if let Some(ref propagation_phase) = self.propagation_phase {
            properties.push(("propagation-phase", propagation_phase));
        }
        let ret = glib::Object::new::<PadController>(&properties).expect("object new");
        ret
    }

    pub fn action_group<P: IsA<gio::ActionGroup>>(mut self, action_group: &P) -> Self {
        self.action_group = Some(action_group.clone().upcast());
        self
    }

    pub fn pad(mut self, pad: &gdk::Device) -> Self {
        self.pad = Some(pad.clone());
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn propagation_limit(mut self, propagation_limit: PropagationLimit) -> Self {
        self.propagation_limit = Some(propagation_limit);
        self
    }

    pub fn propagation_phase(mut self, propagation_phase: PropagationPhase) -> Self {
        self.propagation_phase = Some(propagation_phase);
        self
    }
}

impl fmt::Display for PadController {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("PadController")
    }
}
