// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../../gir/gir-files
// DO NOT EDIT

use crate::{ffi};
use glib::{bitflags::bitflags,prelude::*,translate::*};

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[doc(alias = "AstalHyprlandFullscreen")]
    pub struct Fullscreen: u32 {
        #[doc(alias = "ASTAL_HYPRLAND_FULLSCREEN_CURRENT")]
        const CURRENT = ffi::ASTAL_HYPRLAND_FULLSCREEN_CURRENT as _;
        #[doc(alias = "ASTAL_HYPRLAND_FULLSCREEN_NONE")]
        const NONE = ffi::ASTAL_HYPRLAND_FULLSCREEN_NONE as _;
        #[doc(alias = "ASTAL_HYPRLAND_FULLSCREEN_MAXIMIZED")]
        const MAXIMIZED = ffi::ASTAL_HYPRLAND_FULLSCREEN_MAXIMIZED as _;
        #[doc(alias = "ASTAL_HYPRLAND_FULLSCREEN_FULLSCREEN")]
        const FULLSCREEN = ffi::ASTAL_HYPRLAND_FULLSCREEN_FULLSCREEN as _;
    }
}

#[doc(hidden)]
impl IntoGlib for Fullscreen {
    type GlibType = ffi::AstalHyprlandFullscreen;

    #[inline]
    fn into_glib(self) -> ffi::AstalHyprlandFullscreen {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::AstalHyprlandFullscreen> for Fullscreen {
    #[inline]
    unsafe fn from_glib(value: ffi::AstalHyprlandFullscreen) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for Fullscreen {
                #[inline]
    #[doc(alias = "astal_hyprland_fullscreen_get_type")]
   fn static_type() -> glib::Type {
                    unsafe { from_glib(ffi::astal_hyprland_fullscreen_get_type()) }
                }
            }

impl glib::HasParamSpec for Fullscreen {
                type ParamSpec = glib::ParamSpecFlags;
                type SetValue = Self;
                type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;
    
                fn param_spec_builder() -> Self::BuilderFn {
                    Self::ParamSpec::builder
                }
}

impl glib::value::ValueType for Fullscreen {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for Fullscreen {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for Fullscreen {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<Fullscreen> for glib::Value {
    #[inline]
    fn from(v: Fullscreen) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

