// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ConstraintAttribute;
use crate::ConstraintRelation;
use crate::ConstraintTarget;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::fmt;

glib::glib_wrapper! {
    pub struct Constraint(Object<ffi::GtkConstraint, ffi::GtkConstraintClass>);

    match fn {
        get_type => || ffi::gtk_constraint_get_type(),
    }
}

impl Constraint {
    #[doc(alias = "gtk_constraint_new")]
    pub fn new<P: IsA<ConstraintTarget>, Q: IsA<ConstraintTarget>>(
        target: Option<&P>,
        target_attribute: ConstraintAttribute,
        relation: ConstraintRelation,
        source: Option<&Q>,
        source_attribute: ConstraintAttribute,
        multiplier: f64,
        constant: f64,
        strength: i32,
    ) -> Constraint {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_constraint_new(
                target.map(|p| p.as_ref()).to_glib_none().0,
                target_attribute.to_glib(),
                relation.to_glib(),
                source.map(|p| p.as_ref()).to_glib_none().0,
                source_attribute.to_glib(),
                multiplier,
                constant,
                strength,
            ))
        }
    }

    #[doc(alias = "gtk_constraint_new_constant")]
    pub fn new_constant<P: IsA<ConstraintTarget>>(
        target: Option<&P>,
        target_attribute: ConstraintAttribute,
        relation: ConstraintRelation,
        constant: f64,
        strength: i32,
    ) -> Constraint {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_constraint_new_constant(
                target.map(|p| p.as_ref()).to_glib_none().0,
                target_attribute.to_glib(),
                relation.to_glib(),
                constant,
                strength,
            ))
        }
    }

    #[doc(alias = "gtk_constraint_get_constant")]
    pub fn get_constant(&self) -> f64 {
        unsafe { ffi::gtk_constraint_get_constant(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_constraint_get_multiplier")]
    pub fn get_multiplier(&self) -> f64 {
        unsafe { ffi::gtk_constraint_get_multiplier(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_constraint_get_relation")]
    pub fn get_relation(&self) -> ConstraintRelation {
        unsafe { from_glib(ffi::gtk_constraint_get_relation(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_constraint_get_source")]
    pub fn get_source(&self) -> Option<ConstraintTarget> {
        unsafe { from_glib_none(ffi::gtk_constraint_get_source(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_constraint_get_source_attribute")]
    pub fn get_source_attribute(&self) -> ConstraintAttribute {
        unsafe {
            from_glib(ffi::gtk_constraint_get_source_attribute(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_constraint_get_strength")]
    pub fn get_strength(&self) -> i32 {
        unsafe { ffi::gtk_constraint_get_strength(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_constraint_get_target")]
    pub fn get_target(&self) -> Option<ConstraintTarget> {
        unsafe { from_glib_none(ffi::gtk_constraint_get_target(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_constraint_get_target_attribute")]
    pub fn get_target_attribute(&self) -> ConstraintAttribute {
        unsafe {
            from_glib(ffi::gtk_constraint_get_target_attribute(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_constraint_is_attached")]
    pub fn is_attached(&self) -> bool {
        unsafe { from_glib(ffi::gtk_constraint_is_attached(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_constraint_is_constant")]
    pub fn is_constant(&self) -> bool {
        unsafe { from_glib(ffi::gtk_constraint_is_constant(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_constraint_is_required")]
    pub fn is_required(&self) -> bool {
        unsafe { from_glib(ffi::gtk_constraint_is_required(self.to_glib_none().0)) }
    }
}

#[derive(Clone, Default)]
pub struct ConstraintBuilder {
    constant: Option<f64>,
    multiplier: Option<f64>,
    relation: Option<ConstraintRelation>,
    source: Option<ConstraintTarget>,
    source_attribute: Option<ConstraintAttribute>,
    strength: Option<i32>,
    target: Option<ConstraintTarget>,
    target_attribute: Option<ConstraintAttribute>,
}

impl ConstraintBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Constraint {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref constant) = self.constant {
            properties.push(("constant", constant));
        }
        if let Some(ref multiplier) = self.multiplier {
            properties.push(("multiplier", multiplier));
        }
        if let Some(ref relation) = self.relation {
            properties.push(("relation", relation));
        }
        if let Some(ref source) = self.source {
            properties.push(("source", source));
        }
        if let Some(ref source_attribute) = self.source_attribute {
            properties.push(("source-attribute", source_attribute));
        }
        if let Some(ref strength) = self.strength {
            properties.push(("strength", strength));
        }
        if let Some(ref target) = self.target {
            properties.push(("target", target));
        }
        if let Some(ref target_attribute) = self.target_attribute {
            properties.push(("target-attribute", target_attribute));
        }
        let ret = glib::Object::new::<Constraint>(&properties).expect("object new");
        ret
    }

    pub fn constant(mut self, constant: f64) -> Self {
        self.constant = Some(constant);
        self
    }

    pub fn multiplier(mut self, multiplier: f64) -> Self {
        self.multiplier = Some(multiplier);
        self
    }

    pub fn relation(mut self, relation: ConstraintRelation) -> Self {
        self.relation = Some(relation);
        self
    }

    pub fn source<P: IsA<ConstraintTarget>>(mut self, source: &P) -> Self {
        self.source = Some(source.clone().upcast());
        self
    }

    pub fn source_attribute(mut self, source_attribute: ConstraintAttribute) -> Self {
        self.source_attribute = Some(source_attribute);
        self
    }

    pub fn strength(mut self, strength: i32) -> Self {
        self.strength = Some(strength);
        self
    }

    pub fn target<P: IsA<ConstraintTarget>>(mut self, target: &P) -> Self {
        self.target = Some(target.clone().upcast());
        self
    }

    pub fn target_attribute(mut self, target_attribute: ConstraintAttribute) -> Self {
        self.target_attribute = Some(target_attribute);
        self
    }
}

impl fmt::Display for Constraint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Constraint")
    }
}
