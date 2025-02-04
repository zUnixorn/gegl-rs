// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use crate::{ffi};
use glib::{prelude::*,translate::*};

glib::wrapper! {
    ///
    #[doc(alias = "GeglParamPath")]
    pub struct ParamPath(Shared<ffi::GeglParamPath>);

    match fn {
        ref => |ptr| glib::gobject_ffi::g_param_spec_ref_sink(ptr as *mut glib::gobject_ffi::GParamSpec),
        unref => |ptr| glib::gobject_ffi::g_param_spec_unref(ptr as *mut glib::gobject_ffi::GParamSpec),
    }
}


impl StaticType for ParamPath {
    fn static_type() -> glib::Type {
         unsafe { from_glib(ffi::gegl_param_path_get_type()) }
    }
}

impl ParamPath {}
