// This file was generated by gir (32b0f11) from gir-files (71d73f0)
// DO NOT EDIT

use FontDescription;
use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct FontFace(Object<ffi::PangoFontFace>);

    match fn {
        get_type => || ffi::pango_font_face_get_type(),
    }
}

impl FontFace {
    pub fn describe(&self) -> Option<FontDescription> {
        unsafe {
            from_glib_full(ffi::pango_font_face_describe(self.to_glib_none().0))
        }
    }

    pub fn get_face_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::pango_font_face_get_face_name(self.to_glib_none().0))
        }
    }

    pub fn is_synthesized(&self) -> bool {
        unsafe {
            from_glib(ffi::pango_font_face_is_synthesized(self.to_glib_none().0))
        }
    }

    //pub fn list_sizes(&self, sizes: /*Unimplemented*/Option<CArray TypeId { ns_id: 0, id: 14 }>) -> i32 {
    //    unsafe { TODO: call ffi::pango_font_face_list_sizes() }
    //}
}
