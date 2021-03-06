// This file was generated by gir (9f70278) from gir-files (0bcaef9)
// DO NOT EDIT

use ffi;
use glib::Type;
use glib::StaticType;
use glib::value::{Value, SetValue, FromValue, FromValueOptional};
use gobject_ffi;
use glib::translate::*;

bitflags! {
    pub struct FontMask: u32 {
        const FONT_MASK_FAMILY = 1;
        const FONT_MASK_STYLE = 2;
        const FONT_MASK_VARIANT = 4;
        const FONT_MASK_WEIGHT = 8;
        const FONT_MASK_STRETCH = 16;
        const FONT_MASK_SIZE = 32;
        const FONT_MASK_GRAVITY = 64;
    }
}

#[doc(hidden)]
impl ToGlib for FontMask {
    type GlibType = ffi::PangoFontMask;

    fn to_glib(&self) -> ffi::PangoFontMask {
        ffi::PangoFontMask::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::PangoFontMask> for FontMask {
    fn from_glib(value: ffi::PangoFontMask) -> FontMask {
        FontMask::from_bits_truncate(value.bits())
    }
}

impl StaticType for FontMask {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::pango_font_mask_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for FontMask {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for FontMask {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(ffi::PangoFontMask::from_bits_truncate(gobject_ffi::g_value_get_flags(value.to_glib_none().0)))
    }
}

impl SetValue for FontMask {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib().bits())
    }
}

