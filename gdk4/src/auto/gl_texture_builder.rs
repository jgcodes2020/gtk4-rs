// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{GLContext, MemoryFormat, Texture};
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GdkGLTextureBuilder")]
    pub struct GLTextureBuilder(Object<ffi::GdkGLTextureBuilder, ffi::GdkGLTextureBuilderClass>);

    match fn {
        type_ => || ffi::gdk_gl_texture_builder_get_type(),
    }
}

impl GLTextureBuilder {
    #[doc(alias = "gdk_gl_texture_builder_new")]
    pub fn new() -> GLTextureBuilder {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gdk_gl_texture_builder_new()) }
    }

    #[doc(alias = "gdk_gl_texture_builder_get_context")]
    #[doc(alias = "get_context")]
    pub fn context(&self) -> Option<GLContext> {
        unsafe {
            from_glib_none(ffi::gdk_gl_texture_builder_get_context(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_gl_texture_builder_get_format")]
    #[doc(alias = "get_format")]
    pub fn format(&self) -> MemoryFormat {
        unsafe {
            from_glib(ffi::gdk_gl_texture_builder_get_format(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_gl_texture_builder_get_has_mipmap")]
    #[doc(alias = "get_has_mipmap")]
    pub fn has_mipmap(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_gl_texture_builder_get_has_mipmap(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_gl_texture_builder_get_height")]
    #[doc(alias = "get_height")]
    pub fn height(&self) -> i32 {
        unsafe { ffi::gdk_gl_texture_builder_get_height(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_gl_texture_builder_get_id")]
    #[doc(alias = "get_id")]
    pub fn id(&self) -> u32 {
        unsafe { ffi::gdk_gl_texture_builder_get_id(self.to_glib_none().0) }
    }

    //#[doc(alias = "gdk_gl_texture_builder_get_sync")]
    //#[doc(alias = "get_sync")]
    //pub fn sync(&self) -> /*Unimplemented*/Option<Basic: Pointer> {
    //    unsafe { TODO: call ffi:gdk_gl_texture_builder_get_sync() }
    //}

    #[doc(alias = "gdk_gl_texture_builder_get_update_region")]
    #[doc(alias = "get_update_region")]
    pub fn update_region(&self) -> Option<cairo::Region> {
        unsafe {
            from_glib_none(ffi::gdk_gl_texture_builder_get_update_region(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_gl_texture_builder_get_update_texture")]
    #[doc(alias = "get_update_texture")]
    pub fn update_texture(&self) -> Option<Texture> {
        unsafe {
            from_glib_none(ffi::gdk_gl_texture_builder_get_update_texture(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_gl_texture_builder_get_width")]
    #[doc(alias = "get_width")]
    pub fn width(&self) -> i32 {
        unsafe { ffi::gdk_gl_texture_builder_get_width(self.to_glib_none().0) }
    }

    //#[doc(alias = "gdk_gl_texture_builder_set_sync")]
    //pub fn set_sync(&self, sync: /*Unimplemented*/Option<Basic: Pointer>) {
    //    unsafe { TODO: call ffi:gdk_gl_texture_builder_set_sync() }
    //}
}

#[cfg(feature = "v4_12")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
impl Default for GLTextureBuilder {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for GLTextureBuilder {}
unsafe impl Sync for GLTextureBuilder {}

impl fmt::Display for GLTextureBuilder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("GLTextureBuilder")
    }
}
