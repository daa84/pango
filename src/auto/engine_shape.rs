// This file was generated by gir (9f70278) from gir-files (0bcaef9)
// DO NOT EDIT

use ffi;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct EngineShape(Object<ffi::PangoEngineShape>);

    match fn {
        get_type => || ffi::pango_engine_shape_get_type(),
    }
}

impl EngineShape {}
