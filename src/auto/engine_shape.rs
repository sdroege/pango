// This file was generated by gir (https://github.com/gtk-rs/gir @ 6855214)
// from gir-files (https://github.com/gtk-rs/gir-files @ 3fde76b)
// DO NOT EDIT

use ffi;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct EngineShape(Object<ffi::PangoEngineShape, ffi::PangoEngineShapeClass>);

    match fn {
        get_type => || ffi::pango_engine_shape_get_type(),
    }
}

impl EngineShape {}
